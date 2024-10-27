use rusqlite::{params, Connection, Result};
use std::fs::File;
use std::io::Write;

pub fn extract(url: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Fetch data from the URL via an HTTP request.
    let response = reqwest::blocking::get(url)?;

    // Read the content of the fetched data.
    let content = response.text()?;

    // Create a file at the specified path and write the content.
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

pub fn transform(csv_path: &str, db_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Open the CSV file.
    let mut rdr = csv::Reader::from_path(csv_path)?;

    // Create or connect to an SQLite database file.
    let conn = Connection::open(db_path)?;

    // Create the appropriate table. This example assumes a simple structure.
    // Adjust the table structure according to your actual use case.
    conn.execute(
        "CREATE TABLE IF NOT EXISTS data (student_id INTEGER, name TEXT, attendance_rate INTEGER, final_grade INTEGER)", 
        [],
    )?;

    for result in rdr.deserialize() {
        let (student_id, name, attendance_rate, final_grade): (i32, String, i32, i32) = result?;
        conn.execute(
            "INSERT INTO data (student_id, name, attendance_rate, final_grade) VALUES (?1, ?2, ?3, ?4)",
            params![student_id, name, attendance_rate, final_grade],
        )?;
    }

    Ok(())
}

pub fn create(
    db_path: &str,
    student_id: i32,
    name: &str,
    attendance_rate: i32,
    final_grade: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the SQLite database file.
    let conn = rusqlite::Connection::open(db_path)?;

    // Insert data into the `data` table.
    conn.execute(
        "INSERT INTO data (student_id, name, attendance_rate, final_grade) VALUES (?1, ?2, ?3, ?4)",
        params![student_id, name, attendance_rate, final_grade],
    )?;

    println!("Data successfully created into the database.");

    Ok(())
}

pub fn read(db_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the SQLite database file.
    let conn = rusqlite::Connection::open(db_path)?;

    // Execute the query and fetch the results.
    let mut stmt = conn.prepare("SELECT student_id, name, attendance_rate, final_grade FROM data")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>("student_id")?,
            row.get::<_, String>("name")?,
            row.get::<_, i32>("attendance_rate")?,
            row.get::<_, i32>("final_grade")?,
        ))
    })?;

    // Print the results.
    for row_result in rows {
        let (student_id, name, attendance_rate, final_grade) = row_result?;
        println!("{}, {}, {}, {}", student_id, name, attendance_rate, final_grade);
    }

    Ok(())
}

pub fn update(
    db_path: &str,
    student_id: i32,
    name: &str,
    attendance_rate: i32,
    final_grade: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the SQLite database file.
    let conn = rusqlite::Connection::open(db_path)?;

    // Update the number of passengers for the specified year and month.
    let rows_modified = conn.execute(
        "UPDATE data SET final_grade = ?4 WHERE student_id = ?1 AND name = ?2 AND attendance_rate = ?3",
        params![student_id, name, attendance_rate, final_grade],
    )?;

    if rows_modified == 0 {
        println!("No data found for the specified year and month.");
    } else {
        println!("Data successfully updated in the database.");
    }

    Ok(())
}

pub fn delete(db_path: &str, student_id: i32) -> Result<()> {
    let conn = Connection::open(db_path)?;

    conn.execute("DELETE FROM data WHERE student_id = ?1", params![student_id])?;

    Ok(())
}