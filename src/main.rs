mod lib;

use clap::Parser;
use std::time::Instant;
use sysinfo::{ProcessExt, System, SystemExt};

#[derive(Parser)]
#[command(name = "example")]
#[command(about = "A Rust program to extract, load, and query data with memory and runtime tracking", long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        default_value = "https://raw.githubusercontent.com/fivethirtyeight/data/refs/heads/master/murder_2016/murder_2015_final.csv"
    )]
    url: String,

    #[arg(short, long, default_value = "data/murder_2015_final.csv")]
    file_path: String,
}

fn run_main_logic(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    println!("URL: {}", args.url);
    println!("File path: {}", args.file_path);

    // Start the timer
    let start_time = Instant::now();
    let mut system = System::new_all();
    system.refresh_all();

    // Get the process ID for memory measurement
    let pid = sysinfo::get_current_pid().expect("Failed to get process ID");
    let initial_memory = system.process(pid).map_or(0, |process| process.memory());

    // Run main process stages
    lib::extract(&args.url, &args.file_path).map_err(|e| {
        eprintln!("Error during data extraction: {}", e);
        e
    })?;

    let murder_execute_query = "
        WITH murder_change_rate AS (
            SELECT city, state_s,
                   NULLIF(change, 0) * 1.0 / NULLIF(murders2014, 0) AS Murder_Change_Rate
            FROM Murder2015
        )
        SELECT a.state_s, AVG(b.Murder_Change_Rate) AS average_murdersChange_perState
        FROM Murder2015 a
        LEFT JOIN murder_change_rate b ON a.city = b.city AND a.state_s = b.state_s
        GROUP BY a.state_s
        ORDER BY average_murdersChange_perState DESC;
    ";
    lib::execute_query(murder_execute_query).map_err(|e| {
        eprintln!("Error during SQL execution: {}", e);
        e
    })?;

    lib::load(&args.file_path).map_err(|e| {
        eprintln!("Error during data loading: {}", e);
        e
    })?;

    // Refresh system information to get updated memory usage
    system.refresh_all();
    let final_memory = system.process(pid).map_or(0, |process| process.memory());

    // Calculate elapsed time and memory usage
    let elapsed_time = start_time.elapsed();
    let memory_used = final_memory.saturating_sub(initial_memory);

    println!("Process completed in: {:.2?}", elapsed_time);
    println!("Memory used: {} KB", memory_used);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    run_main_logic(&args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_main_logic() {
        let args = Args {
            url: String::from("https://raw.githubusercontent.com/fivethirtyeight/data/refs/heads/master/murder_2016/murder_2015_final.csv"),
            file_path: String::from("data/murder_2015_final.csv"),
        };

        let result = run_main_logic(&args);
        println!("{:?}", result);
        assert!(result.is_ok(), "Expected Ok result, got Err: {:?}", result);
    }
}
