use anyhow::{anyhow, Context, Result};
use dialoguer::{theme::ColorfulTheme, Select};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use url::Url;

use reqwest::blocking::Client;

#[derive(Serialize)]
struct IssueSearchRequestBody {
    jql: String,
    fields: Vec<String>,
}

#[derive(Deserialize)]
struct IssueSearchResponseBody {
    issues: Option<Vec<Issue>>,
}

#[derive(Deserialize)]
pub struct Issue {
    pub key: String,
    pub fields: HashMap<String, String>,
}

impl fuzzy_select::Select for Issue {
    fn search_content(&self) -> &str {
        self.fields.get("summary").unwrap()
    }

    fn render_before_content(&self) -> Option<impl fmt::Display + '_> {
        None::<Self>
    }

    fn render_after_content(&self) -> Option<impl fmt::Display + '_> {
        None::<Self>
    }
}

impl fmt::Display for Issue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} - {}",
            &self.key,
            &self.fields.get("summary").unwrap()
        )
    }
}

pub struct Config {
    pub host: Url,
    pub credentials: Credentials,
}

pub enum Credentials {
    PersonalAccessToken(String),
    UsernamePassword { username: String, password: String },
}

pub fn search_issues(config: Config, query: &str) -> Result<Vec<Issue>> {
    let body = IssueSearchRequestBody {
        jql: String::from(query),
        fields: vec![String::from("summary")],
    };

    let req = Client::new()
        .post(config.host.join("/rest/api/2/search").unwrap())
        .json(&body);

    let req = match config.credentials {
        Credentials::PersonalAccessToken(token) => req.bearer_auth(token),
        Credentials::UsernamePassword { username, password } => {
            req.basic_auth(username, Some(password))
        }
    };

    let resp = req
        .send()
        .context("Unable to search JIRA for issues")?
        .json::<IssueSearchResponseBody>()
        .context("Unable to decode JIRA response")?;

    resp.issues
        .ok_or_else(|| anyhow!("No issues found for query"))
}

pub fn select_issue(issues: &[Issue]) -> Result<&Issue> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(issues)
        .default(0)
        .interact_opt()?;

    let index = selection.ok_or_else(|| anyhow!("No JIRA issue selected"))?;

    Ok(&issues[index])
}
