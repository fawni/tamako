use webhook::client::{WebhookClient, WebhookResult};

/// Send a webhook with the given whisper
pub async fn send(whisper: &super::Whisper) -> WebhookResult<()> {
    WebhookClient::new(&std::env::var("WEBHOOK_URL")?)
        .send(|m| {
            m.content(if whisper.is_public() {
                "💬 | New whisper!"
            } else {
                "🔒 | New private whisper!"
            })
            .username("tamako")
            .avatar_url("https://i.imgur.com/JwA0Hty.png")
            .embed(|e| {
                e.author(
                    &whisper.name.clone().unwrap_or_else(|| "anon".to_owned()),
                    None,
                    None,
                )
                .description(&whisper.message)
                .footer(&whisper.timestamp, None)
                .color(if whisper.is_public() {
                    "15440289"
                } else {
                    "14826110"
                })
            })
        })
        .await?;

    Ok(())
}
