use std::fmt::{Display, Formatter};

use chumsky::prelude::*;

use crate::{parsers::some_whitespace, traits::Parsable};

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Cloud state
pub enum Clouds {
    /// No cloud was detected, also set for CAVOK
    NoCloudDetected,
    /// No significant cloud was detected below 5000ft
    NoSignificantCloud,
    /// Layers of cloud, described elsewhere
    CloudLayers,
}

impl Display for Clouds {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Clouds::NoCloudDetected => f.write_str("NCD"),
            Clouds::NoSignificantCloud => f.write_str("NSC"),
            Clouds::CloudLayers => Ok(()),
        }
    }
}

impl Parsable for Clouds {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        choice((
            just("NCD")
                .map(|_| Clouds::NoCloudDetected)
                .then_ignore(some_whitespace()),
            just("NSC")
                .map(|_| Clouds::NoSignificantCloud)
                .then_ignore(some_whitespace()),
            just("CLR")
                .map(|_| Clouds::NoCloudDetected)
                .then_ignore(some_whitespace()),
            empty().map(|()| Clouds::CloudLayers),
        ))
    }
}
