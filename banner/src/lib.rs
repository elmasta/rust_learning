use std::collections::HashMap;
use std::num::ParseFloatError;

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(l_h: &str, d: &str) -> Flag {
        Flag{short_hand: "".to_string(), long_hand: l_h.to_string(), desc: d.to_string()}
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub fn div(nmb_a: &str, nmb_b: &str) -> Result<String, ParseFloatError> {
    let pnmb_a: f32 = nmb_a.parse()?;
    let pnmb_b: f32 = nmb_b.parse()?;
    // if pnmb_b == 0.0 {
    //     return Err(ParseFloatError::from("Division by zero"));
    // }
    let result = pnmb_a / pnmb_b;
    Ok(result.to_string())
}

pub fn rem(nmb_a: &str, nmb_b: &str) -> Result<String, ParseFloatError> {
    let pnmb_a: f32 = nmb_a.parse()?;
    let pnmb_b: f32 = nmb_b.parse()?;
    // if pnmb_b == 0.0 {
    //     return Err(ParseFloatError::from("Division by zero"));
    // }
    let result = pnmb_a % pnmb_b;
    Ok(result.to_string())
}

pub struct FlagsHandler {
    pub flags: HashMap<(String, String), Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: (String, String), func: Callback) {
        self.flags.insert(flag, func);
    }
    pub fn exec_func(&mut self, flag: (String, String), argv: &[&str]) -> String {
        if flag == ("-d".to_string(), "--division".to_string()) {
            let result = div(argv[0], argv[1]);
            match result {
                Ok(num) => return num,
                Err(err) => return err.to_string(),
            }
        } else if flag == ("-r".to_string(), "--remainder".to_string()) {
            let result = rem(argv[0], argv[1]);
            match result {
                Ok(num) => return num,
                Err(err) => return err.to_string(),
            }
        } else {
            return "Division by zero".to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() -> FlagsHandler {
        let d = Flag::opt_flag("division", "divides two numbers");
        let r = Flag::opt_flag(
            "remainder",
            "gives the remainder of the division between two numbers",
        );
        let mut handler = FlagsHandler {
            flags: HashMap::new(),
        };

        handler.add_flag((d.short_hand, d.long_hand), div);
        handler.add_flag((r.short_hand, r.long_hand), rem);
        return handler;
    }

    #[test]
    fn ok_test() {
        let mut handler = init();
        assert_eq!(
            handler.exec_func(
                ("-d".to_string(), "--division".to_string()),
                &["1.0", "2.0"]
            ),
            "0.5"
        );
        assert_eq!(
            handler.exec_func(
                ("-r".to_string(), "--remainder".to_string()),
                &["2.0", "2.0"]
            ),
            "0"
        );
        assert_eq!(
            handler.exec_func(
                ("-d".to_string(), "--division".to_string()),
                &["12.323", "212.32"]
            ),
            "0.05803975"
        );
        assert_eq!(
            handler.exec_func(
                ("-r".to_string(), "--remainder".to_string()),
                &["12.323", "212.32"]
            ),
            "12.323"
        );
    }

    #[test]
    fn error_test() {
        let mut handler = init();
        assert_eq!(
            handler.exec_func(("-d".to_string(), "--division".to_string()), &["a", "2.0"]),
            "invalid float literal"
        );
        assert_eq!(
            handler.exec_func(("-r".to_string(), "--remainder".to_string()), &["2.0", "f"]),
            "invalid float literal"
        );
        assert_eq!(
            handler.exec_func(
                ("-d".to_string(), "--division".to_string()),
                &["1.0", "0.0"]
            ),
            "inf"
        );
        assert_eq!(
            handler.exec_func(
                ("-r".to_string(), "--remainder".to_string()),
                &["2.0", "0.0"]
            ),
            "NaN"
        );
    }
}
