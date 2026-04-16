use std::env;
use tokio::sync::OnceCell;
use tokio_postgres::{Client, Error, NoTls};

static CLIENT: OnceCell<Client> = OnceCell::const_new();

pub async fn connect_db() -> Result<(), Error> {
    let (CLIENT, connection) =
        tokio_postgres::connect((env::var("DB_HOST"), env::var("DB_USER")), NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprint!("Connection error {}", e)
        }
    });

    Ok(())
}
