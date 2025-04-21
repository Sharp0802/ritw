use crate::lazy::{lazy_async, statement};
use crate::models::{AppError, Model};
use crate::services::DB;
use async_trait::async_trait;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_512};
use std::error::Error;
use tokio_postgres::types::Type;
use tokio_postgres::{Row, Statement};

static UP_STMT: Lazy<Statement> = statement! {
    r#"CREATE TABLE IF NOT EXISTS users (
        id       VARCHAR PRIMARY KEY,
        name     VARCHAR NOT NULL,
        password BYTEA   NOT NULL
    );"#, &[]
};

static CREATE_STMT: Lazy<Statement> = statement! {
    "INSERT INTO users (id, name, password) VALUES ($1, $2, $3)",
    &[Type::VARCHAR, Type::VARCHAR, Type::BYTEA]
};

static READ_STMT: Lazy<Statement> = statement! {
    "SELECT * FROM users WHERE id = $1",
    &[Type::VARCHAR]
};

static UPDATE_STMT: Lazy<Statement> = statement! {
    "UPDATE users SET name = $2, password = $3 WHERE id = $1",
    &[Type::VARCHAR, Type::VARCHAR, Type::BYTEA]
};

static DELETE_STMT: Lazy<Statement> = statement! {
    "DELETE FROM users WHERE id = $1",
    &[Type::VARCHAR]
};

pub struct User {
    id: String,
    name: String,
    password: Vec<u8>,
}

#[derive(Deserialize)]
pub struct UserCreateInfo {
    id: String,
    name: String,
    password: String,
}

#[derive(Serialize)]
pub struct UserInfo {
    id: String,
    name: String,
}

impl From<User> for UserInfo {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}

#[async_trait]
impl Model<User, UserCreateInfo, str> for User {
    fn id(&self) -> &str {
        &self.id
    }

    async fn up() -> Result<(), Box<dyn Error>> {
        DB::execute(&UP_STMT, &[])
            .await
            .map(|_| ())
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    async fn create(new: &Self) -> Result<Self, AppError> {
        DB::query_one(&CREATE_STMT, &[&new.id, &new.name, &new.password])
            .await
            .map_err(AppError::from)
    }

    async fn read(id: &str) -> Result<Self, AppError> {
        DB::query_one::<Self>(&READ_STMT, &[&id])
            .await
            .map_err(AppError::from)
    }

    async fn update(old: &Self, new: &Self) -> Result<(), AppError> {
        assert_eq!(new.id(), old.id());

        DB::execute(&UPDATE_STMT, &[&old.id, &new.name, &new.password])
            .await
            .map(|_| ())
            .map_err(AppError::from)
    }

    async fn delete(id: &str) -> Result<(), AppError> {
        DB::execute(&DELETE_STMT, &[&id])
            .await
            .map(|_| ())
            .map_err(AppError::from)
    }
}

impl From<UserCreateInfo> for User {
    fn from(value: UserCreateInfo) -> Self {
        Self {
            id: value.id,
            name: value.name,
            password: Sha3_512::digest(value.password.as_bytes()).to_vec(),
        }
    }
}

impl User {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn password(&self) -> &[u8] {
        &self.password
    }
}

impl From<Row> for User {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            password: row.get("password"),
        }
    }
}
