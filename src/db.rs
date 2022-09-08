use crate::{constants, Whisper};
use color_eyre::Result;
use mongodb::{options::ClientOptions, Client, Collection};

pub async fn connect_to_db() -> Result<Client> {
    Ok(Client::with_options(
        ClientOptions::parse(&*constants::DATABASE_URL).await?,
    )?)
}

pub fn get_collection(client: Client) -> Collection<Whisper> {
    client.database("tamako").collection::<Whisper>("whispers")
}
