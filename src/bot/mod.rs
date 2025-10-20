use serenity::all::{Context, EventHandler, GetMessages, GuildId, Message, Ready};
use serenity::async_trait;
use std::collections::{HashMap, VecDeque};
use std::future::Future;
use std::ops::Deref;
use std::sync::{Arc, RwLock};
use crate::message_gen::MessageGenerator;

pub struct Noble {
    message_gen: RwLock<MessageGenerator>,
}

#[async_trait]
impl EventHandler for Noble {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let response = self.message_gen.read().unwrap().next_message();
        if let Err(e) = msg.channel_id.say(&ctx.http, response).await {
            println!("Failed to send message: {}", e);
        }

        let mut gen = self.message_gen.write().unwrap();
        let context = msg.content.clone();
        gen.update(context, None, None);
    }

    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        let mut messages = vec![];

        // todo : make guild specific generators if we want to generalize the bot more
        for g in data_about_bot.guilds.iter() {
            let guild_id = g.id;

            let channels = match guild_id.channels(&ctx.http).await {
                Ok(c) => c,
                Err(e) => {
                    println!("Failed to get channels for guild {}:\n\t'{}'", guild_id, e);
                    continue;
                },
            };

            // read up to the 100 most recent messages from each channel
            for (_, info) in channels.iter() {

                let m = match info.messages(&ctx.http, GetMessages::default().limit(100)).await {
                    Ok(msgs) => msgs,
                    Err(e) => {
                        println!("Failed to get messages for channel {} in guild {}:\n\t'{}'", info.id, guild_id, e);
                        continue;
                    }
                };

                messages.extend(m);
            }

            println!("gathered from guild: {}", guild_id);
        }

        let context = messages
            .iter()
            .filter(|m| !m.author.bot)
            .map(|m| m.content.clone())
            .collect::<Vec<String>>()
            .join(" ");

        let mut gen = self.message_gen.write().unwrap();
        gen.update(context, Some(MessageGenerator::MED_PRECISION), Some(50));
    }
}

impl Noble {
    const MAX_MESSAGES: usize = 255;

    pub fn new() -> Self {
        Self {
            message_gen: RwLock::new(
                MessageGenerator::from_string(
                    "".to_owned(),
                    MessageGenerator::MED_PRECISION,
                    50,
                )
            ),
        }
    }
}