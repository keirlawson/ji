use anyhow::{Context, Result, anyhow};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    #[structopt(default_value = "assignee IN (currentUser()) AND sprint IN openSprints() ORDER BY created DESC")]
    query: String
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let config = envy::from_env::<ji::Config>().map_err(|e| {
        match e {
            envy::Error::MissingValue(value) => anyhow!("Required environment variable {} not set", value.to_uppercase()),
            envy::Error::Custom(message) => anyhow!("{}", message)
        }
    }).context("Unable to load configuration from environment variables")?;

    let issues = ji::search_issues(config, &opt.query)?;

    let issue = ji::select_issue(&issues)?;

    println!("{}", issue.key);

    Ok(())
}