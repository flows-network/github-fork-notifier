use airtable_flows::create_record;
use github_flows::{listen_to_event, EventPayload};
use slack_flows::send_message_to_channel;
#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    let owner = "jaykchen";
    let repo = "a-test";

    listen_to_event(owner, repo, vec!["fork"], handler).await;
}

async fn handler(payload: EventPayload) {
    let account: &str = "jaykchen";
    let base_id: &str = "apptJFYvsGrrywvWh";
    let table_name: &str = "users";

    if let EventPayload::ForkEvent(e) = payload {
        
        let forkee = e.forkee;
        let name = forkee.owner.unwrap().login;
        let html_url = forkee.html_url.unwrap().to_string();
        let time = forkee.created_at.expect("time not found");

        let text = format!("{} forked your {}\n{}", name, html_url, time);
        send_message_to_channel("ik8", "general", text);

        let data = serde_json::json!({
        "Name": name,
        "Repo": html_url,
        "Created": time,
        });
        create_record(account, base_id, table_name, data)
    }
}
