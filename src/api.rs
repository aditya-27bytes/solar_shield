use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DonkiEvent {
    #[serde(rename = "messageType")]
    pub message_type: String,
    #[serde(rename = "messageIssueTime")]
    pub message_issue_time: String,
    #[serde(rename = "messageBody")]
    pub message_body: String,
}

pub struct SolarEvent {
    pub message_type: String,
    pub message_issue_time: String,
    pub message_body: String,
}

pub async fn fetch_solar_events() -> Result<Vec<SolarEvent>, reqwest::Error> {
    let api_key = std::env::var("NASA_API_KEY").unwrap_or("DEMO_KEY".to_string());
    let url = format!(
        "https://api.nasa.gov/DONKI/notifications?startDate=2025-06-01&endDate=2025-06-16&api_key={}",
        api_key
    );

    let resp = reqwest::get(&url).await?.json::<Vec<DonkiEvent>>().await?;

    let events = resp
        .into_iter()
        .map(|e| SolarEvent {
            message_type: e.message_type,
            message_issue_time: e.message_issue_time,
            message_body: e.message_body,
        })
        .collect();

    Ok(events)
}
