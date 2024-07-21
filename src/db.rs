// db.rs
use sqlx::query;
use sqlx::{PgPool, Postgres, Transaction};
use std::error::Error;

pub async fn insert_and_verify(
    transaction: &mut Transaction<'_, Postgres>,
    test_id: i64,
) -> Result<(), Box<dyn Error>> {
    query!(
        r#"INSERT INTO todos (id, description)
        VALUES ( $1, $2 )
        "#,
        test_id,
        "test todo"
    )
    .execute(&mut **transaction)
    .await?;

    // check that inserted todo can be fetched inside the uncommitted transaction
    let _ = query!(r#"SELECT FROM todos WHERE id = $1"#, test_id)
        .fetch_one(&mut **transaction)
        .await?;

    Ok(())
}

pub async fn explicit_rollback_example(
    pool: &PgPool,
    test_id: i64,
) -> Result<(), Box<dyn Error>> {
    let mut transaction = pool.begin().await?;

    insert_and_verify(&mut transaction, test_id).await?;

    transaction.rollback().await?;

    Ok(())
}

pub async fn implicit_rollback_example(
    pool: &PgPool,
    test_id: i64,
) -> Result<(), Box<dyn Error>> {
    let mut transaction = pool.begin().await?;

    insert_and_verify(&mut transaction, test_id).await?;

    // no explicit rollback here but the transaction object is dropped at the end of the scope
    Ok(())
}

pub async fn commit_example(
    pool: &PgPool,
    test_id: i64,
) -> Result<(), Box<dyn Error>> {
    let mut transaction = pool.begin().await?;

    insert_and_verify(&mut transaction, test_id).await?;

    transaction.commit().await?;

    Ok(())
}

