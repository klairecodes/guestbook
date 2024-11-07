// db.rs
use sqlx::query;
use sqlx::{PgPool};
use std::error::Error;

// Initializes the database with some empty tables.
pub async fn initialize_db(
    pool: &PgPool,
) -> Result<(), Box<dyn Error>> {
    query!(r#"
    CREATE TABLE IF NOT EXISTS guestbook_app (
        id              UUID PRIMARY KEY, -- app instance id
        creation_time   TIMESTAMP WITH TIME ZONE, -- time app was created
        guestbooks      UUID REFERENCES guestbook(id)
    );
    "#)
    .execute(pool)
    .await?;

    query!(r#"
    CREATE TABLE IF NOT EXISTS users (
        id              UUID PRIMARY KEY,
        creation_time   TIMESTAMP WITH TIME ZONE, -- time entry was created
        csh_id          VARCHAR(256), -- 256 is unix username limit
        first_name      VARCHAR(64), -- real first name
        last_name       VARCHAR(64) -- real last name
    );
    "#)
    .execute(pool)
    .await?;

    query!(r#"
    CREATE TABLE IF NOT EXISTS guestbook (
        id              UUID PRIMARY KEY,
        start_date      DATE, -- the date of the first entry in the guestbook
        end_date        DATE, -- the date of the last entry in the guestbook
        host            VARCHAR (128), -- who is the host of this book
        assets          TEXT[], -- array of s3 bucket urls for images
        entries         UUID REFERENCES entry(id) -- all the entries in the guestbook
    );
    "#)
    .execute(pool)
    .await?;

    query!(r#"
    CREATE TABLE IF NOT EXISTS entry (
        id              UUID PRIMARY KEY,
        last_edited     TIMESTAMP WITH TIME ZONE, -- time entry was last modified
        written         DATE, -- date the entry was written in the book
        content         TEXT -- transcribed content of the entry
    );
    "#)
    .execute(pool)
    .await?;

    Ok(())
}

// pub async fn insert_and_verify(
//     transaction: &mut Transaction<'_, Postgres>,
//     test_id: i64,
// ) -> Result<(), Box<dyn Error>> {
//     query!(
//         r#"INSERT INTO todos (id, description)
//         VALUES ( $1, $2 )
//         "#,
//         test_id,
//         "test todo"
//     )
//     .execute(&mut **transaction)
//     .await?;
//
//     // check that inserted todo can be fetched inside the uncommitted transaction
//     let _ = query!(r#"SELECT FROM todos WHERE id = $1"#, test_id)
//         .fetch_one(&mut **transaction)
//         .await?;
//
//     Ok(())
// }
//
// pub async fn explicit_rollback_example(
//     pool: &PgPool,
//     test_id: i64,
// ) -> Result<(), Box<dyn Error>> {
//     let mut transaction = pool.begin().await?;
//
//     insert_and_verify(&mut transaction, test_id).await?;
//
//     transaction.rollback().await?;
//
//     Ok(())
// }
//
// pub async fn implicit_rollback_example(
//     pool: &PgPool,
//     test_id: i64,
// ) -> Result<(), Box<dyn Error>> {
//     let mut transaction = pool.begin().await?;
//
//     insert_and_verify(&mut transaction, test_id).await?;
//
//     // no explicit rollback here but the transaction object is dropped at the end of the scope
//     Ok(())
// }
//
// pub async fn commit_example(
//     pool: &PgPool,
//     test_id: i64,
// ) -> Result<(), Box<dyn Error>> {
//     let mut transaction = pool.begin().await?;
//
//     insert_and_verify(&mut transaction, test_id).await?;
//
//     transaction.commit().await?;
//
//     Ok(())
// }

