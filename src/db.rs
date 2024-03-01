use lazy_static::lazy_static;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use tokio::runtime;

// a struct to hold the database pool.
pub struct AppState {
    pub db_pool: PgPool,
}

// initialize the SQLx PostgreSQL pool lazily.
lazy_static! {
    pub static ref APP_STATE: AppState = {
        // Read database connection details from environment variables.
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

        //// Create the database pool.
        //let db_pool = PgPoolOptions::new()
            //.max_connections(5) // Adjust the maximum number of connections as needed.
            //.connect(&database_url);

        // Create the tokio runtime to run asynchronous tasks.
        let rt = runtime::Runtime::new().unwrap();

        // Create the database pool within the tokio runtime.
        let db_pool = rt.block_on(async {
            PgPoolOptions::new()
                .max_connections(5) // Adjust the maximum number of connections as needed.
                .connect(&database_url)
                .await
                .expect("Failed to create database pool")
        });
                //
        // Return the application state with the database pool.
        AppState { db_pool }
    };
}
