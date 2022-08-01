use futures::executor::block_on;
use sqlx::Row;
use sqlx::postgres::PgPool;
use sqlx::postgres::PgPoolOptions;
use sqlx::FromRow;
use crate::signup::CreateInfo;

#[derive(FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct Db {
    pub pool: PgPool,
}

impl Db {
    pub fn connect() -> Db {
        let pool = block_on(PgPoolOptions::new()
            .max_connections(5)
            .connect("postgresql://postgres:1234@localhost:5432/postgres"));

        match pool {
            Ok(pool) => Db { pool },
            Err(e) => panic!("{}", e),
        }
    }

    pub async fn select_all(&self) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&self.pool)
            .await?;

        Ok(users)
    }

    pub async fn create_user(&self, username: &str, email: &str, password: &str) -> Result<i64, sqlx::Error> {
        let id = sqlx::query
            ("INSERT INTO users (username,email,password) VALUES ($1,$2,$3) RETURNING id;")
            .bind(username)
            .bind(email)
            .bind(password)
            .fetch_one(&self.pool)
            .await?;

        let id = id.get::<i64, _>("id");
        println!("new user: {}", id);
        Ok(id)
    }

    pub async fn select_user_by_id(&self, id: i32) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn select_user_by_name(&self, name: &str) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
            .bind(name)
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }
}
