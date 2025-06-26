use crate::api::SolarEvent;
use reqwest::Client;
use serde_json::json;


pub async fn send_alert(event: &SolarEvent) -> Result<(), reqwest::Error> {
    let telegram_token = std::env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let chat_id = std::env::var("TELEGRAM_CHAT_ID").expect("TELEGRAM_CHAT_ID not set");

    let text = format!(
        "<b>🚨 Solar Event Alert 🚨</b>\n\
        <b>Type:</b> {}\n\
        <b>Time:</b> {}\n\n\
        <pre>{}</pre>\n\n\
        <a href=\"https://www.swpc.noaa.gov/products-and-data\">SWPC Forecasts</a>",
        event.message_type,
        event.message_issue_time,
        event.message_body,
    );

    let url = format!("https://api.telegram.org/bot{}/sendMessage", telegram_token);

    let client = Client::new();
    client.post(&url)
        .json(&serde_json::json!({
            "chat_id": chat_id,
            "text": text,
            "parse_mode": "HTML"
        }))
        .send()
        .await?;

    Ok(())
}
