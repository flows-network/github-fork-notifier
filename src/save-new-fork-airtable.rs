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
    let base_id: &str = "from-gh";
    let table_name: &str = "ghgh";

    if let EventPayload::ForkEvent(e) = payload {
        let forkee = e.forkee;
        let id = forkee.id;
        let html_url = forkee.html_url.unwrap().to_string();
        let time = forkee.created_at.expect("time not found");

        let text = format!("{} forked your {}\n{}", id, html_url, time);
        send_message_to_channel("ik8", "general", text);

        let data = serde_json::json!({
        "id": id,
        "html_url": html_url,
        "created": time,
        });
        create_record(account, base_id, table_name, data)
    }
}
