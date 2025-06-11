use rusqlite::{Connection, Row};

use crate::{models::query::QueryBuilder, utils::sql_safe};

mod query;

pub trait FromDB: Sized {
    fn query(conn: &Connection) -> QueryBuilder;
    const TABLE_NAME: &str;
    fn from_row(data_row: Row) -> Self;
    fn to_row(&self) -> String;
}

pub struct User {
    id: usize,
    username: String,
}

impl FromDB for User {
    const TABLE_NAME: &str = "users";
    fn from_row(data_row: Row) -> Self {
        User {
            id: data_row.get("id").unwrap(),
            username: data_row.get("username").unwrap(),
        }
    }

    fn query(conn: &Connection) -> QueryBuilder {
        QueryBuilder::new(conn, Self::TABLE_NAME)
    }

    fn to_row(&self) -> String {
        format!("{}, {}", self.id, sql_safe(self.username.clone()))
    }
}
