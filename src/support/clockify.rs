use std::env::var;
use serde::{Serialize, Deserialize};

use super::utils;

pub struct Clockify {
    api_key: String,
    workspace_id: String,
    user_id: Option<String>,
    base_url: String,
}

impl Clockify {
    pub fn new(
        api_key: Option<String>, 
        workspace_id: Option<String>, 
        base_url: Option<String>
    ) -> Self {
        Self {
            api_key: Clockify::set_api_key(api_key),
            workspace_id: Clockify::set_workspace_id(workspace_id),
            user_id: None,
            base_url: Clockify::set_base_url(base_url),
        }
    }

    pub async fn get_user_id(mut self) -> Result<Option<String>, reqwest::Error> {
        let client = reqwest::Client::new();

        let response = client
            .get(format!("{}/user", self.base_url))
            .header("Accept", "application/json")
            .header("X-Api-Key", self.api_key)
            .send()
            .await?
            .json::<User>()
            .await;

        match response {
            Ok(user) => self.user_id = Some(user.id),
            Err(_) => 
                utils::print_error_and_exit(
                    format!("There was an error getting the user")
                )
        }

        Ok(self.user_id)
    }

    fn set_api_key(api_key: Option<String>) -> String {
        match api_key {
            Some(api_key) => api_key.clone(),
            None => Clockify::get_env_var(format!("CLOCKIFY_API_KEY")),
        }
    }

    fn set_workspace_id(workspace_id: Option<String>) -> String {
        match workspace_id {
            Some(workspace_id) => workspace_id.clone(),
            None => Clockify::get_env_var(format!("CLOCKIFY_WORKSPACE_ID")),
        }
    }

    fn set_base_url(base_url: Option<String>) -> String {
        match base_url {
            Some(base_url) => base_url.clone(),
            None => format!("https://api.clockify.me/api/v1"),
        }
    }

    fn get_env_var(name: String) -> String {
        match var(&name) {
            Ok(result) => result,
            Err(_) => {
                utils::print_error_and_exit(format!("\"{}\" not found", name))
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    email: String,

    id: String,

    name: String,
}
