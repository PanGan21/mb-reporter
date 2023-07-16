# MB Reporter

The mb-reporter is a command-line tool for measuring data usage on a network interface. It calculates the total data usage (transmit and receive) over a specified measurement duration and outputs the results to a JSON file.

## Features

Measures data usage on a specified network interface
Supports custom measurement durations
Outputs results to a JSON file in a human-readable format

## Prerequisites

Before using mb-reporter, ensure that you have the following prerequisites installed on your system:

- Rust programming language (version 1.55 or higher)
- Cargo package manager (included with Rust)

## Installation

Clone the mb-reporter repository to your local machine.

```
git clone https://github.com/your-username/mb-reporter.git
```

```
cd mb-reporter
```

Build the project using Cargo.

```
cargo build --release
```

To measure data usage with mb-reporter, use the following command:

```
cargo run -- <measurement_duration>
```

Replace <measurement_duration> with the desired measurement duration in seconds. For example, to measure data usage for 10 minutes, use:

```
cargo run -- 600
```

The results will be written to a JSON file in the `results` directory. Each measurement generates a new result file with a timestamp in the filename.

## License

This project is licensed under the MIT License.
