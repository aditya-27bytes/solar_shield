use chrono::Utc;

pub fn generate_mock_log(event_type: &str) -> String {
    let timestamp = Utc::now();
    format!("[{}] SOC Alert :: Detected {:?}", timestamp, event_type)
}
