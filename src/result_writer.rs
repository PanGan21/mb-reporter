use chrono::Utc;
use log::info;
use serde::Serialize;
use std::fs::{self, File};
use std::io::Write;
use std::time::Duration;
use tokio::task;

#[derive(Serialize)]
struct ResultData {
    start_time: String,
    end_time: String,
    measurement_duration: String,
    total_tx_mb: f64,
    total_rx_mb: f64,
    total_consumption_mb: f64,
}
pub async fn write_results(total_tx_mb: f64, total_rx_mb: f64, measurement_duration: Duration) {
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

    // Create the "results" folder if it doesn't exist
    fs::create_dir_all("results").expect("Failed to create 'results' folder");

    let filename = format!(
        "results/result_{}_{}.json",
        start_time.timestamp(),
        end_time.timestamp()
    );

    let json_data =
        serde_json::to_string_pretty(&result_data).expect("Failed to serialize JSON data");

    let write_task = task::spawn_blocking(move || {
        let mut file = File::create(filename.clone()).expect("Failed to create file");
        file.write_all(json_data.as_bytes())
            .expect("Failed to write to file");

        info!("Results written to file: {}", filename);
    });

    write_task.await.expect("Failed to execute file write task");

    println!("Total Data Usage: {:.2} MB", total_consumption_mb);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_write_results() {
        let result_folder_path = std::env::current_dir().unwrap().join("results");

        // Define the measurement duration
        let measurement_duration = Duration::from_secs(60);

        // Call the write_results function
        write_results(10.5, 20.7, measurement_duration).await;

        // Allow some time for the file to be written
        tokio::time::sleep(Duration::from_secs(5)).await;

        // Get the list of files in the "results" folder
        let result_files: Vec<_> = fs::read_dir(&result_folder_path)
            .unwrap()
            .map(|entry| entry.unwrap().file_name().to_string_lossy().to_string())
            .collect();

        // // Verify that a result file was created
        assert!(result_files.len() >= 1);

        // // Verify the content of the result file
        let result_file_path = result_folder_path.join(&result_files[0]);
        let result_file_content = fs::read_to_string(&result_file_path).unwrap();

        assert!(result_file_content.contains("start_time"));
        assert!(result_file_content.contains("end_time"));
        assert!(result_file_content.contains("measurement_duration"));
        assert!(result_file_content.contains("total_tx_mb"));
        assert!(result_file_content.contains("total_rx_mb"));
        assert!(result_file_content.contains("total_consumption_mb"));
    }
}
