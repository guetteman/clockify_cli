use std::collections::HashMap;
use chrono::prelude::*;
use tabled::{Row, Format};
use tabled::{Alignment, Full, Modify, builder::Builder, Style};
use math::round;
use owo_colors::OwoColorize;

use support::month;
use support::utils;
use support::ClockifyClient;
use support::clockify::{Report, TimeEntry};

#[path = "../support/mod.rs"]
mod support;

pub async fn handle(month: &String, working_days: &i32) {
    validate_working_days(working_days);

    let valid_month = month::validate_month(month);
    let (from, to) = month::get_time_range(valid_month.clone());

    let api_key = utils::get_env_var(format!("CLOCKIFY_API_KEY"));
    let workspace_id = utils::get_env_var(format!("CLOCKIFY_WORKSPACE_ID"));

    let client = ClockifyClient::new(&api_key, &workspace_id);

    let report = client.get_detailed_report(
        &from.to_rfc3339_opts(SecondsFormat::Millis, true), 
        &to.to_rfc3339_opts(SecondsFormat::Millis, true)
    ).await.unwrap();

    println!(" ");

    println!("From {} to {}", from.to_string(), to.to_string());
    render(&report, &working_days)
}

fn render(report: &Report, working_days: &i32) {
    let mut project_entries: HashMap<String, Vec<&TimeEntry>> = HashMap::new();
    let time_entries = &report.time_entries;
    let mut table = Builder::default()
        .set_header(["Project", "Total (h)"]);

    time_entries.iter().for_each(|entry| {
        if project_entries.contains_key(&entry.project_id) {
            project_entries.get_mut(&entry.project_id).unwrap().push(entry);
        } else {
            let mut new_vector = Vec::new();
            new_vector.push(entry);
            project_entries.insert(entry.project_id.clone(), new_vector);
        }
    });

    for (_key, entries) in project_entries.into_iter() {
        let total = round::half_up(entries.clone().into_iter()
            .fold(0.0, |result, entry| {
                result + (entry.time_interval.duration as f64 / 3600.0)
            }), 2);

        let project = entries[0].clone();

        table = table.add_row([
            format!("{} - {}", project.client_name, project.project_name),
            format!("{}", total)
        ]);
    };

    let total = round::half_up(report.totals[0].total_time as f64 / 3600.0, 2);
    let working_hours = working_days * 8;
    let mpb = round::half_up(total - working_hours as f64, 2);

    table = table
        .add_row(["", ""])
        .add_row(["", ""])
        .add_row(["Working hours", &format!("{}", working_hours)])
        .add_row(["MPB", &format!("{}", mpb)])
        .add_row(["Total", &format!("{}", total)]);

    println!(
        "{}", 
        table
            .build()
            .with(Style::dots())
            .with(Modify::new(Full).with(Alignment::left()))
            .with(Modify::new(Row(0..1)).with(Format(|s| s.green().to_string())))
            .with(Modify::new(Row(1..)).with(Format(|s| s.default_color().to_string())))
    )
}


fn validate_working_days(working_days: &i32) -> &i32 {
    if working_days > &31 || working_days < &1 {
        utils::print_error_and_exit(
            format!("\"Working days\" must be between 1 and 31")
        );
    }

    return working_days;
}
