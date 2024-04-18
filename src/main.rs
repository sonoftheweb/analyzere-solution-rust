use std::env;
use std::io::{self, BufRead};
use std::process;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: compute <threshold> <limit>");
        process::exit(1);
    }

    let threshold: f64 = args[1].parse().unwrap_or_else(|_| {
        eprintln!("Invalid threshold.");
        process::exit(1);
    });

    let limit: f64 = args[2].parse().unwrap_or_else(|_| {
        eprintln!("Invalid limit.");
        process::exit(1);
    });

    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut cumulative_sum = 0.0;
    let mut results = Vec::new();

    for line in handle.lines() {
        let line = line?;
        let input_value: f64 = line.trim().parse().unwrap_or_else(|_| {
            eprintln!("All inputs must be numbers.");
            process::exit(1);
        });

        let output_value = if input_value > threshold {
            input_value - threshold
        } else {
            0.0
        };

        let adjusted_value = if cumulative_sum + output_value > limit {
            limit - cumulative_sum
        } else {
            output_value
        };

        cumulative_sum += adjusted_value;
        results.push(adjusted_value);
    }

    for result in results {
        println!("{:.1}", result);
    }
    println!("{:.1}", cumulative_sum);

    Ok(())
}
