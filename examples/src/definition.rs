use rand::prelude::*;
use std::env;

use merriam_webster_http::MerriamWebsterClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let api_key = env::var("MERRIAM_WEBSTER_API_KEY")?;
    let client = MerriamWebsterClient::new(api_key.into());

    let mut rng = thread_rng();
    let top_words = client.top_words().await?.data.words;
    let word = top_words.choose(&mut rng).unwrap();

    let defs = client
        .collegiate_definition(word.to_string())
        .await?;
    let def = defs.first()
        .expect(format!("No definition found for {}", word).as_str());
    let shortdefs = def.shortdef.as_ref().unwrap();

    println!("The short definitions of \x1b[33;1m{word}\x1b[0m are:\n\n{}", shortdefs.iter().map(|d| format!("\x1b[0;34m-\x1b[0;0m {d}")).collect::<Vec<String>>().join("\n"));

    Ok(())
}
