use std::f32::consts::E;

use sqlx::{Pool, Postgres, error::DatabaseError};
use tracing::error;

use crate::{error::{Result, self}, models::user::{User, UserForUpdate, UserForCreate, UserForDatabase}, Error};

#[derive(Clone)]
pub struct UserService {
    db: Pool<Postgres>
}

impl UserService {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self {
            db
        }
    }
}

impl UserService {
    pub async fn list_users(&self) -> Result<Vec<User>> {
        let users = sqlx::query_as!(UserForDatabase, "SELECT * FROM users")
            .fetch_all(&self.db)
            .await.map_err(|err| {
                error!("list_users: {err}");
                Error::DatabaseError
            })?;

        let users = users
            .into_iter()
            .map(User::from_db)
            .collect::<Result<Vec<_>>>()?;

        Ok(users)
    }

    pub async fn get_user_by_name(&self, name: impl Into<String>) -> Result<Option<User>> {
        let name: String = name.into();

        let user = sqlx::query_as!(
            UserForDatabase,
            "SELECT * FROM users WHERE name=$1;",
            name
        ).fetch_optional(&self.db)
        .await.map_err(|err| {
            error!("get_user_by_name: {err}");
            Error::DatabaseError 
        })?;

        match user {
            Some(user) => Ok(Some(User::from_db(user)?)),
            None => Ok(None)
        }
    }

    pub async fn get_user(&self, id: i64) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            UserForDatabase,
            "SELECT * FROM users WHERE id=$1",
            id
        ).fetch_optional(&self.db)
        .await.map_err(|err| {
            error!("get_user: {err}");
            Error::DatabaseError
        })?;

        Ok(match user {
            Some(user) => Some(User::from_db(user)?),
            None => None
        })
    }

    pub async fn delete_user(&self, user_id: i64) -> Result<()> {
        sqlx::query!("DELETE FROM users WHERE id=$1", user_id)
            .execute(&self.db).await.map_err(|err| {
                error!("delete_user: {err}");
                Error::DatabaseError
            })?;

        Ok(())
    }

    pub async fn update_user(&self, user_id: i64, user: UserForUpdate) -> Result<User> {
        let existing_user = match self.get_user(user_id).await? {
            Some(user) => user,
            None => {
                error!("update_user: Trying to update non-existent user with id {user_id}.");
                return Err(Error::DatabaseError);
            }
        };

        let name = match user.name {
            Some(name) => name,
            None => existing_user.name
        };

        let code = match user.code {
            Some(code) => code,
            None => existing_user.code
        };

        let schedule = match user.schedule {
            Some(schedule) => Some(schedule),
            None => existing_user.schedule
        };

        let schedule = serde_json::to_value(schedule)
            .map_err(|_| Error::SerializationFail)?;

        let existing_user = sqlx::query_as!(
            UserForDatabase,
            "UPDATE users SET name=$1,code=$2,schedule=$3 WHERE id=$4 RETURNING *;",
            name, code, schedule, user_id
        ).fetch_one(&self.db)
        .await.map_err(|err| {
            error!("update_user: {err}");
            Error::DatabaseError
        })?;

        User::from_db(existing_user)
    }

    pub async fn create_user(&self, user: UserForCreate) -> Result<User> {
        let schedule = serde_json::to_value(user.schedule)
            .map_err(|_| Error::SerializationFail)?;

        let code = "";

        let user = sqlx::query_as!(
            UserForDatabase, 
            "INSERT INTO users (name, code, schedule) VALUES ($1, $2, $3) RETURNING *",
            user.name, code, schedule
        ).fetch_one(&self.db)
        .await.map_err(|err| {
            error!("create_user: {err}");
            Error::DatabaseError
        })?;

        User::from_db(user)
    }
}