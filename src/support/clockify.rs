use reqwest::{Method, Client, RequestBuilder};
use serde::{Serialize, Deserialize};

use super::utils;


pub struct ClockifyClient<'clo> {
    api_key: &'clo String,
    workspace_id: &'clo String,
}

impl<'clo> ClockifyClient<'clo> {
    pub fn new(
        api_key: &'clo String, 
        workspace_id: &'clo String, 
    ) -> Self {
        Self {
            api_key,
            workspace_id,
        }
    }

    #[allow(dead_code)]
    pub async fn get_user_id(&self) -> Result<String, reqwest::Error> {
        let url = format!("{}/user", self.base_url());

        let response = self
            .request(Method::GET, url)
            .send()
            .await?
            .json::<User>()
            .await;

        let user_id = match response {
            Ok(user) => user.id,
            Err(_) => 
                utils::print_error_and_exit(
                    format!("There was an error getting the user")
                )
        };

        Ok(user_id)
    }

    #[allow(dead_code)]
    pub async fn list_tasks(&self, from: &str, to: &str) -> Result<Vec<TimeEntryReport>, reqwest::Error> {
        let user_id = self.get_user_id().await.unwrap();

        let url = format!(
            "{}/workspaces/{}/user/{}/time-entries?start={}&end={}&hydrated=1&page-size=5000",
            self.base_url(),
            self.workspace_id,
            user_id,
            from,
            to,
        );

        let response = self
            .request(Method::GET, url)
            .send()
            .await?
            .json::<Vec<TimeEntryReport>>()
            .await;

        match response {
            Ok(entries) => Ok(entries),
            Err(_) => utils::print_error_and_exit(format!("There was an error getting the time entries")),
        }
    }

    #[allow(dead_code)]
    pub async fn get_detailed_report(&self, from: &str, to: &str) -> Result<Report, reqwest::Error> {
        let url = format!(
            "{}/workspaces/{}/reports/detailed",
            self.reports_base_url(),
            self.workspace_id,
        );

        let body = ReportParams {
            start: from,
            end: to,
            ammount_show: "HIDE_AMOUNT",
            detailed_filter: DetailedFilter { page: 1, page_size: 1000 }
        };

        let response = self
            .request(Method::POST, url)
            .json(&body)
            .send()
            .await?
            .json::<Report>()
            .await;

        match response {
            Ok(report) => Ok(report),
            Err(_) => utils::print_error_and_exit(format!("There was an error getting the report")),
        }
    }

    fn request(&self, method: Method, url: String) -> RequestBuilder {
        let client = Client::new();

        client
            .request(method, url)
            .header("Accept", "application/json")
            .header("X-Api-Key", self.api_key)
    }

    fn base_url(&self) -> String {
        format!("https://api.clockify.me/api/v1")
    }

    fn reports_base_url(&self) -> String {
        format!("https://reports.api.clockify.me/v1")
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    email: String,

    id: String,

    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEntryReport {
    #[serde(rename = "projectId")]
    pub project_id: String,

    pub description: String,

    pub project: Project,
}

impl PartialEq for TimeEntryReport {
    fn eq(&self, other: &Self) -> bool {
        self.description.eq(&other.description)
    }
}

impl Ord for TimeEntryReport {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.description.cmp(&other.description)
    }
}

impl Eq for TimeEntryReport {}

impl PartialOrd for TimeEntryReport {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.description.partial_cmp(&other.description)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub id: String,

    pub name: String,

    #[serde(rename = "clientName")]
    pub client_name: String,
}


#[derive(Debug, Serialize, Deserialize)]
struct ReportParams<'clo> {
    #[serde(rename = "dateRangeStart")]
    start: &'clo str,
    
    #[serde(rename = "dateRangeEnd")]
    end: &'clo str,

    #[serde(rename = "detailedFilter")]
    detailed_filter: DetailedFilter,

    #[serde(rename = "amountShown")]
    ammount_show: &'clo str,
}

#[derive(Debug, Serialize, Deserialize)]
struct DetailedFilter {
    page: i32,

    #[serde(rename = "pageSize")]
    page_size: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
    pub totals: Vec<Totals>,

    #[serde(rename = "timeentries")]
    pub time_entries: Vec<TimeEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Totals {
    #[serde(rename = "totalTime")]
    pub total_time: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEntry {
    #[serde(rename = "projectId")]
    pub project_id: String,

    #[serde(rename = "projectName")]
    pub project_name: String,

    #[serde(rename = "clientName")]
    pub client_name: String,
    
    #[serde(rename = "timeInterval")]
    pub time_interval: TimeInterval,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeInterval {
    pub start: String,

    pub end: String,

    pub duration: i32,
}
