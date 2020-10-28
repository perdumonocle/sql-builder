use crate::{baquote, brquote, dquote, quote};

/// Make safe name of identifier if it contains unsafe characters.
///
/// # Examples
/// ```
/// #[macro_use] extern crate sql_builder;
/// # use anyhow::Result;
/// use sql_builder::{SqlBuilder, SqlName};
///
/// # fn main() -> Result<()> {
/// let sql = SqlBuilder::select_from(name!("public", "BOOKS"; "b"))
///     .field(name!("b", "title"))
///     .field(name!("s", "total"))
///     .left()
///     .join(name!("shops"; "s"))
///     .on_eq(name!("b", "id"), name!("s", "book"))
///     .sql()?;
///
/// assert_eq!("SELECT b.title, s.total FROM `public`.`BOOKS` AS b LEFT JOIN shops AS s ON b.id = s.book;", &sql);
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! name {
    ( $n:expr ) => {
        {
            SqlName::new( $n ).safe()
        }
    };
    ( $n:expr, $( $x:expr ),* ) => {
        {
            SqlName::new( $n )
            $(
                .add( $x )
            )*
            .safe()
        }
    };
    ( $n:expr; $a:expr ) => {
        {
            SqlName::new( $n ).alias( $a ).safe()
        }
    };
    ( $n:expr, $( $x:expr ),*; $a:expr ) => {
        {
            SqlName::new( $n )
            $(
                .add( $x )
            )*
            .alias( $a )
            .safe()
        }
    };
}

/// Make quoted name of identifier.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate sql_builder;
/// # use anyhow::Result;
/// use sql_builder::{SqlBuilder, SqlName};
///
/// # fn main() -> Result<()> {
/// let sql = SqlBuilder::select_from(qname!("public", "BOOKS"; "b"))
///     .field(qname!("b", "title"))
///     .field(qname!("s", "total"))
///     .left()
///     .join(qname!("shops"; "s"))
///     .on_eq(qname!("b", "id"), qname!("s", "book"))
///     .sql()?;
///
/// assert_eq!("SELECT 'b'.'title', 's'.'total' FROM 'public'.'BOOKS' AS b LEFT JOIN 'shops' AS s ON 'b'.'id' = 's'.'book';", &sql);
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! qname {
    ( $n:expr ) => {
        {
            SqlName::new( $n ).quoted()
        }
    };
    ( $n:expr, $( $x:expr ),* ) => {
        {
            SqlName::new( $n )
            $(
                .add( $x )
            )*
            .quoted()
        }
    };
    ( $n:expr; $a:expr ) => {
        {
            SqlName::new( $n ).alias( $a ).quoted()
        }
    };
    ( $n:expr, $( $x:expr ),*; $a:expr ) => {
        {
            SqlName::new( $n )
            $(
                .add( $x )
            )*
            .alias( $a )
            .quoted()
        }
    };
}

/// Make backquoted name of identifier.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate sql_builder;
/// # use anyhow::Result;
/// use sql_builder::{SqlBuilder, SqlName};
///
/// # fn main() -> Result<()> {
/// let sql = SqlBuilder::select_from(baname!("public", "BOOKS"; "b"))
///     .field(baname!("b", "title"))
///     .field(baname!("s", "total"))
///     .left()
///     .join(baname!("shops"; "s"))
///     .on_eq(baname!("b", "id"), baname!("s", "book"))
///     .sql()?;
///
/// assert_eq!("SELECT `b`.`title`, `s`.`total` FROM `public`.`BOOKS` AS b LEFT JOIN `shops` AS s ON `b`.`id` = `s`.`book`;", &sql);
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! baname {
    ( $n:expr ) => {
        {
            SqlName::new( $n ).baquoted()
        }
    };
    ( $n:expr, $( $x:expr ),* ) => {
        {
            SqlName::new( $n )
            $(
                .add( $x )
            )*
            .baquoted()
        }
    };
    ( $n:expr; $a:expr ) => {
        {
            SqlName::new( $n ).alias( $a ).baquoted()
        }
    };
    ( $n:expr, $( $x:expr ),*; $a:expr ) => {
        {
            SqlName::new( $n )
            $(
                .add( $x )
            )*
            .alias( $a )
            .baquoted()
        }
    };
}

/// Make brackets quoted name of identifier.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate sql_builder;
/// # use anyhow::Result;
/// use sql_builder::{SqlBuilder, SqlName};
///
/// # fn main() -> Result<()> {
/// let sql = SqlBuilder::select_from(brname!("public", "BOOKS"; "b"))
///     .field(brname!("b", "title"))
///     .field(brname!("s", "total"))
///     .left()
///     .join(brname!("shops"; "s"))
///     .on_eq(brname!("b", "id"), brname!("s", "book"))
///     .sql()?;
///
/// assert_eq!("SELECT [b].[title], [s].[total] FROM [public].[BOOKS] AS b LEFT JOIN [shops] AS s ON [b].[id] = [s].[book];", &sql);
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! brname {
    ( $n:expr ) => {
        {
            SqlName::new( $n ).brquoted()
        }
    };
    ( $n:expr, $( $x:expr ),* ) => {
        {
            SqlName::new( $n )
            $(
                .add( $x )
            )*
            .brquoted()
        }
    };
    ( $n:expr; $a:expr ) => {
        {
            SqlName::new( $n ).alias( $a ).brquoted()
        }
    };
    ( $n:expr, $( $x:expr ),*; $a:expr ) => {
        {
            SqlName::new( $n )
            $(
                .add( $x )
            )*
            .alias( $a )
            .brquoted()
        }
    };
}

/// Make double quoted name of identifier.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate sql_builder;
/// # use anyhow::Result;
/// use sql_builder::{SqlBuilder, SqlName};
///
/// # fn main() -> Result<()> {
/// let sql = SqlBuilder::select_from(dname!("public", "BOOKS"; "b"))
///     .field(dname!("b", "title"))
///     .field(dname!("s", "total"))
///     .left()
///     .join(dname!("shops"; "s"))
///     .on_eq(dname!("b", "id"), dname!("s", "book"))
///     .sql()?;
///
/// assert_eq!("SELECT \"b\".\"title\", \"s\".\"total\" FROM \"public\".\"BOOKS\" AS b LEFT JOIN \"shops\" AS s ON \"b\".\"id\" = \"s\".\"book\";", &sql);
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! dname {
    ( $n:expr ) => {
        {
            SqlName::new( $n ).dquoted()
        }
    };
    ( $n:expr, $( $x:expr ),* ) => {
        {
            SqlName::new( $n )
            $(
                .add( $x )
            )*
            .dquoted()
        }
    };
    ( $n:expr; $a:expr ) => {
        {
            SqlName::new( $n ).alias( $a ).dquoted()
        }
    };
    ( $n:expr, $( $x:expr ),*; $a:expr ) => {
        {
            SqlName::new( $n )
            $(
                .add( $x )
            )*
            .alias( $a )
            .dquoted()
        }
    };
}

/// Create safe name of identifier
///
/// # Examples
/// ```
/// #[macro_use] extern crate sql_builder;
/// # use anyhow::Result;
/// use sql_builder::{SqlBuilder, SqlName};
///
/// # fn main() -> Result<()> {
/// let sql = SqlBuilder::select_from(SqlName::new("public").add("BOOKS").alias("b").baquoted())
///     .field(SqlName::new("b").add("title").baquoted())
///     .field(SqlName::new("s").add("total").baquoted())
///     .left()
///     .join(SqlName::new("shops").alias("s").baquoted())
///     .on_eq(SqlName::new("b").add("id").baquoted(), SqlName::new("s").add("book").baquoted())
///     .sql()?;
///
/// assert_eq!("SELECT `b`.`title`, `s`.`total` FROM `public`.`BOOKS` AS b LEFT JOIN `shops` AS s ON `b`.`id` = `s`.`book`;", &sql);
/// # Ok(())
/// # }
/// ```
///
/// ```
/// #[macro_use] extern crate sql_builder;
/// # use anyhow::Result;
/// use sql_builder::{SqlBuilder, SqlName};
///
/// # fn main() -> Result<()> {
/// let sql = SqlBuilder::select_from(baname!("public", "BOOKS"; "b"))
///     .field(baname!("b", "title"))
///     .field(baname!("s", "total"))
///     .left()
///     .join(baname!("shops"; "s"))
///     .on_eq(baname!("b", "id"), baname!("s", "book"))
///     .sql()?;
///
/// assert_eq!("SELECT `b`.`title`, `s`.`total` FROM `public`.`BOOKS` AS b LEFT JOIN `shops` AS s ON `b`.`id` = `s`.`book`;", &sql);
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct SqlName {
    parts: Vec<String>,
    alias: Option<String>,
}

impl SqlName {
    /// Name of identifier
    pub fn new<S: ToString>(name: S) -> Self {
        Self {
            parts: vec![name.to_string()],
            alias: None,
        }
    }

    /// Add additional part of identifier
    pub fn add<S: ToString>(&mut self, name: S) -> &mut Self {
        self.parts.push(name.to_string());
        self
    }

    /// Set an alias for identifier
    pub fn alias<S: ToString>(&mut self, alias: S) -> &mut Self {
        self.alias = Some(alias.to_string());
        self
    }

    /// Make safe identifier
    pub fn safe(&self) -> String {
        let safe_name = self.make_safe_parts().join(".");
        self.join_with_alias(safe_name)
    }

    /// Make quoted identifier
    pub fn quoted(&self) -> String {
        let safe_name = self
            .parts
            .iter()
            .map(quote)
            .collect::<Vec<String>>()
            .join(".");
        self.join_with_alias(safe_name)
    }

    /// Make backquoted identifier
    pub fn baquoted(&self) -> String {
        let safe_name = self
            .parts
            .iter()
            .map(baquote)
            .collect::<Vec<String>>()
            .join(".");
        self.join_with_alias(safe_name)
    }

    /// Make bracket-quoted identifier
    pub fn brquoted(&self) -> String {
        let safe_name = self
            .parts
            .iter()
            .map(brquote)
            .collect::<Vec<String>>()
            .join(".");
        self.join_with_alias(safe_name)
    }

    /// Make double quoted identifier
    pub fn dquoted(&self) -> String {
        let safe_name = self
            .parts
            .iter()
            .map(dquote)
            .collect::<Vec<String>>()
            .join(".");
        self.join_with_alias(safe_name)
    }

    /// Join safe name with safe alias
    fn join_with_alias(&self, safe_name: String) -> String {
        match &self.alias {
            Some(alias) => {
                let safe_alias = Self::make_safe_name(&alias);
                format!("{} AS {}", safe_name, safe_alias)
            }
            None => safe_name,
        }
    }

    /// Convert all parts into safe form
    fn make_safe_parts(&self) -> Vec<String> {
        if self.all_is_safe() {
            self.parts.clone()
        } else {
            self.parts.iter().map(baquote).collect()
        }
    }

    /// Convert name into safe form
    fn make_safe_name(name: &str) -> String {
        if Self::is_safe(&name) {
            name.to_string()
        } else {
            baquote(name)
        }
    }

    /// Check if name is safe for injection
    fn is_safe(name: &str) -> bool {
        name.chars()
            .all(|c| matches!(c, 'a'..='z' | '0'..='9' | '_'))
    }

    /// Check if all parts is safe for injection
    fn all_is_safe(&self) -> bool {
        self.parts.iter().all(|name| Self::is_safe(&name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_simple_name() -> Result<()> {
        let name = SqlName::new("safe_name").safe();
        assert_eq!(&name, "safe_name");

        let name = SqlName::new("safe_name").alias("sn").safe();
        assert_eq!(&name, "safe_name AS sn");

        let name = name!("safe_name");
        assert_eq!(&name, "safe_name");

        let name = name!("safe_name"; "sn");
        assert_eq!(&name, "safe_name AS sn");

        Ok(())
    }

    #[test]
    fn test_spaced_name() -> Result<()> {
        let name = SqlName::new("spaced name").safe();
        assert_eq!(&name, "`spaced name`");

        let name = SqlName::new("spaced name").alias("s n").safe();
        assert_eq!(&name, "`spaced name` AS `s n`");

        let name = name!("spaced name");
        assert_eq!(&name, "`spaced name`");

        let name = name!("spaced name"; "s n");
        assert_eq!(&name, "`spaced name` AS `s n`");

        Ok(())
    }

    #[test]
    fn test_quoted_name() -> Result<()> {
        let name = SqlName::new("some 'awesome' name").quoted();
        assert_eq!(&name, "'some ''awesome'' name'");

        let name = SqlName::new("some 'awesome' name")
            .alias("awesome name")
            .quoted();
        assert_eq!(&name, "'some ''awesome'' name' AS `awesome name`");

        let name = SqlName::new("some 'awesome' name")
            .add("sub")
            .alias("awesome name")
            .quoted();
        assert_eq!(&name, "'some ''awesome'' name'.'sub' AS `awesome name`");

        let name = qname!("some 'awesome' name");
        assert_eq!(&name, "'some ''awesome'' name'");

        let name = qname!("some 'awesome' name"; "awesome name");
        assert_eq!(&name, "'some ''awesome'' name' AS `awesome name`");

        let name = qname!("some 'awesome' name", "sub"; "awesome name");
        assert_eq!(&name, "'some ''awesome'' name'.'sub' AS `awesome name`");

        Ok(())
    }

    #[test]
    fn test_baquoted_name() -> Result<()> {
        let name = SqlName::new("safe_name").baquoted();
        assert_eq!(&name, "`safe_name`");

        let name = SqlName::new("safe_name").alias("sn").baquoted();
        assert_eq!(&name, "`safe_name` AS sn");

        let name = SqlName::new("safe_name").add("sub").alias("sn").baquoted();
        assert_eq!(&name, "`safe_name`.`sub` AS sn");

        let name = baname!("safe_name");
        assert_eq!(&name, "`safe_name`");

        let name = baname!("safe_name"; "sn");
        assert_eq!(&name, "`safe_name` AS sn");

        let name = baname!("safe_name", "sub"; "sn");
        assert_eq!(&name, "`safe_name`.`sub` AS sn");

        Ok(())
    }

    #[test]
    fn test_brquoted_name() -> Result<()> {
        let name = SqlName::new("safe_name").brquoted();
        assert_eq!(&name, "[safe_name]");

        let name = SqlName::new("safe_name").alias("sn").brquoted();
        assert_eq!(&name, "[safe_name] AS sn");

        let name = SqlName::new("safe_name").add("sub").alias("sn").brquoted();
        assert_eq!(&name, "[safe_name].[sub] AS sn");

        let name = brname!("safe_name");
        assert_eq!(&name, "[safe_name]");

        let name = brname!("safe_name"; "sn");
        assert_eq!(&name, "[safe_name] AS sn");

        let name = brname!("safe_name", "sub"; "sn");
        assert_eq!(&name, "[safe_name].[sub] AS sn");

        Ok(())
    }

    #[test]
    fn test_dquoted_name() -> Result<()> {
        let name = SqlName::new("safe_name").dquoted();
        assert_eq!(&name, "\"safe_name\"");

        let name = SqlName::new("safe_name").alias("sn").dquoted();
        assert_eq!(&name, "\"safe_name\" AS sn");

        let name = SqlName::new("safe_name").add("sub").alias("sn").dquoted();
        assert_eq!(&name, "\"safe_name\".\"sub\" AS sn");

        let name = dname!("safe_name");
        assert_eq!(&name, "\"safe_name\"");

        let name = dname!("safe_name"; "sn");
        assert_eq!(&name, "\"safe_name\" AS sn");

        let name = dname!("safe_name", "sub"; "sn");
        assert_eq!(&name, "\"safe_name\".\"sub\" AS sn");

        Ok(())
    }
}
