use std::fmt::{Display, Formatter};

use chumsky::prelude::*;

use crate::{Data, traits::Parsable};

/// Military airport colour code
#[derive(PartialEq, Eq, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ColourCode {
    /// 20000+ cloud base, 8000m visibility
    BluePlus,
    /// 2500ft cloud base, 8000m visibility
    Blue,
    /// 1500ft cloud base, 5000m visibility
    White,
    /// 700ft cloud base, 3700m visibility
    Green,
    /// 300ft cloud base, 1600m visibility
    Yellow,
    /// 200ft cloud base, 800m visibility
    Amber,
    /// Less than amber
    Red,
}

impl Parsable for Data<ColourCode> {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        choice((
            just("///").map(|_| Data::Unknown),
            just("BLU+").map(|_| Data::Known(ColourCode::BluePlus)),
            just("BLU").map(|_| Data::Known(ColourCode::Blue)),
            just("WHT").map(|_| Data::Known(ColourCode::White)),
            just("GRN").map(|_| Data::Known(ColourCode::Green)),
            just("YLO").map(|_| Data::Known(ColourCode::Yellow)),
            just("AMB").map(|_| Data::Known(ColourCode::Amber)),
            just("RED").map(|_| Data::Known(ColourCode::Red)),
        ))
    }
}

impl Display for ColourCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
