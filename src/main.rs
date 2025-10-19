use serenity::all::{EventHandler, GatewayIntents};
use serenity::{Client};
use crate::bot::Noble;

mod message_gen;
mod bot;

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = "TOKEN".to_owned();
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

