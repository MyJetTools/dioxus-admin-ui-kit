use std::{fmt::Display, marker::PhantomData, rc::Rc, str::FromStr};

use rust_common::validators::{ValueValidationResult, ValueValidator};
use rust_extensions::AsStr;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct InputValue<T: PartialEq + PartialOrd + Display + FromStr + 'static> {
    init_value: Rc<String>,
    value: Rc<String>,
    p: PhantomData<T>,
    min_value: Option<T>,
    max_value: Option<T>,
}

impl<T: PartialEq + PartialOrd + Display + FromStr + 'static> InputValue<T> {
    pub fn new(src: T) -> Self {
        let src = format!("{}", src);
        let src = Rc::new(src);
        Self {
            init_value: src.clone(),
            value: src,
            p: PhantomData,
            min_value: None,
            max_value: None,
        }
    }

    pub fn from_str(src: &str) -> Self {
        let result: Result<T, _> = src.parse();

        let value = match result {
            Ok(_) => src.to_string(),
            Err(_) => String::new(),
        };
        let value = Rc::new(value);

        Self {
            init_value: value.clone(),
            value,
            p: PhantomData,
            min_value: None,
            max_value: None,
        }
    }

    pub fn get_value(&self) -> Option<T> {
        let result: Result<T, _> = self.value.parse();

        match result {
            Ok(result) => Some(result),
            Err(_) => None,
        }
    }

    pub fn set_min_value_mut(&mut self, min_value: T) {
        self.min_value = Some(min_value);
    }
    pub fn set_max_value_mut(&mut self, max_value: T) {
        self.max_value = Some(max_value);
    }

    pub fn set_string_value(&mut self, value: String) {
        self.value = Rc::new(value);
    }

    pub fn validate(&self) -> Result<(), ValueValidationResult> {
        if self.value.len() == 0 {
            return Err(ValueValidationResult::Empty);
        }

        let result: Result<T, _> = self.value.parse();

        let result = match result {
            Ok(result) => result,
            Err(_) => {
                return Err(ValueValidationResult::IllegalChars);
            }
        };

        if let Some(min_value) = &self.min_value {
            if &result < min_value {
                return Err(ValueValidationResult::MinValueViolation);
            }
        }

        if let Some(max_value) = &self.max_value {
            if &result > max_value {
                return Err(ValueValidationResult::MaxValueViolation);
            }
        }

        Ok(())
    }
}

impl<T: PartialEq + PartialOrd + Display + FromStr + 'static> From<T> for InputValue<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: PartialEq + PartialOrd + Display + FromStr + 'static> AsStr for InputValue<T> {
    fn as_str(&self) -> &str {
        &self.value
    }
}

impl<T: PartialEq + PartialOrd + Display + FromStr + 'static> ValueValidator for InputValue<T> {
    fn validate_value(&self) -> Result<(), ValueValidationResult> {
        self.validate_value()
    }
}
