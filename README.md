# sql-builder

Simple SQL code generator.

## Usage

To use `sql-builder`, first add this to your `Cargo.toml`:

```toml
[dependencies]
sql-builder = "0.1"
```

Next, add this to your crate:

```rust
extern crate sql_builder;

use sql_builder::SqlBuilder;
```

Example:

```rust
let sql = SqlBuilder::select_from("company")
    .field("id")
    .field("name")
    .and_where("salary > 25000")
    .sql()?;

assert_eq!("SELECT id, name FROM company WHERE salary > 25000;", &sql);
```

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
- limit, offset
- subquery

### Functions

- escape
- query

## License

This project is licensed under the [MIT license](LICENSE).
