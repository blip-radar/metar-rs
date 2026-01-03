use std::fmt::Display;

/// The kind of METAR produced.
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Kind {
    /// This is a regular METAR.
    Normal,
    /// This METAR was generated automatically without human oversight
    Automatic,
    /// This METAR corrects a previously issued METAR
    Correction,
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Normal => Ok(()),
            Kind::Automatic => f.write_str("AUTO "),
            Kind::Correction => f.write_str("COR "),
        }
    }
}
