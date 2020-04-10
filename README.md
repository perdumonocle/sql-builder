# sql-builder

[![Build Status](https://travis-ci.org/perdumonocle/sql-builder.svg)](https://travis-ci.org/perdumonocle/sql-builder)
[![Latest Version](https://img.shields.io/crates/v/sql-builder.svg)](https://crates.io/crates/sql-builder)
[![Docs](https://docs.rs/sql-builder/badge.svg)](https://docs.rs/sql-builder)

Simple SQL code generator.

## Usage

To use `sql-builder`, first add this to your `Cargo.toml`:

```toml
[dependencies]
sql-builder = "0.11"
```

Example:

```rust
use sql_builder::SqlBuilder;

let sql = SqlBuilder::select_from("company")
    .field("id")
    .field("name")
    .and_where_gt("salary", 25000)
    .sql()?;

assert_eq!("SELECT id, name FROM company WHERE salary > 25000;", &sql);
```

```rust
use sql_builder::prelude::*;

let sql = SqlBuilder::select_from("company")
    .fields(&["id", "name"])
    .and_where("salary BETWEEN ? AND ?".binds(&[&10000, &25000]))
    .and_where("staff BETWEEN ? AND ?".bind(&100).bind(&200))
    .sql()?;

assert_eq!("SELECT id, name FROM company WHERE (salary BETWEEN 10000 AND 25000) AND (staff BETWEEN 100 AND 200);", &sql);
```

See [more examples](https://docs.rs/sql-builder/0.11.3/sql_builder/struct.SqlBuilder.html)

## SQL support

### Statements

- SELECT
- INSERT
- UPDATE
- DELETE

### Operations

- join
- distinct
- group by
- order by
- where
- union
- limit, offset
- subquery

### Functions

- escape
- quote, double quote, back quote
- bind, binds

## License

This project is licensed under the [MIT license](LICENSE).
