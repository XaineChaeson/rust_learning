use std::env;
use std::error::Error;
use std::fs;

use bootstrap_cli::{parse_observations, score_trend, summarize};

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| format!("{}/data/study_log.csv", env!("CARGO_MANIFEST_DIR")));
    let csv = fs::read_to_string(&path)?;
    let observations = parse_observations(&csv)?;
    let summary = summarize(&observations).expect("parsed CSV always has at least one data row");
    let trend = score_trend(&observations);

    println!("Study log: {path}");
    println!("Rows: {}", summary.count);
    println!("Total study minutes: {}", summary.total_study_minutes);
    println!(
        "Average study minutes: {:.1}",
        summary.average_study_minutes
    );
    println!("Average exercise score: {:.1}", summary.average_score);
    println!("Best day: {} ({:.1})", summary.best_day, summary.best_score);

    if !trend.is_empty() {
        let formatted_trend = trend
            .iter()
            .map(|change| format!("{change:+.1}"))
            .collect::<Vec<_>>()
            .join(", ");

        println!("Score changes: {formatted_trend}");
    }

    Ok(())
}
