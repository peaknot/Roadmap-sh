use crate::api::dto::api_errors::ApiError;
use crate::domain::{
    NewUser,
    user_types::{Email, Password, UserName},
};
use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserRequestDTO {
    pub username: String,
    pub email: String,
    pub password: String,
}
impl TryFrom<CreateUserRequestDTO> for NewUser {
    type Error = ApiError;
    fn try_from(input: CreateUserRequestDTO) -> Result<Self, Self::Error> {
        let valid_username = UserName::try_from(input.username)?;
        let valid_email = Email::try_from(input.email)?;
        let valid_password = Password::try_from(input.password)?;

        let hashed_password = password_hasher(valid_password)?;

        Ok(Self {
            username: valid_username,
            email: valid_email,
            password_hash: hashed_password,
        })
    }
}

fn password_hasher(input: Password) -> Result<Password, ApiError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(input.as_str().as_bytes(), &salt)
        .map_err(|_| ApiError::Internal)?
        .to_string();

    Ok(Password::new(password_hash))
}
