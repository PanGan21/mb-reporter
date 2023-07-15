use std::env;
use std::time::Duration;

pub fn parse_args() -> Duration {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Provide the measurement duration in seconds");
    }

    let duration_secs = match args[1].parse::<u64>() {
        Ok(secs) => secs,
        Err(_) => panic!("Invalid measurement duration provided"),
    };

    Duration::from_secs(duration_secs)
}
