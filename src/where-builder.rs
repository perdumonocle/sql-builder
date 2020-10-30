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
}

// /// Build WHERE for SQL.
// struct WhereBulder {
// }
