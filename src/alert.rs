use crate::api::SolarEvent;
use reqwest::Client;
use serde_json::json;


pub async fn send_alert(event: &SolarEvent) -> Result<(), reqwest::Error> {
    let discord_webhook = std::env::var("DISCORD_WEBHOOK").expect("DISCORD_WEBHOOK not set");
    let content = format!(
        "**🚨 Solar Event Alert 🚨**\n\
        **Type:** {}\n\
        **Time:** {}\n\
        ```\n{}\n```",
        event.message_type, event.message_issue_time, event.message_body
    );

    let client = Client::new();
    client.post(discord_webhook)
        .json(&serde_json::json!({ "content": content }))
        .send()
        .await?;

    Ok(())
}
