use std::collections::HashMap;
use std::str::FromStr;
use tabled::{Style, Tabled, Table};
use math::round;

use support::Month;
use support::utils;
use support::ClockifyClient;
use support::clockify::{Report, TimeEntry};

#[path = "../support/mod.rs"]
mod support;

pub async fn handle(month: &String, working_days: &i32) {
    validate_month(month);
    validate_working_days(working_days);

    let api_key = utils::get_env_var(format!("CLOCKIFY_API_KEY"));
    let workspace_id = utils::get_env_var(format!("CLOCKIFY_WORKSPACE_ID"));

    let client = ClockifyClient::new(&api_key, &workspace_id);

    let from = "2022-01-01T00:00:00.000";
    let to = "2022-01-31T23:59:59.000";

    let report = client.get_detailed_report(&from, &to).await.unwrap();

    render(&report, &working_days)
}

fn render(report: &Report, working_days: &i32) {
    let mut project_entries: HashMap<String, Vec<&TimeEntry>> = HashMap::new();
    let mut project_total_entries: Vec<ReportRow> = Vec::new();
    let time_entries = &report.time_entries;

    time_entries.iter().for_each(|entry| {
        if project_entries.contains_key(&entry.project_id) {
            project_entries.get_mut(&entry.project_id).unwrap().push(entry);
        } else {
            let mut new_vector = Vec::new();
            new_vector.push(entry);
            project_entries.insert(entry.project_id.clone(), new_vector);
        }
    });

    project_entries.into_iter().for_each(|(_key, entries)| {
        let total = round::half_up(entries.clone().into_iter()
            .fold(0.0, |result, entry| {
                result + (entry.time_interval.duration as f64 / 3600.0)
            }), 2);

        let project = entries[0].clone();

        project_total_entries.push(
            ReportRow {
                project_id: project.project_id.clone(),
                project_name: project.project_name.clone(),
                client_name: project.client_name.clone(),
                total,
            }
        );
    });

    println!(" ");

    println!(
        "{}", 
        Table::new(project_total_entries)
            .with(Style::psql())
            .to_string()
    )
}

#[derive(Tabled, Debug)]
struct ReportRow {
    #[header(hidden = true)]
    project_id: String,

    #[header("Client")]
    client_name: String,

    #[header("Project name")]
    project_name: String,

    #[header("total (h)")]
    total: f64,
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
