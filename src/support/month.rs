use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Month {
    JAN = 1,
    FEB = 2,
    MAR = 3,
    APR = 4,
    MAY = 5,
    JUN = 6,
    JUL = 7,
    AUG = 8,
    SEP = 9,
    OCT = 10,
    NOV = 11,
    DEC = 12,
}

impl FromStr for Month {
    type Err = MonthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "JAN" => Ok(Self::JAN),
            "FEB" => Ok(Self::FEB),
            "MAR" => Ok(Self::MAR),
            "APR" => Ok(Self::APR),
            "MAY" => Ok(Self::MAY),
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

