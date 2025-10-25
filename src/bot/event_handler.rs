use std::collections::HashMap;
use std::time::Instant;
use rand::{thread_rng, Rng};
use serenity::all::{EventHandler, Message, Ready, Context, GetMessages, GuildId};
use serenity::async_trait;
use crate::bot::Noble;
use crate::message_gen::MessageGenerator;

#[async_trait]
impl EventHandler for Noble {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let start = Instant::now();
        let response = self.servers
            .read()
            .unwrap()
            .get(&msg.guild_id.unwrap())
            .unwrap()
            .next_message(self.trigger_chance);

        let Some(response) = response else {
            return
        };

        if let Err(e) = msg.channel_id.say(&ctx.http, response).await {
            println!("Failed to send message: {}", e);
        } else { println!("Message generated in {:?}", start.elapsed())}

        let mut gen = self.servers.write().unwrap();

        let context = msg.content.clone();
        gen.get_mut(&msg.guild_id.unwrap())
            .unwrap()
            .update(context, None, None);
    }

    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        let mut servers = HashMap::new();

        for g in data_about_bot.guilds.iter() {
            let mut messages = vec![];
            let guild_id = g.id;

            let channels = match guild_id.channels(&ctx.http).await {
                Ok(c) => c,
                Err(e) => {
                    println!("Failed to get channels for guild {}:\n\t'{}'", guild_id, e);
                    continue;
                },
            };

            // read up to the MESSAGE_LIMIT most recent messages from each channel
            for (_, info) in channels.iter() {

                let m = match info.messages(&ctx.http, GetMessages::default().limit(Self::MESSAGE_LIMIT)).await {
                    Ok(msgs) => msgs,
                    Err(e) => {
                        println!("Failed to get messages for channel {} in guild {}:\n\t'{}'", info.id, guild_id, e);
                        continue;
                    }
                };

                messages.extend(m);
            }

            let context = messages
                .iter()
                .filter(|m| !m.author.bot)
                .map(|m| m.content.clone())
                .collect::<Vec<String>>()
                .join(" ");

            servers.insert(guild_id, context);

            println!("gathered from guild: {}", guild_id);
        }

        let mut servs = self.servers.write().unwrap();
        for (k, v) in servers {
            let gen = MessageGenerator::from_string(v, MessageGenerator::MED_PRECISION, 50);
            servs.insert(k, gen);
        }
    }
}