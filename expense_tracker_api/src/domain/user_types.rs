use crate::domain::errors::ValidationError;

#[derive(Clone, PartialEq)]
pub struct UserName(String);

#[derive(Clone, PartialEq)]
pub struct Email(String);

#[derive(Clone, PartialEq)]
pub struct Password(String);

impl TryFrom<String> for UserName {
    type Error = ValidationError;
    fn try_from(input: String) -> Result<Self, Self::Error> {
        let trimmed = input.trim().to_ascii_lowercase();

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
        Ok(Self(trimmed))
    }
}

impl UserName {
    pub fn _as_str(&self) -> &str {
        &self.0
    }
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl TryFrom<String> for Email {
    type Error = ValidationError;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        let trimmed = input.trim().to_ascii_lowercase();

        if trimmed.is_empty() {
            return Err(ValidationError::FieldEmpty);
        }

        // Must contain exactly one '@'
        let mut parts = trimmed.split('@');
        let local = parts.next();
        let domain = parts.next();

        if parts.next().is_some() {
            return Err(ValidationError::InvalidCharacter); // more than one '@'
        }

        let (local, domain) = match (local, domain) {
            (Some(l), Some(d)) => (l, d),
            _ => return Err(ValidationError::InvalidFormat),
        };

        if local.is_empty() || domain.is_empty() {
            return Err(ValidationError::FieldEmpty);
        }

        if !domain.contains('.') {
            return Err(ValidationError::InvalidFormat);
        }

        if trimmed.chars().any(|c| c.is_whitespace()) {
            return Err(ValidationError::InvalidStartCharacter);
        }

        Ok(Self(trimmed))
    }
}
impl Email {
    pub fn _as_str(&self) -> &str {
        &self.0
    }
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl TryFrom<String> for Password {
    type Error = ValidationError;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        let trimmed = input.trim().to_ascii_lowercase();

        if trimmed.len() < 6 || trimmed.len() > 100 {
            return Err(ValidationError::InvalidLength);
        }

        Ok(Self(trimmed))
    }
}

impl Password {
    pub fn new(hash: String) -> Self {
        Password(hash)
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn into_inner(self) -> String {
        self.0
    }
}
