use serenity::all::{GatewayIntents};
use serenity::Client;
use crate::bot::Noble;

mod message_gen;
mod bot;

const BOT_TOKEN: &str = "BOT_TOKEN_HERE";

#[tokio::main]
async fn main() {
    // Login with a bot token
    let token = BOT_TOKEN.to_owned();

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client =
        Client::builder(&token, intents)
            .event_handler(Noble::new())
            .await
            .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

