mod db;

use sqlx::query;
use sqlx::PgPool;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // load all environment variables from .env file
    dotenvy::dotenv()?;

    let conn_str =
        std::env::var("DATABASE_URL").expect("Env var DATABASE_URL is required for this example.");
    let pool = PgPool::connect(&conn_str).await?;

    let test_id = 1;

    // remove any old values that might be in the table already with this id from a previous run
    let _ = query!(r#"DELETE FROM todos WHERE id = $1"#, test_id)
        .execute(&pool)
        .await?;

    db::explicit_rollback_example(&pool, test_id).await?;

    // check that inserted todo is not visible outside the transaction after explicit rollback
    let inserted_todo = query!(r#"SELECT FROM todos WHERE id = $1"#, test_id)
        .fetch_one(&pool)
        .await;

    assert!(inserted_todo.is_err());

    db::implicit_rollback_example(&pool, test_id).await?;

    // check that inserted todo is not visible outside the transaction after implicit rollback
    let inserted_todo = query!(r#"SELECT FROM todos WHERE id = $1"#, test_id)
        .fetch_one(&pool)
        .await;

    assert!(inserted_todo.is_err());

    db::commit_example(&pool, test_id).await?;

    // check that inserted todo is visible outside the transaction after commit
    let inserted_todo = query!(r#"SELECT FROM todos WHERE id = $1"#, test_id)
        .fetch_one(&pool)
        .await;

    assert!(inserted_todo.is_ok());

    Ok(())
}
