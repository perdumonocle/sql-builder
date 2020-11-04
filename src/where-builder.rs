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

/// Build WHERE for SQL.
#[derive(Clone, Default)]
struct Where {
    text: String,
    prefix: Option<String>,
    error: Option<SqlBuilderError>,
}

impl fmt::Display for Where {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Where {
    fn new<S>(smth: S) -> Self
    where
        S: ToString,
    {
        Self {
            text: smth.to_string(),
            ..Self::default()
        }
    }

    fn eq<S>(&mut self, smth: S) -> &mut Self
    where
        S: ToString,
    {
        if let Some(prefix) = &self.prefix {
            self.text.push(' ');
            self.text.push_str(&prefix);
            self.prefix = None;
        }
        self.text.push_str(" = ");
        self.text.push_str(&smth.to_string());
        self
    }

    fn build(&self) -> Result<String, SqlBuilderError> {
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
    fn test_where_eq() {
        let text = Where::new("abc").eq(10).to_string();
        assert_eq!("abc = 10", &text);
    }

    #[test]
    fn test_where_build() {
        let res = Where::new("abc").eq(10).build();
        assert_eq!(Ok("abc = 10".to_string()), res);
    }
}
