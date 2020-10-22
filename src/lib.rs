//! Simple SQL code generator.
//!
//! ## Usage
//!
//! To use `sql-builder`, first add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! sql-builder = "3.1"
//! ```
//!
//! # Examples:
//!
//! ## SELECT
//!
//! ```
//! use sql_builder::SqlBuilder;
//! # use anyhow::Result;
//!
//! # fn main() -> Result<()> {
//! let sql = SqlBuilder::select_from("company")
//!     .field("id")
//!     .field("name")
//!     .and_where_gt("salary", 25_000)
//!     .sql()?;
//!
//! assert_eq!("SELECT id, name FROM company WHERE salary > 25000;", &sql);
//! # Ok(())
//! # }
//! ```
//!
//! ```
//! # use anyhow::Result;
//! use sql_builder::prelude::*;
//!
//! # fn main() -> Result<()> {
//! let sql = SqlBuilder::select_from("company")
//!     .fields(&["id", "name"])
//!     .and_where("salary BETWEEN ? AND ?".binds(&[&10_000, &25_000]))
//!     .and_where("staff BETWEEN ? AND ?".bind(&100).bind(&200))
//!     .sql()?;
//!
//! assert_eq!("SELECT id, name FROM company WHERE (salary BETWEEN 10000 AND 25000) AND (staff BETWEEN 100 AND 200);", &sql);
//! # Ok(())
//! # }
//! ```
//!
//! ## INSERT
//!
//! ```
//! use sql_builder::{SqlBuilder, quote};
//! # use anyhow::Result;
//!
//! # fn main() -> Result<()> {
//! let sql = SqlBuilder::insert_into("company")
//!     .field("name")
//!     .field("salary")
//!     .field("staff")
//!     .values(&[&quote("D&G"), &10_000.to_string(), &100.to_string()])
//!     .values(&[&quote("G&D"), &25_000.to_string(), &200.to_string()])
//!     .sql()?;
//!
//! assert_eq!("INSERT INTO company (name, salary, staff) VALUES ('D&G', 10000, 100), ('G&D', 25000, 200);", &sql);
//! # Ok(())
//! # }
//! ```
//!
//! ```
//! use sql_builder::prelude::*;
//! # use anyhow::Result;
//!
//! # fn main() -> Result<()> {
//! let sql = SqlBuilder::insert_into("company")
//!     .field("name")
//!     .field("salary")
//!     .field("staff")
//!     .values(&["$1, ?, ?"])
//!     .values(&["$2, ?, ?"])
//!     .sql()?
//!     .bind_nums(&[&"D&G", &"G&D"])
//!     .binds(&[&10_000, &100]);
//!
//! assert_eq!("INSERT INTO company (name, salary, staff) VALUES ('D&G', 10000, 100), ('G&D', 10000, 100);", &sql);
//! # Ok(())
//! # }
//! ```
//!
//! ## UPDATE
//!
//! ```
//! use sql_builder::SqlBuilder;
//! # use anyhow::Result;
//!
//! # fn main() -> Result<()> {
//! let sql = SqlBuilder::update_table("company")
//!     .set("salary", "salary + 100")
//!     .and_where_lt("salary", 1_000)
//!     .sql()?;
//!
//! assert_eq!("UPDATE company SET salary = salary + 100 WHERE salary < 1000;", &sql);
//! # Ok(())
//! # }
//! ```
//!
//! ```
//! use sql_builder::prelude::*;
//! # use anyhow::Result;
//!
//! # fn main() -> Result<()> {
//! let sql = SqlBuilder::update_table("company")
//!     .set("salary", "salary + $1")
//!     .set("comment", &quote("up $1$$"))
//!     .and_where("salary < ?".bind(&1_000))
//!     .sql()?
//!     .bind_nums(&[&100]);
//!
//! assert_eq!("UPDATE company SET salary = salary + 100, comment = 'up 100$' WHERE salary < 1000;", &sql);
//! # Ok(())
//! # }
//! ```
//!
//! ## DELETE
//!
//! ```
//! use sql_builder::SqlBuilder;
//! # use anyhow::Result;
//!
//! # fn main() -> Result<()> {
//! let sql = SqlBuilder::delete_from("company")
//!     .or_where_lt("salary", 1_000)
//!     .or_where_gt("salary", 25_000)
//!     .sql()?;
//!
//! assert_eq!("DELETE FROM company WHERE salary < 1000 OR salary > 25000;", &sql);
//! # Ok(())
//! # }
//! ```
//!
//! ```
//! use sql_builder::prelude::*;
//! use std::collections::HashMap;
//! # use anyhow::Result;
//!
//! # fn main() -> Result<()> {
//! let mut names: HashMap<&str, &dyn SqlArg> = HashMap::new();
//! names.insert("min", &1_000);
//! names.insert("max", &25_000);
//!
//! let sql = SqlBuilder::delete_from("company")
//!     .and_where("salary >= :min:")
//!     .and_where("salary <= :max:")
//!     .sql()?
//!     .bind_names(&names);
//!
//! assert_eq!("DELETE FROM company WHERE (salary >= 1000) AND (salary <= 25000);", &sql);
//! # Ok(())
//! # }
//! ```
//!
//! See [more examples](https://docs.rs/sql-builder/3.1.1/sql_builder/struct.SqlBuilder.html)

pub mod arg;
pub mod bind;
pub mod error;
pub mod name;
pub mod prelude;

pub use crate::error::SqlBuilderError;
pub use crate::name::SqlName;
use anyhow::Result;

/// Main SQL builder
#[derive(Clone)]
pub struct SqlBuilder {
    statement: Statement,
    table: String,
    join_natural: bool,
    join_operator: JoinOperator,
    joins: Vec<String>,
    distinct: bool,
    fields: Vec<String>,
    sets: Vec<String>,
    values: Values,
    returning: Option<String>,
    group_by: Vec<String>,
    having: Option<String>,
    unions: String,
    wheres: Vec<String>,
    order_by: Vec<String>,
    limit: Option<String>,
    offset: Option<String>,
}

/// SQL query statement
#[derive(Clone)]
enum Statement {
    SelectFrom,
    SelectValues,
    UpdateTable,
    InsertInto,
    DeleteFrom,
}

/// Operator for JOIN
#[derive(Clone)]
enum JoinOperator {
    Join,
    LeftJoin,
    LeftOuterJoin,
    RightJoin,
    RightOuterJoin,
    InnerJoin,
    CrossJoin,
}

/// INSERT values
#[derive(Clone)]
enum Values {
    Empty,
    List(Vec<String>),
    Select(String),
}

impl SqlBuilder {
    /// Default constructor for struct
    fn default() -> Self {
        Self {
            statement: Statement::SelectFrom,
            table: String::new(),
            join_natural: false,
            join_operator: JoinOperator::Join,
            joins: Vec::new(),
            distinct: false,
            fields: Vec::new(),
            sets: Vec::new(),
            values: Values::Empty,
            returning: None,
            group_by: Vec::new(),
            having: None,
            unions: String::new(),
            wheres: Vec::new(),
            order_by: Vec::new(),
            limit: None,
            offset: None,
        }
    }

    /// Create SELECT query.
    /// You may specify comma separted list of tables.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .and_where("price > 100")
    ///     .and_where_like_left("title", "Harry Potter")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE (price > 100) AND (title LIKE 'Harry Potter%');", &sql);
    /// // add                               ^^^^^
    /// // here                              table
    /// # Ok(())
    /// # }
    /// ```
    pub fn select_from<S: ToString>(table: S) -> Self {
        Self {
            table: table.to_string(),
            ..Self::default()
        }
    }

    /// SELECT from additional table.
    /// Adds table name to comma separted list of tables.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .and_table("newspapers")
    ///     .field("title")
    ///     .field("price")
    ///     .and_where("price > 100")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books, newspapers WHERE price > 100;", &sql);
    /// // add                                      ^^^^^^^^^^
    /// // here                                       table
    /// # Ok(())
    /// # }
    /// ```
    pub fn and_table<S: ToString>(&mut self, table: S) -> &mut Self {
        self.table = format!("{}, {}", self.table, table.to_string());
        self
    }

    /// Create SELECT query without a table.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::{SqlBuilder, quote};
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_values(&["10", &quote("100")])
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT 10, '100';", &sql);
    /// // add             ^^^^^^^^^
    /// // here             values
    /// # Ok(())
    /// # }
    /// ```
    pub fn select_values<S: ToString>(values: &[S]) -> Self {
        let mut sel = Self {
            statement: Statement::SelectValues,
            ..Self::default()
        };
        sel.fields(values);
        sel
    }

    /// Create INSERT query.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::{SqlBuilder, quote};
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::insert_into("books")
    ///     .field("title")
    ///     .field("price")
    ///     .values(&[quote("In Search of Lost Time"), 150.to_string()])
    ///     .values(&["'Don Quixote', 200"])
    ///     .sql()?;
    ///
    /// assert_eq!("INSERT INTO books (title, price) VALUES ('In Search of Lost Time', 150), ('Don Quixote', 200);", &sql);
    /// // add                  ^^^^^
    /// // here                 table
    /// # Ok(())
    /// # }
    /// ```
    pub fn insert_into<S: ToString>(table: S) -> Self {
        Self {
            statement: Statement::InsertInto,
            table: table.to_string(),
            ..Self::default()
        }
    }

    /// Create UPDATE query.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::update_table("books")
    ///     .set("price", "price + 10")
    ///     .sql()?;
    ///
    /// assert_eq!("UPDATE books SET price = price + 10;", &sql);
    /// // add             ^^^^^
    /// // here            table
    /// # Ok(())
    /// # }
    /// ```
    pub fn update_table<S: ToString>(table: S) -> Self {
        Self {
            statement: Statement::UpdateTable,
            table: table.to_string(),
            ..Self::default()
        }
    }

    /// Create DELETE query.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::delete_from("books")
    ///     .and_where("price > 100")
    ///     .sql()?;
    ///
    /// assert_eq!("DELETE FROM books WHERE price > 100;", &sql);
    /// // add                  ^^^^^
    /// // here                 table
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete_from<S: ToString>(table: S) -> Self {
        Self {
            statement: Statement::DeleteFrom,
            table: table.to_string(),
            ..Self::default()
        }
    }

    /// Use NATURAL JOIN
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("total")
    ///     .natural()
    ///     .join("orders")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, total FROM books NATURAL JOIN orders;", &sql);
    /// // add here                                ^^^^^^^
    /// # Ok(())
    /// # }
    /// ```
    pub fn natural(&mut self) -> &mut Self {
        self.join_natural = true;
        self
    }

    /// Use LEFT JOIN
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("total")
    ///     .natural()
    ///     .left()
    ///     .join("orders")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, total FROM books NATURAL LEFT JOIN orders;", &sql);
    /// // add here                                        ^^^^
    /// # Ok(())
    /// # }
    /// ```
    pub fn left(&mut self) -> &mut Self {
        self.join_operator = JoinOperator::LeftJoin;
        self
    }

    /// Use LEFT OUTER JOIN
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("total")
    ///     .natural()
    ///     .left_outer()
    ///     .join("orders")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, total FROM books NATURAL LEFT OUTER JOIN orders;", &sql);
    /// // add here                                        ^^^^^^^^^^
    /// # Ok(())
    /// # }
    /// ```
    pub fn left_outer(&mut self) -> &mut Self {
        self.join_operator = JoinOperator::LeftOuterJoin;
        self
    }

    /// Use RIGHT JOIN
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("total")
    ///     .natural()
    ///     .right()
    ///     .join("orders")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, total FROM books NATURAL RIGHT JOIN orders;", &sql);
    /// // add here                                        ^^^^^
    /// # Ok(())
    /// # }
    /// ```
    pub fn right(&mut self) -> &mut Self {
        self.join_operator = JoinOperator::RightJoin;
        self
    }

    /// Use RIGHT OUTER JOIN
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("total")
    ///     .natural()
    ///     .right_outer()
    ///     .join("orders")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, total FROM books NATURAL RIGHT OUTER JOIN orders;", &sql);
    /// // add here                                        ^^^^^^^^^^^
    /// # Ok(())
    /// # }
    /// ```
    pub fn right_outer(&mut self) -> &mut Self {
        self.join_operator = JoinOperator::RightOuterJoin;
        self
    }

    /// Use INNER JOIN
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("total")
    ///     .natural()
    ///     .inner()
    ///     .join("orders")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, total FROM books NATURAL INNER JOIN orders;", &sql);
    /// // add here                                        ^^^^^
    /// # Ok(())
    /// # }
    /// ```
    pub fn inner(&mut self) -> &mut Self {
        self.join_operator = JoinOperator::InnerJoin;
        self
    }

    /// Use CROSS JOIN
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("total")
    ///     .natural()
    ///     .cross()
    ///     .join("orders")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, total FROM books NATURAL CROSS JOIN orders;", &sql);
    /// // add here                                        ^^^^^
    /// # Ok(())
    /// # }
    /// ```
    pub fn cross(&mut self) -> &mut Self {
        self.join_operator = JoinOperator::CrossJoin;
        self
    }

    /// Join with table.
    ///
    /// ```
    /// #[macro_use] extern crate sql_builder;
    /// # use anyhow::Result;
    /// use sql_builder::{SqlBuilder, SqlName};
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from(name!("books"; "b"))
    ///     .field("b.title")
    ///     .field("s.total")
    ///     .left()
    ///     .join(name!("shops"; "s"))
    ///     .on("b.id = s.book")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT b.title, s.total FROM books AS b LEFT JOIN shops AS s ON b.id = s.book;", &sql);
    /// // add                                                        ^^^^^^^^^^
    /// // here                                                         table
    /// # Ok(())
    /// # }
    /// ```
    pub fn join<S: ToString>(&mut self, table: S) -> &mut Self {
        let mut text = match &self.join_operator {
            JoinOperator::Join if self.join_natural => "NATURAL JOIN ",
            JoinOperator::Join => "JOIN ",
            JoinOperator::LeftJoin if self.join_natural => "NATURAL LEFT JOIN ",
            JoinOperator::LeftJoin => "LEFT JOIN ",
            JoinOperator::LeftOuterJoin if self.join_natural => "NATURAL LEFT OUTER JOIN ",
            JoinOperator::LeftOuterJoin => "LEFT OUTER JOIN ",
            JoinOperator::RightJoin if self.join_natural => "NATURAL RIGHT JOIN ",
            JoinOperator::RightJoin => "RIGHT JOIN ",
            JoinOperator::RightOuterJoin if self.join_natural => "NATURAL RIGHT OUTER JOIN ",
            JoinOperator::RightOuterJoin => "RIGHT OUTER JOIN ",
            JoinOperator::InnerJoin if self.join_natural => "NATURAL INNER JOIN ",
            JoinOperator::InnerJoin => "INNER JOIN ",
            JoinOperator::CrossJoin if self.join_natural => "NATURAL CROSS JOIN ",
            JoinOperator::CrossJoin => "CROSS JOIN ",
        }
        .to_string();

        self.join_natural = false;

        text.push_str(&table.to_string());

        self.joins.push(text);
        self
    }

    /// Join constraint to the last JOIN part.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books AS b")
    ///     .field("b.title")
    ///     .field("s.total")
    ///     .join("shops AS s")
    ///     .on("b.id = s.book")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT b.title, s.total FROM books AS b JOIN shops AS s ON b.id = s.book;", &sql);
    /// // add                                                                 ^^^^^^^^^^^^^
    /// // here                                                                 constraint
    /// # Ok(())
    /// # }
    /// ```
    pub fn on<S: ToString>(&mut self, constraint: S) -> &mut Self {
        if let Some(last) = self.joins.last_mut() {
            last.push_str(" ON ");
            last.push_str(&constraint.to_string());
        }
        self
    }

    /// Join constraint to the last JOIN part.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books AS b")
    ///     .field("b.title")
    ///     .field("s.total")
    ///     .join("shops AS s")
    ///     .on_eq("b.id", "s.book")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT b.title, s.total FROM books AS b JOIN shops AS s ON b.id = s.book;", &sql);
    /// // add                                                                 ^^^^   ^^^^^^
    /// // here                                                                 c1      c2
    /// # Ok(())
    /// # }
    /// ```
    pub fn on_eq<S: ToString, T: ToString>(&mut self, c1: S, c2: T) -> &mut Self {
        if let Some(last) = self.joins.last_mut() {
            last.push_str(" ON ");
            last.push_str(&c1.to_string());
            last.push_str(" = ");
            last.push_str(&c2.to_string());
        }
        self
    }

    /// Set DISTINCT for fields.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .distinct()
    ///     .field("price")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT DISTINCT price FROM books;", &sql);
    /// // add here        ^^^^^^^^
    /// # Ok(())
    /// # }
    /// ```
    pub fn distinct(&mut self) -> &mut Self {
        self.distinct = true;
        self
    }

    /// Add fields.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .fields(&["title", "price"])
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books;", &sql);
    /// // add             ^^^^^^^^^^^^
    /// // here               fields
    /// # Ok(())
    /// # }
    /// ```
    pub fn fields<S: ToString>(&mut self, fields: &[S]) -> &mut Self {
        let mut fields = fields
            .iter()
            .map(|f| (*f).to_string())
            .collect::<Vec<String>>();
        self.fields.append(&mut fields);
        self
    }

    /// Replace fields.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    /// # #[derive(Default)]
    /// # struct ReqData { filter: Option<String>, price_min: Option<u64>, price_max: Option<u64>,
    /// # limit: Option<usize>, offset: Option<usize> }
    ///
    /// # fn main() -> Result<()> {
    /// # let req_data = ReqData::default();
    /// // Prepare query for total count
    ///
    /// let mut db = SqlBuilder::select_from("books");
    ///
    /// db.field("COUNT(id)");
    ///
    /// if let Some(filter) = &req_data.filter {
    ///   db.and_where_like_any("LOWER(title)", filter.to_lowercase());
    /// }
    ///
    /// if let Some(price_min) = &req_data.price_min {
    ///   db.and_where_ge("price", price_min);
    /// }
    ///
    /// if let Some(price_max) = &req_data.price_max {
    ///   db.and_where_le("price", price_max);
    /// }
    ///
    /// let sql_count = db.sql()?;
    /// println!("Database query: total_count: {}", &sql_count);
    ///
    /// // Prepare query for results
    ///
    /// db.set_fields(&["id", "title", "price"]);
    ///
    /// if let (Some(limit), Some(offset)) = (req_data.limit, req_data.offset) {
    ///   db.limit(limit).offset(offset);
    /// }
    ///
    /// let sql_results = db.sql()?;
    /// println!("Database query: results: {}", &sql_results);
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_fields<S: ToString>(&mut self, fields: &[S]) -> &mut Self {
        let fields = fields
            .iter()
            .map(|f| (*f).to_string())
            .collect::<Vec<String>>();
        self.fields = fields;
        self
    }

    /// Add field.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books;", &sql);
    /// // add             ^^^^^  ^^^^^
    /// // here            field  field
    /// # Ok(())
    /// # }
    /// ```
    pub fn field<S: ToString>(&mut self, field: S) -> &mut Self {
        self.fields.push(field.to_string());
        self
    }

    /// Replace fields with choosed one.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    /// # #[derive(Default)]
    /// # struct ReqData { filter: Option<String>, price_min: Option<u64>, price_max: Option<u64>,
    /// # limit: Option<usize>, offset: Option<usize> }
    ///
    /// # fn main() -> Result<()> {
    /// # let req_data = ReqData::default();
    /// // Prepare query for total count
    ///
    /// let mut db = SqlBuilder::select_from("books");
    ///
    /// db.field("COUNT(id)");
    ///
    /// if let Some(filter) = &req_data.filter {
    ///   db.and_where_like_any("LOWER(title)", filter.to_lowercase());
    /// }
    ///
    /// if let Some(price_min) = &req_data.price_min {
    ///   db.and_where_ge("price", price_min);
    /// }
    ///
    /// if let Some(price_max) = &req_data.price_max {
    ///   db.and_where_le("price", price_max);
    /// }
    ///
    /// let sql_count = db.sql()?;
    /// println!("Database query: total_count: {}", &sql_count);
    ///
    /// // Prepare query for results
    ///
    /// db.set_field("id");
    /// db.field("title");
    /// db.field("price");
    ///
    /// if let (Some(limit), Some(offset)) = (req_data.limit, req_data.offset) {
    ///   db.limit(limit).offset(offset);
    /// }
    ///
    /// let sql_results = db.sql()?;
    /// println!("Database query: results: {}", &sql_results);
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_field<S: ToString>(&mut self, field: S) -> &mut Self {
        self.fields = vec![field.to_string()];
        self
    }

    /// Add COUNT(field).
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .count("price")
    ///     .group_by("title")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, COUNT(price) FROM books GROUP BY title;", &sql);
    /// // add                          ^^^^^
    /// // here                         field
    /// # Ok(())
    /// # }
    /// ```
    pub fn count<S: ToString>(&mut self, field: S) -> &mut Self {
        self.fields.push(format!("COUNT({})", field.to_string()));
        self
    }

    /// Add COUNT(field) AS name.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .count_as("price", "cnt")
    ///     .group_by("title")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, COUNT(price) AS cnt FROM books GROUP BY title;", &sql);
    /// // add                          ^^^^^
    /// // here                         field
    /// # Ok(())
    /// # }
    /// ```
    pub fn count_as<S, T>(&mut self, field: S, name: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        self.fields.push(format!(
            "COUNT({}) AS {}",
            field.to_string(),
            name.to_string()
        ));
        self
    }

    /// Add SET part (for UPDATE).
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::update_table("books")
    ///     .set("price", "price + 10")
    ///     .sql()?;
    ///
    /// assert_eq!("UPDATE books SET price = price + 10;", &sql);
    /// // add                       ^^^^^   ^^^^^^^^^^
    /// // here                      field     value
    /// # Ok(())
    /// # }
    /// ```
    pub fn set<S, T>(&mut self, field: S, value: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let expr = format!("{} = {}", &field.to_string(), &value.to_string());
        self.sets.push(expr);
        self
    }

    /// Add SET part with escaped string value (for UPDATE).
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::update_table("books")
    ///     .set_str("comment", "Don't distribute!")
    ///     .and_where_le("price", "100")
    ///     .sql()?;
    ///
    /// assert_eq!("UPDATE books SET comment = 'Don''t distribute!' WHERE price <= 100;", &sql);
    /// // add                       ^^^^^^^    ^^^^^^^^^^^^^^^^^^
    /// // here                       field           value
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_str<S, T>(&mut self, field: S, value: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let expr = format!("{} = '{}'", &field.to_string(), &esc(&value.to_string()));
        self.sets.push(expr);
        self
    }

    /// Add VALUES part (for INSERT).
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::{SqlBuilder, quote};
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::insert_into("books")
    ///     .field("title")
    ///     .field("price")
    ///     .values(&[quote("In Search of Lost Time"), 150.to_string()])
    ///     .values(&["'Don Quixote', 200"])
    ///     .sql()?;
    ///
    /// assert_eq!("INSERT INTO books (title, price) VALUES ('In Search of Lost Time', 150), ('Don Quixote', 200);", &sql);
    /// // add                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^    ^^^^^^^^^^^^^^^^^^
    /// // here                                                         values                      values
    /// # Ok(())
    /// # }
    /// ```
    pub fn values<S: ToString>(&mut self, values: &[S]) -> &mut Self {
        let values: Vec<String> = values
            .iter()
            .map(|v| (*v).to_string())
            .collect::<Vec<String>>();
        let values = format!("({})", values.join(", "));

        match &mut self.values {
            Values::Empty => self.values = Values::List(vec![values]),
            Values::Select(_) => self.values = Values::List(vec![values]),
            Values::List(v) => v.push(values),
        };

        self
    }

    /// Add SELECT part (for INSERT).
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let query = SqlBuilder::select_from("warehouse")
    ///     .field("title")
    ///     .field("preliminary_price * 2")
    ///     .query()?;
    ///
    /// assert_eq!("SELECT title, preliminary_price * 2 FROM warehouse", &query);
    ///
    /// let sql = SqlBuilder::insert_into("books")
    ///     .field("title")
    ///     .field("price")
    ///     .select(&query)
    ///     .sql()?;
    ///
    /// assert_eq!("INSERT INTO books (title, price) SELECT title, preliminary_price * 2 FROM warehouse;", &sql);
    /// // add                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    /// // here                                                            query
    /// # Ok(())
    /// # }
    /// ```
    pub fn select<S: ToString>(&mut self, query: S) -> &mut Self {
        self.values = Values::Select(query.to_string());
        self
    }

    /// Add RETURNING part.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::insert_into("books")
    ///     .field("title")
    ///     .field("price")
    ///     .values(&["'Don Quixote', 200"])
    ///     .returning("id")
    ///     .sql()?;
    ///
    /// assert_eq!("INSERT INTO books (title, price) VALUES ('Don Quixote', 200) RETURNING id;", &sql);
    /// // add                                                                             ^^
    /// // here                                                                           field
    /// # Ok(())
    /// # }
    /// ```
    pub fn returning<S: ToString>(&mut self, field: S) -> &mut Self {
        self.returning = Some(field.to_string());
        self
    }

    /// Add RETURNING id.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::insert_into("books")
    ///     .field("title")
    ///     .field("price")
    ///     .values(&["'Don Quixote', 200"])
    ///     .returning_id()
    ///     .sql()?;
    ///
    /// assert_eq!("INSERT INTO books (title, price) VALUES ('Don Quixote', 200) RETURNING id;", &sql);
    /// // add here                                                              ^^^^^^^^^^^^
    /// # Ok(())
    /// # }
    /// ```
    pub fn returning_id(&mut self) -> &mut Self {
        self.returning("id")
    }

    /// Add GROUP BY part.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("price")
    ///     .field("COUNT(price) AS cnt")
    ///     .group_by("price")
    ///     .order_desc("cnt")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT price, COUNT(price) AS cnt FROM books GROUP BY price ORDER BY cnt DESC;", &sql);
    /// // add                                                            ^^^^^
    /// // here                                                           field
    /// # Ok(())
    /// # }
    /// ```
    pub fn group_by<S: ToString>(&mut self, field: S) -> &mut Self {
        self.group_by.push(field.to_string());
        self
    }

    /// Add HAVING condition.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("price")
    ///     .field("COUNT(price) AS cnt")
    ///     .group_by("price")
    ///     .having("price > 100")
    ///     .order_desc("cnt")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT price, COUNT(price) AS cnt FROM books GROUP BY price HAVING price > 100 ORDER BY cnt DESC;", &sql);
    /// // add                                                                         ^^^^^^^^^^^
    /// // here                                                                           cond
    /// # Ok(())
    /// # }
    /// ```
    pub fn having<S: ToString>(&mut self, cond: S) -> &mut Self {
        self.having = Some(cond.to_string());
        self
    }

    /// Add WHERE condition.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .and_where("price > 100")
    ///     .and_where("title LIKE 'Harry Potter%'")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE (price > 100) AND (title LIKE 'Harry Potter%');", &sql);
    /// // add                                            ^^^^^^^^^^^       ^^^^^^^^^^^^^^^^^^^^^^^^^^
    /// // here                                              cond                      cond
    /// # Ok(())
    /// # }
    /// ```
    pub fn and_where<S: ToString>(&mut self, cond: S) -> &mut Self {
        self.wheres.push(cond.to_string());
        self
    }

    /// Add WHERE condition for equal parts.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::{SqlBuilder, quote};
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("price")
    ///     .and_where_eq("title", &quote("Harry Potter and the Philosopher's Stone"))
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT price FROM books WHERE title = 'Harry Potter and the Philosopher''s Stone';", &sql);
    /// // add                                    ^^^^^   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    /// // here                                   field                      value
    /// # Ok(())
    /// # }
    /// ```
    pub fn and_where_eq<S, T>(&mut self, field: S, value: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" = ");
        cond.push_str(&value.to_string());
        self.and_where(&cond)
    }

    /// Add WHERE condition for non-equal parts.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::{SqlBuilder, quote};
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("price")
    ///     .and_where_ne("title", &quote("Harry Potter and the Philosopher's Stone"))
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT price FROM books WHERE title <> 'Harry Potter and the Philosopher''s Stone';", &sql);
    /// // add                                    ^^^^^    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    /// // here                                   field                       value
    /// # Ok(())
    /// # }
    /// ```
    pub fn and_where_ne<S, T>(&mut self, field: S, value: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" <> ");
        cond.push_str(&value.to_string());
        self.and_where(&cond)
    }

    /// Add WHERE condition for field greater than value.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .and_where_gt("price", 300)
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price > 300;", &sql);
    /// // add                                           ^^^^^   ^^^
    /// // here                                          field  value
    /// # Ok(())
    /// # }
    /// ```
    pub fn and_where_gt<S, T>(&mut self, field: S, value: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" > ");
        cond.push_str(&value.to_string());
        self.and_where(&cond)
    }

    /// Add WHERE condition for field not less than value.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .and_where_ge("price", 300)
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price >= 300;", &sql);
    /// // add                                           ^^^^^    ^^^
    /// // here                                          field   value
    /// # Ok(())
    /// # }
    /// ```
    pub fn and_where_ge<S, T>(&mut self, field: S, value: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" >= ");
        cond.push_str(&value.to_string());
        self.and_where(&cond)
    }

    /// Add WHERE condition for field less than value.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .and_where_lt("price", 300)
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price < 300;", &sql);
    /// // add                                           ^^^^^   ^^^
    /// // here                                          field  value
    /// # Ok(())
    /// # }
    /// ```
    pub fn and_where_lt<S, T>(&mut self, field: S, value: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" < ");
        cond.push_str(&value.to_string());
        self.and_where(&cond)
    }

    /// Add WHERE condition for field not greater than value.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .and_where_le("price", 300)
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price <= 300;", &sql);
    /// // add                                           ^^^^^    ^^^
    /// // here                                          field   value
    /// # Ok(())
    /// # }
    /// ```
    pub fn and_where_le<S, T>(&mut self, field: S, value: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" <= ");
        cond.push_str(&value.to_string());
        self.and_where(&cond)
    }

    /// Add WHERE LIKE condition.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("price")
    ///     .and_where_like("title", "%Philosopher's%")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT price FROM books WHERE title LIKE '%Philosopher''s%';", &sql);
    /// // add                                    ^^^^^       ^^^^^^^^^^^^^^^^
    /// // here                                   field             mask
    /// # Ok(())
    /// # }
    /// ```
    pub fn and_where_like<S, T>(&mut self, field: S, mask: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" LIKE '");
        cond.push_str(&esc(&mask.to_string()));
        cond.push('\'');
        self.and_where(&cond)
    }

    /// Add WHERE LIKE %condition.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("price")
    ///     .and_where_like_right("title", "Stone")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT price FROM books WHERE title LIKE '%Stone';", &sql);
    /// // add                                    ^^^^^        ^^^^^
    /// // here                                   field        mask
    /// # Ok(())
    /// # }
    /// ```
    pub fn and_where_like_right<S, T>(&mut self, field: S, mask: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" LIKE '%");
        cond.push_str(&esc(&mask.to_string()));
        cond.push('\'');
        self.and_where(&cond)
    }

    /// Add WHERE LIKE condition%.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("price")
    ///     .and_where_like_left("title", "Harry")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT price FROM books WHERE title LIKE 'Harry%';", &sql);
    /// // add                                    ^^^^^       ^^^^^
    /// // here                                   field       mask
    /// # Ok(())
    /// # }
    /// ```
    pub fn and_where_like_left<S, T>(&mut self, field: S, mask: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" LIKE '");
        cond.push_str(&esc(&mask.to_string()));
        cond.push_str("%'");
        self.and_where(&cond)
    }

    /// Add WHERE LIKE %condition%.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("price")
    ///     .and_where_like_any("title", " and ")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT price FROM books WHERE title LIKE '% and %';", &sql);
    /// // add                                    ^^^^^        ^^^^^
    /// // here                                   field        mask
    /// # Ok(())
    /// # }
    /// ```
    pub fn and_where_like_any<S, T>(&mut self, field: S, mask: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" LIKE '%");
        cond.push_str(&esc(&mask.to_string()));
        cond.push_str("%'");
        self.and_where(&cond)
    }

    /// Add WHERE NOT LIKE condition.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .and_where_not_like("title", "%Alice's%")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title FROM books WHERE title NOT LIKE '%Alice''s%';", &sql);
    /// // add                                    ^^^^^           ^^^^^^^^^^
    /// // here                                   field              mask
    /// # Ok(())
    /// # }
    /// ```
    pub fn and_where_not_like<S, T>(&mut self, field: S, mask: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" NOT LIKE '");
        cond.push_str(&esc(&mask.to_string()));
        cond.push('\'');
        self.and_where(&cond)
    }

    /// Add WHERE NOT LIKE %condition.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("price")
    ///     .and_where_not_like_right("title", "Stone")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT price FROM books WHERE title NOT LIKE '%Stone';", &sql);
    /// // add                                    ^^^^^            ^^^^^
    /// // here                                   field            mask
    /// # Ok(())
    /// # }
    /// ```
    pub fn and_where_not_like_right<S, T>(&mut self, field: S, mask: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" NOT LIKE '%");
        cond.push_str(&esc(&mask.to_string()));
        cond.push('\'');
        self.and_where(&cond)
    }

    /// Add WHERE NOT LIKE condition%.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("price")
    ///     .and_where_not_like_left("title", "Harry")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT price FROM books WHERE title NOT LIKE 'Harry%';", &sql);
    /// // add                                    ^^^^^           ^^^^^
    /// // here                                   field           mask
    /// # Ok(())
    /// # }
    /// ```
    pub fn and_where_not_like_left<S, T>(&mut self, field: S, mask: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" NOT LIKE '");
        cond.push_str(&esc(&mask.to_string()));
        cond.push_str("%'");
        self.and_where(&cond)
    }

    /// Add WHERE NOT LIKE %condition%.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("price")
    ///     .and_where_not_like_any("title", " and ")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT price FROM books WHERE title NOT LIKE '% and %';", &sql);
    /// // add                                    ^^^^^            ^^^^^
    /// // here                                   field            mask
    /// # Ok(())
    /// # }
    /// ```
    pub fn and_where_not_like_any<S, T>(&mut self, field: S, mask: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" NOT LIKE '%");
        cond.push_str(&esc(&mask.to_string()));
        cond.push_str("%'");
        self.and_where(&cond)
    }

    /// Add WHERE IS NULL condition.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .and_where_is_null("price")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title FROM books WHERE price IS NULL;", &sql);
    /// // add                                    ^^^^^
    /// // here                                   field
    /// # Ok(())
    /// # }
    /// ```
    pub fn and_where_is_null<S: ToString>(&mut self, field: S) -> &mut Self {
        let mut cond = field.to_string();
        cond.push_str(" IS NULL");
        self.and_where(&cond)
    }

    /// Add WHERE IS NOT NULL condition.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .and_where_is_not_null("price")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title FROM books WHERE price IS NOT NULL;", &sql);
    /// // add                                    ^^^^^
    /// // here                                   field
    /// # Ok(())
    /// # }
    /// ```
    pub fn and_where_is_not_null<S: ToString>(&mut self, field: S) -> &mut Self {
        let mut cond = field.to_string();
        cond.push_str(" IS NOT NULL");
        self.and_where(&cond)
    }

    /// Add WHERE field IN (list).
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::{SqlBuilder, quote};
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .and_where_in("title", &[quote("G"), quote("L"), quote("t")])
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE title IN ('G', 'L', 't');", &sql);
    /// // add                                           ^^^^^     ^^^^^^^^^^^^^
    /// // here                                          field         list
    /// # Ok(())
    /// # }
    /// ```
    pub fn and_where_in<S, T>(&mut self, field: S, list: &[T]) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let list: Vec<String> = list
            .iter()
            .map(|v| (*v).to_string())
            .collect::<Vec<String>>();
        let list = list.join(", ");

        let mut cond = field.to_string();
        cond.push_str(" IN (");
        cond.push_str(&list);
        cond.push(')');
        self.and_where(&cond)
    }

    /// Add WHERE field IN (string list).
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::{SqlBuilder, quote};
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .and_where_in_quoted("title", &["G", "L", "t"])
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE title IN ('G', 'L', 't');", &sql);
    /// // add                                           ^^^^^     ^^^^^^^^^^^^^
    /// // here                                          field         list
    /// # Ok(())
    /// # }
    /// ```
    pub fn and_where_in_quoted<S, T>(&mut self, field: S, list: &[T]) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let list: Vec<String> = list
            .iter()
            .map(|v| quote((*v).to_string()))
            .collect::<Vec<String>>();
        let list = list.join(", ");

        let mut cond = field.to_string();
        cond.push_str(" IN (");
        cond.push_str(&list);
        cond.push(')');
        self.and_where(&cond)
    }

    /// Add WHERE field NOT IN (list).
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::{SqlBuilder, quote};
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .and_where_not_in("title", &[quote("G"), quote("L"), quote("t")])
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE title NOT IN ('G', 'L', 't');", &sql);
    /// // add                                           ^^^^^         ^^^^^^^^^^^^^
    /// // here                                          field             list
    /// # Ok(())
    /// # }
    /// ```
    pub fn and_where_not_in<S, T>(&mut self, field: S, list: &[T]) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let list: Vec<String> = list
            .iter()
            .map(|v| (*v).to_string())
            .collect::<Vec<String>>();
        let list = list.join(", ");

        let mut cond = field.to_string();
        cond.push_str(" NOT IN (");
        cond.push_str(&list);
        cond.push(')');
        self.and_where(&cond)
    }

    /// Add WHERE field NOT IN (string list).
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::{SqlBuilder, quote};
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .and_where_not_in_quoted("title", &["G", "L", "t"])
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE title NOT IN ('G', 'L', 't');", &sql);
    /// // add                                           ^^^^^         ^^^^^^^^^^^^^
    /// // here                                          field             list
    /// # Ok(())
    /// # }
    /// ```
    pub fn and_where_not_in_quoted<S, T>(&mut self, field: S, list: &[T]) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let list: Vec<String> = list
            .iter()
            .map(|v| quote((*v).to_string()))
            .collect::<Vec<String>>();
        let list = list.join(", ");

        let mut cond = field.to_string();
        cond.push_str(" NOT IN (");
        cond.push_str(&list);
        cond.push(')');
        self.and_where(&cond)
    }

    /// Add WHERE field IN (query).
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::{SqlBuilder, quote};
    ///
    /// # fn main() -> Result<()> {
    /// let query = SqlBuilder::select_from("shop")
    ///     .field("title")
    ///     .and_where("sold")
    ///     .query()?;
    ///
    /// assert_eq!("SELECT title FROM shop WHERE sold", &query);
    ///
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .and_where_in_query("title", &query)
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE title IN (SELECT title FROM shop WHERE sold);", &sql);
    /// // add                                           ^^^^^     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    /// // here                                          field                   query
    /// # Ok(())
    /// # }
    /// ```
    pub fn and_where_in_query<S, T>(&mut self, field: S, query: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" IN (");
        cond.push_str(&query.to_string());
        cond.push(')');
        self.and_where(&cond)
    }

    /// Add WHERE field NOT IN (query).
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::{SqlBuilder, quote};
    ///
    /// # fn main() -> Result<()> {
    /// let query = SqlBuilder::select_from("shop")
    ///     .field("title")
    ///     .and_where("sold")
    ///     .query()?;
    ///
    /// assert_eq!("SELECT title FROM shop WHERE sold", &query);
    ///
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .and_where_not_in_query("title", &query)
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE title NOT IN (SELECT title FROM shop WHERE sold);", &sql);
    /// // add                                           ^^^^^         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    /// // here                                          field                       query
    /// # Ok(())
    /// # }
    /// ```
    pub fn and_where_not_in_query<S, T>(&mut self, field: S, query: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" NOT IN (");
        cond.push_str(&query.to_string());
        cond.push(')');
        self.and_where(&cond)
    }

    /// Add WHERE field BETWEEN values.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .and_where_between("price", 10_000, 20_000)
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price BETWEEN 10000 AND 20000;", &sql);
    /// // add                                           ^^^^^         ^^^^^     ^^^^^
    /// // here                                          field          min       max
    /// # Ok(())
    /// # }
    /// ```
    pub fn and_where_between<S, T, U>(&mut self, field: S, min: T, max: U) -> &mut Self
    where
        S: ToString,
        T: ToString,
        U: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" BETWEEN ");
        cond.push_str(&min.to_string());
        cond.push_str(" AND ");
        cond.push_str(&max.to_string());
        self.and_where(&cond)
    }

    /// Add WHERE field NOT BETWEEN values.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .and_where_not_between("price", 10_000, 20_000)
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price NOT BETWEEN 10000 AND 20000;", &sql);
    /// // add                                           ^^^^^             ^^^^^     ^^^^^
    /// // here                                          field              min       max
    /// # Ok(())
    /// # }
    /// ```
    pub fn and_where_not_between<S, T, U>(&mut self, field: S, min: T, max: U) -> &mut Self
    where
        S: ToString,
        T: ToString,
        U: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" NOT BETWEEN ");
        cond.push_str(&min.to_string());
        cond.push_str(" AND ");
        cond.push_str(&max.to_string());
        self.and_where(&cond)
    }

    /// Add OR condition to the last WHERE condition.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .and_where("price < 10")
    ///     .or_where("price > 1000")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price < 10 OR price > 1000;", &sql);
    /// // add                                                         ^^^^^^^^^^^^
    /// // here                                                            cond
    /// # Ok(())
    /// # }
    /// ```
    pub fn or_where<S: ToString>(&mut self, cond: S) -> &mut Self {
        if self.wheres.is_empty() {
            self.wheres.push(cond.to_string());
        } else if let Some(last) = self.wheres.last_mut() {
            last.push_str(" OR ");
            last.push_str(&cond.to_string());
        }
        self
    }

    /// Add OR condition of equal parts to the last WHERE condition.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::{SqlBuilder, quote};
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("price")
    ///     .and_where_eq("title", &quote("Harry Potter and the Philosopher's Stone"))
    ///     .or_where_eq("title", &quote("Harry Potter and the Chamber of Secrets"))
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT price FROM books WHERE title = 'Harry Potter and the Philosopher''s Stone' OR title = 'Harry Potter and the Chamber of Secrets';", &sql);
    /// // add                                                                                           ^^^^^   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    /// // here                                                                                          field                     value
    /// # Ok(())
    /// # }
    /// ```
    pub fn or_where_eq<S, T>(&mut self, field: S, value: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" = ");
        cond.push_str(&value.to_string());
        self.or_where(&cond)
    }

    /// Add OR condition of non-equal parts to the last WHERE condition.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::{SqlBuilder, quote};
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("price")
    ///     .or_where_ne("title", &quote("Harry Potter and the Philosopher's Stone"))
    ///     .or_where_ne("title", &quote("Harry Potter and the Chamber of Secrets"))
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT price FROM books WHERE title <> 'Harry Potter and the Philosopher''s Stone' OR title <> 'Harry Potter and the Chamber of Secrets';", &sql);
    /// // add                                    ^^^^^    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^    ^^^^^    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    /// // here                                   field                       value                       field                      value
    /// # Ok(())
    /// # }
    /// ```
    pub fn or_where_ne<S, T>(&mut self, field: S, value: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" <> ");
        cond.push_str(&value.to_string());
        self.or_where(&cond)
    }

    /// Add OR condition for field greater than value to the last WHERE condition.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .and_where_lt("price", 100)
    ///     .or_where_gt("price", 300)
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price < 100 OR price > 300;", &sql);
    /// // add                                                          ^^^^^   ^^^
    /// // here                                                         field  value
    /// # Ok(())
    /// # }
    /// ```
    pub fn or_where_gt<S, T>(&mut self, field: S, value: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" > ");
        cond.push_str(&value.to_string());
        self.or_where(&cond)
    }

    /// Add OR condition for field not less than value to the last WHERE condition.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .or_where_lt("price", 100)
    ///     .or_where_ge("price", 300)
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price < 100 OR price >= 300;", &sql);
    /// // add                                                          ^^^^^    ^^^
    /// // here                                                         field   value
    /// # Ok(())
    /// # }
    /// ```
    pub fn or_where_ge<S, T>(&mut self, field: S, value: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" >= ");
        cond.push_str(&value.to_string());
        self.or_where(&cond)
    }

    /// Add OR condition for field less than value to the last WHERE condition.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .and_where_lt("price", 100)
    ///     .or_where_lt("price", 300)
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price < 100 OR price < 300;", &sql);
    /// // add                                                          ^^^^^   ^^^
    /// // here                                                         field  value
    /// # Ok(())
    /// # }
    /// ```
    pub fn or_where_lt<S, T>(&mut self, field: S, value: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" < ");
        cond.push_str(&value.to_string());
        self.or_where(&cond)
    }

    /// Add OR condition for field not greater than value to the last WHERE condition.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .or_where_le("price", 100)
    ///     .or_where_ge("price", 300)
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price <= 100 OR price >= 300;", &sql);
    /// // add                                           ^^^^^    ^^^
    /// // here                                          field   value
    /// # Ok(())
    /// # }
    /// ```
    pub fn or_where_le<S, T>(&mut self, field: S, value: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" <= ");
        cond.push_str(&value.to_string());
        self.or_where(&cond)
    }

    /// Add OR LIKE condition to the last WHERE condition.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("price")
    ///     .or_where_like("title", "%Alice's%")
    ///     .or_where_like("title", "%Philosopher's%")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT price FROM books WHERE title LIKE '%Alice''s%' OR title LIKE '%Philosopher''s%';", &sql);
    /// // add                                    ^^^^^      ^^^^^^^^^^^^    ^^^^^      ^^^^^^^^^^^^^^^^^^
    /// // here                                   field          mask        field             mask
    /// # Ok(())
    /// # }
    /// ```
    pub fn or_where_like<S, T>(&mut self, field: S, mask: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" LIKE '");
        cond.push_str(&esc(&mask.to_string()));
        cond.push('\'');
        self.or_where(&cond)
    }

    /// Add OR LIKE condition to the last WHERE %condition.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("price")
    ///     .or_where_like_right("title", "Alice's")
    ///     .or_where_like_right("title", "Philosopher's")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT price FROM books WHERE title LIKE '%Alice''s' OR title LIKE '%Philosopher''s';", &sql);
    /// // add                                    ^^^^^        ^^^^^^^^     ^^^^^        ^^^^^^^^^^^^^^
    /// // here                                   field          mask       field             mask
    /// # Ok(())
    /// # }
    /// ```
    pub fn or_where_like_right<S, T>(&mut self, field: S, mask: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" LIKE '%");
        cond.push_str(&esc(&mask.to_string()));
        cond.push('\'');
        self.or_where(&cond)
    }

    /// Add OR LIKE condition to the last WHERE condition%.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("price")
    ///     .or_where_like_left("title", "Alice's")
    ///     .or_where_like_left("title", "Philosopher's")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT price FROM books WHERE title LIKE 'Alice''s%' OR title LIKE 'Philosopher''s%';", &sql);
    /// // add                                    ^^^^^       ^^^^^^^^      ^^^^^       ^^^^^^^^^^^^^^
    /// // here                                   field         mask        field            mask
    /// # Ok(())
    /// # }
    /// ```
    pub fn or_where_like_left<S, T>(&mut self, field: S, mask: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" LIKE '");
        cond.push_str(&esc(&mask.to_string()));
        cond.push_str("%'");
        self.or_where(&cond)
    }

    /// Add OR LIKE condition to the last WHERE %condition%.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("price")
    ///     .or_where_like_any("title", "Alice's")
    ///     .or_where_like_any("title", "Philosopher's")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT price FROM books WHERE title LIKE '%Alice''s%' OR title LIKE '%Philosopher''s%';", &sql);
    /// // add                                    ^^^^^      ^^^^^^^^^^^^    ^^^^^      ^^^^^^^^^^^^^^^^^^
    /// // here                                   field          mask        field             mask
    /// # Ok(())
    /// # }
    /// ```
    pub fn or_where_like_any<S, T>(&mut self, field: S, mask: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" LIKE '%");
        cond.push_str(&esc(&mask.to_string()));
        cond.push_str("%'");
        self.or_where(&cond)
    }

    /// Add OR NOT LIKE condition to the last WHERE condition.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .and_where_not_like("title", "%Alice's%")
    ///     .or_where_not_like("title", "%Philosopher's%")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title FROM books WHERE title NOT LIKE '%Alice''s%' OR title NOT LIKE '%Philosopher''s%';", &sql);
    /// // add                                                                   ^^^^^          ^^^^^^^^^^^^^^^^^^
    /// // here                                                                  field                 mask
    /// # Ok(())
    /// # }
    /// ```
    pub fn or_where_not_like<S, T>(&mut self, field: S, mask: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" NOT LIKE '");
        cond.push_str(&esc(&mask.to_string()));
        cond.push('\'');
        self.or_where(&cond)
    }

    /// Add OR NOT LIKE condition to the last WHERE %condition.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("price")
    ///     .or_where_not_like_right("title", "Alice's")
    ///     .or_where_not_like_right("title", "Philosopher's")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT price FROM books WHERE title NOT LIKE '%Alice''s' OR title NOT LIKE '%Philosopher''s';", &sql);
    /// // add                                    ^^^^^            ^^^^^^^^     ^^^^^            ^^^^^^^^^^^^^^
    /// // here                                   field              mask       field                 mask
    /// # Ok(())
    /// # }
    /// ```
    pub fn or_where_not_like_right<S, T>(&mut self, field: S, mask: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" NOT LIKE '%");
        cond.push_str(&esc(&mask.to_string()));
        cond.push('\'');
        self.or_where(&cond)
    }

    /// Add OR NOT LIKE condition to the last WHERE condition%.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("price")
    ///     .or_where_not_like_left("title", "Alice's")
    ///     .or_where_not_like_left("title", "Philosopher's")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT price FROM books WHERE title NOT LIKE 'Alice''s%' OR title NOT LIKE 'Philosopher''s%';", &sql);
    /// // add                                    ^^^^^           ^^^^^^^^      ^^^^^           ^^^^^^^^^^^^^^
    /// // here                                   field             mask        field                mask
    /// # Ok(())
    /// # }
    /// ```
    pub fn or_where_not_like_left<S, T>(&mut self, field: S, mask: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" NOT LIKE '");
        cond.push_str(&esc(&mask.to_string()));
        cond.push_str("%'");
        self.or_where(&cond)
    }

    /// Add OR NOT LIKE condition to the last WHERE %condition%.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("price")
    ///     .or_where_not_like_any("title", "Alice's")
    ///     .or_where_not_like_any("title", "Philosopher's")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT price FROM books WHERE title NOT LIKE '%Alice''s%' OR title NOT LIKE '%Philosopher''s%';", &sql);
    /// // add                                    ^^^^^          ^^^^^^^^^^^^    ^^^^^          ^^^^^^^^^^^^^^^^^^
    /// // here                                   field              mask        field                 mask
    /// # Ok(())
    /// # }
    /// ```
    pub fn or_where_not_like_any<S, T>(&mut self, field: S, mask: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" NOT LIKE '%");
        cond.push_str(&esc(&mask.to_string()));
        cond.push_str("%'");
        self.or_where(&cond)
    }

    /// Add OR IS NULL condition to the last WHERE condition.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .and_where_eq("price", 0)
    ///     .or_where_is_null("price")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title FROM books WHERE price = 0 OR price IS NULL;", &sql);
    /// // add                                                 ^^^^^
    /// // here                                                field
    /// # Ok(())
    /// # }
    /// ```
    pub fn or_where_is_null<S: ToString>(&mut self, field: S) -> &mut Self {
        let mut cond = field.to_string();
        cond.push_str(" IS NULL");
        self.or_where(&cond)
    }

    /// Add OR IS NOT NULL condition to the last WHERE condition.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .or_where_is_not_null("title")
    ///     .or_where_is_not_null("price")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title FROM books WHERE title IS NOT NULL OR price IS NOT NULL;", &sql);
    /// // add                                    ^^^^^                ^^^^^
    /// // here                                   field                field
    /// # Ok(())
    /// # }
    /// ```
    pub fn or_where_is_not_null<S: ToString>(&mut self, field: S) -> &mut Self {
        let mut cond = field.to_string();
        cond.push_str(" IS NOT NULL");
        self.or_where(&cond)
    }

    /// Add OR field IN (list) to the last WHERE condition.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::{SqlBuilder, quote};
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .or_where_lt("price", 100)
    ///     .or_where_in("title", &[quote("G"), quote("L"), quote("t")])
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price < 100 OR title IN ('G', 'L', 't');", &sql);
    /// // add                                                          ^^^^^     ^^^^^^^^^^^^^
    /// // here                                                         field         list
    /// # Ok(())
    /// # }
    /// ```
    pub fn or_where_in<S, T>(&mut self, field: S, list: &[T]) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let list: Vec<String> = list
            .iter()
            .map(|v| (*v).to_string())
            .collect::<Vec<String>>();
        let list = list.join(", ");

        let mut cond = field.to_string();
        cond.push_str(" IN (");
        cond.push_str(&list);
        cond.push(')');
        self.or_where(&cond)
    }

    /// Add OR field IN (string list) to the last WHERE condition.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::{SqlBuilder, quote};
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .or_where_lt("price", 100)
    ///     .or_where_in_quoted("title", &["G", "L", "t"])
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price < 100 OR title IN ('G', 'L', 't');", &sql);
    /// // add                                                          ^^^^^     ^^^^^^^^^^^^^
    /// // here                                                         field         list
    /// # Ok(())
    /// # }
    /// ```
    pub fn or_where_in_quoted<S, T>(&mut self, field: S, list: &[T]) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let list: Vec<String> = list
            .iter()
            .map(|v| quote((*v).to_string()))
            .collect::<Vec<String>>();
        let list = list.join(", ");

        let mut cond = field.to_string();
        cond.push_str(" IN (");
        cond.push_str(&list);
        cond.push(')');
        self.or_where(&cond)
    }

    /// Add OR field NOT IN (list) to the last WHERE condition.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::{SqlBuilder, quote};
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .or_where_lt("price", 100)
    ///     .or_where_not_in("title", &[quote("G"), quote("L"), quote("t")])
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price < 100 OR title NOT IN ('G', 'L', 't');", &sql);
    /// // add                                                          ^^^^^         ^^^^^^^^^^^^^
    /// // here                                                         field             list
    /// # Ok(())
    /// # }
    /// ```
    pub fn or_where_not_in<S, T>(&mut self, field: S, list: &[T]) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let list: Vec<String> = list
            .iter()
            .map(|v| (*v).to_string())
            .collect::<Vec<String>>();
        let list = list.join(", ");

        let mut cond = field.to_string();
        cond.push_str(" NOT IN (");
        cond.push_str(&list);
        cond.push(')');
        self.or_where(&cond)
    }

    /// Add OR field NOT IN (string list) to the last WHERE condition.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::{SqlBuilder, quote};
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .or_where_lt("price", 100)
    ///     .or_where_not_in_quoted("title", &["G", "L", "t"])
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price < 100 OR title NOT IN ('G', 'L', 't');", &sql);
    /// // add                                                          ^^^^^         ^^^^^^^^^^^^^
    /// // here                                                         field             list
    /// # Ok(())
    /// # }
    /// ```
    pub fn or_where_not_in_quoted<S, T>(&mut self, field: S, list: &[T]) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let list: Vec<String> = list
            .iter()
            .map(|v| quote((*v).to_string()))
            .collect::<Vec<String>>();
        let list = list.join(", ");

        let mut cond = field.to_string();
        cond.push_str(" NOT IN (");
        cond.push_str(&list);
        cond.push(')');
        self.or_where(&cond)
    }

    /// Add OR field IN (query) to the last WHERE condition.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let query = SqlBuilder::select_from("shop")
    ///     .field("title")
    ///     .and_where("sold")
    ///     .query()?;
    ///
    /// assert_eq!("SELECT title FROM shop WHERE sold", &query);
    ///
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .or_where_lt("price", 100)
    ///     .or_where_in_query("title", &query)
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price < 100 OR title IN (SELECT title FROM shop WHERE sold);", &sql);
    /// // add                                                          ^^^^^     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    /// // here                                                         field                   query
    /// # Ok(())
    /// # }
    /// ```
    pub fn or_where_in_query<S, T>(&mut self, field: S, query: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" IN (");
        cond.push_str(&query.to_string());
        cond.push(')');
        self.or_where(&cond)
    }

    /// Add OR field NOT IN (query) to the last WHERE condition.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let query = SqlBuilder::select_from("shop")
    ///     .field("title")
    ///     .and_where("sold")
    ///     .query()?;
    ///
    /// assert_eq!("SELECT title FROM shop WHERE sold", &query);
    ///
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .or_where_lt("price", 100)
    ///     .or_where_not_in_query("title", &query)
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price < 100 OR title NOT IN (SELECT title FROM shop WHERE sold);", &sql);
    /// // add                                                          ^^^^^         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    /// // here                                                         field                       query
    /// # Ok(())
    /// # }
    /// ```
    pub fn or_where_not_in_query<S, T>(&mut self, field: S, query: T) -> &mut Self
    where
        S: ToString,
        T: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" NOT IN (");
        cond.push_str(&query.to_string());
        cond.push(')');
        self.or_where(&cond)
    }

    /// Add OR field BETWEEN values to the last WHERE condition.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .or_where_between("price", 100, 200)
    ///     .or_where_between("price", 10_000, 20_000)
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price BETWEEN 100 AND 200 OR price BETWEEN 10000 AND 20000;", &sql);
    /// // add                                           ^^^^^         ^^^     ^^^    ^^^^^         ^^^^^     ^^^^^
    /// // here                                          field         min     max    field          min       max
    /// # Ok(())
    /// # }
    /// ```
    pub fn or_where_between<S, T, U>(&mut self, field: S, min: T, max: U) -> &mut Self
    where
        S: ToString,
        T: ToString,
        U: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" BETWEEN ");
        cond.push_str(&min.to_string());
        cond.push_str(" AND ");
        cond.push_str(&max.to_string());
        self.or_where(&cond)
    }

    /// Add OR field NOT BETWEEN values to the last WHERE condition.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .or_where_not_between("price", 100, 200)
    ///     .or_where_not_between("price", 10_000, 20_000)
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price NOT BETWEEN 100 AND 200 OR price NOT BETWEEN 10000 AND 20000;", &sql);
    /// // add                                           ^^^^^             ^^^     ^^^    ^^^^^             ^^^^^     ^^^^^
    /// // here                                          field             min     max    field              min       max
    /// # Ok(())
    /// # }
    /// ```
    pub fn or_where_not_between<S, T, U>(&mut self, field: S, min: T, max: U) -> &mut Self
    where
        S: ToString,
        T: ToString,
        U: ToString,
    {
        let mut cond = field.to_string();
        cond.push_str(" NOT BETWEEN ");
        cond.push_str(&min.to_string());
        cond.push_str(" AND ");
        cond.push_str(&max.to_string());
        self.or_where(&cond)
    }

    /// Union query with subquery.
    /// ORDER BY must be in the last subquery.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let append = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .and_where("price < 100")
    ///     .order_asc("title")
    ///     .query()?;
    ///
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .and_where_like_left("title", "Harry Potter")
    ///     .order_desc("price")
    ///     .union(&append)
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE title LIKE 'Harry Potter%' UNION SELECT title, price FROM books WHERE price < 100 ORDER BY title;", &sql);
    /// // add                                                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    /// // here                                                                                                        query
    /// # Ok(())
    /// # }
    /// ```
    pub fn union<S: ToString>(&mut self, query: S) -> &mut Self {
        let append = format!(" UNION {}", &query.to_string());
        self.unions.push_str(&append);
        self
    }

    /// Union query with all subquery.
    /// ORDER BY must be in the last subquery.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let append = SqlBuilder::select_values(&["'The Great Gatsby'", "124"])
    ///     .query_values()?;
    ///
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .and_where_like_left("title", "Harry Potter")
    ///     .order_desc("price")
    ///     .union_all(&append)
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE title LIKE 'Harry Potter%' UNION ALL SELECT 'The Great Gatsby', 124;", &sql);
    /// // add                                                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    /// // here                                                                                           query
    /// # Ok(())
    /// # }
    /// ```
    pub fn union_all<S: ToString>(&mut self, query: S) -> &mut Self {
        self.unions.push_str(" UNION ALL ");
        self.unions.push_str(&query.to_string());
        self
    }

    /// Add ORDER BY.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .and_where_like_left("title", "Harry Potter")
    ///     .order_by("price", false)
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE title LIKE 'Harry Potter%' ORDER BY price;", &sql);
    /// // add                                                                               ^^^^^
    /// // here                                                                              field
    /// # Ok(())
    /// # }
    /// ```
    pub fn order_by<S: ToString>(&mut self, field: S, desc: bool) -> &mut Self {
        let order = if desc {
            format!("{} DESC", &field.to_string())
        } else {
            field.to_string()
        };
        self.order_by.push(order);
        self
    }

    /// Add ORDER BY ASC.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .and_where_like_left("title", "Harry Potter")
    ///     .order_asc("title")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE title LIKE 'Harry Potter%' ORDER BY title;", &sql);
    /// // add                                                                               ^^^^^
    /// // here                                                                              field
    /// # Ok(())
    /// # }
    /// ```
    pub fn order_asc<S: ToString>(&mut self, field: S) -> &mut Self {
        self.order_by(&field.to_string(), false)
    }

    /// Add ORDER BY DESC.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .and_where_like_left("title", "Harry Potter")
    ///     .order_desc("price")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE title LIKE 'Harry Potter%' ORDER BY price DESC;", &sql);
    /// // add                                                                               ^^^^^
    /// // here                                                                              field
    /// # Ok(())
    /// # }
    /// ```
    pub fn order_desc<S: ToString>(&mut self, field: S) -> &mut Self {
        self.order_by(&field.to_string(), true)
    }

    /// Set LIMIT.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .and_where_like_left("title", "Harry Potter")
    ///     .order_desc("price")
    ///     .limit(10)
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE title LIKE 'Harry Potter%' ORDER BY price DESC LIMIT 10;", &sql);
    /// // add                                                                                                ^^
    /// // here                                                                                              limit
    /// # Ok(())
    /// # }
    /// ```
    pub fn limit<S: ToString>(&mut self, limit: S) -> &mut Self {
        self.limit = Some(limit.to_string());
        self
    }

    /// Set OFFSET.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .and_where_like_left("title", "Harry Potter")
    ///     .order_desc("price")
    ///     .limit(10)
    ///     .offset(100)
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE title LIKE 'Harry Potter%' ORDER BY price DESC LIMIT 10 OFFSET 100;", &sql);
    /// // add                                                                                                          ^^^
    /// // here                                                                                                        offset
    /// # Ok(())
    /// # }
    /// ```
    pub fn offset<S: ToString>(&mut self, offset: S) -> &mut Self {
        self.offset = Some(offset.to_string());
        self
    }

    /// Build complete SQL command.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books").sql()?;
    ///
    /// assert_eq!("SELECT * FROM books;", &sql);
    /// # Ok(())
    /// # }
    /// ```
    pub fn sql(&self) -> Result<String> {
        match self.statement {
            Statement::SelectFrom => self.sql_select(),
            Statement::SelectValues => self.sql_select_values(),
            Statement::UpdateTable => self.sql_update(),
            Statement::InsertInto => self.sql_insert(),
            Statement::DeleteFrom => self.sql_delete(),
        }
    }

    /// Build complete SQL command for SELECT statement
    fn sql_select(&self) -> Result<String> {
        // Checks
        if self.table.is_empty() {
            return Err(SqlBuilderError::NoTableName.into());
        }

        // Build query
        let mut text = self.query()?;
        text.push(';');
        Ok(text)
    }

    /// Build complete SQL command for SELECT statement without a table
    fn sql_select_values(&self) -> Result<String> {
        // Checks
        if self.fields.is_empty() {
            return Err(SqlBuilderError::NoValues.into());
        }

        // Build query
        let mut text = self.query_values()?;
        text.push(';');
        Ok(text)
    }

    /// Build subquery SQL command.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let cat = SqlBuilder::select_from("books")
    ///     .field("CASE WHEN price < 100 THEN 'cheap' ELSE 'expensive' END AS category")
    ///     .subquery()?;
    ///
    /// assert_eq!("(SELECT CASE WHEN price < 100 THEN 'cheap' ELSE 'expensive' END AS category FROM books)", &cat);
    ///
    /// let sql = SqlBuilder::select_from(&cat)
    ///     .field("category")
    ///     .field("COUNT(category) AS cnt")
    ///     .group_by("category")
    ///     .order_desc("cnt")
    ///     .order_asc("category")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT category, COUNT(category) AS cnt FROM (SELECT CASE WHEN price < 100 THEN 'cheap' ELSE 'expensive' END AS category FROM books) GROUP BY category ORDER BY cnt DESC, category;", &sql);
    /// # Ok(())
    /// # }
    /// ```
    pub fn subquery(&self) -> Result<String> {
        let text = self.query()?;
        let text = format!("({})", &text);
        Ok(text)
    }

    /// Build named subquery SQL command.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let cat = SqlBuilder::select_from("books")
    ///     .field("CASE WHEN price < 100 THEN 'cheap' ELSE 'expensive' END")
    ///     .subquery_as("category")?;
    ///
    /// assert_eq!("(SELECT CASE WHEN price < 100 THEN 'cheap' ELSE 'expensive' END FROM books) AS category", &cat);
    /// // add                                                                                     ^^^^^^^^
    /// // here                                                                                      name
    ///
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .field(&cat)
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price, (SELECT CASE WHEN price < 100 THEN 'cheap' ELSE 'expensive' END FROM books) AS category FROM books;", &sql);
    /// # Ok(())
    /// # }
    /// ```
    pub fn subquery_as<S: ToString>(&self, name: S) -> Result<String> {
        let mut text = "(".to_string();
        text.push_str(&self.query()?);
        text.push_str(") AS ");
        text.push_str(&name.to_string());
        Ok(text)
    }

    /// SQL command generator for query or subquery.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<()> {
    /// let query = SqlBuilder::select_from("warehouse")
    ///     .field("title")
    ///     .field("preliminary_price * 2")
    ///     .query()?;
    ///
    /// assert_eq!("SELECT title, preliminary_price * 2 FROM warehouse", &query);
    ///
    /// let sql = SqlBuilder::insert_into("books")
    ///     .field("title")
    ///     .field("price")
    ///     .select(&query)
    ///     .sql()?;
    ///
    /// assert_eq!("INSERT INTO books (title, price) SELECT title, preliminary_price * 2 FROM warehouse;", &sql);
    /// # Ok(())
    /// # }
    /// ```
    pub fn query(&self) -> Result<String> {
        // Distinct results
        let distinct = if self.distinct { " DISTINCT" } else { "" };

        // Make fields
        let fields = if self.fields.is_empty() {
            "*".to_string()
        } else {
            self.fields.join(", ")
        };

        // Make JOIN parts
        let joins = if self.joins.is_empty() {
            String::new()
        } else {
            format!(" {}", self.joins.join(" "))
        };

        // Make GROUP BY part
        let group_by = if self.group_by.is_empty() {
            String::new()
        } else {
            let having = if let Some(having) = &self.having {
                format!(" HAVING {}", having)
            } else {
                String::new()
            };
            format!(" GROUP BY {}{}", self.group_by.join(", "), having)
        };

        // Make WHERE part
        let wheres = SqlBuilder::make_wheres(&self.wheres);

        // Make ORDER BY part
        let order_by = if self.order_by.is_empty() || !self.unions.is_empty() {
            String::new()
        } else {
            format!(" ORDER BY {}", self.order_by.join(", "))
        };

        // Make LIMIT part
        let limit = match &self.limit {
            Some(limit) => format!(" LIMIT {}", limit),
            None => String::new(),
        };

        // Make OFFSET part
        let offset = match &self.offset {
            Some(offset) => format!(" OFFSET {}", offset),
            None => String::new(),
        };

        // Make SQL
        let sql = format!("SELECT{distinct} {fields} FROM {table}{joins}{wheres}{group_by}{unions}{order_by}{limit}{offset}",
            distinct = distinct,
            fields = fields,
            table = &self.table,
            joins = joins,
            group_by = group_by,
            wheres = wheres,
            unions = &self.unions,
            order_by = order_by,
            limit = limit,
            offset = offset,
        );
        Ok(sql)
    }

    /// SQL command generator for query or subquery without a table.
    ///
    /// ```
    /// # use anyhow::Result;
    /// use sql_builder::{SqlBuilder, quote};
    ///
    /// # fn main() -> Result<()> {
    /// let values = SqlBuilder::select_values(&["10", &quote("100")])
    ///     .query_values()?;
    ///
    /// assert_eq!("SELECT 10, '100'", &values);
    /// # Ok(())
    /// # }
    /// ```
    pub fn query_values(&self) -> Result<String> {
        // Make values
        let fields = self.fields.join(", ");

        // Make SQL
        let sql = format!("SELECT {fields}", fields = fields);
        Ok(sql)
    }

    /// Build SQL command for INSERT statement
    fn sql_insert(&self) -> Result<String> {
        // Checks
        if self.table.is_empty() {
            return Err(SqlBuilderError::NoTableName.into());
        }

        // Make SET part
        let fields = self.fields.join(", ");

        // Add values or query
        let sql = match &self.values {
            Values::Empty => return Err(SqlBuilderError::NoValues.into()),
            Values::List(values) => {
                if values.is_empty() {
                    return Err(SqlBuilderError::NoValues.into());
                }

                // Make VALUES part
                let values = values.join(", ");

                // Make RETURNING part
                let returning = if let Some(ret) = &self.returning {
                    format!(" RETURNING {}", ret)
                } else {
                    "".to_string()
                };

                // Make SQL
                format!(
                    "INSERT INTO {table} ({fields}) VALUES {values}{returning};",
                    table = &self.table,
                    fields = fields,
                    values = values,
                    returning = returning,
                )
            }
            Values::Select(query) => {
                // Make SQL
                format!(
                    "INSERT INTO {table} ({fields}) {query};",
                    table = &self.table,
                    fields = fields,
                    query = query,
                )
            }
        };

        Ok(sql)
    }

    /// Build SQL command for UPDATE statement
    fn sql_update(&self) -> Result<String> {
        // Checks
        if self.table.is_empty() {
            return Err(SqlBuilderError::NoTableName.into());
        }
        if self.sets.is_empty() {
            return Err(SqlBuilderError::NoSetFields.into());
        }

        // Make SET part
        let sets = self.sets.join(", ");

        // Make WHERE part
        let wheres = SqlBuilder::make_wheres(&self.wheres);

        // Make RETURNING part
        let returning = if let Some(ret) = &self.returning {
            format!(" RETURNING {}", ret)
        } else {
            "".to_string()
        };

        // Make SQL
        let sql = format!(
            "UPDATE {table} SET {sets}{wheres}{returning};",
            table = &self.table,
            sets = sets,
            wheres = wheres,
            returning = returning,
        );
        Ok(sql)
    }

    /// Build SQL command for DELETE statement
    fn sql_delete(&self) -> Result<String> {
        // Checks
        if self.table.is_empty() {
            return Err(SqlBuilderError::NoTableName.into());
        }

        // Make WHERE part
        let wheres = SqlBuilder::make_wheres(&self.wheres);

        // Make SQL
        let sql = format!(
            "DELETE FROM {table}{wheres};",
            table = &self.table,
            wheres = wheres,
        );
        Ok(sql)
    }

    /// Make WHERE part
    fn make_wheres(wheres: &[String]) -> String {
        match wheres.len() {
            0 => String::new(),
            1 => {
                let wheres = wheres[0].to_string();
                format!(" WHERE {}", wheres)
            }
            _ => {
                let wheres: Vec<String> = wheres.iter().map(|w| format!("({})", w)).collect();
                format!(" WHERE {}", wheres.join(" AND "))
            }
        }
    }
}

/// Escape string for SQL.
///
/// ```
/// use sql_builder::esc;
///
/// let sql = esc("Hello, 'World'");
///
/// assert_eq!(&sql, "Hello, ''World''");
/// ```
pub fn esc<S: ToString>(src: S) -> String {
    src.to_string().replace("'", "''")
}

/// Quote string for SQL.
///
/// ```
/// use sql_builder::quote;
///
/// let sql = quote("Hello, 'World'");
///
/// assert_eq!(&sql, "'Hello, ''World'''");
/// ```
pub fn quote<S: ToString>(src: S) -> String {
    format!("'{}'", esc(src.to_string()))
}

/// Backquote string for SQL.
///
/// ```
/// use sql_builder::baquote;
///
/// let sql = baquote("Hello, 'World'");
///
/// assert_eq!(&sql, "`Hello, 'World'`");
/// ```
pub fn baquote<S: ToString>(src: S) -> String {
    format!("`{}`", src.to_string().replace("`", "\\`"))
}

/// Quote string with [brackets].
///
/// ```
/// use sql_builder::brquote;
///
/// let sql = brquote("Hello, [awesome] World");
///
/// assert_eq!(&sql, "[Hello, [awesome]] World]");
/// ```
pub fn brquote<S: ToString>(src: S) -> String {
    format!("[{}]", src.to_string().replace("]", "]]"))
}

/// Double quote string for SQL.
///
/// ```
/// use sql_builder::dquote;
///
/// let sql = dquote("Hello, 'World'");
///
/// assert_eq!(&sql, "\"Hello, 'World'\"");
/// ```
pub fn dquote<S: ToString>(src: S) -> String {
    format!("\"{}\"", src.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_esc() -> Result<()> {
        let sql = esc("Hello, 'World'");

        assert_eq!(&sql, "Hello, ''World''");

        Ok(())
    }

    #[test]
    fn test_quote() -> Result<()> {
        let sql = quote("Hello, 'World'");
        assert_eq!(&sql, "'Hello, ''World'''");

        let sql = baquote("Hello, 'World'");
        assert_eq!(&sql, "`Hello, 'World'`");

        let sql = dquote("Hello, 'World'");
        assert_eq!(&sql, "\"Hello, 'World'\"");

        Ok(())
    }

    #[test]
    fn test_select_only_values() -> Result<()> {
        let values = SqlBuilder::select_values(&["10", &quote("100")]).sql()?;

        assert_eq!("SELECT 10, '100';", &values);

        Ok(())
    }

    #[test]
    fn test_select_all_books() -> Result<()> {
        let sql = SqlBuilder::select_from("books").sql()?;

        assert_eq!(&sql, "SELECT * FROM books;");

        Ok(())
    }

    #[test]
    fn test_show_all_prices() -> Result<()> {
        let sql = SqlBuilder::select_from("books")
            .distinct()
            .field("price")
            .sql()?;

        assert_eq!(&sql, "SELECT DISTINCT price FROM books;");

        Ok(())
    }

    #[test]
    fn test_select_title_and_price() -> Result<()> {
        let sql = SqlBuilder::select_from("books")
            .fields(&["title", "price"])
            .sql()?;

        assert_eq!(&sql, "SELECT title, price FROM books;");

        let sql = SqlBuilder::select_from("books")
            .field("title")
            .field("price")
            .sql()?;

        assert_eq!(&sql, "SELECT title, price FROM books;");

        Ok(())
    }

    #[test]
    fn test_select_expensive_books() -> Result<()> {
        let sql = SqlBuilder::select_from("books")
            .field("title")
            .field("price")
            .and_where("price > 100")
            .sql()?;

        assert_eq!(&sql, "SELECT title, price FROM books WHERE price > 100;");

        let sql = SqlBuilder::select_from("books")
            .field("title")
            .field("price")
            .and_where_gt("price", 200)
            .sql()?;

        assert_eq!(&sql, "SELECT title, price FROM books WHERE price > 200;");

        let sql = SqlBuilder::select_from("books")
            .field("title")
            .field("price")
            .and_where_ge("price", 300)
            .sql()?;

        assert_eq!(&sql, "SELECT title, price FROM books WHERE price >= 300;");

        Ok(())
    }

    #[test]
    fn test_select_price_for_harry_potter_and_phil_stone() -> Result<()> {
        let sql = SqlBuilder::select_from("books")
            .field("price")
            .and_where_eq("title", quote("Harry Potter and the Philosopher's Stone"))
            .sql()?;

        assert_eq!(
            &sql,
            "SELECT price FROM books WHERE title = 'Harry Potter and the Philosopher''s Stone';"
        );

        Ok(())
    }

    #[test]
    fn test_select_price_not_for_harry_potter_and_phil_stone() -> Result<()> {
        let sql = SqlBuilder::select_from("books")
            .field("price")
            .and_where_ne("title", quote("Harry Potter and the Philosopher's Stone"))
            .sql()?;

        assert_eq!(
            &sql,
            "SELECT price FROM books WHERE title <> 'Harry Potter and the Philosopher''s Stone';"
        );

        Ok(())
    }

    #[test]
    fn test_select_expensive_harry_potter() -> Result<()> {
        let sql = SqlBuilder::select_from("books")
            .field("title")
            .field("price")
            .and_where("price > 100")
            .and_where_like_left("title", "Harry Potter")
            .sql()?;

        assert_eq!(
            &sql,
            "SELECT title, price FROM books WHERE (price > 100) AND (title LIKE 'Harry Potter%');"
        );

        Ok(())
    }

    #[test]
    fn test_select_strange_books() -> Result<()> {
        let sql = SqlBuilder::select_from("books")
            .field("title")
            .field("price")
            .and_where("price < 2")
            .or_where("price > 1000")
            .or_where_eq("title", quote("Harry Potter and the Philosopher's Stone"))
            .or_where_ne("price", 100)
            .or_where_like("title", "Alice's")
            .or_where_not_like_any("LOWER(title)", " the ")
            .or_where_is_null("title")
            .or_where_is_not_null("price")
            .sql()?;

        assert_eq!(
            &sql,
            "SELECT title, price FROM books WHERE price < 2 OR price > 1000 OR title = 'Harry Potter and the Philosopher''s Stone' OR price <> 100 OR title LIKE 'Alice''s' OR LOWER(title) NOT LIKE '% the %' OR title IS NULL OR price IS NOT NULL;"
        );

        Ok(())
    }

    #[test]
    fn test_order_harry_potter_by_price() -> Result<()> {
        let sql = SqlBuilder::select_from("books")
            .field("title")
            .field("price")
            .and_where_like_left("title", "Harry Potter")
            .order_by("price", false)
            .sql()?;

        assert_eq!(
            &sql,
            "SELECT title, price FROM books WHERE title LIKE 'Harry Potter%' ORDER BY price;"
        );

        let sql = SqlBuilder::select_from("books")
            .field("title")
            .field("price")
            .and_where_like_left("title", "Harry Potter")
            .order_desc("price")
            .sql()?;

        assert_eq!(
            &sql,
            "SELECT title, price FROM books WHERE title LIKE 'Harry Potter%' ORDER BY price DESC;"
        );

        let sql = SqlBuilder::select_from("books")
            .field("title")
            .field("price")
            .and_where_like_left("title", "Harry Potter")
            .order_desc("price")
            .order_asc("title")
            .sql()?;

        assert_eq!(&sql, "SELECT title, price FROM books WHERE title LIKE 'Harry Potter%' ORDER BY price DESC, title;");

        Ok(())
    }

    #[test]
    fn test_find_cheap_or_harry_potter() -> Result<()> {
        let append = SqlBuilder::select_from("books")
            .field("title")
            .field("price")
            .and_where("price < 100")
            .order_asc("title")
            .query()?;

        let sql = SqlBuilder::select_from("books")
            .field("title")
            .field("price")
            .and_where_like_left("title", "Harry Potter")
            .order_desc("price")
            .union(&append)
            .sql()?;

        assert_eq!(
            "SELECT title, price FROM books WHERE title LIKE 'Harry Potter%' UNION SELECT title, price FROM books WHERE price < 100 ORDER BY title;",
            &sql
        );

        let append = SqlBuilder::select_values(&["'The Great Gatsby'", "124"]).query_values()?;

        let sql = SqlBuilder::select_from("books")
            .field("title")
            .field("price")
            .and_where_like_left("title", "Harry Potter")
            .order_desc("price")
            .union_all(&append)
            .sql()?;

        assert_eq!(
            "SELECT title, price FROM books WHERE title LIKE 'Harry Potter%' UNION ALL SELECT 'The Great Gatsby', 124;",
            &sql
        );

        Ok(())
    }

    #[test]
    fn test_select_first_3_harry_potter_books() -> Result<()> {
        let sql = SqlBuilder::select_from("books")
            .field("title")
            .field("price")
            .and_where_like_left("title", "Harry Potter")
            .order_asc("title")
            .limit(3)
            .sql()?;

        assert_eq!(&sql, "SELECT title, price FROM books WHERE title LIKE 'Harry Potter%' ORDER BY title LIMIT 3;");

        Ok(())
    }

    #[test]
    fn test_select_harry_potter_from_second_book() -> Result<()> {
        let sql = SqlBuilder::select_from("books")
            .field("title")
            .field("price")
            .and_where_like_left("title", "Harry Potter")
            .order_asc("title")
            .offset(2)
            .sql()?;

        assert_eq!(&sql, "SELECT title, price FROM books WHERE title LIKE 'Harry Potter%' ORDER BY title OFFSET 2;");

        let sql = SqlBuilder::select_from("books")
            .field("title")
            .field("price")
            .and_where_like_left("title", "Harry Potter")
            .order_asc("title")
            .limit(3)
            .offset(2)
            .sql()?;

        assert_eq!(&sql, "SELECT title, price FROM books WHERE title LIKE 'Harry Potter%' ORDER BY title LIMIT 3 OFFSET 2;");

        Ok(())
    }

    #[test]
    fn test_find_books_not_about_alice() -> Result<()> {
        let sql = SqlBuilder::select_from("books")
            .field("title")
            .and_where_not_like_any("title", "Alice's")
            .sql()?;

        assert_eq!(
            "SELECT title FROM books WHERE title NOT LIKE '%Alice''s%';",
            &sql
        );

        Ok(())
    }

    #[test]
    fn test_books_without_price() -> Result<()> {
        let sql = SqlBuilder::select_from("books")
            .field("title")
            .and_where_is_null("price")
            .sql()?;

        assert_eq!(&sql, "SELECT title FROM books WHERE price IS NULL;");

        let sql = SqlBuilder::select_from("books")
            .field("title")
            .and_where_is_not_null("price")
            .sql()?;

        assert_eq!(&sql, "SELECT title FROM books WHERE price IS NOT NULL;");

        Ok(())
    }

    #[test]
    fn test_group_books_by_price() -> Result<()> {
        let sql = SqlBuilder::select_from("books")
            .field("price")
            .field("COUNT(price) AS cnt")
            .group_by("price")
            .order_desc("cnt")
            .sql()?;

        assert_eq!(
            &sql,
            "SELECT price, COUNT(price) AS cnt FROM books GROUP BY price ORDER BY cnt DESC;"
        );

        let sql = SqlBuilder::select_from("books")
            .field("price")
            .field("COUNT(price) AS cnt")
            .group_by("price")
            .having("price > 100")
            .order_desc("cnt")
            .sql()?;

        assert_eq!(&sql, "SELECT price, COUNT(price) AS cnt FROM books GROUP BY price HAVING price > 100 ORDER BY cnt DESC;");

        let sql = SqlBuilder::select_from("books")
            .field("price")
            .field("COUNT(price) AS cnt")
            .group_by("price")
            .and_where("price > 100")
            .order_desc("cnt")
            .sql()?;

        assert_eq!(&sql, "SELECT price, COUNT(price) AS cnt FROM books WHERE price > 100 GROUP BY price ORDER BY cnt DESC;");

        Ok(())
    }

    #[test]
    fn test_group_books_by_price_category() -> Result<()> {
        let cat = SqlBuilder::select_from("books")
            .field("CASE WHEN price < 100 THEN 'cheap' ELSE 'expensive' END AS category")
            .subquery()?;

        assert_eq!("(SELECT CASE WHEN price < 100 THEN 'cheap' ELSE 'expensive' END AS category FROM books)", &cat);

        let sql = SqlBuilder::select_from(&cat)
            .field("category")
            .field("COUNT(category) AS cnt")
            .group_by("category")
            .order_desc("cnt")
            .order_asc("category")
            .sql()?;

        assert_eq!("SELECT category, COUNT(category) AS cnt FROM (SELECT CASE WHEN price < 100 THEN 'cheap' ELSE 'expensive' END AS category FROM books) GROUP BY category ORDER BY cnt DESC, category;", &sql);

        let cat = SqlBuilder::select_from("books")
            .field("CASE WHEN price < 100 THEN 'cheap' ELSE 'expensive' END")
            .subquery_as("category")?;

        assert_eq!("(SELECT CASE WHEN price < 100 THEN 'cheap' ELSE 'expensive' END FROM books) AS category", &cat);

        let sql = SqlBuilder::select_from("books")
            .field("title")
            .field("price")
            .field(&cat)
            .sql()?;

        assert_eq!("SELECT title, price, (SELECT CASE WHEN price < 100 THEN 'cheap' ELSE 'expensive' END FROM books) AS category FROM books;", &sql);

        Ok(())
    }

    #[test]
    fn test_grow_price() -> Result<()> {
        let sql = SqlBuilder::update_table("books")
            .set("price", "price + 10")
            .sql()?;

        assert_eq!(&sql, "UPDATE books SET price = price + 10;");

        let sql = SqlBuilder::update_table("books")
            .set("price", "price * 0.1")
            .and_where_like_left("title", "Harry Potter")
            .returning_id()
            .sql()?;

        assert_eq!(
            &sql,
            "UPDATE books SET price = price * 0.1 WHERE title LIKE 'Harry Potter%' RETURNING id;"
        );

        Ok(())
    }

    #[test]
    fn test_add_new_books() -> Result<()> {
        let sql = SqlBuilder::insert_into("books")
            .field("title")
            .field("price")
            .values(&[quote("In Search of Lost Time"), 150.to_string()])
            .values(&["'Don Quixote', 200"])
            .sql()?;

        assert_eq!(&sql, "INSERT INTO books (title, price) VALUES ('In Search of Lost Time', 150), ('Don Quixote', 200);");

        let sql = SqlBuilder::insert_into("books")
            .field("title")
            .field("price")
            .values(&["'Don Quixote', 200"])
            .returning_id()
            .sql()?;

        assert_eq!(
            &sql,
            "INSERT INTO books (title, price) VALUES ('Don Quixote', 200) RETURNING id;"
        );

        Ok(())
    }

    #[test]
    fn test_add_books_from_warehouse() -> Result<()> {
        let query = SqlBuilder::select_from("warehouse")
            .field("title")
            .field("preliminary_price * 2")
            .query()?;

        assert_eq!("SELECT title, preliminary_price * 2 FROM warehouse", &query);

        let sql = SqlBuilder::insert_into("books")
            .field("title")
            .field("price")
            .select(&query)
            .sql()?;

        assert_eq!(
            "INSERT INTO books (title, price) SELECT title, preliminary_price * 2 FROM warehouse;",
            &sql
        );

        Ok(())
    }

    #[test]
    fn test_sold_all_harry_potter() -> Result<()> {
        let sql = SqlBuilder::update_table("books")
            .set("price", 0)
            .set("title", "'[SOLD!]' || title")
            .and_where_like_left("title", "Harry Potter")
            .sql()?;

        assert_eq!(&sql, "UPDATE books SET price = 0, title = '[SOLD!]' || title WHERE title LIKE 'Harry Potter%';");

        Ok(())
    }

    #[test]
    fn test_mark_as_not_distr() -> Result<()> {
        let sql = SqlBuilder::update_table("books")
            .set_str("comment", "Don't distribute!")
            .and_where_le("price", "100")
            .returning("id, comment")
            .sql()?;

        assert_eq!(
            "UPDATE books SET comment = 'Don''t distribute!' WHERE price <= 100 RETURNING id, comment;",
            &sql
        );

        Ok(())
    }

    #[test]
    fn test_remove_all_expensive_books() -> Result<()> {
        let sql = SqlBuilder::delete_from("books")
            .and_where("price > 100")
            .sql()?;

        assert_eq!(&sql, "DELETE FROM books WHERE price > 100;");

        Ok(())
    }

    #[test]
    fn test_count_books_in_shops() -> Result<()> {
        let sql = SqlBuilder::select_from("books AS b")
            .field("b.title")
            .field("s.total")
            .left_outer()
            .join("shops AS s")
            .on("b.id = s.book")
            .sql()?;

        assert_eq!(
            &sql,
            "SELECT b.title, s.total FROM books AS b LEFT OUTER JOIN shops AS s ON b.id = s.book;"
        );

        let sql = SqlBuilder::select_from("books AS b")
            .field("b.title")
            .field("s.total")
            .left_outer()
            .join("shops AS s")
            .on_eq("b.id", "s.book")
            .sql()?;

        assert_eq!(
            &sql,
            "SELECT b.title, s.total FROM books AS b LEFT OUTER JOIN shops AS s ON b.id = s.book;"
        );

        Ok(())
    }
}

//#[cfg(test)]
//mod benches {
//    use super::*;
//    use test::Bencher;
//
//    #[bench]
//    fn bench_select_from(b: &mut Bencher) {
//        b.iter(|| SqlBuilder::select_from("foo"));
//    }
//
//    #[bench]
//    fn bench_select_values(b: &mut Bencher) {
//        b.iter(|| SqlBuilder::select_values(&["foo", "bar"]));
//    }
//
//    #[bench]
//    fn bench_insert_into(b: &mut Bencher) {
//        b.iter(|| SqlBuilder::insert_into("foo"));
//    }
//
//    #[bench]
//    fn bench_update_table(b: &mut Bencher) {
//        b.iter(|| SqlBuilder::update_table("foo"));
//    }
//
//    #[bench]
//    fn bench_delete_from(b: &mut Bencher) {
//        b.iter(|| SqlBuilder::delete_from("foo"));
//    }
//
//    #[bench]
//    fn bench_natural(b: &mut Bencher) {
//        let builder = SqlBuilder::select_from("foo");
//        b.iter(|| {
//            let mut b = builder.clone();
//            b.natural();
//        });
//    }
//
//    //#[bench]
//    //fn bench_left(b: &mut Bencher) {
//    //    b.iter(|| left());
//    //}
//
//    //#[bench]
//    //fn bench_left_outer(b: &mut Bencher) {
//    //    b.iter(|| left_outer());
//    //}
//
//    //#[bench]
//    //fn bench_right(b: &mut Bencher) {
//    //    b.iter(|| right());
//    //}
//
//    //#[bench]
//    //fn bench_right_outer(b: &mut Bencher) {
//    //    b.iter(|| right_outer());
//    //}
//
//    //#[bench]
//    //fn bench_inner(b: &mut Bencher) {
//    //    b.iter(|| inner());
//    //}
//
//    //#[bench]
//    //fn bench_cross(b: &mut Bencher) {
//    //    b.iter(|| cross());
//    //}
//
//    //#[bench]
//    //fn bench_join(b: &mut Bencher) {
//    //    b.iter(|| join());
//    //}
//
//    //#[bench]
//    //fn bench_on(b: &mut Bencher) {
//    //    b.iter(|| on());
//    //}
//
//    //#[bench]
//    //fn bench_distinct(b: &mut Bencher) {
//    //    b.iter(|| distinct());
//    //}
//
//    //#[bench]
//    //fn bench_fields(b: &mut Bencher) {
//    //    b.iter(|| fields());
//    //}
//
//    //#[bench]
//    //fn bench_set_fields(b: &mut Bencher) {
//    //    b.iter(|| set_fields());
//    //}
//
//    //#[bench]
//    //fn bench_field(b: &mut Bencher) {
//    //    b.iter(|| field());
//    //}
//
//    //#[bench]
//    //fn bench_set_field(b: &mut Bencher) {
//    //    b.iter(|| set_field());
//    //}
//
//    //#[bench]
//    //fn bench_set(b: &mut Bencher) {
//    //    b.iter(|| set());
//    //}
//
//    //#[bench]
//    //fn bench_set_str(b: &mut Bencher) {
//    //    b.iter(|| set_str());
//    //}
//
//    //#[bench]
//    //fn bench_values(b: &mut Bencher) {
//    //    b.iter(|| values());
//    //}
//
//    //#[bench]
//    //fn bench_select(b: &mut Bencher) {
//    //    b.iter(|| select());
//    //}
//
//    //#[bench]
//    //fn bench_group_by(b: &mut Bencher) {
//    //    b.iter(|| group_by());
//    //}
//
//    //#[bench]
//    //fn bench_having(b: &mut Bencher) {
//    //    b.iter(|| having());
//    //}
//
//    //#[bench]
//    //fn bench_and_where(b: &mut Bencher) {
//    //    b.iter(|| and_where());
//    //}
//
//    //#[bench]
//    //fn bench_and_where_eq(b: &mut Bencher) {
//    //    b.iter(|| and_where_eq());
//    //}
//
//    //#[bench]
//    //fn bench_and_where_ne(b: &mut Bencher) {
//    //    b.iter(|| and_where_ne());
//    //}
//
//    //#[bench]
//    //fn bench_and_where_gt(b: &mut Bencher) {
//    //    b.iter(|| and_where_gt());
//    //}
//
//    //#[bench]
//    //fn bench_and_where_ge(b: &mut Bencher) {
//    //    b.iter(|| and_where_ge());
//    //}
//
//    //#[bench]
//    //fn bench_and_where_lt(b: &mut Bencher) {
//    //    b.iter(|| and_where_lt());
//    //}
//
//    //#[bench]
//    //fn bench_and_where_le(b: &mut Bencher) {
//    //    b.iter(|| and_where_le());
//    //}
//
//    //#[bench]
//    //fn bench_and_where_like(b: &mut Bencher) {
//    //    b.iter(|| and_where_like());
//    //}
//
//    //#[bench]
//    //fn bench_and_where_like_right(b: &mut Bencher) {
//    //    b.iter(|| and_where_like_right());
//    //}
//
//    //#[bench]
//    //fn bench_and_where_like_left(b: &mut Bencher) {
//    //    b.iter(|| and_where_like_left());
//    //}
//
//    //#[bench]
//    //fn bench_and_where_like_any(b: &mut Bencher) {
//    //    b.iter(|| and_where_like_any());
//    //}
//
//    //#[bench]
//    //fn bench_and_where_not_like(b: &mut Bencher) {
//    //    b.iter(|| and_where_not_like());
//    //}
//
//    //#[bench]
//    //fn bench_and_where_not_like_right(b: &mut Bencher) {
//    //    b.iter(|| and_where_not_like_right());
//    //}
//
//    //#[bench]
//    //fn bench_and_where_not_like_left(b: &mut Bencher) {
//    //    b.iter(|| and_where_not_like_left());
//    //}
//
//    //#[bench]
//    //fn bench_and_where_not_like_any(b: &mut Bencher) {
//    //    b.iter(|| and_where_not_like_any());
//    //}
//
//    //#[bench]
//    //fn bench_and_where_is_null(b: &mut Bencher) {
//    //    b.iter(|| and_where_is_null());
//    //}
//
//    //#[bench]
//    //fn bench_and_where_is_not_null(b: &mut Bencher) {
//    //    b.iter(|| and_where_is_not_null());
//    //}
//
//    //#[bench]
//    //fn bench_or_where(b: &mut Bencher) {
//    //    b.iter(|| or_where());
//    //}
//
//    //#[bench]
//    //fn bench_or_where_eq(b: &mut Bencher) {
//    //    b.iter(|| or_where_eq());
//    //}
//
//    //#[bench]
//    //fn bench_or_where_ne(b: &mut Bencher) {
//    //    b.iter(|| or_where_ne());
//    //}
//
//    //#[bench]
//    //fn bench_or_where_gt(b: &mut Bencher) {
//    //    b.iter(|| or_where_gt());
//    //}
//
//    //#[bench]
//    //fn bench_or_where_ge(b: &mut Bencher) {
//    //    b.iter(|| or_where_ge());
//    //}
//
//    //#[bench]
//    //fn bench_or_where_lt(b: &mut Bencher) {
//    //    b.iter(|| or_where_lt());
//    //}
//
//    //#[bench]
//    //fn bench_or_where_le(b: &mut Bencher) {
//    //    b.iter(|| or_where_le());
//    //}
//
//    //#[bench]
//    //fn bench_or_where_like(b: &mut Bencher) {
//    //    b.iter(|| or_where_like());
//    //}
//
//    //#[bench]
//    //fn bench_or_where_like_right(b: &mut Bencher) {
//    //    b.iter(|| or_where_like_right());
//    //}
//
//    //#[bench]
//    //fn bench_or_where_like_left(b: &mut Bencher) {
//    //    b.iter(|| or_where_like_left());
//    //}
//
//    //#[bench]
//    //fn bench_or_where_like_any(b: &mut Bencher) {
//    //    b.iter(|| or_where_like_any());
//    //}
//
//    //#[bench]
//    //fn bench_or_where_not_like(b: &mut Bencher) {
//    //    b.iter(|| or_where_not_like());
//    //}
//
//    //#[bench]
//    //fn bench_or_where_not_like_right(b: &mut Bencher) {
//    //    b.iter(|| or_where_not_like_right());
//    //}
//
//    //#[bench]
//    //fn bench_or_where_not_like_left(b: &mut Bencher) {
//    //    b.iter(|| or_where_not_like_left());
//    //}
//
//    //#[bench]
//    //fn bench_or_where_not_like_any(b: &mut Bencher) {
//    //    b.iter(|| or_where_not_like_any());
//    //}
//
//    //#[bench]
//    //fn bench_or_where_is_null(b: &mut Bencher) {
//    //    b.iter(|| or_where_is_null());
//    //}
//
//    //#[bench]
//    //fn bench_or_where_is_not_null(b: &mut Bencher) {
//    //    b.iter(|| or_where_is_not_null());
//    //}
//
//    //#[bench]
//    //fn bench_union(b: &mut Bencher) {
//    //    b.iter(|| union());
//    //}
//
//    //#[bench]
//    //fn bench_union_all(b: &mut Bencher) {
//    //    b.iter(|| union_all());
//    //}
//
//    //#[bench]
//    //fn bench_order_by(b: &mut Bencher) {
//    //    b.iter(|| order_by());
//    //}
//
//    //#[bench]
//    //fn bench_order_asc(b: &mut Bencher) {
//    //    b.iter(|| order_asc());
//    //}
//
//    //#[bench]
//    //fn bench_order_desc(b: &mut Bencher) {
//    //    b.iter(|| order_desc());
//    //}
//
//    //#[bench]
//    //fn bench_limit(b: &mut Bencher) {
//    //    b.iter(|| limit());
//    //}
//
//    //#[bench]
//    //fn bench_offset(b: &mut Bencher) {
//    //    b.iter(|| offset());
//    //}
//
//    //#[bench]
//    //fn bench_sql(b: &mut Bencher) {
//    //    b.iter(|| sql());
//    //}
//
//    //#[bench]
//    //fn bench_subquery(b: &mut Bencher) {
//    //    b.iter(|| subquery());
//    //}
//
//    //#[bench]
//    //fn bench_subquery_as(b: &mut Bencher) {
//    //    b.iter(|| subquery_as());
//    //}
//
//    //#[bench]
//    //fn bench_query(b: &mut Bencher) {
//    //    b.iter(|| query());
//    //}
//
//    //#[bench]
//    //fn bench_query_values(b: &mut Bencher) {
//    //    b.iter(|| query_values());
//    //}
//
//    #[bench]
//    fn bench_esc(b: &mut Bencher) {
//        b.iter(|| esc("Hello, 'World'"));
//    }
//
//    #[bench]
//    fn bench_quote(b: &mut Bencher) {
//        b.iter(|| quote("Hello, 'World'"));
//    }
//
//    #[bench]
//    fn bench_x_select_only_values(b: &mut Bencher) {
//        b.iter(|| SqlBuilder::select_values(&["10", &quote("100")]).sql());
//    }
//
//    #[bench]
//    fn bench_x_select_all_books(b: &mut Bencher) {
//        b.iter(|| SqlBuilder::select_from("books").sql());
//    }
//
//    #[bench]
//    fn bench_x_show_all_prices(b: &mut Bencher) {
//        b.iter(|| {
//            SqlBuilder::select_from("books")
//                .distinct()
//                .field("price")
//                .sql()
//        });
//    }
//
//    #[bench]
//    fn bench_x_select_title_and_price_1(b: &mut Bencher) {
//        b.iter(|| {
//            SqlBuilder::select_from("books")
//                .fields(&["title", "price"])
//                .sql()
//        });
//    }
//
//    #[bench]
//    fn bench_x_select_title_and_price_2(b: &mut Bencher) {
//        b.iter(|| {
//            SqlBuilder::select_from("books")
//                .field("title")
//                .field("price")
//                .sql()
//        });
//    }
//
//    #[bench]
//    fn bench_x_select_expensive_books_1(b: &mut Bencher) {
//        b.iter(|| {
//            SqlBuilder::select_from("books")
//                .field("title")
//                .field("price")
//                .and_where("price > 100")
//                .sql()
//        });
//    }
//
//    #[bench]
//    fn bench_x_select_expensive_books_2(b: &mut Bencher) {
//        b.iter(|| {
//            SqlBuilder::select_from("books")
//                .field("title")
//                .field("price")
//                .and_where_gt("price", 200)
//                .sql()
//        });
//    }
//}
