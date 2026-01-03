use std::fmt::{Display, Formatter};

use chumsky::prelude::*;

use crate::{CompassDirection, Data, traits::Parsable};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A cloud type description
pub enum CloudType {
    /// A normal cloud
    Normal,
    /// A cumulonimbus cloud
    Cumulonimbus,
    /// A towering cumulus cloud
    ToweringCumulus,
}

impl Parsable for CloudType {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        choice((
            just("TCU").map(|_| CloudType::ToweringCumulus),
            just("CB").map(|_| CloudType::Cumulonimbus),
            empty().map(|()| CloudType::Normal),
        ))
    }
}

impl Parsable for (Vec<CompassDirection>, Data<CloudType>) {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        Data::parser_inline(3, CloudType::parser())
            .then(
                group((just("/"), CompassDirection::parser()))
                    .map(|(_, dir)| dir)
                    .repeated()
                    .at_least(1)
                    .collect::<Vec<_>>(),
            )
            .map(|(typ, dirs)| (dirs, typ))
    }
}

impl Display for CloudType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CloudType::Normal => Ok(()),
            CloudType::Cumulonimbus => f.write_str("CB"),
            CloudType::ToweringCumulus => f.write_str("TCU"),
        }
    }
}
