use chrono::{Local, NaiveDate};
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct Appointment {
    #[serde(rename = "apptDate")]
    appt_date: String,
    open: u32,
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <endpoint> <before-date MM-DD-YYYY>", args[0]);
        std::process::exit(1);
    }

    let endpoint = &args[1];
    let after_date_str = &args[2];

    let before_date = NaiveDate::parse_from_str(after_date_str, "%m-%d-%Y").unwrap_or_else(|_| {
        eprintln!(
            "Invalid date '{}'. Expected format: MM-DD-YYYY",
            after_date_str
        );
        std::process::exit(1);
    });

    println!(
        "Watching for open appointments before {} — checking every 5 minutes.",
        before_date
    );

    loop {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        match fetch_open(endpoint, before_date).await {
            Ok(available) if available.is_empty() => {
                println!("[{timestamp}] No open appointments found before {before_date}.");
            }
            Ok(available) => {
                println!("[{timestamp}] Open appointments found:");
                for appt in &available {
                    println!("  {} — {} open slot(s)", appt.appt_date, appt.open);
                }
            }
            Err(e) => {
                eprintln!("[{timestamp}] Error fetching appointments: {e}");
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(300)).await;
    }
}

async fn fetch_open(
    endpoint: &str,
    before_date: NaiveDate,
) -> Result<Vec<Appointment>, Box<dyn std::error::Error>> {
    let appointments: Vec<Appointment> = reqwest::get(endpoint).await?.json().await?;

    let available = appointments
        .into_iter()
        .filter(|a| {
            NaiveDate::parse_from_str(&a.appt_date, "%Y-%m-%d")
                .map(|d| d < before_date && a.open > 0)
                .unwrap_or(false)
        })
        .collect();

    Ok(available)
}
