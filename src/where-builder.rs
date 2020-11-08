use crate::error::SqlBuilderError;
use std::fmt;

#[macro_export]
macro_rules! and {
    ( $f:expr, $( $l:expr ),* ) => {
        {
            let mut x = String::from("(");
            x.push_str( & $f .to_string() );
            $(
                x.push_str(") AND (");
                x.push_str( & $l .to_string() );
            )*
            x.push(')');
            x
        }
    };
}

#[macro_export]
macro_rules! or {
    ( $f:expr, $( $l:expr ),* ) => {
        {
            let mut x = String::from( $f );
            $(
                x.push_str(" OR ");
                x.push_str( & $l .to_string() );
            )*
            x
        }
    };
}

#[macro_export]
macro_rules! not {
    ( $f:expr ) => {{
        let mut x = String::from("NOT ");
        x.push_str(&$f.to_string());
        x
    }};
}

#[macro_export]
macro_rules! brackets {
    ( $el:expr ) => {
        {
            let mut x = String::from("(");
            x.push_str( & $el .to_string() );
            x.push(')');
            x
        }
    };
    ( $first:expr, $( $el:expr ),* ) => {
        {
            let mut x = String::from("(");
            x.push_str( & $first .to_string() );
            $(
                x.push_str(", ");
                x.push_str( & $el .to_string() );
            )*
            x.push(')');
            x
        }
    };
}

/// Build WHERE for SQL.
#[derive(Clone, Default)]
pub struct Where {
    text: String,
    prefix: Option<String>,
    error: Option<SqlBuilderError>,
    was_and: bool,
}

impl fmt::Display for Where {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Where {
    pub fn new<S>(smth: S) -> Self
    where
        S: ToString,
    {
        // Checks
        let text = smth.to_string();
        if text.is_empty() {
            return Self {
                error: Some(SqlBuilderError::NoWhereField),
                ..Self::default()
            };
        }

        // Create
        Self {
            text,
            ..Self::default()
        }
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn in_brackets(&mut self) -> &mut Self {
        // Checks
        if self.text.is_empty() {
            self.error = Some(SqlBuilderError::NoWhereField);
            return self;
        }

        // Change
        self.text.insert(0, '(');
        self.text.push(')');
        self
    }

    pub fn not(&mut self) -> &mut Self {
        // Checks
        if self.text.is_empty() {
            self.error = Some(SqlBuilderError::NoWhereField);
            return self;
        }

        // Change
        self.text.insert_str(0, "NOT ");
        self
    }

    pub fn and<S>(&mut self, smth: S) -> &mut Self
    where
        S: ToString,
    {
        // Checks
        if self.text.is_empty() {
            self.error = Some(SqlBuilderError::NoWhereField);
            return self;
        }
        let smth = smth.to_string();
        if smth.is_empty() {
            self.error = Some(SqlBuilderError::NoWhereValue(self.text.clone()));
            return self;
        }

        // Change
        if !self.was_and {
            self.text.insert(0, '(');
            self.text.push(')');
            self.was_and = true;
        }
        self.text.push_str(" AND (");
        self.text.push_str(&smth);
        self.text.push(')');
        self
    }

    pub fn or<S>(&mut self, smth: S) -> &mut Self
    where
        S: ToString,
    {
        // Checks
        if self.text.is_empty() {
            self.error = Some(SqlBuilderError::NoWhereField);
            return self;
        }
        let smth = smth.to_string();
        if smth.is_empty() {
            self.error = Some(SqlBuilderError::NoWhereValue(self.text.clone()));
            return self;
        }

        // Change
        self.text.push_str(" OR ");
        self.text.push_str(&smth);
        self
    }

    pub fn eq<S>(&mut self, smth: S) -> &mut Self
    where
        S: ToString,
    {
        // Checks
        let smth = smth.to_string();
        if smth.is_empty() {
            self.error = Some(SqlBuilderError::NoWhereValue(self.text.clone()));
            return self;
        }

        // Change
        if let Some(prefix) = &self.prefix {
            self.text.push(' ');
            self.text.push_str(&prefix);
            self.prefix = None;
        }
        self.text.push_str(" = ");
        self.text.push_str(&smth);
        self
    }

    pub fn ne<S>(&mut self, smth: S) -> &mut Self
    where
        S: ToString,
    {
        // Checks
        let smth = smth.to_string();
        if smth.is_empty() {
            self.error = Some(SqlBuilderError::NoWhereValue(self.text.clone()));
            return self;
        }

        // Change
        if let Some(prefix) = &self.prefix {
            self.text.push(' ');
            self.text.push_str(&prefix);
            self.prefix = None;
        }
        self.text.push_str(" <> ");
        self.text.push_str(&smth);
        self
    }

    pub fn build(&self) -> Result<String, SqlBuilderError> {
        match &self.error {
            Some(err) => Err(err.clone()),
            None => Ok(self.text.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_and() {
        let sql = and!("10", "20", "30");
        assert_eq!("(10) AND (20) AND (30)", sql);
    }

    #[test]
    fn test_macro_or() {
        let sql = or!("10", "20", "30");
        assert_eq!("10 OR 20 OR 30", sql);
    }

    #[test]
    fn test_macro_not() {
        let sql = not!("10");
        assert_eq!("NOT 10", sql);
    }

    #[test]
    fn test_macro_brackets() {
        let sql = brackets!("10", "20", "30");
        assert_eq!("(10, 20, 30)", sql);

        let sql = brackets!("10");
        assert_eq!("(10)", sql);
    }

    #[test]
    fn test_macro_and_or_not() {
        let sql = and!("10", or!("20", not!("30"), "40"));
        assert_eq!("(10) AND (20 OR NOT 30 OR 40)", &sql);
    }

    #[test]
    fn test_new_where() {
        let text = Where::new("abc").to_string();
        assert_eq!("abc", &text);
    }

    #[test]
    fn test_where_brackets() {
        let text = Where::new("abc").eq(10).in_brackets().to_string();
        assert_eq!("(abc = 10)", &text);
    }

    #[test]
    fn test_where_build() {
        let res = Where::new("abc").eq(10).build();
        assert_eq!(Ok("abc = 10".to_string()), res);
    }

    #[test]
    fn test_where_not() {
        let text = Where::new("abc").eq(10).in_brackets().not().to_string();
        assert_eq!("NOT (abc = 10)", &text);
    }

    #[test]
    fn test_where_and() {
        let text = Where::new("abc").eq(10).and(20).to_string();
        assert_eq!("(abc = 10) AND (20)", &text);
    }

    #[test]
    fn test_where_or() {
        let text = Where::new("abc").eq(10).or(20).to_string();
        assert_eq!("abc = 10 OR 20", &text);
    }

    #[test]
    fn test_where_eq() {
        let text = Where::new("abc").eq(10).to_string();
        assert_eq!("abc = 10", &text);
    }

    #[test]
    fn test_where_ne() {
        let text = Where::new("abc").ne(10).to_string();
        assert_eq!("abc <> 10", &text);
    }
}
