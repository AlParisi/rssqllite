# Simple SQL Database Clone in Rust

This project is a basic SQL-lite database clone implemented in Rust, inspired by [cstack's db_tutorial](https://cstack.github.io/db_tutorial/), a guide for building a simple database in C. The goal of this project is to understand the fundamental concepts of a database system, including storage, querying, and indexing, using Rust.

## Project Overview

Our Rust-based database implementation draws from the key components demonstrated in `db_tutorial`, enhancing and reinterpreting them with Rust's unique language features and memory safety principles. The database is designed to support a single table with a fixed schema, providing basic functionality for inserting and selecting rows. This structure is ideal for learning the internals of database operations and exploring B-tree data structures for indexing.

### Features

- **Insert and Select Operations**: Basic commands allow the insertion of rows and selection of all rows in the table.
- **In-Memory B-Tree Indexing**: The project utilizes a B-tree to store and efficiently search rows by ID. The B-tree ensures balanced data access, enabling faster query performance as the database grows.
- **Command-Line Interface**: Users can interact with the database through a simple CLI, inspired by the db_tutorial’s structure.

### Project Structure

- **Row**: Represents the schema for each row, including fields such as `id`, `username`, and `email`.
- **B-Tree**: A balanced tree data structure used to store rows and provide efficient indexing. Each node holds keys and pointers to its child nodes.
- **Table**: Acts as the main structure that stores rows in a B-tree and provides interfaces for commands like `insert` and `select`.

### Example Usage

To run the database, clone the repository and build the project with `cargo build`, then start the CLI with `cargo run`. From there, you can enter commands in the format shown below:

```plaintext
db > insert 1 user1 user1@example.com
db > insert 2 user2 user2@example.com
db > select
```

### Commands Supported
- **Insert**: Adds a new row to the table. Example: insert 1 username email@example.com
- **Select**: Retrieves and displays all rows in the table.
- **.exit**: Exits the CLI.

### Getting Started

#### Prerequisites
- ***Rust*** (v1.50 or later is recommended)
- ***Cargo*** (Rust’s package manager)

### Installation

1. Clone the repository:

```bash
git clone https://github.com/yourusername/rust-db-clone.git
cd rust-db-clone
```

2. Build the project:
```bash
cargo build
```
3. Run the CLI:

```bash
cargo run
```

### Development Notes

This project was developed as a learning exercise, with much of the implementation inspired by db_tutorial. We aimed to preserve the simplicity of the original while exploring Rust's memory safety and ownership model, which provides certain advantages, especially in handling pointers and preventing data races.

### Future Improvements
This database prototype provides a foundation for exploring more advanced database concepts, such as:

- Persistent storage on disk
- Support for variable-length columns and multiple tables
- Query optimization and advanced SQL features

### Acknowledgments
Special thanks to cstack for providing an excellent tutorial, which served as the foundation and inspiration for this Rust implementation.