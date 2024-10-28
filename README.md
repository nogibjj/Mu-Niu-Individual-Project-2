[![CI](https://github.com/nogibjj/Mu-Niu-Individual-Project-2/actions/workflows/CI.yml/badge.svg)](https://github.com/nogibjj/Mu-Niu-Individual-Project-2/actions/workflows/CI.yml)

## Mu-Niu-Individual-Project-2

### Dataset

This student performance dataset records student id,student name, their attendance rate,and their final_grade.

### Functionalities
- **lib.rs**
  - `extract`: Reads the CSV file.
  - `transform`: Converts CSV data for database storage.
  - `create`, `read`, `update`, `delete`: Perform CRUD operations.

- **main.rs**: Extracts CSV data, converts to database, and performs CRUD.

- **test.rs**: Tests all CRUD functions.

### Install Dependencies
Dependencies are specified in `Cargo.toml`, and installed using `cargo build --release`.


### Use of LLM
- Debugging code issues and setting up configurations.
- Refine `README` file

### GitHub Actions
- **Test**: Test SQLite CRUD with `cargo test --quiet`.
- **Run**: Run code with `cargo run`.
- **Format**: Format code with `cargo fmt --quiet`.
- **Lint**: Check code with `cargo clippy --quiet`.


### Demo Video
A video walkthrough of setting up and using the Rust CLI Binary will be available at the following link. _(To be updated)_


### Binary Download Link

https://github.com/nogibjj/Mu-Niu-Individual-Project-2/actions/runs/11543659030/artifacts/2109679812