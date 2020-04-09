use crate::arg::SqlArg;

pub trait Bind {
    fn bind(&self, arg: &dyn SqlArg) -> String;
    fn binds(&self, args: &[&dyn SqlArg]) -> String;
}

impl Bind for &str {
    fn bind(&self, arg: &dyn SqlArg) -> String {
        self.replace('?', &arg.sql_arg())
    }

    fn binds(&self, args: &[&dyn SqlArg]) -> String {
        (*self).to_string().binds(args)
    }
}

impl Bind for String {
    fn bind(&self, arg: &dyn SqlArg) -> String {
        self.replace('?', &arg.sql_arg())
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
