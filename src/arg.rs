use crate::quote;

pub trait SqlArg {
    fn sql_arg(&self) -> String;
}

impl SqlArg for str {
    fn sql_arg(&self) -> String {
        quote(self)
    }
}

impl SqlArg for &str {
    fn sql_arg(&self) -> String {
        quote(self)
    }
}

impl SqlArg for String {
    fn sql_arg(&self) -> String {
        quote(self)
    }
}

impl SqlArg for i8 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for &i8 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for u8 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for &u8 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for i16 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for &i16 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for u16 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for &u16 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for i32 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for &i32 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for u32 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for &u32 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for i64 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for &i64 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for u64 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for &u64 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for i128 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for &i128 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for u128 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for &u128 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for isize {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for &isize {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for usize {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for &usize {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for bool {
    fn sql_arg(&self) -> String {
        String::from(match *self {
            true => "TRUE",
            false => "FALSE",
        })
    }
}

impl SqlArg for &bool {
    fn sql_arg(&self) -> String {
        String::from(match *self {
            true => "TRUE",
            false => "FALSE",
        })
    }
}
