use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

// User struct derived from Serialize and Deserialize to allow for JSON serialization and deserialization
#[derive(Serialize, Deserialize, Clone, FromRow, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
}
