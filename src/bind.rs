use crate::arg::SqlArg;

pub trait Bind {
    fn bind(&self, arg: &dyn SqlArg) -> String;
    fn binds(&self, args: &[&dyn SqlArg]) -> String;
}

impl Bind for &str {
    fn bind(&self, arg: &dyn SqlArg) -> String {
        (*self).to_string().bind(arg)
    }

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
    use std::error::Error;

    #[test]
    fn test_bind() -> Result<(), Box<dyn Error>> {
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

        Ok(())
    }

    #[test]
    fn test_binds() -> Result<(), Box<dyn Error>> {
        assert_eq!("10f20o30o10", &"?f?o?o?".binds(&[&10, &20, &30]));
        assert_eq!(
            "'abc'f'def'o'ghi'o'abc'",
            &"?f?o?o?".binds(&[&"abc", &"def", &"ghi"])
        );
        assert_eq!(
            "10f20o30o10",
            &String::from("?f?o?o?").binds(&[&10, &20, &30])
        );

        Ok(())
    }
}
