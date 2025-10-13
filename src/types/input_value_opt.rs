use std::{fmt::Display, marker::PhantomData, str::FromStr};

use rust_common::validators::ValueValidationResult;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct InputValueOpt<T: PartialEq + PartialOrd + Display + FromStr + 'static> {
    init_value: String,
    value: String,
    p: PhantomData<T>,
    min_value: Option<T>,
    max_value: Option<T>,
}

impl<T: PartialEq + PartialOrd + Display + FromStr + 'static> InputValueOpt<T> {
    pub fn new(src: Option<T>) -> Self {
        let Some(src) = src else {
            let value = String::new();
            return Self {
                init_value: value.clone(),
                value,
                p: PhantomData,
                min_value: None,
                max_value: None,
            };
        };

        let src = format!("{}", src);
        Self {
            init_value: src.clone(),
            value: src,
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

    /*

       pub fn set_min_value(mut self, min_value: T) -> Self {
           self.min_value = Some(min_value);
           self
       }
       pub fn set_max_value(mut self, max_value: T) -> Self {
           self.max_value = Some(max_value);
           self
       }
    */

    pub fn set_min_value_mut(&mut self, min_value: T) {
        self.min_value = Some(min_value);
    }
    pub fn set_max_value_mut(&mut self, max_value: T) {
        self.max_value = Some(max_value);
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    pub fn value_can_be_saved(&self) -> Option<bool> {
        if self.init_value == self.value {
            return None;
        }
        let result = match self.validate() {
            Ok(_) => true,
            Err(err) => match err {
                ValueValidationResult::Empty => true,
                _ => false,
            },
        };

        Some(result)
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

impl<T: PartialEq + PartialOrd + Display + FromStr + 'static> From<Option<T>> for InputValueOpt<T> {
    fn from(value: Option<T>) -> Self {
        Self::new(value)
    }
}

impl<T: PartialEq + PartialOrd + Display + FromStr> rust_extensions::AsStr for InputValueOpt<T> {
    fn as_str(&self) -> &str {
        &self.value
    }
}

impl<T: PartialEq + PartialOrd + Display + FromStr> rust_common::validators::ValueValidator
    for InputValueOpt<T>
{
    fn validate_value(&self) -> Result<(), ValueValidationResult> {
        self.validate()
    }
}
