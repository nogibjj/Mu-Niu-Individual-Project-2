use psr::{create, delete, extract, read, transform, update};
use std::fs;

fn main() {
    // Delete the database file.
    let _ = fs::remove_file("student_performanceDB.db");

    extract(
        "https://raw.githubusercontent.com/nogibjj/Mu-Niu-Python-Script-Rust/refs/heads/main/data/student_performance.csv",
        "student_performance.csv",
    )
    .unwrap();

    let csv_path = "student_performance.csv";
    let db_path = "student_performanceDB.db";
    match transform(csv_path, db_path) {
        Ok(_) => println!("CSV file has been successfully converted to SQLite DB."),
        Err(e) => println!("An error occurred during conversion: {}", e),
    }

    match create("student_performanceDB.db", 11, "Linxi", 100, 100) {
        Ok(_) => println!("Successfully inserted data into the SQLite DB."),
        Err(e) => println!("Error occurred while inserting data: {}", e),
    }

    match read("student_performanceDB.db") {
        Ok(_) => println!("Successfully read from the SQLite DB."),
        Err(e) => println!("Error occurred while reading data: {}", e),
    }

    println!();

    match update("student_performanceDB.db", 11, "Linxi", 100, 110) {
        Ok(_) => println!("Successfully updated data in the SQLite DB."),
        Err(e) => println!("Error occurred while updating data: {}", e),
    }

    match read("student_performanceDB.db") {
        Ok(_) => println!("Successfully read from the SQLite DB."),
        Err(e) => println!("Error occurred while reading data: {}", e),
    }

    println!();

    let student_id_to_delete = 1;
    match delete(db_path, student_id_to_delete) {
        Ok(_) => println!(
            "Successfully deleted data for student {}.",
            student_id_to_delete
        ),
        Err(e) => println!("An error occurred: {}", e),
    }

    match read("student_performanceDB.db") {
        Ok(_) => println!("Successfully read from the SQLite DB."),
        Err(e) => println!("Error occurred while reading data: {}", e),
    }

    println!();
}
