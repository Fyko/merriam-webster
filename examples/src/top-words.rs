use std::env;

use merriam_webster_http::MerriamWebsterClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let api_key = env::var("MERRIAM_WEBSTER_API_KEY")?;
    let client = MerriamWebsterClient::new(api_key.into());

    let top_words = client.top_words().await?;

    println!("The current top words are: {}", top_words.data.words.join(", "));

    Ok(())
}
