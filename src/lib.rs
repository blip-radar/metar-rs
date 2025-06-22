#![deny(missing_docs)]

//! # METAR parsing library for Rust
//!
//! ## Quick usage
//!
//! This simple usage will print out the parsed data from the METAR.
//!
//! ```rust
//! use metar::Metar;
//!
//! let metar = "EGHI 282120Z 19015KT 140V220 6000 RA SCT006 BKN009 16/14 Q1006";
//! let r = Metar::parse(metar).unwrap();
//! println!("{:#?}", r);
//! ```
//!
//! ## Issues
//!
//! METARs are complicated structures. If you come across a METAR that doesn't parse
//! correctly, please open an issue and include the METAR. This will aid in debugging
//! the issue significantly.

mod parser;
mod types;

use std::fmt;
pub use types::*;

#[derive(PartialEq, Clone, Debug)]
/// A complete METAR
pub struct Metar {
    /// The station making the METAR measurement
    pub station: String,
    /// The measurement time
    pub time: Time,
    /// If the measurement was generated automatically
    pub is_auto: bool,
    /// The current wind information
    pub wind: Wind,
    /// The current visibility
    pub visibility: Data<Visibility>,
    /// The Runway Visual Ranges
    pub rvr: Vec<RunwayVisualRange>,
    /// The current clouds
    pub clouds: Data<Clouds>,
    /// The current vertical visibility, in feet
    pub vert_visibility: Option<VertVisibility>,
    /// The current weather conditions
    pub weather: Data<Vec<Weather>>,
    /// The current temperature
    pub temperature: Data<i32>,
    /// The current dewpoint
    pub dewpoint: Data<i32>,
    /// The current air pressure
    pub pressure: Pressure,
    /// Recent weather phenomena
    pub recent_weather: Option<Vec<WeatherCondition>>,
    /// Remarks added on to the METAR
    pub remarks: Option<String>,
    /// The trend
    pub trend: Vec<Trend>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// An error when parsing a METAR
pub struct MetarError {
    /// The string being parsed
    pub string: String,
    /// The start index of the error
    pub start: usize,
    /// The length of the error'd section
    pub length: usize,
    /// The kind of error that occurred
    pub variant: pest::error::ErrorVariant<parser::Rule>,
}

impl std::error::Error for MetarError {}

impl fmt::Display for MetarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut caret = String::new();
        for _ in 0..self.start {
            caret.push(' ');
        }
        caret.push('^');
        for _ in 1..self.length {
            caret.push('~');
        }
        writeln!(f, "{}\n{}\n{:?}", self.string, caret, self.variant)
    }
}

impl Metar {
    /// Parse a string into a METAR
    pub fn parse<S>(data: S) -> Result<Self, MetarError>
    where
        S: Into<String>,
    {
        parser::parse(data.into())
    }
}

impl fmt::Display for Metar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.station)?;
        f.write_str(" ")?;

        write!(f, "{} ", self.time)?;
        if self.is_auto {
            f.write_str("AUTO ")?;
        }
        write!(f, "{} ", self.wind)?;

        write!(f, "{} ", self.visibility.to_opt_string(4))?;

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

        if !self
            .visibility
            .as_option()
            .is_some_and(|vis| *vis == Visibility::CAVOK)
        {
            let clouds = self.clouds.to_opt_string(9);
            if !clouds.is_empty() {
                write!(f, "{clouds} ")?;
            }
        }

        write!(
            f,
            "{:0>2}/{:0>2}",
            self.temperature.to_opt_string(2),
            self.dewpoint.to_opt_string(2)
        )?;

        write!(f, " {}", self.pressure)?;

        if let Some(recent) = &self.recent_weather {
            write!(
                f,
                " RE{}",
                recent.iter().map(ToString::to_string).collect::<String>()
            )?;
        }

        for trend in &self.trend {
            write!(f, " {trend}")?;
        }

        if let Some(remarks) = &self.remarks {
            write!(f, " {remarks}")?;
        }

        Ok(())
    }
}
