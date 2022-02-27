use std::str::FromStr;
use chrono::prelude::*;

use super::utils;

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

pub fn get_time_range(month: Month) -> (DateTime<chrono::Utc>, DateTime<chrono::Utc>)  {
    let month_int = month as u32;
    let current_year = Utc::now().year();
    let start = Utc.ymd(current_year, month_int, 1).and_hms(0,0,0);
    let end = start.with_month(month_int + 1).unwrap();

    (start, end)
}

pub fn validate_month(month: &String) -> Month {
    match Month::from_str(month) {
        Ok(month) => month,
        Err(_) => {
            utils::print_error_and_exit(
                format!("\"{}\" is not a valid month", &month)
            );
        },
    }
}
