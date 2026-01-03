use std::fmt::Display;
use std::fmt::Formatter;

use chumsky::prelude::*;

use crate::parsers::some_whitespace;
use crate::traits::Parsable;

use super::Data;
use super::WindDirection;
use super::WindSpeed;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Wind information.
pub enum Wind {
    /// Calm winds are at 0 kts
    Calm,
    /// Winds are present. More information is available in the struct.
    Present {
        /// The wind direction, in degrees
        dir: WindDirection,
        /// The current wind speed
        speed: WindSpeed,
        /// The direction the wind may be varying between, smaller always comes first
        varying: Option<(Data<u32>, Data<u32>)>,
    },
}

impl Parsable for Wind {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        choice((
            just("CALM")
                .map(|_| Wind::Calm)
                .then_ignore(some_whitespace()),
            group((
                WindDirection::parser(),
                WindSpeed::parser().then_ignore(some_whitespace()),
                choice((
                    group((WindDirection::parser(), just("V"), WindDirection::parser()))
                        .map(|(from, _, to)| Some((from.unwrap_heading(), to.unwrap_heading())))
                        .then_ignore(some_whitespace()),
                    empty().map(|()| None),
                )),
            ))
            .map(|(dir, speed, varying)| Wind::Present {
                dir,
                speed,
                varying,
            }),
        ))
    }
}

impl Display for Wind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Wind::Calm => f.write_str("CALM"),
            Wind::Present {
                dir,
                speed,
                varying,
            } => {
                dir.fmt(f)?;
                speed.fmt(f)?;
                if let Some((from, to)) = varying {
                    write!(
                        f,
                        " {:0>3}V{:0>3}",
                        from.to_opt_string(3),
                        to.to_opt_string(3)
                    )?;
                }

                Ok(())
            }
        }
    }
}
