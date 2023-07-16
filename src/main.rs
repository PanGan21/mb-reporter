use crate::{
    args::parse_args, data_usage::bytes_to_megabytes, measurement::DataUsageMeasurement,
    network_interface::NetworkInterface,
};
use result_writer::write_results;

mod args;
mod data_usage;
mod measurement;
mod network_interface;
mod result_writer;

#[tokio::main]
async fn main() {
    env_logger::init();

    let interface = NetworkInterface::new("en0");
    let measurement_duration = parse_args();

    let data_measurement = DataUsageMeasurement::new(interface, measurement_duration);
    let data_usage = data_measurement.measure_data_usage();

    let total_tx_mb = bytes_to_megabytes(data_usage.tx_bytes);
    let total_rx_mb = bytes_to_megabytes(data_usage.rx_bytes);

    println!("Total Data Usage - Transmit: {:.2} MB", total_tx_mb);
    println!("Total Data Usage - Receive: {:.2} MB", total_rx_mb);

    write_results(total_tx_mb, total_rx_mb, measurement_duration).await;
}
