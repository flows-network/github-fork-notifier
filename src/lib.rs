use airtable_flows::create_record;
use github_flows::{listen_to_event, EventPayload, GithubLogin::Provided};
use slack_flows::send_message_to_channel;
use dotenv::dotenv;
use std::env;
#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    dotenv().ok();
    let github_login = env::var("github_login").unwrap_or("alabulei1".to_string());
    let github_owner = env::var("github_owner").unwrap_or("WasmEdge".to_string());
    let github_repo = env::var("github_repo").unwrap_or("WasmEdge".to_string());
    listen_to_event(
        &Provided(github_login),
        &github_owner,
        &github_repo,
        vec!["fork"],
        handler,
    )
    .await;
}

async fn handler(payload: EventPayload) {
    let airtable_token_name = env::var("airtable_token_name").unwrap_or("github".to_string());
    let airtable_base_id = env::var("airtable_base_id").unwrap_or("appNEswczILgUsxML".to_string());
    let airtable_table_name = env::var("airtable_table_name").unwrap_or("fork".to_string());

    if let EventPayload::ForkEvent(e) = payload {
        let forkee = e.forkee;
        let name = forkee.owner.unwrap().login;
        let html_url = forkee.html_url.unwrap().to_string();
        let time = forkee.created_at.expect("time not found");

        let text = format!("{name} forked your {html_url}\n{time}");
        send_message_to_channel("secondstate", "github-status", text);

        let data = serde_json::json!({
        "Name": name,
        "Repo": html_url,
        "Created": time,
        });
        create_record(
            &airtable_token_name,
            &airtable_base_id,
            &airtable_table_name,
            data
        )
    }
}
