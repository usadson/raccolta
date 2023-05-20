# Raccolta
A database engine & library.

## Getting Started
You can get started by invoking the shell by running `cargo run`. This will open up a shell wherein you can type statements. The shell will try to provide suggestions when typing, to make the usage simpler.

Keep in mind that not all syntax can be parsed, and not all parsed syntax can be executed at the moment.

## Design
This repository is split up into multiple crates:

1. [`raccolta-engine`](raccolta-engine/) is the RDBMS (Relation Database Management System), which executes statements and manages the tables
2. [`raccolta-shell`](raccolta-shell/) is a shell for the engine, wherein statements can be executed and errors are highlighted.
3. [`raccolta-syntax`](raccolta-syntax/) is the lexer and parser of SQL, which parses statements, clauses, specifications, and expressions.

The typical workflow of the RDBMS is the following:
1. The `shell` starts up, which waits for statements to be fed into.
2. The `shell` forwards these statements to `racccolta-syntax`, which parses them.
3. When an error occurs, `shell` logically displays this error, accompanied by providing help and hints leveraged by the [`StatementParseError`s](raccolta-syntax/src/parse.rs) system of `raccolta-syntax`.
4. When no error occurs, the statement is forwarded to `raccolta-engine`, which validates the statement of correct references and data types, and executes the statement.
5. `shell` displays the messages received from `raccolta-engine` and displays the resulting table.

## Goals
This section describes goals for the library and application, including the things I want to define out-of-scope.

### Philosophical
1. Reserving identifiers that are widely used in SQL dialects (`AUTO_INCREMENT`, `TOP`, etc.)
2. Having good error messages, clearly describing the error: what went wrong, how can you fix it, and other hints and helpful information to aid the user in the usage of SQL and Raccolta. A good example of this is the `Rust` error messages, on the other hand, a bad example is the MySQL error messages (at least when they were in my day, e.g. simply stating "Syntax Error at `<last character>`")

### Features
1. Creating databases, schemas and tables.
2. Inserting data with `INSERT INTO` statements into tables.
3. `SELECT`ing data from tables and expressions
4. Ordering results (`ORDER BY`)
5. Grouping results (`GROUP BY`)
6. Filtering (`WHERE` and `HAVING`)
7. Getting RDBMS system information

### Non-goals
1. Full interoperability with some or all existing SQL libraries and/or applications. This is simply not feasible, given the scope of these software works.
2. Stored procedures and functions
3. Table or database statistics
4. Multiple engines or (existing) plugin support by a third party.
5. Access Control (e.g. `GRANT` permissions)
