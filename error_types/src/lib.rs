pub use chrono::{Utc, NaiveDate};

// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (String, String),
    pub date: String,
    pub err: String
}

// "No user name"
// "At least 8 characters"
// "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)"

impl FormError {
    pub fn new(form_values: (String, String), date: String, err: String) -> FormError {
        FormError{form_values: form_values, date, err}
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub first_name: String,
    pub last_name: String,
    pub birth: NaiveDate, //that will convert a string "2015-09-05" to a date of that format.
    pub birth_location: String,
    pub password: String,
}

impl Form {
    pub fn new(
        first_name: String,
        last_name: String,
        birth: NaiveDate,
        birth_location: String,
        password: String,
    ) -> Form {
        Form{first_name, last_name, birth, birth_location, password}    
    }
    
    pub fn validate(&self) -> Result<Vec<&str>, FormError> {
        if self.first_name.is_empty() {
            return Err(FormError::new(("first_name".to_string(), "".to_string()), Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(), "No user name".to_string()));
        } else {
            if self.password.len() < 8 {
                return Err(FormError::new(("password".to_string(), self.password.to_string()), Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(), "At least 8 characters".to_string()));
            } else {
                let mut has_alpha = false;
                let mut has_digit = false;
                let mut has_non_alphanumeric = false;

                for c in self.password.chars() {
                    if c.is_alphabetic() {
                        has_alpha = true;
                    } else if c.is_digit(10) {
                        has_digit = true;
                    } else if !c.is_alphanumeric() {
                        has_non_alphanumeric = true;
                    }
                    if has_alpha && has_digit && has_non_alphanumeric {
                        return Ok(["Valid first name", "Valid password"].to_vec());
                    }
                }
                Err(FormError::new(("password".to_string(), self.password.to_string()), Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(), "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)".to_string()))
            }
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestForm<'a> {
        form: Form,
        validation: Result<Vec<&'a str>, FormError>,
    }

    #[allow(dead_code)]
    fn create_date(date: &str) -> NaiveDate {
        NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap()
    }

    impl<'a> TestForm<'_> {
        // all test cases
        fn new() -> Vec<TestForm<'a>> {
            vec![
                TestForm {
                    form : Form::new(
                    String::from("Katy"),
                    String::from("Silva"),
                    create_date("2015-09-05"),
                    String::from("Africa"),
                    String::from("qwTw12&%$3sa1dty_")),
                    validation: Ok(vec!["Valid first name", "Valid password"]),
                },
                TestForm {
                    form : Form::new(
                    String::from(""),
                    String::from("Bear"),
                    create_date("2015-09-05"),
                    String::from("Africa"),
                    String::from("qwTw12&%$3sa1dty_")),
                    validation: Err(FormError {
                        form_values: (String::from("first_name"),
                        String::from("")),
                        date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        err: String::from("No user name")}),
                },
                TestForm {
                    form : Form::new(
                    String::from("Someone"),
                    String::from("Bear"),
                    create_date("2015-09-05"),
                    String::from("Africa"),
                    String::from("12345")),
                    validation: Err(FormError {
                        form_values: (String::from("password"), String::from("12345")),
                        date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        err: String::from("At least 8 characters") }),
                },
                TestForm {
                    form : Form::new(
                    String::from("Someone"),
                    String::from("Bear"),
                    create_date("2015-09-05"),
                    String::from("Africa"),
                    String::from("sdASDsrW")),
                    validation: Err(FormError {
                        form_values: (String::from("password"), String::from("sdASDsrW")),
                        date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        err: String::from("Combination of different ASCII character types (numbers, letters and none alphanumeric characters)") }),
                },
                TestForm {
                    form : Form::new(
                    String::from("Someone"),
                    String::from("Bear"),
                    create_date("2015-09-05"),
                    String::from("Africa"),
                    String::from("dsGE1SAD213")),
                    validation: Err(FormError {
                        form_values: (String::from("password"), String::from("dsGE1SAD213")),
                        date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        err: String::from("Combination of different ASCII character types (numbers, letters and none alphanumeric characters)") }),
                },
                TestForm {
                    form : Form::new(
                    String::from("Someone"),
                    String::from("Bear"),
                    create_date("2015-09-05"),
                    String::from("Africa"),
                    String::from("dsaSD&%DF!?=")),
                    validation: Err(FormError {
                        form_values: (String::from("password"), String::from("dsaSD&%DF!?=")),
                        date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        err: String::from("Combination of different ASCII character types (numbers, letters and none alphanumeric characters)") }),
                }
            ]
        }
    }

    #[test]
    fn test_error_type() {
        let form_cases = TestForm::new();

        for v in form_cases {
            assert_eq!(
                v.form.validate(),
                v.validation,
                "Tested with {:?}",
                v.validation.as_ref().err().unwrap().form_values
            );
        }
    }
}
