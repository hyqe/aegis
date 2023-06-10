

use sqlx::{postgres::PgPoolOptions, Row};
use std::env;


pub struct User {
    id: String,
    email: String,
}

pub async fn create_user_table() -> Result<(), sqlx::Error> {
    let uri = env::var("PG").expect("Error: PG not found");

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(uri.as_str()).await?;

    sqlx::query("C INTO users (id, email) VALUES (?, ?);")
    .bind(u.id.clone())
    .bind(u.email.clone())
    .execute(&pool).await?;

    Ok(())
}


pub async fn create_user(u: &User) -> Result<(), sqlx::Error> {
    let uri = env::var("PG").expect("Error: PG not found");

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(uri.as_str()).await?;

    sqlx::query("INSERT INTO users (id, email) VALUES (?, ?);")
    .bind(u.id.clone())
    .bind(u.email.clone())
    .execute(&pool).await?;

    Ok(())
}




pub async fn get_user(id: String) -> Result<User, sqlx::Error> {
    let uri = env::var("PG").expect("Error: PG not found");

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(uri.as_str()).await?;

    let row = sqlx::query("SELECT id, email FROM users WHERE id = ?;")
    .bind(id)
    .fetch_one(&pool).await?;

    Ok(User{
        id: row.get(0),
        email: row.get(1),
    })
}

