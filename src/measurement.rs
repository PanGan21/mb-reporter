use super::data_usage::{self, DataUsage};
use super::network_interface::NetworkInterface;
use log::info;
use std::thread;
use std::time::Duration;

pub struct DataUsageMeasurement {
    interface: NetworkInterface,
    measurement_duration: Duration,
}

impl DataUsageMeasurement {
    pub fn new(interface: NetworkInterface, measurement_duration: Duration) -> Self {
        DataUsageMeasurement {
            interface,
            measurement_duration,
        }
    }

    pub fn measure_data_usage(&self) -> DataUsage {
        let mut total_tx_bytes: u64 = 0;
        let mut total_rx_bytes: u64 = 0;

        info!("Fetching initial data usage");
        let initial_data = data_usage::get_data_usage(&self.interface.name);

        info!(
            "Sleeping for {} seconds",
            self.measurement_duration.as_secs()
        );
        thread::sleep(self.measurement_duration);

        info!("Fetching final data usage");
        let final_data = data_usage::get_data_usage(&self.interface.name);

        let tx_bytes = final_data.tx_bytes - initial_data.tx_bytes;
        let rx_bytes = final_data.rx_bytes - initial_data.rx_bytes;

        total_tx_bytes += tx_bytes;
        total_rx_bytes += rx_bytes;

        DataUsage {
            tx_bytes: total_tx_bytes,
            rx_bytes: total_rx_bytes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_measure_data_usage() {
        // Create a dummy network interface
        let interface = NetworkInterface::new("en0");

        // Define the measurement duration
        let measurement_duration = Duration::from_secs(10);

        // Create a DataUsageMeasurement instance
        let data_measurement = DataUsageMeasurement::new(interface, measurement_duration);

        // Call the measure_data_usage method
        let data_usage = data_measurement.measure_data_usage();

        // Verify that the data usage is greater than 0
        assert!(data_usage.tx_bytes > 0);
        assert!(data_usage.rx_bytes > 0);
    }
}
