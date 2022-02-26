use std::str::FromStr;

#[derive(Debug)]
pub enum Month {
    JAN,
    FEB,
    MAR,
    APR,
    JUN,
    JUL,
    AUG,
    SEP,
    OCT,
    NOV,
    DEC,
}

impl FromStr for Month {
    type Err = MonthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "JAN" => Ok(Self::JAN),
            "FEB" => Ok(Self::FEB),
            "MAR" => Ok(Self::MAR),
            "APR" => Ok(Self::APR),
            "JUN" => Ok(Self::JUN),
            "JUL" => Ok(Self::JUL),
            "AUG" => Ok(Self::AUG),
            "SEP" => Ok(Self::SEP),
            "OCT" => Ok(Self::OCT),
            "NOV" => Ok(Self::NOV),
            "DEC" => Ok(Self::DEC),
            _ => Err(MonthError),
        }
    }
}

pub struct MonthError;

