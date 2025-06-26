mod api;
mod analysis;
mod alert;
mod telegram;
mod tui_dashboard;
mod soc_sim;

use clap::Parser;
use colored::*;
use dotenv::dotenv;
use chrono::Utc;
use std::fs::OpenOptions;
use std::io::Write;
use tokio::time::{sleep, Duration};

#[derive(Parser)]
#[command(name = "SolarShield", about = "Space Weather Cyber Defense Simulator", version)]
struct Cli {
    #[arg(short = 'u', long, help = "Enable TUI dashboard")]
    tui: bool,

    #[arg(short = 'g', long, help = "Enable Telegram alerts")]
    telegram: bool,

    #[arg(short = 'l', long, help = "Write alerts to a log file")]
    logfile: bool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cli = Cli::parse();

    println!("{}", "🚀 Starting SolarShield...".green());

    loop {
        match api::fetch_solar_events().await {
            Ok(events) => {
                for event in events {
                    let severity = analysis::analyze_severity(&event);

                    match severity {
                        analysis::Severity::Critical | analysis::Severity::Warning => {
                            println!(
                                "{} {} at {}",
                                "ALERT:".red().bold(),
                                event.message_type,
                                event.message_issue_time
                            );

                            if let Err(e) = alert::send_alert(&event).await {
                                eprintln!("Discord alert failed: {:?}", e);
                            }

                            if cli.telegram {
                                if let Err(e) = telegram::send_alert(&event).await {
                                    eprintln!("Telegram alert failed: {:?}", e);
                                }
                            }

                            if cli.logfile {
                                if let Err(e) = log_to_file(&event).await {
                                    eprintln!("Failed to log to file: {:?}", e);
                                }
                            }

                            if cli.tui {
                                let display_text = format!(
                                    "🚨 {} 🚨\n{}\n\n{}",
                                    event.message_type, event.message_issue_time, event.message_body
                                );
                                if let Err(e) = tui_dashboard::show_dashboard(&display_text) {
                                    eprintln!("TUI error: {:?}", e);
                                }
                            }

                            println!("{}", soc_sim::generate_mock_log(&event.message_type));
                        }
                        _ => {}
                    }
                }
            }
            Err(e) => eprintln!("{}", format!("❌ Error fetching NASA events: {:?}", e).red()),
        }

        sleep(Duration::from_secs(300)).await;
    }
}

async fn log_to_file(event: &api::SolarEvent) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("solarshield.log")?;

    let timestamp = Utc::now();
    let log_entry = format!(
        "[{}] {}: {}\n\n{}\n\n",
        timestamp, event.message_type, event.message_issue_time, event.message_body
    );

    file.write_all(log_entry.as_bytes())?;
    Ok(())
}
