# ji

![build](https://github.com/keirlawson/ji/workflows/build/badge.svg)

A simple command line tool allowing the user to retrieve the JIRA issue key for an issue they are currently working on.

![ji demo](https://raw.githubusercontent.com/keirlawson/ji/master/demo.gif)

The `<query>` parameter allows the specification of a JQL query, which defaults to finding all issues assigned to the user within any current sprints. The associated issue key is then printed to standard out, ready to be piped to other commands.

## Installation

For users with a rust toolchain ji can be readily installed via cargo, `cargo install ji`

Linux binaries are also published on github releases for those without cargo.

## Configuration

ji is configured via the following environment variables:

* `JIRA_HOST` - the JIRA instance to query, for example `https://mycompany.atlassian.net`
* `JIRA_USER` - the JIRA username to log in with, for SSOed JIRA cloud this will usually be an email address
* `JIRA_PASSWORD` - the password associated with the user, in the case of SSOed JIRA cloud this should be an API token.

## Usage example

ji is designed to be used in combination with programs like `xargs` to pass the key of a selected issue to another command.

For example, it can be combined with GitHub's `gh` to add information about a issue associated with a pull request like so.

```sh
ji | xargs -I {} gh pr create --body "Fixes {}" --fill
```