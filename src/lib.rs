use airtable_flows::create_record;
use dotenv::dotenv;
use github_flows::{
    get_octo, listen_to_event,
    octocrab::{models::orgs::Organization, Result as OctoResult},
    EventPayload,
    GithubLogin::Default,
};
use slack_flows::send_message_to_channel;
use std::env;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    dotenv().ok();

    let github_owner = env::var("github_owner").unwrap_or("WasmEdge".to_string());
    let github_repo = env::var("github_repo").unwrap_or("WasmEdge".to_string());
    listen_to_event(&Default, &github_owner, &github_repo, vec!["fork"], handler).await;
}

async fn handler(payload: EventPayload) {
    let airtable_token_name = env::var("airtable_token_name").unwrap_or("github".to_string());
    let airtable_base_id = env::var("airtable_base_id").unwrap_or("appNEswczILgUsxML".to_string());
    let airtable_table_name = env::var("airtable_table_name").unwrap_or("fork".to_string());

    let slack_workspace = env::var("slack_workspace").unwrap_or("secondstate".to_string());
    let slack_channel = env::var("slack_channel").unwrap_or("github-status".to_string());

    if let EventPayload::ForkEvent(e) = payload {
        let octocrab = get_octo(&Default);

        let forkee = e.forkee;

        let html_url = forkee.html_url.unwrap().to_string();
        let time = forkee.created_at.expect("time not found");
        let forkee_as_user = forkee.owner.unwrap();

        let org_url = forkee_as_user.organizations_url;
        let forkee_login = forkee_as_user.login;

        let mut email = "".to_string();
        let mut twitter_handle = "".to_string();

        // let user_route = format!("https://api.github.com/users/jaykchen");
        let user_route = format!("users/{forkee_login}");
        let response: OctoResult<User> = octocrab.get(&user_route, None::<&()>).await;
        match response {
            Err(_) => {
                send_message_to_channel(
                    &slack_workspace,
                    &slack_channel,
                    "error getting email".to_string(),
                );
            }
            Ok(user_obj) => {
                email = user_obj.email.unwrap_or("".to_string());
                twitter_handle = user_obj.twitter_username.unwrap_or("".to_string());
                send_message_to_channel(&slack_workspace, &slack_channel, email.clone());
            }
        }

        let mut org_name = "".to_string();
        let mut org_company = "".to_string();

        // let org_route = format!("https://api.github.com/orgs/flows-network");
        let org_route = format!("orgs/{forkee_login}");
        let response: OctoResult<Organization> = octocrab.get(&org_route, None::<&()>).await;
        match response {
            Err(_) => {
                send_message_to_channel(
                    &slack_workspace,
                    &slack_channel,
                    "error getting org".to_string(),
                );
            }
            Ok(org_obj) => {
                org_name = org_obj.name.unwrap_or("no org name".to_string());
                org_company = org_obj.company.unwrap_or("no company name".to_string());
                send_message_to_channel(&slack_workspace, &slack_channel, org_name.clone());

            }
        };

        let text = format!("{forkee_login} forked your {html_url}\n{time}");
        send_message_to_channel(&slack_workspace, &slack_channel, text);

        let data = serde_json::json!({
        "Name": forkee_login,
        "Repo": html_url,
        "Email": email,
        "Twitter": twitter_handle,
        "OrgName": org_name,
        "OrgCompany": org_company,
        "Org": org_url,
        "Created": time,
        });
        create_record(
            &airtable_token_name,
            &airtable_base_id,
            &airtable_table_name,
            data,
        )
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    login: String,
    id: u32,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    organizations_url: String,
    blog: String,
    twitter_username: Option<String>,
    email: Option<String>,
    followers: u32,
}
