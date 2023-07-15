use chrono::Utc;
use log::info;
use serde::Serialize;
use std::fs::{self, File};
use std::io::Write;
use std::time::Duration;

#[derive(Serialize)]
struct ResultData {
    start_time: String,
    end_time: String,
    measurement_duration: String,
    total_tx_mb: f64,
    total_rx_mb: f64,
    total_consumption_mb: f64,
}
pub fn write_results(total_tx_mb: f64, total_rx_mb: f64, measurement_duration: Duration) {
    let start_time = Utc::now();
    let end_time = start_time + chrono::Duration::from_std(measurement_duration).unwrap();

    let measurement_duration_str = format!("{:?}", measurement_duration);

    let total_consumption_mb = total_tx_mb + total_rx_mb;

    let result_data = ResultData {
        start_time: start_time.to_rfc3339(),
        end_time: end_time.to_rfc3339(),
        measurement_duration: measurement_duration_str,
        total_tx_mb,
        total_rx_mb,
        total_consumption_mb,
    };

    // Create the "result" folder if it doesn't exist
    fs::create_dir_all("result").unwrap();

    let filename = format!(
        "results/result_{}_{}.json",
        start_time.timestamp(),
        end_time.timestamp()
    );

    let json_data =
        serde_json::to_string_pretty(&result_data).expect("Failed to serialize JSON data");

    let mut file = File::create(filename.clone()).expect("Failed to create file");
    file.write_all(json_data.as_bytes())
        .expect("Failed to write to file");

    info!("Results written to file: {}", filename);
    println!("Total Data Usage: {:.2} MB", total_consumption_mb);
}
