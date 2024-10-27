#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod tests {
    use psr::{create, delete, extract, read, transform, update};
    use rusqlite::params;
    use rusqlite::Connection;
    use std::fs;
    use std::sync::Once;

    lazy_static! {
        static ref INIT: Once = Once::new();
    }

    #[test]
    fn test_extract() {
        // Define test URL and save path.
        let test_url = "https://raw.githubusercontent.com/nogibjj/Mu-Niu-Individual-Project-2/refs/heads/main/psr/tests/test.csv";
        let test_path = "test.csv";

        // Execute the extract function.
        let result = extract(test_url, test_path);

        // Check if the result is Ok(()).
        assert!(result.is_ok(), "Extract function failed with {:?}", result);

        // Check if the file was actually created.
        assert!(
            fs::metadata(test_path).is_ok(),
            "Failed to create the file at {}",
            test_path
        );
    }

    #[test]
    fn test_transform() {
        // Create sample CSV data.
        let csv_path = "test.csv";
        let db_path = "testDB.db";

        // Execute the transform function.
        let result = transform(csv_path, db_path);

        // Check if the result is Ok(()).
        assert!(
            result.is_ok(),
            "Transform function failed with {:?}",
            result
        );
    }

    #[test]
    fn test_create() {
        let db_path = "testDB.db";

        // Create data in the database.
        let result = create(db_path, 300, "Oliver", 20, 100);
        assert!(result.is_ok(), "Create function failed with {:?}", result);

        // Verify the created data.
        let conn = Connection::open(db_path).unwrap();
        let final_grade: i32 = conn
            .query_row(
                "SELECT final_grade FROM data WHERE student_id = 300 AND name = 'Oliver' AND attendance_rate = 20",
                params![],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(final_grade, 100);
    }

    #[test]
    fn test_read() {
        let db_path = "testDB.db";

        let conn = Connection::open(db_path).unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS data (student_id INTEGER, name TEXT, attendance_rate INTEGER, final_grade INTEGER)",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO data (student_id, name, attendance_rate, final_grade) VALUES (400, 'Paul', 24, 13)",
            [],
        )
        .unwrap();

        // Read data from the database.
        let result = read(db_path);
        assert!(result.is_ok(), "Read function failed with {:?}", result);
    }

    #[test]
    fn test_update() {
        let db_path = "testDB.db";
        let conn = Connection::open(db_path).unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS data (student_id INTEGER, name TEXT, attendance_rate INTEGER, final_grade INTEGER)",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO data (student_id, name, attendance_rate, final_grade) VALUES (500, 'George', 24, 8)",
            [],
        )
        .unwrap();

        // Update data in the database.
        let result = update(db_path, 500, "George", 24, 29);
        assert!(result.is_ok(), "Update function failed with {:?}", result);

        // Verify the updated data.
        let student_id: i32 = conn
            .query_row(
                "SELECT student_id FROM data WHERE name = 'George' AND attendance_rate = 24 AND final_grade = 29",
                params![],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(student_id, 500);
    }

    #[test]
    fn test_delete() {
        let db_path = "testDB.db";
        let conn = Connection::open(db_path).unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS data (student_id INTEGER, name TEXT, attendance_rate INTEGER, final_grade INTEGER)",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO data (student_id, name, attendance_rate, final_grade) VALUES (600, 'Coop', 2, 31)",
            [],
        )
        .unwrap();

        // Delete data from the database.
        let result = delete(db_path, 600);
        assert!(result.is_ok(), "Delete function failed with {:?}", result);

        // Verify the data was properly deleted.
        let count: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM data WHERE student_id = 600",
                params![],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 0);
    }
}
