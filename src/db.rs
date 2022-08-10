use crate::{Result, Whisper};
use mongodb::{options::ClientOptions, Client, Collection};
use std::env;
// use std::env;

pub async fn connect_to_db() -> Result<Client> {
    let client = Client::with_options(ClientOptions::parse(env::var("DATABASE_URL")?).await?)?;
    Ok(client)
}

pub fn get_collection(client: Client) -> Collection<Whisper> {
    client.database("tamako").collection::<Whisper>("whispers")
}
