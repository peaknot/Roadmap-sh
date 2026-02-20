use crate::domain::errors::ValidationError;

#[derive(Clone)]
pub struct Description(String);
#[derive(Clone, Copy)]
pub struct Amount(i64);

impl TryFrom<&str> for Description {
    type Error = ValidationError;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let trimmed = input.trim().to_lowercase();

        if trimmed.is_empty() {
            return Err(ValidationError::FieldEmpty);
        }
        if trimmed.len() < 3 || trimmed.len() > 15 {
            return Err(ValidationError::InvalidLength);
        }

        if !trimmed.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(ValidationError::InvalidCharacter);
        }

        if !trimmed
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_alphanumeric())
        {
            return Err(ValidationError::InvalidStartCharacter);
        }
        Ok(Self(trimmed.to_string()))
    }
}
impl Description {
    pub fn _as_str(&self) -> &str {
        &self.0
    }
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl TryFrom<i64> for Amount {
    type Error = ValidationError;
    fn try_from(input: i64) -> Result<Self, Self::Error> {
        if input <= 0 || input > 300_000 {
            return Err(ValidationError::InvalidAmount);
        }
        Ok(Self(input))
    }
}
impl Amount {
    pub fn as_i64(&self) -> i64 {
        self.0
    }
}
