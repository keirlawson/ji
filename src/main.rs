use anyhow::Result;
use ji::{Credentials, Issue};
use std::{collections::HashMap, env};
use structopt::StructOpt;
use url::Url;

#[derive(StructOpt)]
struct Opt {
    #[structopt(
        default_value = "assignee IN (currentUser()) AND sprint IN openSprints() AND statusCategory NOT IN (Done) ORDER BY created DESC"
    )]
    query: String,
    /// If specified adds a "no ticket" option which produces the specified shortcode
    #[structopt(short, long)]
    no_ticket: Option<String>,
}

fn read_config() -> Result<ji::Config> {
    let host = env::var("JIRA_HOST")?;
    let host = Url::parse(&host)?;
    let pat = env::var("JIRA_TOKEN").map(ji::Credentials::PersonalAccessToken);
    let creds = pat.or_else(|_| {
        env::var("JIRA_USER").and_then(|user| {
            env::var("JIRA_PASSWORD").map(|pass| Credentials::UsernamePassword {
                username: user,
                password: pass,
            })
        })
    });
    creds
        .map(|credentials| ji::Config { host, credentials })
        .map_err(|e| e.into())
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let config = read_config()?;

    let mut issues = ji::search_issues(config, &opt.query)?;

    if let Some(no_ticket) = opt.no_ticket {
        let mut fields = HashMap::new();
        fields.insert("summary".to_owned(), "No ticket".to_owned());
        issues.push(Issue {
            key: no_ticket,
            fields,
        })
    };
    use fuzzy_select::FuzzySelect;

    let selected = FuzzySelect::new().with_options(issues).select()?;

    println!("{}", selected.key);

    Ok(())
}
