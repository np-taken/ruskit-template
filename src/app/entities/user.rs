use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use rustavel_derive::GenerateValidationFields;
use crate::framework::database::model::{Field, ModelValidation};
use validator::ValidationError;

#[derive(Debug, Serialize, Deserialize, FromRow, GenerateValidationFields)]
pub struct User {
    #[sqlx(default)]
    pub id: i64,
    pub name: String,
    pub email: String,
    pub created_at: i64,
    pub updated_at: i64,
} 