# sql-builder

[![Build Status](https://travis-ci.org/perdumonocle/sql-builder.svg)](https://travis-ci.org/perdumonocle/sql-builder)
[![Latest Version](https://img.shields.io/crates/v/sql-builder.svg)](https://crates.io/crates/sql-builder)
[![Docs](https://docs.rs/sql-builder/badge.svg)](https://docs.rs/sql-builder)

Simple SQL code generator.

## Usage

To use `sql-builder`, add this to your `Cargo.toml`:

```toml
[dependencies]
sql-builder = "3.1"
```

# Examples:

## SELECT

```rust
use sql_builder::SqlBuilder;

let sql = SqlBuilder::select_from("company")
    .field("id")
    .field("name")
    .and_where_gt("salary", 25_000)
    .sql()?;

assert_eq!("SELECT id, name FROM company WHERE salary > 25000;", &sql);
```

```rust
use sql_builder::prelude::*;

let sql = SqlBuilder::select_from("company")
    .fields(&["id", "name"])
    .and_where("salary BETWEEN ? AND ?".binds(&[&10_000, &25_000]))
    .and_where("staff BETWEEN ? AND ?".bind(&100).bind(&200))
    .sql()?;

assert_eq!("SELECT id, name FROM company WHERE (salary BETWEEN 10000 AND 25000) AND (staff BETWEEN 100 AND 200);", &sql);
```

## INSERT

```rust
use sql_builder::{SqlBuilder, quote};

let sql = SqlBuilder::insert_into("company")
    .field("name")
    .field("salary")
    .field("staff")
    .values(&[&quote("D&G"), &10_000.to_string(), &100.to_string()])
    .values(&[&quote("G&D"), &25_000.to_string(), &200.to_string()])
    .sql()?;

assert_eq!("INSERT INTO company (name, salary, staff) VALUES ('D&G', 10000, 100), ('G&D', 25000, 200);", &sql);
```

```rust
use sql_builder::prelude::*;

let sql = SqlBuilder::insert_into("company")
    .field("name")
    .field("salary")
    .field("staff")
    .values(&["$1, ?, ?"])
    .values(&["$2, ?, ?"])
    .sql()?
    .bind_nums(&[&"D&G", &"G&D"])
    .binds(&[&10_000, &100]);

assert_eq!("INSERT INTO company (name, salary, staff) VALUES ('D&G', 10000, 100), ('G&D', 10000, 100);", &sql);
```

## UPDATE

```rust
use sql_builder::SqlBuilder;

let sql = SqlBuilder::update_table("company")
    .set("salary", "salary + 100")
    .and_where_lt("salary", 1_000)
    .sql()?;

assert_eq!("UPDATE company SET salary = salary + 100 WHERE salary < 1000;", &sql);
```

```rust
use sql_builder::prelude::*;

let sql = SqlBuilder::update_table("company")
    .set("salary", "salary + $1")
    .set("comment", &quote("up $1$$"))
    .and_where("salary < ?".bind(&1_000))
    .sql()?
    .bind_nums(&[&100]);

assert_eq!("UPDATE company SET salary = salary + 100, comment = 'up 100$' WHERE salary < 1000;", &sql);
```

## DELETE

```rust
use sql_builder::SqlBuilder;

let sql = SqlBuilder::delete_from("company")
    .or_where_lt("salary", 1_000)
    .or_where_gt("salary", 25_000)
    .sql()?;

assert_eq!("DELETE FROM company WHERE salary < 1000 OR salary > 25000;", &sql);
```

```rust
use sql_builder::prelude::*;
use std::collections::HashMap;

let mut names: HashMap<&str, &dyn SqlArg> = HashMap::new();
names.insert("min", &1_000);
names.insert("max", &25_000);

let sql = SqlBuilder::delete_from("company")
    .and_where("salary >= :min:")
    .and_where("salary <= :max:")
    .sql()?
    .bind_names(&names);

assert_eq!("DELETE FROM company WHERE (salary >= 1000) AND (salary <= 25000);", &sql);
```

See [more examples](https://docs.rs/sql-builder/3.1.1/sql_builder/struct.SqlBuilder.html)

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
- quote, double quote, back quote, brackets quote
- bind, binds, bind\_num, bind\_nums, bind\_name, bind\_names

### Macroes

- name, qname, baname, brname, dname

## License

This project is licensed under the [MIT license](LICENSE).
