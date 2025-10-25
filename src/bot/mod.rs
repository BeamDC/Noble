mod event_handler;

use crate::message_gen::MessageGenerator;
use serenity::all::{EventHandler, GuildId};
use std::collections::HashMap;
use std::sync::RwLock;

pub struct Noble {
    servers: RwLock<HashMap<GuildId, MessageGenerator>>,
    /// the likelihood of the bot generating a message, per message sent.
    /// this is calculated as a 1 in `trigger_chance` chance of happening,
    /// so larger values result in less frequent generation
    trigger_chance: usize,
}

impl Noble {
    const MESSAGE_LIMIT: u8 = 255;

    pub fn new() -> Self {
        Self {
            servers: RwLock::new(HashMap::new()),
            ..Default::default()
        }
    }
}

impl Default for Noble {
    fn default() -> Self {
        Self {
            servers: RwLock::new(HashMap::new()),
            trigger_chance: 100 // 1 in 100 chance
        }
    }
}

