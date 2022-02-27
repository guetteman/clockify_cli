use std::collections::HashMap;

use chrono::SecondsFormat;
use owo_colors::OwoColorize;

use support::month;
use support::utils;
use support::ClockifyClient;
use support::clockify::TimeEntryReport;

#[path = "../support/mod.rs"]
mod support;

pub async fn handle(month: &String) {
    let valid_month = month::validate_month(month);
    let (from, to) = month::get_time_range(valid_month.clone());

    let api_key = utils::get_env_var(format!("CLOCKIFY_API_KEY"));
    let workspace_id = utils::get_env_var(format!("CLOCKIFY_WORKSPACE_ID"));

    let client = ClockifyClient::new(&api_key, &workspace_id);

    let time_entries = client.list_tasks(
        &from.to_rfc3339_opts(SecondsFormat::Millis, true), 
        &to.to_rfc3339_opts(SecondsFormat::Millis, true)
    ).await.unwrap();

    println!(" ");
    println!("From {} to {}", from.to_string(), to.to_string());
    println!(" ");

    render(&time_entries);
}

fn render(time_entries: &Vec<TimeEntryReport>) {
    let mut project_entries: HashMap<String, Vec<&TimeEntryReport>> = HashMap::new();

    time_entries.iter().for_each(|entry| {
        if project_entries.contains_key(&entry.project_id) {
            project_entries.get_mut(&entry.project_id).unwrap().push(entry);
        } else {
            let mut new_vector = Vec::new();
            new_vector.push(entry);
            project_entries.insert(entry.project_id.clone(), new_vector);
        }
    });

    for (_key, mut entries) in project_entries.into_iter() {
        entries.sort();
        entries.dedup();
        let project = entries[0].project.clone();

        println!("{} - {}", project.client_name.green().to_string(), project.name.green().to_string());
        println!(" ");

        for entry in entries.into_iter() {
            println!("- {}", entry.description); 
        }

        println!(" ");
    };
}
