use std::fmt::{Display, Formatter};

use crate::{
    CloudLayer, CloudType, Clouds, ColourCode, CompassDirection, Data, Kind, MetarError, Pressure,
    RunwayCondition, RunwayVisualRange, SeaCondition, Time, Trend, VerticalVisibility, Visibility,
    Weather, WeatherCondition, Wind, WindDirection, WindSpeed, WindshearWarnings,
    parsers::{any_whitespace, some_whitespace, temperature},
    traits::Parsable,
};
use chumsky::prelude::*;

#[derive(PartialEq, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A complete METAR
pub struct Metar {
    /// The station making the METAR measurement
    pub station: String,
    /// The measurement time
    pub time: Time,
    /// The kind of METAR, i.e. Normal, Automatic or Correction
    pub kind: Kind,
    /// The current wind information
    pub wind: Wind,
    /// The current visibility
    pub visibility: Data<Visibility>,
    /// If the visibility is reduced further in a specific direction,
    /// that will be covered here. If the direction is [`None`], it is
    /// reduced in a nonspecific direction.
    pub reduced_directional_visibility: Vec<(Option<CompassDirection>, Data<Visibility>)>,
    /// Specific visual ranges for runways
    pub rvr: Vec<RunwayVisualRange>,
    /// The current clouds
    pub clouds: Clouds,
    /// The current clouds
    pub cloud_layers: Vec<CloudLayer>,
    /// The current vertical visibility, in feet
    pub vert_visibility: Option<VerticalVisibility>,
    /// The current weather conditions
    pub weather: Data<Vec<Weather>>,
    /// The current temperature
    pub temperature: Data<f32>,
    /// The current dewpoint
    pub dewpoint: Data<f32>,
    /// The current air pressure
    pub pressure: Pressure,
    /// Military airport colour code
    pub colour_code: Option<Data<ColourCode>>,
    /// Additional recent weather conditions
    pub recent_weather: Vec<Data<Vec<WeatherCondition>>>,
    /// Windshear warnings
    pub windshear_warnings: Option<WindshearWarnings>,
    /// Sea surface condition
    pub sea_condition: Option<SeaCondition>,
    /// The condition of runways
    pub runway_conditions: Vec<RunwayCondition>,
    /// Trends of the weather changing in the near future
    pub trends: Vec<Trend>,
    /// Clouds in the vicinity may be specified separately
    pub clouds_in_vicinity: Vec<(Vec<CompassDirection>, Data<CloudType>)>,
    /// Remarks added on to the METAR
    pub remarks: Option<String>,
}

impl Parsable for Metar {
    #[allow(clippy::too_many_lines)]
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<MetarError<'src>>> {
        fn method<'src>() -> impl Parser<'src, &'src str, Kind, extra::Err<crate::MetarError<'src>>>
        {
            choice((
                just("AUTO")
                    .map(|_| Kind::Automatic)
                    .then_ignore(some_whitespace()),
                just("COR")
                    .map(|_| Kind::Correction)
                    .then_ignore(some_whitespace()),
                just("CCA")
                    .map(|_| Kind::Correction)
                    .then_ignore(some_whitespace()),
                empty().map(|()| Kind::Normal),
            ))
        }
        let station = regex("[A-Z0-9]{4}");

        group((
            just("METAR")
                .then_ignore(some_whitespace())
                .map(|_| ())
                .or(empty()),
            method(),
            station.then_ignore(some_whitespace()),
            Time::parser().then_ignore(some_whitespace()),
            method(),
            choice((
                Wind::parser(),
                empty().map(|()| Wind::Present {
                    dir: WindDirection::Heading(Data::Unknown),
                    speed: WindSpeed::Knots {
                        speed: Data::Unknown,
                        gusting: None,
                    },
                    varying: None,
                }),
            )),
            choice((
                Data::parser_inline(4, Visibility::parser()).then_ignore(some_whitespace()),
                empty().map(|()| Data::Unknown),
            )),
            <(Option<CompassDirection>, Data<Visibility>) as Parsable>::parser()
                .separated_by(some_whitespace())
                .allow_trailing()
                .collect::<Vec<_>>(),
            RunwayVisualRange::parser()
                .separated_by(some_whitespace())
                .allow_trailing()
                .collect::<Vec<_>>(),
            choice((
                just("SKC")
                    .map(|_| (Data::Known(vec![]), None, Clouds::NoCloudDetected, vec![]))
                    .then_ignore(some_whitespace()),
                just("CLR")
                    .map(|_| (Data::Known(vec![]), None, Clouds::NoCloudDetected, vec![]))
                    .then_ignore(some_whitespace()),
                group((
                    Data::parser_inline(
                        2,
                        Weather::parser()
                            .separated_by(some_whitespace())
                            .collect::<Vec<_>>(),
                    )
                    .then_ignore(some_whitespace())
                    .or(empty().map(|()| Data::Known(vec![]))),
                    VerticalVisibility::parser()
                        .map(Some)
                        .then_ignore(some_whitespace())
                        .or(empty().map(|()| None)),
                    Clouds::parser(),
                    CloudLayer::parser()
                        .separated_by(some_whitespace())
                        .allow_trailing()
                        .collect::<Vec<_>>(),
                ))
                .map(|(wx, vvis, clouds, layers)| (wx, vvis, clouds, layers)),
                empty().map(|()| (Data::Known(vec![]), None, Clouds::NoCloudDetected, vec![])),
            )),
            group((
                Data::parser_inline(2, temperature()),
                just("/"),
                Data::parser_inline(2, temperature()).or(empty().map(|()| Data::Unknown)),
            ))
            .map(|(temp, _, dewp)| (temp, dewp))
            .then_ignore(some_whitespace())
            .or(empty().map(|()| (Data::Unknown, Data::Unknown))),
            Pressure::parser()
                .then_ignore(some_whitespace())
                .or(empty().map(|()| Pressure::Hectopascals(Data::Unknown))),
            choice((
                just("RE")
                    .then(Data::parser_inline(
                        2,
                        WeatherCondition::parser()
                            .repeated()
                            .at_least(1)
                            .collect::<Vec<_>>(),
                    ))
                    .map(|(_, wx)| wx)
                    .separated_by(some_whitespace())
                    .collect::<Vec<_>>()
                    .then_ignore(some_whitespace()),
                empty().map(|()| vec![]),
            )),
            Data::<ColourCode>::parser()
                .map(Some)
                .then_ignore(some_whitespace())
                .or(empty().map(|()| None)),
            WindshearWarnings::parser()
                .map(Some)
                .then_ignore(some_whitespace())
                .or(empty().map(|()| None)),
            RunwayCondition::parser()
                .separated_by(some_whitespace())
                .allow_trailing()
                .collect::<Vec<_>>(),
            SeaCondition::parser()
                .map(Some)
                .then_ignore(some_whitespace())
                .or(empty().map(|()| None)),
            Trend::parser()
                .separated_by(any_whitespace())
                .allow_trailing()
                .collect::<Vec<_>>(),
            <(Vec<CompassDirection>, Data<CloudType>) as Parsable>::parser()
                .separated_by(some_whitespace())
                .allow_trailing()
                .collect::<Vec<_>>(),
            just("RMK")
                .then(none_of("=").repeated().collect::<String>())
                .map(|(_, s)| Some(s.trim().to_string()))
                .or(empty().map(|()| None)),
            any_whitespace(),
            choice((just("=").map(|_| ()), empty().map(|()| ()))),
        ))
        .map(
            |(
                (),
                early_kind,
                station,
                time,
                kind,
                wind,
                visibility,
                reduced_directional_visibility,
                rvr,
                (weather, vert_visibility, clouds, cloud_layers),
                (temperature, dewpoint),
                pressure,
                recent_weather,
                colour_code,
                windshear_warnings,
                runway_conditions,
                sea_condition,
                trends,
                clouds_in_vicinity,
                remarks,
                (),
                (),
            )| {
                Metar {
                    station: station.to_string(),
                    time,
                    kind: if early_kind == Kind::Normal {
                        kind
                    } else {
                        early_kind
                    },
                    wind,
                    visibility,
                    reduced_directional_visibility,
                    rvr,
                    weather,
                    vert_visibility,
                    clouds,
                    cloud_layers,
                    temperature,
                    dewpoint,
                    pressure,
                    colour_code,
                    recent_weather,
                    windshear_warnings,
                    sea_condition,
                    runway_conditions,
                    trends,
                    clouds_in_vicinity,
                    remarks,
                }
            },
        )
    }
}

impl Metar {
    /// Parse a string into a METAR.
    ///
    /// # Errors
    ///
    /// Returns a [`MetarError`] if parsing failed.
    pub fn parse(data: &str) -> Result<Self, Vec<MetarError<'_>>> {
        <Metar as Parsable>::parse(data).map_err(|v| {
            v.into_iter()
                .map(|mut e| {
                    e.string = data;
                    e
                })
                .collect::<Vec<_>>()
        })
    }
}

impl Display for Metar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.station)?;
        f.write_str(" ")?;

        write!(f, "{} ", self.time)?;
        self.kind.fmt(f)?;
        write!(f, "{} ", self.wind)?;

        write!(f, "{} ", self.visibility.to_opt_string(4))?;

        for (dir, reduced_vis) in &self.reduced_directional_visibility {
            if let Some(dir) = dir {
                write!(f, "{dir}")?;
            }
            write!(f, "{} ", reduced_vis.to_opt_string(4))?;
        }

        for rvr in &self.rvr {
            write!(f, "{rvr} ")?;
        }

        match &self.weather {
            Data::Known(wx) => {
                let wx_str = wx
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" ");
                if !wx_str.is_empty() {
                    write!(f, "{wx_str} ")?;
                }
            }
            Data::Unknown => f.write_str("// ")?,
        }

        if let Some(vv) = &self.vert_visibility {
            write!(f, "{vv} ")?;
        }

        if self.visibility != Data::Known(Visibility::CAVOK) && self.clouds != Clouds::CloudLayers {
            write!(f, "{} ", self.clouds)?;
        }
        for layer in &self.cloud_layers {
            write!(f, "{layer} ")?;
        }

        write!(
            f,
            "{}{}/{}{}",
            if let Data::Known(t) = self.temperature
                && t.is_sign_negative()
            {
                "M"
            } else {
                ""
            },
            self.temperature
                .map(|temp| format!("{:02.0}", f32::abs(temp)))
                .to_opt_string(2),
            if let Data::Known(dp) = self.dewpoint
                && dp.is_sign_negative()
            {
                "M"
            } else {
                ""
            },
            self.dewpoint
                .map(|dp| format!("{:02.0}", f32::abs(dp)))
                .to_opt_string(2)
        )?;

        write!(f, " {}", self.pressure)?;

        for wx in &self.recent_weather {
            f.write_str(" RE")?;
            if let Data::Known(wx_conditions) = wx {
                for wx_condition in wx_conditions {
                    write!(f, "{wx_condition}")?;
                }
            }
        }

        if let Some(colour) = &self.colour_code {
            write!(f, " {}", colour.to_opt_string(3))?;
        }

        for trend in &self.trends {
            write!(f, " {trend}")?;
        }

        if let Some(remarks) = &self.remarks {
            write!(f, " RMK {remarks}")?;
        }

        Ok(())
    }
}
