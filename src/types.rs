use std::fmt;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
/// Data that is provided in a metar which might be unknown.
/// Note that this differs from an `Option<T>` field which is used when data
/// might not be given at all. In the cases where `Data<T>` is used, data is
/// usually given but has been replaced in the METAR by slashes, indicating
/// that it is not known.
pub enum Data<T> {
    /// The data is known and given
    Known(T),
    /// The data isn't or cannot be known
    Unknown,
}

impl<T> Data<T> {
    /// Unwraps the inner data type, panics otherwise
    pub fn unwrap(&self) -> &T {
        match self {
            Data::Known(v) => v,
            Data::Unknown => panic!("cannot unwrap unknown data"),
        }
    }

    /// Mutably unwraps the inner data type, panics otherwise
    pub fn unwrap_mut(&mut self) -> &mut T {
        match self {
            Data::Known(v) => v,
            Data::Unknown => panic!("cannot unwrap unknown data"),
        }
    }

    /// Handle as Option to be able to use its API
    pub fn as_option(&self) -> Option<&T> {
        match self {
            Data::Known(v) => Some(v),
            Data::Unknown => None,
        }
    }

    /// Convert to Option to be able to use its API
    pub fn into_option(self) -> Option<T> {
        match self {
            Data::Known(v) => Some(v),
            Data::Unknown => None,
        }
    }
}

impl<T: fmt::Display> Data<T> {
    /// Returns a String replacing unknown values with n*"/"
    pub fn to_opt_string(&self, n: usize) -> String {
        match self {
            Data::Known(w) => w.to_string(),
            Data::Unknown => "/".repeat(n),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// A struct to store time as it is represented in a METAR
pub struct Time {
    /// The date the METAR was made
    pub date: u8,
    /// The hour the METAR was made
    pub hour: u8,
    /// The minute the METAR was made
    pub minute: u8,
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}{:02}{:02}Z", self.date, self.hour, self.minute)
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
/// A representation of wind direction
pub enum WindDirection {
    /// A heading defining wind direction
    Heading(u32),
    /// Wind direction is variable
    Variable,
}

impl fmt::Display for WindDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WindDirection::Heading(h) => write!(f, "{h:03}"),
            WindDirection::Variable => f.write_str("VRB"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
/// A representation of the wind unit
pub enum WindUnit {
    /// Nautical miles per hour
    Knots,
    /// Kilometres per hour
    KilometresPerHour,
    /// Metres per second
    MetresPerSecond,
}

impl fmt::Display for WindUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            WindUnit::Knots => "KT",
            WindUnit::KilometresPerHour => "KPH",
            WindUnit::MetresPerSecond => "MPS",
        })
    }
}

#[derive(PartialEq, Clone, Debug)]
/// Horizontal visibility
pub enum Visibility {
    /// Visibility OK
    CAVOK,
    /// Metres
    Metres(u16),
    /// Statute miles, usually used in the US
    StatuteMiles(f32),
}

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Visibility::CAVOK => f.write_str("CAVOK"),
            Visibility::Metres(m) => write!(f, "{m:04}"),
            // FIXME fractions
            Visibility::StatuteMiles(sm) => write!(f, "{sm}SM"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
/// Runway Visual Range
pub struct RunwayVisualRange {
    /// Runway for which the Runway Visual Range is applicable
    pub runway: String,
    /// Trend of the Runway Visual Range
    pub trend: RvrTrend,
    /// Measured value of the Runway Visual Range
    pub value: RvrValue,
    /// Optionally a second value to which value varies to
    pub varying_to: Option<RvrValue>,
}

impl fmt::Display for RunwayVisualRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "R{}/{}{}{}",
            self.runway,
            self.value,
            self.varying_to
                .as_ref()
                .map_or(String::new(), |v| format!("V{v}")),
            self.trend
        )
    }
}

#[derive(PartialEq, Clone, Debug)]
/// Trend of the RVR
pub enum RvrTrend {
    /// Improving
    UpwardTendency,
    /// No significant change
    NoChange,
    /// Worsening
    DownwardTendency,
}

impl fmt::Display for RvrTrend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            RvrTrend::UpwardTendency => "U",
            RvrTrend::NoChange => "N",
            RvrTrend::DownwardTendency => "D",
        })
    }
}

#[derive(PartialEq, Clone, Debug)]
/// Value of the RVR
pub enum RvrValue {
    /// Greater than the value due to capability of measuring instruments
    GreaterThan(u32),
    /// The value measured
    Exactly(u32),
    /// Lesser than the value due to capability of measuring instruments
    LessThan(u32),
}

impl fmt::Display for RvrValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GreaterThan(v) => write!(f, "P{v:04}"),
            Self::Exactly(v) => write!(f, "{v:04}"),
            Self::LessThan(v) => write!(f, "M{v:04}"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
/// Measured air pressure
pub enum Pressure {
    /// Pressure in hectopascals
    Hectopascals(Data<u16>),
    /// Pressure in inches of mercury (inHg)
    InchesOfMercury(Data<f32>),
}

impl fmt::Display for Pressure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pressure::Hectopascals(hpa) => write!(f, "Q{:0>4}", hpa.to_opt_string(4)),
            Pressure::InchesOfMercury(Data::Known(inhg)) => write!(f, "A{:04.0}", inhg * 100.0),
            Pressure::InchesOfMercury(Data::Unknown) => Ok(()),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// Vertical visibility measurement
pub enum VertVisibility {
    /// A distance of vertical visibility
    Distance(u32),
    /// The vertical visibility value is present, so is reduced, but by an amount that hasn't or
    /// cannot be measured
    ReducedByUnknownAmount,
}

impl fmt::Display for VertVisibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VertVisibility::Distance(d) => write!(f, "VV{d:0>3}"),
            VertVisibility::ReducedByUnknownAmount => f.write_str("VV///"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
/// Cloud state
pub enum Clouds {
    /// No cloud was detected, also set for CAVOK
    NoCloudDetected,
    /// No significant cloud was detected below 5000ft
    NoSignificantCloud,
    /// Layers of cloud, described elsewhere
    CloudLayers(Vec<CloudLayer>),
}

impl fmt::Display for Clouds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Clouds::NoCloudDetected => f.write_str("NCD"),
            Clouds::NoSignificantCloud => f.write_str("NSC"),
            Clouds::CloudLayers(cls) => f.write_str(
                &cls.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" "),
            ),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
/// Cloud cover
pub enum CloudLayer {
    /// Few clouds (1/8)
    Few(CloudType, Data<u32>),
    /// Scattered cloud cover (3/8)
    Scattered(CloudType, Data<u32>),
    /// Broken cloud cover (5/8)
    Broken(CloudType, Data<u32>),
    /// Overcast cloud cover (7/8)
    Overcast(CloudType, Data<u32>),
    /// Cloud cover of an unknown density
    Unknown(CloudType, Data<u32>),
}

impl fmt::Display for CloudLayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CloudLayer::Few(cloud_type, height) => {
                write!(f, "FEW{:0>3}{}", height.to_opt_string(3), cloud_type)
            }
            CloudLayer::Scattered(cloud_type, height) => {
                write!(f, "SCT{:0>3}{}", height.to_opt_string(3), cloud_type)
            }
            CloudLayer::Broken(cloud_type, height) => {
                write!(f, "BKN{:0>3}{}", height.to_opt_string(3), cloud_type)
            }
            CloudLayer::Overcast(cloud_type, height) => {
                write!(f, "OVC{:0>3}{}", height.to_opt_string(3), cloud_type)
            }
            CloudLayer::Unknown(cloud_type, height) => {
                write!(f, "///{:0>3}{}", height.to_opt_string(3), cloud_type)
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
/// A cloud type description
pub enum CloudType {
    /// A normal cloud
    Normal,
    /// A cumulonimbus cloud
    Cumulonimbus,
    /// A towering cumulus cloud
    ToweringCumulus,
    /// An unknown cloud type
    Unknown,
}

impl fmt::Display for CloudType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            CloudType::Normal => "",
            CloudType::Cumulonimbus => "CB",
            CloudType::ToweringCumulus => "TCU",
            CloudType::Unknown => "///",
        })
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// A weather information block
pub struct Weather {
    /// The intensity of this weather block
    pub intensity: WeatherIntensity,
    /// The weather condition/s this block describes.
    pub conditions: Vec<WeatherCondition>,
}

impl fmt::Display for Weather {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.intensity,
            self.conditions
                .iter()
                .map(ToString::to_string)
                .collect::<String>()
        )
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// Intensity of weather
pub enum WeatherIntensity {
    /// Light (-)
    Light,
    /// Moderate (no prefix)
    Moderate,
    /// Heavy (+)
    Heavy,
    /// In the vicinity (VC)
    InVicinity,
    // /// Recent (RE)
    // Recent,
}

impl fmt::Display for WeatherIntensity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            WeatherIntensity::Light => "-",
            WeatherIntensity::Moderate => "",
            WeatherIntensity::Heavy => "+",
            WeatherIntensity::InVicinity => "VC",
        })
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// Descriptor of weather
pub enum WeatherCondition {
    /// Descriptor - Shallow (MI)
    Shallow,
    /// Descriptor - Partial (PR)
    Partial,
    /// Descriptor - Patches (BC)
    Patches,
    /// Descriptor - Low drifting (DR)
    LowDrifting,
    /// Descriptor - Blowing (BL)
    Blowing,
    /// Descriptor - Showers (SH)
    Showers,
    /// Descriptor - Thunderstorm (TS)
    Thunderstorm,
    /// Descriptor - Freezing (FZ)
    Freezing,
    /// Precipitation - Rain (RA)
    Rain,
    /// Precipitation - Drizzle (DZ)
    Drizzle,
    /// Precipitation - Snow (SN)
    Snow,
    /// Precipitation - Snow Grains (SG)
    SnowGrains,
    /// Precipitation - Ice Crystals (IC)
    IceCrystals,
    /// Precipitation - Ice pellets (PL)
    IcePellets,
    /// Precipitation - Hail (including small hail in the US) (GR)
    Hail,
    /// Precipitation - Snow Pellets and/or Small Hail (except in US) (GS)
    SnowPelletsOrSmallHail,
    /// Precipitation - Unknown precipitation (UP)
    UnknownPrecipitation,
    /// Obscuration - Fog (FG)
    Fog,
    /// Obscuration - Volcanic Ash (VA)
    VolcanicAsh,
    /// Obscuration - Mist (BR)
    Mist,
    /// Obscuration - Haze (HZ)
    Haze,
    /// Obscuration - Widespread dust (DU)
    WidespreadDust,
    /// Obscuration - Smoke (FU)
    Smoke,
    /// Obscuration - Sand (SA)
    Sand,
    /// Obscuration - Spray (PY)
    Spray,
    /// Other - Squall (SQ)
    Squall,
    /// Other - Dust or Sand Whirls (PO)
    Dust,
    /// Other - Duststorm (DS)
    Duststorm,
    /// Other - Sandstorm (SS)
    Sandstorm,
    /// Other - Funnel Cloud (FC)
    FunnelCloud,
}

impl fmt::Display for WeatherCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            WeatherCondition::Shallow => "MI",
            WeatherCondition::Partial => "PR",
            WeatherCondition::Patches => "BC",
            WeatherCondition::LowDrifting => "DR",
            WeatherCondition::Blowing => "BL",
            WeatherCondition::Showers => "SH",
            WeatherCondition::Thunderstorm => "TS",
            WeatherCondition::Freezing => "FZ",
            WeatherCondition::Rain => "RA",
            WeatherCondition::Drizzle => "DZ",
            WeatherCondition::Snow => "SN",
            WeatherCondition::SnowGrains => "SG",
            WeatherCondition::IceCrystals => "IC",
            WeatherCondition::IcePellets => "PL",
            WeatherCondition::Hail => "GR",
            WeatherCondition::SnowPelletsOrSmallHail => "GS",
            WeatherCondition::UnknownPrecipitation => "UP",
            WeatherCondition::Fog => "FG",
            WeatherCondition::VolcanicAsh => "VA",
            WeatherCondition::Mist => "BR",
            WeatherCondition::Haze => "HZ",
            WeatherCondition::WidespreadDust => "DU",
            WeatherCondition::Smoke => "FU",
            WeatherCondition::Sand => "SA",
            WeatherCondition::Spray => "PY",
            WeatherCondition::Squall => "SQ",
            WeatherCondition::Dust => "PO",
            WeatherCondition::Duststorm => "DS",
            WeatherCondition::Sandstorm => "SS",
            WeatherCondition::FunnelCloud => "FC",
        })
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// Wind information.
pub struct Wind {
    /// The wind direction, in degrees
    pub dir: Data<WindDirection>,
    /// The current wind speed
    pub speed: Data<u32>,
    /// The direction the wind may be varying between, smaller always comes first
    pub varying: Option<(u32, u32)>,
    /// The gusting speed of the wind
    pub gusting: Option<u32>,
    /// The unit of the wind
    pub unit: WindUnit,
}

impl fmt::Display for Wind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.dir.to_opt_string(3))?;
        f.write_str(
            &self
                .speed
                .as_option()
                .map_or("//".to_string(), |v| format!("{v:0>2}")),
        )?;
        if let Some(gusts) = self.gusting {
            write!(f, "G{gusts}")?;
        }
        write!(f, "{}", self.unit)?;
        if let Some((from, to)) = self.varying {
            write!(f, " {from:0>3}V{to:0>3}")?
        }

        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug)]
/// Trend of weather
pub enum Trend {
    /// No significant change
    NoSignificantChange,
    /// Temporarily changing for less than an hour
    Temporarily(WeatherChangeConditions),
    /// A permanent change in the weather conditions
    Becoming(WeatherChangeConditions),
}

impl fmt::Display for Trend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Trend::NoSignificantChange => f.write_str("NOSIG"),
            Trend::Temporarily(wx) => write!(f, "TEMPO{wx}"),
            Trend::Becoming(wx) => write!(f, "BECMG{wx}"),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Default)]
/// Conditions the weather will change to
pub struct WeatherChangeConditions {
    /// When the change will occur
    pub weather_change_time: Option<WeatherChangeTime>,
    /// If there will be no significant weather
    pub no_significant_weather: bool,
    /// The wind information the weather will change to
    pub wind: Option<Wind>,
    /// The visibility information the weather will change to
    pub visibility: Option<Visibility>,
    /// The cloud information the weather will change to
    pub clouds: Option<Clouds>,
    /// The military weather colour code
    pub colour_code: Option<ColourCode>,
    /// The weather phenomena the conditions will change to
    pub weather: Vec<Weather>,
}

impl fmt::Display for WeatherChangeConditions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(time) = &self.weather_change_time {
            write!(f, " {time}")?;
        }
        if self.no_significant_weather {
            f.write_str(" NSW")?;
        }
        if let Some(wind) = &self.wind {
            write!(f, " {wind}")?;
        }
        if let Some(vis) = &self.visibility {
            write!(f, " {vis}")?;
        }
        if !self.weather.is_empty() {
            write!(
                f,
                " {}",
                self.weather
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" ")
            )?;
        }
        if let Some(clouds) = &self.clouds {
            let val = clouds.to_string();
            if !val.is_empty() {
                f.write_str(" ")?;
                f.write_str(&val)?;
            }
        }
        if let Some(colour_code) = &self.colour_code {
            write!(f, " {colour_code}")?;
        }

        Ok(())
    }
}

#[derive(PartialEq, Debug, Clone)]
/// When the weather will change
pub enum WeatherChangeTime {
    /// From when the changed weather will be valid
    From(u16),
    /// Until when the changed weather will be valid
    Till(u16),
    /// When the changed weather will be valid
    At(u16),
}

impl fmt::Display for WeatherChangeTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WeatherChangeTime::From(time) => write!(f, "FM{time:04}"),
            WeatherChangeTime::Till(time) => write!(f, "TL{time:04}"),
            WeatherChangeTime::At(time) => write!(f, "AT{time:04}"),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
/// Military weather colour code
pub enum ColourCode {
    /// BLU+: visibility>=8000m, ceiling >=20000ft
    BluePlus,
    /// BLU: visibility>=8000m, ceiling 2500-20000ft
    Blue,
    /// WHT: visibility 5000-8000m, ceiling 1500-2500ft
    White,
    /// GRN: visibility 3700-5000m, ceiling 700-1500ft
    Green,
    /// YLO: visibility 1600-3700m, ceiling 300-700ft
    Yellow,
    /// AMB: visibility 800-1600m, ceiling 200-300ft
    Amber,
    /// RED: visibility <800m, ceiling <200ft
    Red,
}

impl fmt::Display for ColourCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            ColourCode::BluePlus => "BLU+",
            ColourCode::Blue => "BLU",
            ColourCode::White => "WHT",
            ColourCode::Green => "GRN",
            ColourCode::Yellow => "YLO",
            ColourCode::Amber => "AMB",
            ColourCode::Red => "RED",
        })
    }
}
