mod db;
mod daily_task;
mod bet;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::Client;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use std::env;
use tokio::time::Duration;
use rusqlite::Connection;

struct Handler {
    db: Arc<Mutex<Connection>>,
}

static BET_HANDLER: Lazy<Mutex<bet::BetOverlord>> = Lazy::new(|| {
    Mutex::new(bet::BetOverlord::new())
});

fn setup_betting_manager() {
    // First we need to add the people who can bet
    // Add Jackson
    BET_HANDLER.lock().unwrap().add_better("<@259508260082548747>".to_string());
    // Add Mason
    BET_HANDLER.lock().unwrap().add_better("<@236622475612389377>".to_string());
    // Add Jonathan
    BET_HANDLER.lock().unwrap().add_better("<@489595366174490624>".to_string());
    // Add Logan
    BET_HANDLER.lock().unwrap().add_better("<@258772151585341440>".to_string());
    // Add Brandon
    BET_HANDLER.lock().unwrap().add_better("<@451064565963161611>".to_string());
    // Add Wyatt
    BET_HANDLER.lock().unwrap().add_better("<@303219081941614592>".to_string());
    // Add Bryan
    BET_HANDLER.lock().unwrap().add_better("<@259826437022810112>".to_string());
    // Add Kwangwon
    BET_HANDLER.lock().unwrap().add_better("<@389916126626185216>".to_string());
    //Now we need to add the trusted third party members\
    // Add Brandon
    BET_HANDLER.lock().unwrap().add_trusted("<@451064565963161611>".to_string());
    // Add Kwangwon
    BET_HANDLER.lock().unwrap().add_trusted("<@389916126626185216>".to_string());
    // Add Daniel
    BET_HANDLER.lock().unwrap().add_trusted("<@230147129492897794>".to_string());
}

#[serenity::async_trait]
impl EventHandler for Handler {

    async fn ready(&self, ctx: Context, _data: Ready) {

        setup_betting_manager();

        tokio::spawn({
            let http = ctx.http.clone();
            let db_connection = self.db.clone();

            async move {
                loop {
                    // Run blocking DB logic in spawn_blocking
                    let db_clone = db_connection.clone();
                    let message = tokio::task::spawn_blocking(move || {
                        // daily_check must be sync or block_on if async
                        let mut bet_handler = BET_HANDLER.lock().unwrap();
                        tokio::runtime::Handle::current().block_on(daily_task::daily_check(db_clone, &mut *bet_handler))
                    })
                    .await
                    .expect("spawn_blocking failed");

                    let channel_id = ChannelId::new(1404935148419682304);
                    let _ = channel_id.say(&http, message).await;

                    tokio::time::sleep(Duration::from_secs(60 * 60 * 24)).await;
                }
            }
        });
    }

    async fn message(&self, ctx: Context, msg: Message) {
        // Fetch the channel object
        //TODO: message validation because you cannot trust the people who are going to use this bot
        if let Ok(channel) = msg.channel_id.to_channel(&ctx).await {
            if let serenity::model::channel::Channel::Guild(guild_channel) = channel {
                if guild_channel.name == "tekken-tracker" && msg.content.starts_with("!bet") && BET_HANDLER.lock().unwrap().can_bet(&msg.author.to_string()) {
                    let parts: Vec<&str> = msg.content.split_whitespace().collect();
                    // the type is def wrong here for the bets argh
                    let bet_amount = parts[2].parse::<f32>().unwrap_or(-1.0);
                    let bet_init = msg.author.to_string();
                    let bet_recp = parts[1].to_string();
                    if BET_HANDLER.lock().unwrap().can_bet(&bet_recp) && BET_HANDLER.lock().unwrap().hour_check(&bet_init, &bet_recp, bet_amount) {
                        let ticket_no = BET_HANDLER.lock().unwrap().handle_bet_creation(bet_init, bet_recp, bet_amount);
                        let _ = msg.channel_id.say(&ctx.http, ticket_no.to_string()).await;
                    }
                }
                if guild_channel.name == "tekken-tracker" && msg.content.starts_with("!winner") && BET_HANDLER.lock().unwrap().is_trusted(&msg.author.to_string()) {
                    let _parts: Vec<&str> = msg.content.split_whitespace().collect();

                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::all();
    let db_connection = match db::init_db() {
        Ok(conn) => std::sync::Arc::new(std::sync::Mutex::new(conn)),
        Err(e) => {
            eprintln!("Failed to initialize DB: {}", e);
            return;
        }
    };
    let handler = Handler {
        db: db_connection.clone(),
    };
    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
