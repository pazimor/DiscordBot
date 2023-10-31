use std::env;

mod ping_cmd;
// pub use ping_cmd::{ping, ping_ip, pick_vpn, pc_on, pc_on, pc_off};

use crate::ping_cmd::{PING_DEFAULT_COMMAND, PING_IP_COMMAND, PICK_VPN_COMMAND, PC_ON_COMMAND, PC_OFF_COMMAND};

use serenity::async_trait;
use serenity::prelude::*;
use serenity::framework::standard::macros::{group};
use serenity::framework::standard::{StandardFramework};

#[group]
#[commands(ping_default, ping_ip, pick_vpn, pc_on, pc_off)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    println!("debut");
    let token = env::var("DISCORD_TOKEN").expect("token");
    println!("token {}", token);
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("client error: {:?}", why);
    }
}


