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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    #[test]
    fn test_parse_args_valid_duration() {
        let args = vec!["program_name".to_string(), "10".to_string()];
        let duration = parse_args(&args);

        // Assert the result
        let expected_duration = Duration::from_secs(10);
        assert_eq!(duration, expected_duration);
    }

    #[test]
    #[should_panic(expected = "Provide the measurement duration in seconds")]
    fn test_parse_args_missing_duration() {
        let args = vec!["program_name".to_string()];
        parse_args(&args);
    }

    #[test]
    #[should_panic(expected = "Invalid measurement duration provided")]
    fn test_parse_args_invalid_duration() {
        let args = vec!["program_name".to_string(), "abc".to_string()];
        parse_args(&args);
    }

    fn parse_args(args: &[String]) -> Duration {
        if args.len() < 2 {
            panic!("Provide the measurement duration in seconds");
        }

        let duration_secs = match args[1].parse::<u64>() {
            Ok(secs) => secs,
            Err(_) => panic!("Invalid measurement duration provided"),
        };

        Duration::from_secs(duration_secs)
    }
}
