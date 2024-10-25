use csv::ReaderBuilder;
use reqwest::blocking::get;
use rusqlite::{params, Connection, Result};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

// Make functions public for use in main.rs

/// Download data from a URL and save to a specified file path.
pub fn extract(url: &str, file_path: &str) -> io::Result<()> {
    let response = get(url).expect("Failed to make request");
    if response.status().is_success() {
        if let Some(parent) = Path::new(file_path).parent() {
            fs::create_dir_all(parent)?;
        }

        let mut file = File::create(file_path)?;
        let content = response.bytes().expect("Failed to read bytes");
        file.write_all(&content)?;
        println!("File successfully downloaded to {}", file_path);
    } else {
        eprintln!(
            "Failed to retrieve the file. HTTP Status Code: {}",
            response.status()
        );
    }
    Ok(())
}

/// Load data from a CSV file into an SQLite database.
pub fn load(dataset: &str) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("Murder2015.db")?;

    // Adjust schema if you need `state_s`
    conn.execute("DROP TABLE IF EXISTS Murder2015", [])?;
    conn.execute(
        "CREATE TABLE Murder2015 (city TEXT, state_s TEXT, murders2014 INT, murders2015 INT, change INT)",
        [],
    )?;

    let mut rdr = ReaderBuilder::new().from_path(dataset)?;
    let mut insert_stmt = conn.prepare("INSERT INTO Murder2015 VALUES (?, ?, ?, ?, ?)")?;

    for result in rdr.records() {
        let record = result?;
        insert_stmt.execute(params![
            record[0].to_string(),
            record[1].to_string(),
            record[2].parse::<i32>().unwrap_or(0), // Default to 0 on parse failure
            record[3].parse::<i32>().unwrap_or(0),
            record[4].parse::<i32>().unwrap_or(0),
        ])?;
    }
    println!("Data loaded into Murder2015.db");
    Ok(())
}

/// Execute a SQL query and print results.
pub fn execute_query(query: &str) -> Result<()> {
    let conn = Connection::open("Murder2015.db")?;
    let mut stmt = conn.prepare(query)?;
    let mut rows = stmt.query([])?;

    println!("Query results:");
    while let Some(row) = rows.next()? {
        let city: String = row.get(0)?;
        let state: String = row.get(1)?;
        let murders2014: i32 = row.get(2)?;
        let murders2015: i32 = row.get(3)?;
        let change: i32 = row.get(4)?;
        println!(
            "city: {}, state: {}, murders2014: {}, murders2015: {}, change: {}",
            city, state, murders2014, murders2015, change
        );
    }
    Ok(())
}
