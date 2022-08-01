use futures::executor::block_on;
use sqlx::postgres::PgPool;
use sqlx::postgres::PgPoolOptions;
use sqlx::FromRow;

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
            .connect("postgresql://postgres:0wXfOxfV8V4D70jCpJII@containers-us-west-38.railway.app:5873/railway"));

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
