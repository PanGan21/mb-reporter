use libc::{getifaddrs, if_data, ifaddrs};
use log::error;
use std::ffi::CStr;

#[derive(Default)]
pub struct DataUsage {
    pub tx_bytes: u64,
    pub rx_bytes: u64,
}

pub fn get_data_usage(interface_name: &str) -> DataUsage {
    let mut ifap: *mut ifaddrs = std::ptr::null_mut();
    let mut data_usage = DataUsage::default();

    unsafe {
        if getifaddrs(&mut ifap) != 0 {
            error!("Failed to get network interface information");
            panic!("Failed to get network interface information");
        }

        let mut ifa = ifap;
        while !ifa.is_null() {
            let ifa_name = (*ifa).ifa_name;
            let name = CStr::from_ptr(ifa_name).to_string_lossy().into_owned();

            // Check if the interface matches the desired name
            if name == interface_name {
                let ifa_data = (*ifa).ifa_data as *const if_data;
                let data = &*ifa_data;

                // Retrieve the data usage information
                data_usage.tx_bytes = data.ifi_obytes as u64;
                data_usage.rx_bytes = data.ifi_ibytes as u64;

                break;
            }

            ifa = (*ifa).ifa_next;
        }

        libc::freeifaddrs(ifap);
    }

    data_usage
}

pub fn bytes_to_megabytes(bytes: u64) -> f64 {
    bytes as f64 / 1_000_000.0
}
