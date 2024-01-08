use tokio_postgres::{Error, NoTls};
use config::PARAMS;


pub async fn migration_user() -> Result<(), Error> {
    
    let (client, connection) = tokio_postgres::connect(&PARAMS, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let query_create_users ="
    CREATE TABLE IF NOT EXISTS users (
        id              SERIAL PRIMARY KEY,
        username        VARCHAR UNIQUE NOT NULL,
        password        VARCHAR NOT NULL,
        email           VARCHAR UNIQUE NOT NULL
        )
    ";

    match client.batch_execute(query_create_users).await {
        Ok(_) => eprintln!("Berhasil Create Table Users"),
        Err(e) => eprintln!("Error: {}", e)
    }

    Ok(())
}