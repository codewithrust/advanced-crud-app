use crate::models::user_model::User as UserModel;
use async_std::task;
use sqlx::{Pool, Postgres};

use super::repository_trait::RepositoryTrait;
use sqlx::Row;

// Repository is a struct that implements the RepositoryTrait trait
// it accepts lifetime 'a and has a field db that is a reference to a Pool<Postgres>
pub struct Repository<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> Repository<'a> {
    pub fn new(db: &'a Pool<Postgres>) -> Self {
        Repository { db }
    }
}

impl<'a> RepositoryTrait<'a, UserModel> for Repository<'a> {
    fn create(&self, item: UserModel) -> Result<UserModel, String> {
        // block_on is a function that blocks the current thread until the provided future has resolved
        let result = task::block_on(async {
            sqlx::query("INSERT INTO users (id, username, email) VALUES ($1, $2, $3) RETURNING *")
                .bind(&item.id)
                .bind(&item.username)
                .bind(&item.email)
                .fetch_one(self.db)
                .await
        });

        if result.is_err() {
            return Err(result.err().unwrap().to_string());
        }

        // unwrap is a function that returns the value of a Result or panics
        let row = result.unwrap();

        // get is a function that returns a value from a Row
        let id = row.get::<String, _>("id");
        let name = row.get::<String, _>("username");
        let email = row.get::<String, _>("email");

        // Ok is an enum variant that represents success
        // it is used to return a successful result
        Ok(UserModel {
            id,
            email,
            username: name,
        })
    }

    fn get_by_id(&self, id: String) -> Result<UserModel, String> {
        let result = task::block_on(async {
            sqlx::query("SELECT * FROM users WHERE id = $1")
                .bind(id)
                .fetch_one(self.db)
                .await
        });
        let row = result.unwrap();

        let id = row.get::<String, _>("id");
        let name = row.get::<String, _>("username");
        let email = row.get::<String, _>("email");

        Ok(UserModel {
            id,
            email,
            username: name,
        })
    }

    fn get_all(&self) -> Result<Vec<UserModel>, String> {
        let result =
            task::block_on(async { sqlx::query("SELECT * FROM users").fetch_all(self.db).await });

        if result.is_err() {
            return Err(result.err().unwrap().to_string());
        }

        let rows = result.unwrap();
        let mut users = Vec::new();

        for row in rows {
            let id = row.get::<String, _>("id");
            let name = row.get::<String, _>("username");
            let email = row.get::<String, _>("email");

            users.push(UserModel {
                id,
                email,
                username: name,
            });
        }

        Ok(users)
    }

    fn update(&self, item: UserModel) -> Result<(), String> {
        let result = task::block_on(async {
            sqlx::query("UPDATE users SET username = $1, email = $2 WHERE id = $3")
                .bind(&item.username)
                .bind(&item.email)
                .bind(&item.id)
                .execute(self.db)
                .await
        });

        if result.is_err() {
            return Err(result.err().unwrap().to_string());
        } else {
            Ok(())
        }
    }

    fn delete(&self, id: String) -> Result<(), String> {
        let result = task::block_on(async {
            sqlx::query("DELETE FROM users WHERE id = $1")
                .bind(id)
                .execute(self.db)
                .await
        });

        if result.is_err() {
            return Err(result.err().unwrap().to_string());
        } else {
            Ok(())
        }
    }
}
