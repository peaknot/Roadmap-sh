use crate::domain::errors::ValidationError;

#[derive(Clone)]
pub enum Category {
    Food,
    Fare,
    Groceries,
    Leisure,
    Electronics,
    Utilities,
    Clothing,
    Health,
}
impl TryFrom<String> for Category {
    type Error = ValidationError;
    fn try_from(input: String) -> Result<Self, Self::Error> {
        let normalized = input.trim().to_ascii_lowercase();

        if normalized.is_empty() {
            return Err(ValidationError::InvalidCategory);
        }
        if normalized.len() < 3 {
            return Err(ValidationError::InvalidCategory);
        }
        if !normalized.chars().all(|f| f.is_ascii_alphabetic()) {
            return Err(ValidationError::InvalidCategory);
        }
        match normalized.as_str() {
            "food" => Ok(Category::Food),
            "fare" => Ok(Category::Fare),
            "groceries" => Ok(Category::Groceries),
            "leisure" => Ok(Category::Leisure),
            "electronics" => Ok(Category::Electronics),
            "utilities" => Ok(Category::Utilities),
            "clothing" => Ok(Category::Clothing),
            "health" => Ok(Category::Health),
            _ => Err(ValidationError::InvalidCategory),
        }
    }
}

impl Category {
    pub fn _as_str(&self) -> &'static str {
        //TODO make case insensitive
        match self {
            Category::Food => "Food",
            Category::Fare => "Fare",
            Category::Groceries => "Groceries",
            Category::Leisure => "Leisure",
            Category::Electronics => "Electronics",
            Category::Utilities => "Utilities",
            Category::Clothing => "Clothing",
            Category::Health => "Health",
        }
    }

    pub fn into_inner(&self) -> String {
        match self {
            Category::Food => String::from("Food"),
            Category::Fare => String::from("Fare"),
            Category::Groceries => String::from("Groceries"),
            Category::Leisure => String::from("Leisure"),
            Category::Electronics => String::from("Electronics"),
            Category::Utilities => String::from("Utilities"),
            Category::Clothing => String::from("Clothing"),
            Category::Health => String::from("Health"),
        }
    }
}
