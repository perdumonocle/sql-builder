use crate::arg::SqlArg;
use std::collections::HashMap;

pub trait Bind {
    /// Replace first ? with a value.
    ///
    /// ```
    /// # use std::error::Error;
    /// # use anyhow::Result;
    /// use sql_builder::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .fields(&["title", "price"])
    ///     .and_where("price BETWEEN ? AND ?".bind(&100).bind(&200))
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price BETWEEN 100 AND 200;", &sql);
    /// # Ok(())
    /// # }
    /// ```
    fn bind(&self, arg: &dyn SqlArg) -> String;

    /// Cyclic bindings of values.
    ///
    /// ```
    /// # use std::error::Error;
    /// # use anyhow::Result;
    /// use sql_builder::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .fields(&["title", "price"])
    ///     .and_where("price > ? AND title LIKE ?".binds(&[&100, &"Harry Potter%"]))
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price > 100 AND title LIKE 'Harry Potter%';", &sql);
    /// # Ok(())
    /// # }
    /// ```
    fn binds(&self, args: &[&dyn SqlArg]) -> String;

    /// Replace all $N with a value.
    ///
    /// ```
    /// # use std::error::Error;
    /// # use anyhow::Result;
    /// use sql_builder::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .fields(&["title", "price"])
    ///     .and_where("price > $1 AND price < $1 + $2"
    ///                    .bind_num(1, &100)
    ///                    .bind_num(2, &200))
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price > 100 AND price < 100 + 200;", &sql);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ```
    /// # use std::error::Error;
    /// # use anyhow::Result;
    /// use sql_builder::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .fields(&["title", "price"])
    ///     .and_where("price > $1")
    ///     .and_where("price < $1 + $2")
    ///     .sql()?
    ///     .bind_num(1, &100)
    ///     .bind_num(2, &200);
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE (price > 100) AND (price < 100 + 200);", &sql);
    /// # Ok(())
    /// # }
    /// ```
    fn bind_num(&self, num: u16, arg: &dyn SqlArg) -> String;

    /// Replace $1, $2, ... with elements of array.
    /// Escape the $ symbol with another $ symbol.
    ///
    /// ```
    /// # use std::error::Error;
    /// # use anyhow::Result;
    /// use sql_builder::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .fields(&["title", "price"])
    ///     .and_where("price > $1 AND price < $1 + $2".bind_nums(&[&100, &200]))
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price > 100 AND price < 100 + 200;", &sql);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ```
    /// # use std::error::Error;
    /// # use anyhow::Result;
    /// use sql_builder::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .fields(&["title", "price"])
    ///     .and_where("price > $1")
    ///     .and_where("price < $1 + $2")
    ///     .sql()?
    ///     .bind_nums(&[&100, &200]);
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE (price > 100) AND (price < 100 + 200);", &sql);
    /// # Ok(())
    /// # }
    /// ```
    fn bind_nums(&self, args: &[&dyn SqlArg]) -> String;

    /// Replace all :name: with a value.
    ///
    /// ```
    /// # use std::error::Error;
    /// # use anyhow::Result;
    /// use sql_builder::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::insert_into("books")
    ///     .fields(&["title", "price"])
    ///     .values(&[":name:, :costs:"])
    ///     .sql()?
    ///     .bind_name(&"name", &"Harry Potter and the Philosopher's Stone")
    ///     .bind_name(&"costs", &150);
    ///
    /// assert_eq!("INSERT INTO books (title, price) VALUES ('Harry Potter and the Philosopher''s Stone', 150);", &sql);
    /// # Ok(())
    /// # }
    /// ```
    fn bind_name(&self, name: &dyn ToString, arg: &dyn SqlArg) -> String;

    /// Replace each :name: from map.
    /// Escape the : symbol with another : symbol.
    ///
    /// ```
    /// # use std::error::Error;
    /// # use anyhow::Result;
    /// use sql_builder::prelude::*;
    /// use std::collections::HashMap;
    ///
    /// # fn main() -> Result<()> {
    /// let mut names: HashMap<&str, &dyn SqlArg> = HashMap::new();
    /// names.insert("name", &"Harry Potter and the Philosopher's Stone");
    /// names.insert("costs", &150);
    ///
    /// let sql = SqlBuilder::insert_into("books")
    ///     .fields(&["title", "price"])
    ///     .values(&[":name:, :costs:"])
    ///     .sql()?
    ///     .bind_names(&names);
    ///
    /// assert_eq!("INSERT INTO books (title, price) VALUES ('Harry Potter and the Philosopher''s Stone', 150);", &sql);
    /// # Ok(())
    /// # }
    /// ```
    fn bind_names(&self, names: &dyn BindNames) -> String;
}

impl Bind for &str {
    /// Replace first ? with a value.
    ///
    /// ```
    /// # use std::error::Error;
    /// # use anyhow::Result;
    /// use sql_builder::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .fields(&["title", "price"])
    ///     .and_where("price BETWEEN ? AND ?".bind(&100).bind(&200))
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price BETWEEN 100 AND 200;", &sql);
    /// # Ok(())
    /// # }
    /// ```
    fn bind(&self, arg: &dyn SqlArg) -> String {
        (*self).to_string().bind(arg)
    }

    /// Cyclic bindings of values.
    ///
    /// ```
    /// # use std::error::Error;
    /// # use anyhow::Result;
    /// use sql_builder::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .fields(&["title", "price"])
    ///     .and_where("price > ? AND title LIKE ?".binds(&[&100, &"Harry Potter%"]))
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price > 100 AND title LIKE 'Harry Potter%';", &sql);
    /// # Ok(())
    /// # }
    /// ```
    fn binds(&self, args: &[&dyn SqlArg]) -> String {
        (*self).to_string().binds(args)
    }

    /// Replace all $N with a value.
    ///
    /// ```
    /// # use std::error::Error;
    /// # use anyhow::Result;
    /// use sql_builder::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .fields(&["title", "price"])
    ///     .and_where("price > $1 AND price < $1 + $2"
    ///                    .bind_num(1, &100)
    ///                    .bind_num(2, &200))
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price > 100 AND price < 100 + 200;", &sql);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ```
    /// # use std::error::Error;
    /// # use anyhow::Result;
    /// use sql_builder::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .fields(&["title", "price"])
    ///     .and_where("price > $1")
    ///     .and_where("price < $1 + $2")
    ///     .sql()?
    ///     .bind_num(1, &100)
    ///     .bind_num(2, &200);
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE (price > 100) AND (price < 100 + 200);", &sql);
    /// # Ok(())
    /// # }
    /// ```
    fn bind_num(&self, num: u16, arg: &dyn SqlArg) -> String {
        (*self).to_string().bind_num(num, arg)
    }

    /// Replace $1, $2, ... with elements of array.
    /// Escape the $ symbol with another $ symbol.
    ///
    /// ```
    /// # use std::error::Error;
    /// # use anyhow::Result;
    /// use sql_builder::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .fields(&["title", "price"])
    ///     .and_where("price > $1 AND price < $1 + $2".bind_nums(&[&100, &200]))
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price > 100 AND price < 100 + 200;", &sql);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ```
    /// # use std::error::Error;
    /// # use anyhow::Result;
    /// use sql_builder::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .fields(&["title", "price"])
    ///     .and_where("price > $1")
    ///     .and_where("price < $1 + $2")
    ///     .sql()?
    ///     .bind_nums(&[&100, &200]);
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE (price > 100) AND (price < 100 + 200);", &sql);
    /// # Ok(())
    /// # }
    /// ```
    fn bind_nums(&self, args: &[&dyn SqlArg]) -> String {
        (*self).to_string().bind_nums(args)
    }

    /// Replace all :name: with a value.
    ///
    /// ```
    /// # use std::error::Error;
    /// # use anyhow::Result;
    /// use sql_builder::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::insert_into("books")
    ///     .fields(&["title", "price"])
    ///     .values(&[":name:, :costs:"])
    ///     .sql()?
    ///     .bind_name(&"name", &"Harry Potter and the Philosopher's Stone")
    ///     .bind_name(&"costs", &150);
    ///
    /// assert_eq!("INSERT INTO books (title, price) VALUES ('Harry Potter and the Philosopher''s Stone', 150);", &sql);
    /// # Ok(())
    /// # }
    /// ```
    fn bind_name(&self, name: &dyn ToString, arg: &dyn SqlArg) -> String {
        (*self).to_string().bind_name(name, arg)
    }

    /// Replace each :name: from map.
    /// Escape the : symbol with another : symbol.
    ///
    /// ```
    /// # use std::error::Error;
    /// # use anyhow::Result;
    /// use sql_builder::prelude::*;
    /// use std::collections::HashMap;
    ///
    /// # fn main() -> Result<()> {
    /// let mut names: HashMap<&str, &dyn SqlArg> = HashMap::new();
    /// names.insert("name", &"Harry Potter and the Philosopher's Stone");
    /// names.insert("costs", &150);
    ///
    /// let sql = SqlBuilder::insert_into("books")
    ///     .fields(&["title", "price"])
    ///     .values(&[":name:, :costs:"])
    ///     .sql()?
    ///     .bind_names(&names);
    ///
    /// assert_eq!("INSERT INTO books (title, price) VALUES ('Harry Potter and the Philosopher''s Stone', 150);", &sql);
    /// # Ok(())
    /// # }
    /// ```
    fn bind_names<'a>(&self, names: &dyn BindNames) -> String {
        (*self).to_string().bind_names(names)
    }
}

impl Bind for String {
    /// Replace first ? with a value.
    ///
    /// ```
    /// # use std::error::Error;
    /// # use anyhow::Result;
    /// use sql_builder::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .fields(&["title", "price"])
    ///     .and_where("price BETWEEN ? AND ?".bind(&100).bind(&200))
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price BETWEEN 100 AND 200;", &sql);
    /// # Ok(())
    /// # }
    /// ```
    fn bind(&self, arg: &dyn SqlArg) -> String {
        self.replacen('?', &arg.sql_arg(), 1)
    }

    /// Cyclic bindings of values.
    ///
    /// ```
    /// # use std::error::Error;
    /// # use anyhow::Result;
    /// use sql_builder::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .fields(&["title", "price"])
    ///     .and_where("price > ? AND title LIKE ?".binds(&[&100, &"Harry Potter%"]))
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price > 100 AND title LIKE 'Harry Potter%';", &sql);
    /// # Ok(())
    /// # }
    /// ```
    fn binds(&self, args: &[&dyn SqlArg]) -> String {
        let mut offset = 0;
        let mut res = String::new();
        let len = args.len();
        for ch in self.chars() {
            if ch == '?' {
                res.push_str(&args[offset].sql_arg());
                offset = (offset + 1) % len;
            } else {
                res.push(ch);
            }
        }
        res
    }

    /// Replace all $N with a value.
    ///
    /// ```
    /// # use std::error::Error;
    /// # use anyhow::Result;
    /// use sql_builder::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .fields(&["title", "price"])
    ///     .and_where("price > $1 AND price < $1 + $2"
    ///                    .bind_num(1, &100)
    ///                    .bind_num(2, &200))
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price > 100 AND price < 100 + 200;", &sql);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ```
    /// # use std::error::Error;
    /// # use anyhow::Result;
    /// use sql_builder::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .fields(&["title", "price"])
    ///     .and_where("price > $1")
    ///     .and_where("price < $1 + $2")
    ///     .sql()?
    ///     .bind_num(1, &100)
    ///     .bind_num(2, &200);
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE (price > 100) AND (price < 100 + 200);", &sql);
    /// # Ok(())
    /// # }
    /// ```
    fn bind_num(&self, num: u16, arg: &dyn SqlArg) -> String {
        let rep = format!("${}", &num);
        self.replace(&rep, &arg.sql_arg())
    }

    /// Replace $1, $2, ... with elements of array.
    /// Escape the $ symbol with another $ symbol.
    ///
    /// ```
    /// # use std::error::Error;
    /// # use anyhow::Result;
    /// use sql_builder::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .fields(&["title", "price"])
    ///     .and_where("price > $1 AND price < $1 + $2".bind_nums(&[&100, &200]))
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price > 100 AND price < 100 + 200;", &sql);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ```
    /// # use std::error::Error;
    /// # use anyhow::Result;
    /// use sql_builder::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .fields(&["title", "price"])
    ///     .and_where("price > $1")
    ///     .and_where("price < $1 + $2")
    ///     .sql()?
    ///     .bind_nums(&[&100, &200]);
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE (price > 100) AND (price < 100 + 200);", &sql);
    /// # Ok(())
    /// # }
    /// ```
    fn bind_nums(&self, args: &[&dyn SqlArg]) -> String {
        let mut res = String::new();
        let mut num = 0usize;
        let mut wait_digit = false;
        let len = args.len();
        for ch in self.chars() {
            if ch == '$' {
                if wait_digit {
                    if num > 0 {
                        let idx = num - 1;
                        if len > idx {
                            res.push_str(&args[idx].sql_arg());
                        }
                        num = 0;
                    } else {
                        wait_digit = false;
                        res.push(ch);
                    }
                } else {
                    wait_digit = true;
                }
            } else if wait_digit {
                if let Some(digit) = ch.to_digit(10) {
                    num = num * 10 + digit as usize;
                } else {
                    let idx = num - 1;
                    if len > idx {
                        res.push_str(&args[idx].sql_arg());
                    }
                    res.push(ch);
                    wait_digit = false;
                    num = 0;
                }
            } else {
                res.push(ch);
            }
        }
        if wait_digit && num > 0 {
            let idx = num - 1;
            if len > idx {
                res.push_str(&args[idx].sql_arg());
            }
        }
        res
    }

    /// Replace all :name: with a value.
    ///
    /// ```
    /// # use std::error::Error;
    /// # use anyhow::Result;
    /// use sql_builder::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::insert_into("books")
    ///     .fields(&["title", "price"])
    ///     .values(&[":name:, :costs:"])
    ///     .sql()?
    ///     .bind_name(&"name", &"Harry Potter and the Philosopher's Stone")
    ///     .bind_name(&"costs", &150);
    ///
    /// assert_eq!("INSERT INTO books (title, price) VALUES ('Harry Potter and the Philosopher''s Stone', 150);", &sql);
    /// # Ok(())
    /// # }
    /// ```
    fn bind_name(&self, name: &dyn ToString, arg: &dyn SqlArg) -> String {
        let rep = format!(":{}:", &name.to_string());
        self.replace(&rep, &arg.sql_arg())
    }

    /// Replace each :name: from map.
    /// Escape the : symbol with another : symbol.
    ///
    /// ```
    /// # use std::error::Error;
    /// # use anyhow::Result;
    /// use sql_builder::prelude::*;
    /// use std::collections::HashMap;
    ///
    /// # fn main() -> Result<()> {
    /// let mut names: HashMap<&str, &dyn SqlArg> = HashMap::new();
    /// names.insert("name", &"Harry Potter and the Philosopher's Stone");
    /// names.insert("costs", &150);
    ///
    /// let sql = SqlBuilder::insert_into("books")
    ///     .fields(&["title", "price"])
    ///     .values(&[":name:, :costs:"])
    ///     .sql()?
    ///     .bind_names(&names);
    ///
    /// assert_eq!("INSERT INTO books (title, price) VALUES ('Harry Potter and the Philosopher''s Stone', 150);", &sql);
    /// # Ok(())
    /// # }
    /// ```
    fn bind_names<'a>(&self, names: &dyn BindNames) -> String {
        let mut res = String::new();
        let mut key = String::new();
        let mut wait_colon = false;
        let names = names.names_map();
        for ch in self.chars() {
            if ch == ':' {
                if wait_colon {
                    if key.is_empty() {
                        res.push(ch);
                    } else {
                        let skey = key.to_string();
                        if let Some(value) = names.get(&*skey) {
                            res.push_str(&value.sql_arg());
                        } else {
                            res.push_str("NULL");
                        }
                        key = String::new();
                    }
                    wait_colon = false;
                } else {
                    wait_colon = true;
                }
            } else if wait_colon {
                key.push(ch);
            } else {
                res.push(ch);
            }
        }
        if wait_colon {
            res.push(';');
            res.push_str(&key);
        }
        res
    }
}

pub trait BindNames<'a> {
    fn names_map(&self) -> HashMap<&'a str, &dyn SqlArg>;
}

impl<'a> BindNames<'a> for HashMap<&'a str, &dyn SqlArg> {
    fn names_map(&self) -> HashMap<&'a str, &dyn SqlArg> {
        self.to_owned()
    }
}

impl<'a> BindNames<'a> for &HashMap<&'a str, &dyn SqlArg> {
    fn names_map(&self) -> HashMap<&'a str, &dyn SqlArg> {
        self.to_owned().to_owned()
    }
}

impl<'a> BindNames<'a> for Vec<(&'a str, &dyn SqlArg)> {
    fn names_map(&self) -> HashMap<&'a str, &dyn SqlArg> {
        let mut map = HashMap::new();
        for (k, v) in self.iter() {
            map.insert(*k, *v);
        }
        map
    }
}

impl<'a> BindNames<'a> for &[(&'a str, &dyn SqlArg)] {
    fn names_map(&self) -> HashMap<&'a str, &dyn SqlArg> {
        let mut map = HashMap::new();
        for (k, v) in self.iter() {
            map.insert(*k, *v);
        }
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use anyhow::Result;

    #[test]
    fn test_bind() -> Result<()> {
        let foo = "f?o?o";

        assert_eq!("'lol'foo?", &"?foo?".bind(&"lol"));
        assert_eq!("'lol'foo10", &"?foo?".bind(&"lol").bind(&10));
        assert_eq!("'lol'foo?", &"?foo?".bind(&String::from("lol")));
        assert_eq!("'lol'foo?", &String::from("?foo?").bind(&"lol"));
        assert_eq!("f'lol'o?o", &foo.bind(&"lol"));
        assert_eq!("fo'f?o?o'o", &"fo?o".bind(&foo));
        assert_eq!("fo10o", &"fo?o".bind(&10_usize));
        assert_eq!("fo10o", &"fo?o".bind(&10));
        assert_eq!("fo10o", &"fo?o".bind(&10_isize));
        assert_eq!("foTRUEo", &"fo?o".bind(&true));
        assert_eq!("foFALSEo", &"fo?o".bind(&false));
        assert_eq!(
            "10f'lol'o10o$3",
            &"$1f$2o$1o$3".bind_num(1, &10_u8).bind_num(2, &"lol")
        );
        assert_eq!("f'lol'oo:def:", &"f:abc:oo:def:".bind_name(&"abc", &"lol"));

        Ok(())
    }

    #[test]
    fn test_binds() -> Result<()> {
        assert_eq!("10f20o30o10", &"?f?o?o?".binds(&[&10, &20, &30]));
        assert_eq!(
            "'abc'f'def'o'ghi'o'abc'",
            &"?f?o?o?".binds(&[&"abc", &"def", &"ghi"])
        );
        assert_eq!(
            "10f20o30o10",
            &String::from("?f?o?o?").binds(&[&10, &20, &30])
        );
        assert_eq!(
            "10f'AAA'oTRUEo10",
            &String::from("?f?o?o?").binds(&[&10, &"AAA", &true])
        );
        assert_eq!(
            "10f'AAA'o$oTRUE",
            &String::from("$1f$02o$$o$3$4").bind_nums(&[&10, &"AAA", &true])
        );
        assert_eq!(
            "1f1.5o0.0000001o1",
            &"?f?o?o?".binds(&[&1.0, &1.5, &0.0000001])
        );

        Ok(())
    }

    #[test]
    fn test_bind_doc() -> Result<()> {
        let sql = SqlBuilder::select_from("books")
            .fields(&["title", "price"])
            .and_where("price > ? AND title LIKE ?".binds(&[&100, &"Harry Potter%"]))
            .sql()?;

        assert_eq!(
            "SELECT title, price FROM books WHERE price > 100 AND title LIKE 'Harry Potter%';",
            &sql
        );

        Ok(())
    }

    #[test]
    fn test_bind_names() -> Result<()> {
        let mut names: HashMap<&str, &dyn SqlArg> = HashMap::new();
        names.insert("aaa", &10);
        names.insert("bbb", &20);
        names.insert("ccc", &"tt");
        names.insert("ddd", &40);

        let sql = SqlBuilder::insert_into("books")
            .fields(&["title", "price"])
            .values(&["'a_book', :aaa:"])
            .values(&["'c_book', :ccc:"])
            .values(&["'e_book', :eee:"])
            .sql()?
            .bind_names(&names);

        assert_eq!(
            "INSERT INTO books (title, price) VALUES ('a_book', 10), ('c_book', 'tt'), ('e_book', NULL);",
            &sql
        );

        let names: Vec<(&str, &dyn SqlArg)> =
            vec![("aaa", &10), ("bbb", &20), ("ccc", &"tt"), ("ddd", &40)];

        let sql = SqlBuilder::insert_into("books")
            .fields(&["title", "price"])
            .values(&["'a_book', :aaa:"])
            .values(&["'c_book', :ccc:"])
            .values(&["'e_book', :eee:"])
            .sql()?
            .bind_names(&names);

        assert_eq!(
            "INSERT INTO books (title, price) VALUES ('a_book', 10), ('c_book', 'tt'), ('e_book', NULL);",
            &sql
        );

        Ok(())
    }

    #[test]
    fn test_null() -> Result<()> {
        let foo: Option<&str> = None;
        assert_eq!("foNULLo", &"fo?o".bind(&foo));

        let foo = Some("foo");
        assert_eq!("fo'foo'o", &"fo?o".bind(&foo));

        Ok(())
    }
}
