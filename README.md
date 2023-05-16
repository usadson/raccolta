# Raccolta
A database engine & library.

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
