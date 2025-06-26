pub enum Severity {
    Info,
    Warning,
    Critical,
}

pub fn analyze_severity(event: &crate::api::SolarEvent) -> Severity {
    if event.message_type.contains("CME") && event.message_body.contains("impact") {
        Severity::Critical
    } else if event.message_type.contains("CME") {
        Severity::Warning
    } else {
        Severity::Info
    }
}
