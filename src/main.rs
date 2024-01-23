use anyhow::Result;
use ji::Credentials;
use std::env;
use structopt::StructOpt;
use url::Url;

#[derive(StructOpt)]
struct Opt {
    #[structopt(
        default_value = "assignee IN (currentUser()) AND sprint IN openSprints() ORDER BY created DESC"
    )]
    query: String,
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

    let issues = ji::search_issues(config, &opt.query)?;

    let issue = ji::select_issue(&issues)?;

    println!("{}", issue.key);

    Ok(())
}
