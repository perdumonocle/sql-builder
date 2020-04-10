use crate::arg::SqlArg;

pub trait Bind {
    fn bind(&self, arg: &dyn SqlArg) -> String;
    fn binds(&self, args: &[&dyn SqlArg]) -> String;
}

impl Bind for &str {
    /// Replace first ? with a value.
    ///
    /// ```
    /// # use std::error::Error;
    /// use sql_builder::prelude::*;
    ///
    /// # fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
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
    /// use sql_builder::prelude::*;
    ///
    /// # fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .fields(&["title", "price"])
    ///     .and_where("price > ? AND title LIKE ?".binds(&[&100, &"Harry Potter%"]))
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price > 100 AND title LIKE 'Harry Potter%';", &sql);
    /// // add             ^^^^^^^^^^^^
    /// // here               fields
    /// # Ok(())
    /// # }
    /// ```
    fn binds(&self, args: &[&dyn SqlArg]) -> String {
        (*self).to_string().binds(args)
    }
}

impl Bind for String {
    fn bind(&self, arg: &dyn SqlArg) -> String {
        self.replacen('?', &arg.sql_arg(), 1)
    }

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use std::error::Error;

    #[test]
    fn test_bind() -> Result<(), Box<dyn Error + Send + Sync>> {
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

        Ok(())
    }

    #[test]
    fn test_binds() -> Result<(), Box<dyn Error + Send + Sync>> {
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

        Ok(())
    }

    #[test]
    fn test_bind_doc() -> Result<(), Box<dyn Error + Send + Sync>> {
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
}
