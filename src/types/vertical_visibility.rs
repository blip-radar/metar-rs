use std::fmt::{Display, Formatter};

use chumsky::prelude::*;

use crate::traits::Parsable;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Vertical visibility measurement
pub enum VerticalVisibility {
    /// A distance of vertical visibility
    Distance(u32),
    /// The vertical visibility value is present, so is reduced, but by an amount that hasn't or
    /// cannot be measured
    ReducedByUnknownAmount,
}

impl Parsable for VerticalVisibility {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        choice((
            just("VV///").map(|_| VerticalVisibility::ReducedByUnknownAmount),
            just("VV").then(text::digits(10).exactly(3).to_slice()).map(
                |(_, digits): (&str, &str)| VerticalVisibility::Distance(digits.parse().unwrap()),
            ),
        ))
    }
}

impl Display for VerticalVisibility {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VerticalVisibility::Distance(vv) => write!(f, "VV{vv:03}"),
            VerticalVisibility::ReducedByUnknownAmount => f.write_str("VV///"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vvis() {
        assert_eq!(
            VerticalVisibility::parse("VV///").unwrap(),
            VerticalVisibility::ReducedByUnknownAmount
        );
        assert_eq!(
            VerticalVisibility::parse("VV350").unwrap(),
            VerticalVisibility::Distance(350)
        );
    }
}
