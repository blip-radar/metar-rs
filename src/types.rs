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

    /// Convert to option to be able to use its API
    fn as_option(&self) -> Option<&T> {
        match self {
            Data::Known(v) => Some(v),
            Data::Unknown => None,
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

impl fmt::Display for Data<WindDirection> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Data::Known(w) => w.fmt(f),
            Data::Unknown => f.write_str("///"),
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

#[derive(PartialEq, Clone, Debug)]
/// Measured air pressure
pub enum Pressure {
    /// Pressure in hectopascals
    Hectopascals(u16),
    /// Pressure in inches of mercury (inHg)
    InchesOfMercury(f32),
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

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
/// Cloud state
pub enum Clouds {
    /// No cloud was detected, also set for CAVOK
    NoCloudDetected,
    /// No significant cloud was detected below 5000ft
    NoSignificantCloud,
    /// Layers of cloud, described elsewhere
    CloudLayers,
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// Cloud cover
pub enum CloudLayer {
    /// Few clouds (1/8)
    Few(CloudType, Option<u32>),
    /// Scattered cloud cover (3/8)
    Scattered(CloudType, Option<u32>),
    /// Broken cloud cover (5/8)
    Broken(CloudType, Option<u32>),
    /// Overcast cloud cover (7/8)
    Overcast(CloudType, Option<u32>),
    /// Cloud cover of an unknown density
    Unknown(CloudType, Option<u32>),
}

#[derive(PartialEq, Eq, Clone, Debug)]
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

#[derive(PartialEq, Eq, Clone, Debug)]
/// A weather information block
pub struct Weather {
    /// The intensity of this weather block
    pub intensity: WeatherIntensity,
    /// The weather condition/s this block describes.
    pub conditions: Vec<WeatherCondition>,
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
    /// Recent (RE)
    Recent,
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
        self.dir.fmt(f)?;
        f.write_str(
            &self
                .speed
                .as_option()
                .map_or("//".to_string(), |v| format!("{v:02}")),
        )?;
        if let Some(gusts) = self.gusting {
            write!(f, "G{gusts}")?;
        }
        write!(f, "{}", self.unit)?;
        if let Some((from, to)) = self.varying {
            write!(f, " {from:03}V{to:03}")?
        }

        Ok(())
    }
}
