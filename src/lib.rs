//! Simple SQL code generator.
//!
//! ## Usage
//!
//! To use `sql-builder`, first add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! sql-builder = "0.2"
//! ```
//!
//! Next, add this to your crate:
//!
//! ```
//! extern crate sql_builder;
//!
//! use sql_builder::SqlBuilder;
//! ```
//!
//! # Examples:
//!
//! ```
//! extern crate sql_builder;
//!
//! # use std::error::Error;
//! use sql_builder::SqlBuilder;
//!
//! # fn main() -> Result<(), Box<dyn Error>> {
//! let sql = SqlBuilder::select_from("company")
//!     .field("id")
//!     .field("name")
//!     .and_where("salary > 25000")
//!     .sql()?;
//!
//! assert_eq!("SELECT id, name FROM company WHERE salary > 25000;", &sql);
//! # Ok(())
//! # }
//! ```

use std::error::Error;

/// Main SQL builder
pub struct SqlBuilder {
    statement: Statement,
    table: String,
    joins: Vec<String>,
    distinct: bool,
    fields: Vec<String>,
    sets: Vec<String>,
    values: Values,
    group_by: Vec<String>,
    having: Option<String>,
    unions: String,
    wheres: Vec<String>,
    order_by: Vec<String>,
    limit: Option<usize>,
    offset: Option<usize>,
}

/// SQL query statement
enum Statement {
    SelectFrom,
    SelectValues,
    UpdateTable,
    InsertInto,
    DeleteFrom,
}

/// INSERT values
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
            joins: Vec::new(),
            distinct: false,
            fields: Vec::new(),
            sets: Vec::new(),
            values: Values::Empty,
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
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .and_where("price > 100")
    ///     .and_where_like("title", "Harry Potter%")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE (price > 100) AND (title LIKE 'Harry Potter%');", &sql);
    /// // add                               ^^^^^
    /// // here                              table
    /// # Ok(())
    /// # }
    /// ```
    pub fn select_from(table: &str) -> Self {
        Self {
            table: table.to_string(),
            ..Self::default()
        }
    }

    /// Create SELECT query without a table.
    ///
    /// ```
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::{SqlBuilder, quote};
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let sql = SqlBuilder::select_values(&["10", &quote("100")])
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT 10, '100';", &sql);
    /// // add             ^^^^^^^^^
    /// // here             values
    /// # Ok(())
    /// # }
    /// ```
    pub fn select_values(values: &[&str]) -> Self {
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
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::{SqlBuilder, quote};
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let sql = SqlBuilder::insert_into("books")
    ///     .field("title")
    ///     .field("price")
    ///     .values(&[&quote("In Search of Lost Time"), "150"])
    ///     .values(&["'Don Quixote', 200"])
    ///     .sql()?;
    ///
    /// assert_eq!("INSERT INTO books (title, price) VALUES ('In Search of Lost Time', 150), ('Don Quixote', 200);", &sql);
    /// // add                  ^^^^^
    /// // here                 table
    /// # Ok(())
    /// # }
    /// ```
    pub fn insert_into(table: &str) -> Self {
        Self {
            statement: Statement::InsertInto,
            table: table.to_string(),
            ..Self::default()
        }
    }

    /// Create UPDATE query.
    ///
    /// ```
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
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
    pub fn update_table(table: &str) -> Self {
        Self {
            statement: Statement::UpdateTable,
            table: table.to_string(),
            ..Self::default()
        }
    }

    /// Create DELETE query.
    ///
    /// ```
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
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
    pub fn delete_from(table: &str) -> Self {
        Self {
            statement: Statement::DeleteFrom,
            table: table.to_string(),
            ..Self::default()
        }
    }

    /// Join with table.
    ///
    /// ```
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let sql = SqlBuilder::select_from("books AS b")
    ///     .field("b.title")
    ///     .field("s.total")
    ///     .join("shops AS s", Some("LEFT OUTER"), Some("ON b.id = s.book"))
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT b.title, s.total FROM books AS b LEFT OUTER JOIN shops AS s ON b.id = s.book;", &sql);
    /// // add                                              ^^^^^^^^^^      ^^^^^^^^^^ ^^^^^^^^^^^^^^^^
    /// // here                                              operator         table       constraint
    /// # Ok(())
    /// # }
    /// ```
    pub fn join(
        &mut self,
        table: &str,
        operator: Option<&str>,
        constraint: Option<&str>,
    ) -> &mut Self {
        let operator = if let Some(oper) = operator {
            format!("{} JOIN ", &oper)
        } else {
            String::new()
        };

        let constraint = if let Some(cons) = constraint {
            format!(" {}", &cons)
        } else {
            String::new()
        };

        let text = format!("{}{}{}", &operator, &table, &constraint);

        self.joins.push(text);
        self
    }

    /// Set DISTINCT for fields.
    ///
    /// ```
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
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
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
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
    pub fn fields(&mut self, fields: &[&str]) -> &mut Self {
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
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::SqlBuilder;
    /// # #[derive(Default)]
    /// # struct ReqData { filter: Option<String>, price_min: Option<u64>, price_max: Option<u64>,
    /// # limit: Option<usize>, offset: Option<usize> }
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// # let req_data = ReqData::default();
    /// // Prepare query for total count
    ///
    /// let mut db = SqlBuilder::select_from("books");
    ///
    /// db.field("COUNT(id)");
    ///
    /// if let Some(filter) = &req_data.filter {
    ///   let item = format!("LOWER(title) LIKE '%{}%'", filter.to_lowercase());
    ///   db.and_where(&item);
    /// }
    ///
    /// if let Some(price_min) = &req_data.price_min {
    ///   let item = format!("price >= {}", price_min);
    ///   db.and_where(&item);
    /// }
    ///
    /// if let Some(price_max) = &req_data.price_max {
    ///   let item = format!("price <= {}", price_max);
    ///   db.and_where(&item);
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
    pub fn set_fields(&mut self, fields: &[&str]) -> &mut Self {
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
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
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
    pub fn field(&mut self, field: &str) -> &mut Self {
        self.fields.push(field.to_string());
        self
    }

    /// Replace fields with choosed one.
    ///
    /// ```
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::SqlBuilder;
    /// # #[derive(Default)]
    /// # struct ReqData { filter: Option<String>, price_min: Option<u64>, price_max: Option<u64>,
    /// # limit: Option<usize>, offset: Option<usize> }
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// # let req_data = ReqData::default();
    /// // Prepare query for total count
    ///
    /// let mut db = SqlBuilder::select_from("books");
    ///
    /// db.field("COUNT(id)");
    ///
    /// if let Some(filter) = &req_data.filter {
    ///   let item = format!("LOWER(title) LIKE '%{}%'", filter.to_lowercase());
    ///   db.and_where(&item);
    /// }
    ///
    /// if let Some(price_min) = &req_data.price_min {
    ///   let item = format!("price >= {}", price_min);
    ///   db.and_where(&item);
    /// }
    ///
    /// if let Some(price_max) = &req_data.price_max {
    ///   let item = format!("price <= {}", price_max);
    ///   db.and_where(&item);
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
    pub fn set_field(&mut self, field: &str) -> &mut Self {
        self.fields = vec![field.to_string()];
        self
    }

    /// Add SET part (for UPDATE).
    ///
    /// ```
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
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
    pub fn set(&mut self, field: &str, value: &str) -> &mut Self {
        let expr = format!("{} = {}", &field, &value);
        self.sets.push(expr);
        self
    }

    /// Add VALUES part (for INSERT).
    ///
    /// ```
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::{SqlBuilder, quote};
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let sql = SqlBuilder::insert_into("books")
    ///     .field("title")
    ///     .field("price")
    ///     .values(&[&quote("In Search of Lost Time"), "150"])
    ///     .values(&["'Don Quixote', 200"])
    ///     .sql()?;
    ///
    /// assert_eq!("INSERT INTO books (title, price) VALUES ('In Search of Lost Time', 150), ('Don Quixote', 200);", &sql);
    /// // add                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^    ^^^^^^^^^^^^^^^^^^
    /// // here                                                         values                      values
    /// # Ok(())
    /// # }
    /// ```
    pub fn values(&mut self, values: &[&str]) -> &mut Self {
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
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
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
    pub fn select(&mut self, query: &str) -> &mut Self {
        self.values = Values::Select(query.to_string());
        self
    }

    /// Add GROUP BY part.
    ///
    /// ```
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::{SqlBuilder, quote};
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
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
    pub fn group_by(&mut self, field: &str) -> &mut Self {
        self.group_by.push(field.to_string());
        self
    }

    /// Add HAVING condition.
    ///
    /// ```
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::{SqlBuilder, quote};
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
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
    pub fn having(&mut self, cond: &str) -> &mut Self {
        self.having = Some(cond.to_string());
        self
    }

    /// Add WHERE condition.
    ///
    /// ```
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::{SqlBuilder, quote};
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
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
    pub fn and_where(&mut self, cond: &str) -> &mut Self {
        self.wheres.push(cond.to_string());
        self
    }

    /// Add WHERE condition for equal parts.
    ///
    /// ```
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::{SqlBuilder, quote};
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
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
    pub fn and_where_eq(&mut self, field: &str, value: &str) -> &mut Self {
        let cond = format!("{} = {}", &field, &value);
        self.and_where(&cond)
    }

    /// Add WHERE condition for non-equal parts.
    ///
    /// ```
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::{SqlBuilder, quote};
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
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
    pub fn and_where_ne(&mut self, field: &str, value: &str) -> &mut Self {
        let cond = format!("{} <> {}", &field, &value);
        self.and_where(&cond)
    }

    /// Add WHERE LIKE condition.
    ///
    /// ```
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
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
    pub fn and_where_like(&mut self, field: &str, mask: &str) -> &mut Self {
        let cond = format!("{} LIKE '{}'", &field, &esc(&mask));
        self.and_where(&cond)
    }

    /// Add WHERE NOT LIKE condition.
    ///
    /// ```
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
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
    pub fn and_where_not_like(&mut self, field: &str, mask: &str) -> &mut Self {
        let cond = format!("{} NOT LIKE '{}'", &field, &esc(&mask));
        self.and_where(&cond)
    }

    /// Add WHERE IS NULL condition.
    ///
    /// ```
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
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
    pub fn and_where_is_null(&mut self, field: &str) -> &mut Self {
        let cond = format!("{} IS NULL", &field);
        self.and_where(&cond)
    }

    /// Add WHERE IS NOT NULL condition.
    ///
    /// ```
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
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
    pub fn and_where_is_not_null(&mut self, field: &str) -> &mut Self {
        let cond = format!("{} IS NOT NULL", &field);
        self.and_where(&cond)
    }

    /// Union query with subquery.
    /// ORDER BY must be in the last subquery.
    ///
    /// ```
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
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
    ///     .and_where_like("title", "Harry Potter%")
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
    pub fn union(&mut self, query: &str) -> &mut Self {
        let append = format!(" UNION {}", &query);
        self.unions.push_str(&append);
        self
    }

    /// Union query with all subquery.
    /// ORDER BY must be in the last subquery.
    ///
    /// ```
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let append = SqlBuilder::select_values(&["'The Great Gatsby'", "124"])
    ///     .query_values()?;
    ///
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .and_where_like("title", "Harry Potter%")
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
    pub fn union_all(&mut self, query: &str) -> &mut Self {
        let append = format!(" UNION ALL {}", &query);
        self.unions.push_str(&append);
        self
    }

    /// Add ORDER BY.
    ///
    /// ```
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .and_where_like("title", "Harry Potter%")
    ///     .order_by("price", false)
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE title LIKE 'Harry Potter%' ORDER BY price;", &sql);
    /// // add                                                                               ^^^^^
    /// // here                                                                              field
    /// # Ok(())
    /// # }
    /// ```
    pub fn order_by(&mut self, field: &str, desc: bool) -> &mut Self {
        let order = if desc {
            format!("{} DESC", &field)
        } else {
            field.to_string()
        };
        self.order_by.push(order);
        self
    }

    /// Add ORDER BY ASC.
    ///
    /// ```
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .and_where_like("title", "Harry Potter%")
    ///     .order_asc("title")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE title LIKE 'Harry Potter%' ORDER BY title;", &sql);
    /// // add                                                                               ^^^^^
    /// // here                                                                              field
    /// # Ok(())
    /// # }
    /// ```
    pub fn order_asc(&mut self, field: &str) -> &mut Self {
        self.order_by(&field, false)
    }

    /// Add ORDER BY DESC.
    ///
    /// ```
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .and_where_like("title", "Harry Potter%")
    ///     .order_desc("price")
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE title LIKE 'Harry Potter%' ORDER BY price DESC;", &sql);
    /// // add                                                                               ^^^^^
    /// // here                                                                              field
    /// # Ok(())
    /// # }
    /// ```
    pub fn order_desc(&mut self, field: &str) -> &mut Self {
        self.order_by(&field, true)
    }

    /// Set LIMIT.
    ///
    /// ```
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .and_where_like("title", "Harry Potter%")
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
    pub fn limit(&mut self, limit: usize) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    /// Set OFFSET.
    ///
    /// ```
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .field("title")
    ///     .field("price")
    ///     .and_where_like("title", "Harry Potter%")
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
    pub fn offset(&mut self, offset: usize) -> &mut Self {
        self.offset = Some(offset);
        self
    }

    /// Build complete SQL command.
    ///
    /// ```
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let sql = SqlBuilder::select_from("books").sql()?;
    ///
    /// assert_eq!("SELECT * FROM books;", &sql);
    /// # Ok(())
    /// # }
    /// ```
    pub fn sql(&self) -> Result<String, Box<dyn Error>> {
        match self.statement {
            Statement::SelectFrom => self.sql_select(),
            Statement::SelectValues => self.sql_select_values(),
            Statement::UpdateTable => self.sql_update(),
            Statement::InsertInto => self.sql_insert(),
            Statement::DeleteFrom => self.sql_delete(),
        }
    }

    /// Build complete SQL command for SELECT statement
    fn sql_select(&self) -> Result<String, Box<dyn Error>> {
        // Checks
        if self.table.is_empty() {
            return Err("No table name".into());
        }

        // Build query
        let mut text = self.query()?;
        text.push(';');
        Ok(text)
    }

    /// Build complete SQL command for SELECT statement without a table
    fn sql_select_values(&self) -> Result<String, Box<dyn Error>> {
        // Checks
        if self.fields.is_empty() {
            return Err("No values".into());
        }

        // Build query
        let mut text = self.query_values()?;
        text.push(';');
        Ok(text)
    }

    /// Build subquery SQL command.
    ///
    /// ```
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
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
    pub fn subquery(&self) -> Result<String, Box<dyn Error>> {
        let text = self.query()?;
        let text = format!("({})", &text);
        Ok(text)
    }

    /// Build named subquery SQL command
    ///
    /// ```
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
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
    pub fn subquery_as(&self, name: &str) -> Result<String, Box<dyn Error>> {
        let text = self.query()?;
        let text = format!("({}) AS {}", &text, &name);
        Ok(text)
    }

    /// SQL command generator for query or subquery.
    ///
    /// ```
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::SqlBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
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
    pub fn query(&self) -> Result<String, Box<dyn Error>> {
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
        let limit = match self.limit {
            Some(limit) => format!(" LIMIT {}", limit),
            None => String::new(),
        };

        // Make OFFSET part
        let offset = match self.offset {
            Some(offset) => format!(" OFFSET {}", offset),
            None => String::new(),
        };

        // Make SQL
        let sql = format!("SELECT{distinct} {fields} FROM {table}{joins}{group_by}{wheres}{unions}{order_by}{limit}{offset}",
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
    /// extern crate sql_builder;
    ///
    /// # use std::error::Error;
    /// use sql_builder::{SqlBuilder, quote};
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let values = SqlBuilder::select_values(&["10", &quote("100")])
    ///     .query_values()?;
    ///
    /// assert_eq!("SELECT 10, '100'", &values);
    /// # Ok(())
    /// # }
    /// ```
    pub fn query_values(&self) -> Result<String, Box<dyn Error>> {
        // Make values
        let fields = self.fields.join(", ");

        // Make SQL
        let sql = format!("SELECT {fields}", fields = fields,);
        Ok(sql)
    }

    /// Build SQL command for INSERT statement
    fn sql_insert(&self) -> Result<String, Box<dyn Error>> {
        // Checks
        if self.table.is_empty() {
            return Err("No table name".into());
        }

        // Make SET part
        let fields = self.fields.join(", ");

        // Add values or query
        let sql = match &self.values {
            Values::Empty => return Err("No values".into()),
            Values::List(values) => {
                if values.is_empty() {
                    return Err("No values".into());
                }

                // Make VALUES part
                let values = values.join(", ");

                // Make SQL
                format!(
                    "INSERT INTO {table} ({fields}) VALUES {values};",
                    table = &self.table,
                    fields = fields,
                    values = values,
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
    fn sql_update(&self) -> Result<String, Box<dyn Error>> {
        // Checks
        if self.table.is_empty() {
            return Err("No table name".into());
        }
        if self.sets.is_empty() {
            return Err("No set fields".into());
        }

        // Make SET part
        let sets = self.sets.join(", ");

        // Make WHERE part
        let wheres = SqlBuilder::make_wheres(&self.wheres);

        // Make SQL
        let sql = format!(
            "UPDATE {table} SET {sets}{wheres};",
            table = &self.table,
            sets = sets,
            wheres = wheres,
        );
        Ok(sql)
    }

    /// Build SQL command for DELETE statement
    fn sql_delete(&self) -> Result<String, Box<dyn Error>> {
        // Checks
        if self.table.is_empty() {
            return Err("No table name".into());
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
/// extern crate sql_builder;
///
/// use sql_builder::esc;
///
/// let sql = esc("Hello, 'World'");
///
/// assert_eq!(&sql, "Hello, ''World''");
/// ```
pub fn esc(src: &str) -> String {
    src.replace("'", "''")
}

/// Quote string for SQL.
///
/// ```
/// extern crate sql_builder;
///
/// use sql_builder::quote;
///
/// let sql = quote("Hello, 'World'");
///
/// assert_eq!(&sql, "'Hello, ''World'''");
/// ```
pub fn quote(src: &str) -> String {
    format!("'{}'", esc(src))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_esc() -> Result<(), Box<dyn Error>> {
        let sql = esc("Hello, 'World'");

        assert_eq!(&sql, "Hello, ''World''");

        Ok(())
    }

    #[test]
    fn test_quote() -> Result<(), Box<dyn Error>> {
        let sql = quote("Hello, 'World'");

        assert_eq!(&sql, "'Hello, ''World'''");

        Ok(())
    }

    #[test]
    fn test_select_only_values() -> Result<(), Box<dyn Error>> {
        let values = SqlBuilder::select_values(&["10", &quote("100")]).sql()?;

        assert_eq!("SELECT 10, '100';", &values);

        Ok(())
    }

    #[test]
    fn test_select_all_books() -> Result<(), Box<dyn Error>> {
        let sql = SqlBuilder::select_from("books").sql()?;

        assert_eq!(&sql, "SELECT * FROM books;");

        Ok(())
    }

    #[test]
    fn test_show_all_prices() -> Result<(), Box<dyn Error>> {
        let sql = SqlBuilder::select_from("books")
            .distinct()
            .field("price")
            .sql()?;

        assert_eq!(&sql, "SELECT DISTINCT price FROM books;");

        Ok(())
    }

    #[test]
    fn test_select_title_and_price() -> Result<(), Box<dyn Error>> {
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
    fn test_select_expensive_books() -> Result<(), Box<dyn Error>> {
        let sql = SqlBuilder::select_from("books")
            .field("title")
            .field("price")
            .and_where("price > 100")
            .sql()?;

        assert_eq!(&sql, "SELECT title, price FROM books WHERE price > 100;");

        Ok(())
    }

    #[test]
    fn test_select_price_for_harry_potter_and_phil_stone() -> Result<(), Box<dyn Error>> {
        let sql = SqlBuilder::select_from("books")
            .field("price")
            .and_where_eq("title", &quote("Harry Potter and the Philosopher's Stone"))
            .sql()?;

        assert_eq!(
            &sql,
            "SELECT price FROM books WHERE title = 'Harry Potter and the Philosopher''s Stone';"
        );

        Ok(())
    }

    #[test]
    fn test_select_price_not_for_harry_potter_and_phil_stone() -> Result<(), Box<dyn Error>> {
        let sql = SqlBuilder::select_from("books")
            .field("price")
            .and_where_ne("title", &quote("Harry Potter and the Philosopher's Stone"))
            .sql()?;

        assert_eq!(
            &sql,
            "SELECT price FROM books WHERE title <> 'Harry Potter and the Philosopher''s Stone';"
        );

        Ok(())
    }

    #[test]
    fn test_select_expensive_harry_potter() -> Result<(), Box<dyn Error>> {
        let sql = SqlBuilder::select_from("books")
            .field("title")
            .field("price")
            .and_where("price > 100")
            .and_where_like("title", "Harry Potter%")
            .sql()?;

        assert_eq!(
            &sql,
            "SELECT title, price FROM books WHERE (price > 100) AND (title LIKE 'Harry Potter%');"
        );

        Ok(())
    }

    #[test]
    fn test_order_harry_potter_by_price() -> Result<(), Box<dyn Error>> {
        let sql = SqlBuilder::select_from("books")
            .field("title")
            .field("price")
            .and_where_like("title", "Harry Potter%")
            .order_by("price", false)
            .sql()?;

        assert_eq!(
            &sql,
            "SELECT title, price FROM books WHERE title LIKE 'Harry Potter%' ORDER BY price;"
        );

        let sql = SqlBuilder::select_from("books")
            .field("title")
            .field("price")
            .and_where_like("title", "Harry Potter%")
            .order_desc("price")
            .sql()?;

        assert_eq!(
            &sql,
            "SELECT title, price FROM books WHERE title LIKE 'Harry Potter%' ORDER BY price DESC;"
        );

        let sql = SqlBuilder::select_from("books")
            .field("title")
            .field("price")
            .and_where_like("title", "Harry Potter%")
            .order_desc("price")
            .order_asc("title")
            .sql()?;

        assert_eq!(&sql, "SELECT title, price FROM books WHERE title LIKE 'Harry Potter%' ORDER BY price DESC, title;");

        Ok(())
    }

    #[test]
    fn test_find_cheap_or_harry_potter() -> Result<(), Box<dyn Error>> {
        let append = SqlBuilder::select_from("books")
            .field("title")
            .field("price")
            .and_where("price < 100")
            .order_asc("title")
            .query()?;

        let sql = SqlBuilder::select_from("books")
            .field("title")
            .field("price")
            .and_where_like("title", "Harry Potter%")
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
            .and_where_like("title", "Harry Potter%")
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
    fn test_select_first_3_harry_potter_books() -> Result<(), Box<dyn Error>> {
        let sql = SqlBuilder::select_from("books")
            .field("title")
            .field("price")
            .and_where_like("title", "Harry Potter%")
            .order_asc("title")
            .limit(3)
            .sql()?;

        assert_eq!(&sql, "SELECT title, price FROM books WHERE title LIKE 'Harry Potter%' ORDER BY title LIMIT 3;");

        Ok(())
    }

    #[test]
    fn test_select_harry_potter_from_second_book() -> Result<(), Box<dyn Error>> {
        let sql = SqlBuilder::select_from("books")
            .field("title")
            .field("price")
            .and_where_like("title", "Harry Potter%")
            .order_asc("title")
            .offset(2)
            .sql()?;

        assert_eq!(&sql, "SELECT title, price FROM books WHERE title LIKE 'Harry Potter%' ORDER BY title OFFSET 2;");

        let sql = SqlBuilder::select_from("books")
            .field("title")
            .field("price")
            .and_where_like("title", "Harry Potter%")
            .order_asc("title")
            .limit(3)
            .offset(2)
            .sql()?;

        assert_eq!(&sql, "SELECT title, price FROM books WHERE title LIKE 'Harry Potter%' ORDER BY title LIMIT 3 OFFSET 2;");

        Ok(())
    }

    #[test]
    fn test_find_books_not_about_alice() -> Result<(), Box<dyn Error>> {
        let sql = SqlBuilder::select_from("books")
            .field("title")
            .and_where_not_like("title", "%Alice's%")
            .sql()?;

        assert_eq!(
            "SELECT title FROM books WHERE title NOT LIKE '%Alice''s%';",
            &sql
        );

        Ok(())
    }

    #[test]
    fn test_books_without_price() -> Result<(), Box<dyn Error>> {
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
    fn test_group_books_by_price() -> Result<(), Box<dyn Error>> {
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

        Ok(())
    }

    #[test]
    fn test_group_books_by_price_category() -> Result<(), Box<dyn Error>> {
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
    fn test_grow_price() -> Result<(), Box<dyn Error>> {
        let sql = SqlBuilder::update_table("books")
            .set("price", "price + 10")
            .sql()?;

        assert_eq!(&sql, "UPDATE books SET price = price + 10;");

        let sql = SqlBuilder::update_table("books")
            .set("price", "price * 0.1")
            .and_where_like("title", "Harry Potter%")
            .sql()?;

        assert_eq!(
            &sql,
            "UPDATE books SET price = price * 0.1 WHERE title LIKE 'Harry Potter%';"
        );

        Ok(())
    }

    #[test]
    fn test_add_new_books() -> Result<(), Box<dyn Error>> {
        let sql = SqlBuilder::insert_into("books")
            .field("title")
            .field("price")
            .values(&[&quote("In Search of Lost Time"), "150"])
            .values(&["'Don Quixote', 200"])
            .sql()?;

        assert_eq!(&sql, "INSERT INTO books (title, price) VALUES ('In Search of Lost Time', 150), ('Don Quixote', 200);");

        Ok(())
    }

    #[test]
    fn test_add_books_from_warehouse() -> Result<(), Box<dyn Error>> {
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
    fn test_sold_all_harry_potter() -> Result<(), Box<dyn Error>> {
        let sql = SqlBuilder::update_table("books")
            .set("price", "0")
            .set("title", "'[SOLD!]' || title")
            .and_where_like("title", "Harry Potter%")
            .sql()?;

        assert_eq!(&sql, "UPDATE books SET price = 0, title = '[SOLD!]' || title WHERE title LIKE 'Harry Potter%';");

        Ok(())
    }

    #[test]
    fn test_remove_all_expensive_books() -> Result<(), Box<dyn Error>> {
        let sql = SqlBuilder::delete_from("books")
            .and_where("price > 100")
            .sql()?;

        assert_eq!(&sql, "DELETE FROM books WHERE price > 100;");

        Ok(())
    }

    #[test]
    fn test_count_books_in_shops() -> Result<(), Box<dyn Error>> {
        let sql = SqlBuilder::select_from("books AS b")
            .field("b.title")
            .field("s.total")
            .join("shops AS s", Some("LEFT OUTER"), Some("ON b.id = s.book"))
            .sql()?;

        assert_eq!(
            &sql,
            "SELECT b.title, s.total FROM books AS b LEFT OUTER JOIN shops AS s ON b.id = s.book;"
        );

        Ok(())
    }
}
