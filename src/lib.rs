use csv::ReaderBuilder;
use reqwest;
use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub mod extract {
    use super::*;

    /// Function to download data from a URL and save it to a file
    pub async fn extract(url: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
        if let Some(parent) = Path::new(file_path).parent() {
            fs::create_dir_all(parent)?;
        }
        let response = reqwest::get(url).await?;
        if response.status().is_success() {
            let content = response.bytes().await?;
            let mut file = fs::File::create(file_path)?;
            file.write_all(&content)?;
            println!("File successfully downloaded to {}", file_path);
        } else {
            println!(
                "Failed to retrieve the file. HTTP Status Code: {}",
                response.status()
            );
        }
        Ok(())
    }
}

pub mod database {
    use super::*;

    /// Connect to the SQLite database
    pub fn connect_to_db(db_path: &str) -> Result<Connection> {
        Connection::open(db_path)
    }

    /// Execute an SQL query on the provided connection
    pub fn execute_query(conn: &Connection, query: &str) -> Result<()> {
        conn.execute_batch(query)?;
        println!("Query executed successfully.");
        Ok(())
    }

    pub fn create_data(conn: &Connection) -> Result<()> {
        conn.execute(
            "INSERT INTO Murder2015 (city, state, murders2014, murders2015, change) 
            VALUES (?1, ?2, ?3, ?4, ?5)",
            params!["city1", "NC", 1, 2, 1],
        )?;
        let mut stmt = conn.prepare("SELECT * FROM Murder2015 WHERE city = ?1")?;
        let rows = stmt.query_map(params!["city1"], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;
        println!("create: ");
        for row in rows {
            println!("{:?}", row.unwrap());
        }
        Ok(())
    }

    pub fn read_data(conn: &Connection) -> Result<()> {
        let mut stmt = conn.prepare("SELECT * FROM Murder2015 LIMIT 10")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;
        println!("read 10 from Murder2015:");
        for row in rows {
            println!("{:?}", row.unwrap());
        }
        Ok(())
    }

    pub fn update_data(conn: &Connection) -> Result<()> {
        conn.execute(
            "UPDATE Murder2015 SET murders2015 = ?1, change = ?2 WHERE city = ?3",
            params![3, 2, "city2"],
        )?;
        println!("Update Success");
        Ok(())
    }

    pub fn delete_data(conn: &Connection) -> Result<()> {
        conn.execute("DELETE FROM Murder2015 WHERE city = ?1", params!["city2"])?;
        println!("Delete Success");
        Ok(())
    }
}

pub mod load {
    use super::*;

    pub fn load(dataset: &str) -> Result<(), Box<dyn Error>> {
        let file = File::open(dataset)?;
        let mut rdr = ReaderBuilder::new().from_reader(file);
        let conn = Connection::open("Murder2015.db")?;

        conn.execute("DROP TABLE IF EXISTS Murder2015", [])?;
        conn.execute(
            "CREATE TABLE Murder2015 (
                city TEXT,
                state TEXT,
                murders2014 INTEGER,
                murders2015 INTEGER,
                change INTEGER
            )",
            [],
        )?;

        let mut insert_stmt = conn.prepare("INSERT INTO Murder2015 VALUES (?, ?, ?, ?, ?)")?;
        for result in rdr.records() {
            let record = result?;
            insert_stmt.execute(params![
                record.get(0).unwrap(),
                record.get(1).unwrap(),
                record.get(2).unwrap().parse::<i32>().unwrap(),
                record.get(3).unwrap().parse::<i32>().unwrap(),
                record.get(4).unwrap().parse::<i32>().unwrap(),
            ])?;
        }
        println!("Data successfully loaded into Murder2015.db");
        Ok(())
    }
}
