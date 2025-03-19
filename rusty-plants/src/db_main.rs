use std::error::{Error};
use sqlx::{Connection, ConnectOptions, Pool};
use sqlx::postgres::{PgConnectOptions, PgConnection, PgPool, PgQueryResult, PgSslMode, Postgres};
use tokio::main;

async fn connect() -> Result<Pool<Postgres>,Box<dyn Error>> {
    // Manually-constructed options
    let conn = PgConnectOptions::new()
        .host("localhost")
        .port(5432)
        .username("postgres")
        .password("myPassword")
        .ssl_mode(PgSslMode::Require)
        .connect()
        .await?;

    // Modifying options parsed from a string
    let mut opts: PgConnectOptions = "postgres://localhost/mydb".parse()?;

    // Change the log verbosity level for queries.
    // Information about SQL queries is logged at `DEBUG` level by default.
    opts = opts.log_statements(log::LevelFilter::Trace);

    let pool = PgPool::connect_with(opts).await?;

    Ok(pool)
}

#[tokio::main]
async fn main() {
    let pool = connect().await.unwrap();
    let create_table_query = "
    CREATE TABLE Edges (
      id          INT PRIMARY KEY,
      parentId    INT,
      childId     INT,
      stringStart INT,
      stringEnd   INT
);";
    // TODO How to get the following to match by Type in Rust,
    // with extracted variables that we can use in each arm
    let result = sqlx::query(create_table_query).execute(&pool).await;
    match &result {
        Error => {
            println!("Error {:?}", result);
        },
        PgQueryResult => {
            println!("Success {:?}", result);
        },
    }
}