#[derive(Debug, Clone, Copy)]
pub enum ValueValidationResult {
    Empty,
    IllegalChars,
    MinValueViolation,
    MaxValueViolation,
}

pub trait ValueValidator {
    fn validate_value(&self) -> Result<(), ValueValidationResult>;
}

impl ValueValidator for String {
    fn validate_value(&self) -> Result<(), ValueValidationResult> {
        if self.len() == 0 {
            return Err(ValueValidationResult::Empty);
        }

        Ok(())
    }
}

impl<'s> ValueValidator for &'s str {
    fn validate_value(&self) -> Result<(), ValueValidationResult> {
        if self.len() == 0 {
            return Err(ValueValidationResult::Empty);
        }

        Ok(())
    }
}

impl<'s> ValueValidator for &'s String {
    fn validate_value(&self) -> Result<(), ValueValidationResult> {
        if self.len() == 0 {
            return Err(ValueValidationResult::Empty);
        }

        Ok(())
    }
}
