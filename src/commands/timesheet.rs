use std::str::FromStr;

use support::Month;
use support::utils;
use support::Clockify;

#[path = "../support/mod.rs"]
mod support;

pub async fn handle(month: &String, working_days: &i32) {
    validate_month(month);
    validate_working_days(working_days);

    let client = Clockify::new(None, None, None);

    client.get_user_id().await;

    println!("month {}, working_days {}", month, working_days);
}

fn validate_month(month: &String) -> Month {
    match Month::from_str(month) {
        Ok(month) => month,
        Err(_) => {
            utils::print_error_and_exit(
                format!("\"{}\" is not a valid month", &month)
            );
        },
    }
}

fn validate_working_days(working_days: &i32) -> &i32 {
    if working_days > &31 || working_days < &1 {
        utils::print_error_and_exit(
            format!("\"Working days\" must be between 1 and 31")
        );
    }

    return working_days;
}
