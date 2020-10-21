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

#[derive(Deserialize, Debug)]
pub struct Config {
    jira_host: Url,
    jira_user: String,
    jira_password: Option<String>,
}

pub fn search_issues(config: Config, query: &str) -> Result<Vec<Issue>> {
    let body = IssueSearchRequestBody {
        jql: String::from(query),
        fields: vec![String::from("summary")],
    };

    let resp = Client::new()
        .post(config.jira_host.join("/rest/api/2/search").unwrap())
        .json(&body)
        .basic_auth(config.jira_user, config.jira_password)
        .send()
        .context("Unable to search JIRA for issues")?
        .json::<IssueSearchResponseBody>()
        .context("Unable to decode JIRA response")?;

    resp.issues.ok_or(anyhow!("No issues found for query"))
}

pub fn select_issue(issues: &Vec<Issue>) -> Result<&Issue> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&issues)
        .default(0)
        .interact_opt()?;

    let index = selection.ok_or(anyhow!("No JIRA issue selected"))?;

    Ok(&issues[index])
}
