use super::types::Data::*;
use super::types::*;
use super::Metar;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/metar.pest"]
pub struct MetarParser;

impl super::MetarError {
    fn from_pest_err(e: pest::error::Error<Rule>, data: String) -> Self {
        match e.location {
            pest::error::InputLocation::Pos(p) => Self {
                string: data,
                start: p,
                length: 0,
                variant: e.variant,
            },
            pest::error::InputLocation::Span((s, end)) => Self {
                string: data,
                start: s,
                length: end - s,
                variant: e.variant,
            },
        }
    }
}

pub(crate) fn parse(data: String) -> Result<super::Metar, super::MetarError> {
    let res = MetarParser::parse(Rule::metar, &data);
    res.map(|mut pairs| {
        let metar_pair = pairs.next().unwrap();
        metar_pair.into()
    })
    .map_err(|e| super::MetarError::from_pest_err(e, data))
}

impl<'i> From<Pair<'i, Rule>> for Metar {
    fn from(pair: Pair<'i, Rule>) -> Self {
        let mut metar = Metar {
            station: "ZZZZ".to_owned(),
            time: Time {
                date: 0,
                hour: 0,
                minute: 0,
            },
            is_auto: false,
            wind: Wind {
                dir: Unknown,
                speed: Unknown,
                varying: None,
                gusting: None,
                unit: WindUnit::Knots,
            },
            visibility: Unknown,
            rvr: vec![],
            clouds: Known(Clouds::NoCloudDetected),
            vert_visibility: None,
            weather: Data::Known(vec![]),
            temperature: Unknown,
            dewpoint: Unknown,
            // Unknown QNH is Q////, i.e. handled below, inHg is simply omitted so handled here
            pressure: Pressure::InchesOfMercury(Unknown),
            recent_weather: None,
            remarks: None,
            trend: vec![],
        };

        assert_eq!(pair.as_rule(), Rule::metar);
        for part in pair.into_inner() {
            match part.as_rule() {
                Rule::station => metar.station = part.as_str().to_owned(),
                Rule::observation_time => metar.time = Time::from(part),
                Rule::auto => metar.is_auto = true,
                Rule::wind => metar.wind = Wind::from(part),
                Rule::wind_varying => {
                    let mut hdgs = part.into_inner();
                    let from = hdgs.next().unwrap().as_str().parse().unwrap();
                    let to = hdgs.next().unwrap().as_str().parse().unwrap();
                    metar.wind.varying = Some((from, to));
                }
                Rule::atmos_condition => {
                    let atmos = AtmosphericConditions::from(part);
                    metar.visibility = atmos.visibility;
                    metar.weather = atmos.weather;
                    metar.clouds = atmos.clouds;
                    metar.vert_visibility = atmos.vert_visibility;
                    metar.rvr = atmos.rvr;
                }
                Rule::temperatures => {
                    let mut temps = part.into_inner();
                    let temp = temps.next().unwrap();
                    let dewp = temps.next().unwrap();
                    metar.temperature = match temp.as_str() {
                        "//" => Unknown,
                        v => {
                            if let Some(stripped) = v.strip_prefix('M') {
                                Known(-stripped.parse::<i32>().unwrap())
                            } else {
                                Known(v.parse().unwrap())
                            }
                        }
                    };
                    metar.dewpoint = match dewp.as_str() {
                        "//" => Unknown,
                        v => {
                            if let Some(stripped) = v.strip_prefix('M') {
                                Known(-stripped.parse::<i32>().unwrap())
                            } else {
                                Known(v.parse().unwrap())
                            }
                        }
                    };
                }
                Rule::pressure => metar.pressure = Pressure::from(part),
                Rule::recents => {
                    metar.recent_weather =
                        Some(part.into_inner().map(WeatherCondition::from).collect())
                }
                Rule::trend => {
                    for trend in part.into_inner() {
                        metar.trend.push(Trend::from(trend));
                    }
                }
                Rule::remarks => metar.remarks = Some(part.as_str().to_owned()),
                _ => (),
            }
        }

        metar
    }
}

struct AtmosphericConditions {
    visibility: Data<Visibility>,
    clouds: Data<Clouds>,
    weather: Data<Vec<Weather>>,
    vert_visibility: Option<VertVisibility>,
    rvr: Vec<RunwayVisualRange>,
}
impl Default for AtmosphericConditions {
    fn default() -> Self {
        Self {
            visibility: Unknown,
            clouds: Known(Clouds::CloudLayers(vec![])),
            weather: Known(vec![]),
            vert_visibility: None,
            rvr: vec![],
        }
    }
}
impl<'i> From<Pair<'i, Rule>> for AtmosphericConditions {
    fn from(pair: Pair<'i, Rule>) -> Self {
        let mut res = Self::default();
        if pair.as_str() == "CAVOK" {
            res.visibility = Known(Visibility::CAVOK);
            res.clouds = Known(Clouds::NoCloudDetected);
        } else if pair.as_str() == "SKC" {
            res.clouds = Known(Clouds::NoCloudDetected);
        } else {
            for c in pair.into_inner() {
                match c.as_rule() {
                    Rule::visibility_horizontal => {
                        if c.as_str() == "////" {
                            continue;
                        } else if c.as_str().ends_with("SM") {
                            // Statute miles
                            let mut total = 0f32;
                            let dist = &c.as_str()[..c.as_str().len() - 2];
                            let pairs = dist.split(' ');
                            for p in pairs {
                                if p.contains('/') {
                                    let mut pairs = p.split('/');
                                    let n: f32 = pairs.next().unwrap().parse().unwrap();
                                    let d: f32 = pairs.next().unwrap().parse().unwrap();
                                    total += n / d;
                                } else {
                                    total += p.parse::<f32>().unwrap();
                                }
                            }
                            res.visibility = Known(Visibility::StatuteMiles(total));
                        } else {
                            // Metres
                            res.visibility = Known(Visibility::Metres(c.as_str().parse().unwrap()));
                        }
                    }
                    Rule::visibility_vertical => {
                        let data = &c.as_str()[2..];
                        match data {
                            "///" => {
                                res.vert_visibility = Some(VertVisibility::ReducedByUnknownAmount)
                            }
                            _ => {
                                res.vert_visibility =
                                    Some(VertVisibility::Distance(data.parse().unwrap()))
                            }
                        }
                    }
                    Rule::wx => {
                        if c.as_str().starts_with("//") {
                            res.weather = Unknown
                        } else {
                            match &mut res.weather {
                                Known(wx) => wx.push(Weather::from(c)),
                                Unknown => unreachable!(),
                            }
                        }
                    }
                    Rule::no_clouds_detected => res.clouds = Known(Clouds::NoCloudDetected),
                    Rule::cloud => {
                        if let Known(Clouds::CloudLayers(cls)) = &mut res.clouds {
                            cls.push(CloudLayer::from(c));
                        } else {
                            res.clouds = Known(Clouds::CloudLayers(vec![CloudLayer::from(c)]));
                        }
                    }
                    Rule::rvr => res.rvr.push(RunwayVisualRange::from(c)),
                    rule => unreachable!("{rule:?}"),
                }
            }
        }
        res
    }
}

impl<'i> From<Pair<'i, Rule>> for Time {
    fn from(pair: Pair<'i, Rule>) -> Self {
        let mut time = Time {
            date: 0,
            hour: 0,
            minute: 0,
        };
        assert_eq!(pair.as_rule(), Rule::observation_time);
        for part in pair.into_inner() {
            match part.as_rule() {
                Rule::observation_day => time.date = part.as_str().parse().unwrap(),
                Rule::observation_hour => time.hour = part.as_str().parse().unwrap(),
                Rule::observation_minute => time.minute = part.as_str().parse().unwrap(),
                _ => (),
            }
        }
        time
    }
}

impl<'i> From<Pair<'i, Rule>> for Wind {
    fn from(pair: Pair<'i, Rule>) -> Self {
        let mut wind = Wind {
            dir: Unknown,
            speed: Unknown,
            unit: WindUnit::Knots,
            varying: None,
            gusting: None,
        };
        assert_eq!(pair.as_rule(), Rule::wind);

        for part in pair.into_inner() {
            match part.as_rule() {
                Rule::wind_dir => {
                    wind.dir = match part.as_str() {
                        "///" => Unknown,
                        "VRB" => Known(WindDirection::Variable),
                        v => Known(WindDirection::Heading(v.parse().unwrap())),
                    };
                }
                Rule::wind_speed => {
                    let mut s = part.as_str();
                    if s == "//" {
                        break;
                    }
                    if s.starts_with('P') {
                        s = &s[1..];
                    }
                    wind.speed = Known(s.parse().unwrap());
                }
                Rule::wind_gusts => {
                    wind.gusting = Some(part.as_str()[1..].parse().unwrap());
                }
                Rule::wind_unit => {
                    let unit_s = part.as_str();
                    wind.unit = match unit_s {
                        "KT" => WindUnit::Knots,
                        "KPH" => WindUnit::KilometresPerHour,
                        "MPS" => WindUnit::MetresPerSecond,
                        _ => unreachable!(),
                    }
                }
                _ => (),
            }
        }

        wind
    }
}

impl<'i> From<Pair<'i, Rule>> for Weather {
    fn from(pair: Pair<'i, Rule>) -> Self {
        let mut wx = Weather {
            intensity: WeatherIntensity::Moderate,
            conditions: Vec::new(),
        };
        assert_eq!(pair.as_rule(), Rule::wx);
        for part in pair.into_inner() {
            match part.as_rule() {
                Rule::wx_intensity => {
                    wx.intensity = match part.as_str() {
                        "+" => WeatherIntensity::Heavy,
                        "-" => WeatherIntensity::Light,
                        "VC" => WeatherIntensity::InVicinity,
                        _ => unreachable!(),
                    }
                }
                Rule::wx_condition => {
                    wx.conditions.push(WeatherCondition::from(part));
                }
                _ => (),
            }
        }
        wx
    }
}

impl<'i> From<Pair<'i, Rule>> for WeatherCondition {
    fn from(pair: Pair<'i, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::wx_condition);
        match pair.as_str() {
            "MI" => WeatherCondition::Shallow,
            "PR" => WeatherCondition::Partial,
            "BC" => WeatherCondition::Patches,
            "DR" => WeatherCondition::LowDrifting,
            "BL" => WeatherCondition::Blowing,
            "SH" => WeatherCondition::Showers,
            "TS" => WeatherCondition::Thunderstorm,
            "FZ" => WeatherCondition::Freezing,
            "RA" => WeatherCondition::Rain,
            "DZ" => WeatherCondition::Drizzle,
            "SN" => WeatherCondition::Snow,
            "SG" => WeatherCondition::SnowGrains,
            "IC" => WeatherCondition::IceCrystals,
            "PL" => WeatherCondition::IcePellets,
            "GR" => WeatherCondition::Hail,
            "GS" => WeatherCondition::SnowPelletsOrSmallHail,
            "UP" => WeatherCondition::UnknownPrecipitation,
            "FG" => WeatherCondition::Fog,
            "VA" => WeatherCondition::VolcanicAsh,
            "BR" => WeatherCondition::Mist,
            "HZ" => WeatherCondition::Haze,
            "DU" => WeatherCondition::WidespreadDust,
            "FU" => WeatherCondition::Smoke,
            "SA" => WeatherCondition::Sand,
            "PY" => WeatherCondition::Spray,
            "SQ" => WeatherCondition::Squall,
            "PO" => WeatherCondition::Dust,
            "DS" => WeatherCondition::Duststorm,
            "SS" => WeatherCondition::Sandstorm,
            "FC" => WeatherCondition::FunnelCloud,
            _ => unreachable!(),
        }
    }
}

impl<'i> From<Pair<'i, Rule>> for CloudLayer {
    fn from(pair: Pair<'i, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::cloud);
        let mut density = "";
        let mut typ = CloudType::Normal;
        let mut floor = Unknown;

        for part in pair.into_inner() {
            match part.as_rule() {
                Rule::cloud_density => density = part.as_str(),
                Rule::cloud_type => {
                    match part.as_str() {
                        "///" => typ = CloudType::Unknown,
                        "CB" => typ = CloudType::Cumulonimbus,
                        "TCU" => typ = CloudType::ToweringCumulus,
                        _ => unreachable!(),
                    };
                }
                Rule::cloud_floor => match part.as_str() {
                    "///" => floor = Unknown,
                    _ => floor = Known(part.as_str().parse().unwrap()),
                },
                _ => (),
            }
        }

        match density {
            "///" => CloudLayer::Unknown(typ, floor),
            "FEW" => CloudLayer::Few(typ, floor),
            "SCT" => CloudLayer::Scattered(typ, floor),
            "BKN" => CloudLayer::Broken(typ, floor),
            "OVC" => CloudLayer::Overcast(typ, floor),
            _ => unreachable!(),
        }
    }
}

impl<'i> From<Pair<'i, Rule>> for Pressure {
    fn from(pair: Pair<'i, Rule>) -> Self {
        let s = pair.as_str();
        let data = &s[1..];

        if s.starts_with('Q') {
            // QNH
            if data == "////" {
                Pressure::Hectopascals(Unknown)
            } else {
                Pressure::Hectopascals(Known(data.parse().unwrap()))
            }
        } else if s.starts_with('A') {
            // inHg
            Pressure::InchesOfMercury(Known(data.parse::<f32>().unwrap() / 100f32))
        } else {
            unreachable!()
        }
    }
}

impl<'i> From<Pair<'i, Rule>> for Trend {
    fn from(pair: Pair<'i, Rule>) -> Self {
        match pair.as_rule() {
            Rule::no_significant_change => Trend::NoSignificantChange,
            Rule::tempo => {
                let mut tempo = pair.into_inner();
                let time_or_change = tempo.next().unwrap();
                let wx_change = if let Rule::wx_change_time = time_or_change.as_rule() {
                    // TODO parse change time
                    tempo.next().unwrap()
                } else {
                    time_or_change
                };

                Trend::Temporarily(WeatherChangeConditions::from(wx_change))
            }
            Rule::becoming => {
                let mut becoming = pair.into_inner();
                let time_or_change = becoming.next().unwrap();
                let wx_change = if let Rule::wx_change_time = time_or_change.as_rule() {
                    // TODO parse change time
                    becoming.next().unwrap()
                } else {
                    time_or_change
                };

                Trend::Becoming(WeatherChangeConditions::from(wx_change))
            }
            rule => unreachable!("{rule:?}"),
        }
    }
}

impl<'i> From<Pair<'i, Rule>> for WeatherChangeConditions {
    fn from(pair: Pair<'i, Rule>) -> Self {
        let mut wx_change = WeatherChangeConditions::default();
        for part in pair.into_inner() {
            match part.as_rule() {
                Rule::no_significant_weather => wx_change.no_significant_weather = true,
                Rule::wind => wx_change.wind = Some(Wind::from(part)),
                Rule::wind_varying => {
                    let mut hdgs = part.into_inner();
                    let from = hdgs.next().unwrap().as_str().parse().unwrap();
                    let to = hdgs.next().unwrap().as_str().parse().unwrap();
                    if let Some(wind) = &mut wx_change.wind {
                        wind.varying = Some((from, to));
                    }
                }
                Rule::atmos_condition => {
                    let atmos = AtmosphericConditions::from(part);
                    wx_change.visibility = atmos.visibility.into_option();
                    wx_change.weather = atmos.weather.into_option().unwrap_or(vec![]);
                    wx_change.clouds = atmos.clouds.into_option();
                }
                rule => unreachable!("{rule:?}"),
            }
        }

        wx_change
    }
}

impl<'i> From<Pair<'i, Rule>> for RunwayVisualRange {
    fn from(pair: Pair<'i, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::rvr);
        let mut rvr = pair.into_inner();
        let runway = rvr.next().unwrap().as_str().to_string();

        let value_or_range = rvr.next().unwrap();
        let (value, varying_to) = match value_or_range.as_rule() {
            Rule::rvr_visibility => (RvrValue::from(value_or_range), None),
            Rule::rvr_visibility_range => {
                let mut range = value_or_range.into_inner();

                (
                    RvrValue::from(range.next().unwrap()),
                    Some(RvrValue::from(range.next().unwrap())),
                )
            }
            rule => unreachable!("{rule:?}"),
        };
        let trend = rvr
            .next()
            .map_or(RvrTrend::NoChange, |trend| match trend.as_str() {
                "U" => RvrTrend::UpwardTendency,
                "D" => RvrTrend::DownwardTendency,
                "N" => RvrTrend::NoChange,
                other => unreachable!("{other}"),
            });

        RunwayVisualRange {
            runway,
            trend,
            value,
            varying_to,
        }
    }
}

impl<'i> From<Pair<'i, Rule>> for RvrValue {
    fn from(pair: Pair<'i, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::rvr_visibility);
        let val = pair.as_str();
        match &val[..1] {
            "P" => Self::GreaterThan(val[1..].parse().unwrap()),
            "M" => Self::LessThan(val[1..].parse().unwrap()),
            _ => Self::Exactly(val.parse().unwrap()),
        }
    }
}
