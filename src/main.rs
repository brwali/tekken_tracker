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
    const JACKSON_ID:&str = "259508260082548747";
    BET_HANDLER.lock().unwrap().add_better(JACKSON_ID.to_string());
    BET_HANDLER.lock().unwrap().add_relation(JACKSON_ID.to_string(), "Jackson".to_string());
    BET_HANDLER.lock().unwrap().update_bet_hours(JACKSON_ID.to_string(), 10.0);
    // Add Mason
    const MASON_ID:&str  = "236622475612389377";
    BET_HANDLER.lock().unwrap().add_better(MASON_ID.to_string());
    BET_HANDLER.lock().unwrap().add_relation(MASON_ID.to_string(), "Mason".to_string());
    BET_HANDLER.lock().unwrap().update_bet_hours(MASON_ID.to_string(), 10.0);
    // Add Jonathan
    const JON_ID:&str  = "489595366174490624";
    BET_HANDLER.lock().unwrap().add_better(JON_ID.to_string());
    BET_HANDLER.lock().unwrap().add_relation(JON_ID.to_string(), "Jonathan".to_string());
    BET_HANDLER.lock().unwrap().update_bet_hours(JON_ID.to_string(), 10.0);
    // Add Logan
    const LOGAN_ID:&str  = "258772151585341440";
    BET_HANDLER.lock().unwrap().add_better(LOGAN_ID.to_string());
    BET_HANDLER.lock().unwrap().add_relation(LOGAN_ID.to_string(), "Logan".to_string());
    BET_HANDLER.lock().unwrap().update_bet_hours(LOGAN_ID.to_string(), 10.0);
    // Add Brandon
    const BRANDON_ID:&str  = "451064565963161611";
    BET_HANDLER.lock().unwrap().add_better(BRANDON_ID.to_string());
    BET_HANDLER.lock().unwrap().add_relation(BRANDON_ID.to_string(), "Brandon".to_string());
    BET_HANDLER.lock().unwrap().update_bet_hours(BRANDON_ID.to_string(), 10.0);
    // Add Wyatt
    const WYATT_ID:&str  = "303219081941614592";
    BET_HANDLER.lock().unwrap().add_better(WYATT_ID.to_string());
    BET_HANDLER.lock().unwrap().add_relation(WYATT_ID.to_string(), "Wyatt".to_string());
    BET_HANDLER.lock().unwrap().update_bet_hours(WYATT_ID.to_string(), 10.0);
    // Add Bryan
    const BRYAN_ID:&str  = "259826437022810112";
    BET_HANDLER.lock().unwrap().add_better(BRYAN_ID.to_string());
    BET_HANDLER.lock().unwrap().add_relation(BRYAN_ID.to_string(), "Bryan".to_string());
    BET_HANDLER.lock().unwrap().update_bet_hours(BRYAN_ID.to_string(), 10.0);
    // Add Kwangwon
    const KWANGWON_ID:&str  = "389916126626185216";
    BET_HANDLER.lock().unwrap().add_better(KWANGWON_ID.to_string());
    BET_HANDLER.lock().unwrap().add_relation(KWANGWON_ID.to_string(), "Kwangwon".to_string());
    BET_HANDLER.lock().unwrap().update_bet_hours(KWANGWON_ID.to_string(), 10.0);
    //Now we need to add the trusted third party members\
    // Add Brandon
    BET_HANDLER.lock().unwrap().add_trusted(BRANDON_ID.to_string());
    // Add Kwangwon
    BET_HANDLER.lock().unwrap().add_trusted(KWANGWON_ID.to_string());
    // Add Daniel
    const DANIEL_ID:&str  = "230147129492897794";
    BET_HANDLER.lock().unwrap().add_trusted(DANIEL_ID.to_string());
    BET_HANDLER.lock().unwrap().add_relation(DANIEL_ID.to_string(), "Daniel".to_string());
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
                if guild_channel.name == "tekken-tracker" && msg.content.starts_with("!bet") {
                    let author = msg.author.to_string();
                    let cleaned = author
                        .trim_start_matches('<')
                        .trim_end_matches('>')
                        .strip_prefix('@')
                        .unwrap_or("Bad request"); 
                    if BET_HANDLER.lock().unwrap().can_bet(&cleaned) {
                        let parts: Vec<&str> = msg.content.split_whitespace().collect();
                        // the type is def wrong here for the bets argh
                        let bet_amount = parts[2].parse::<f32>().unwrap_or(-1.0);
                        let bet_recp = parts[1].to_string();
                        let parsed_bet_recp = bet_recp
                            .trim_start_matches('<')
                            .trim_end_matches('>')
                            .strip_prefix('@')
                            .unwrap_or("Bad request");
                        if BET_HANDLER.lock().unwrap().can_bet(&parsed_bet_recp) && BET_HANDLER.lock().unwrap().hour_check(&cleaned, &parsed_bet_recp, bet_amount) {
                            let ticket_no = BET_HANDLER.lock().unwrap().handle_bet_creation(cleaned.to_string(), parsed_bet_recp.to_string(), bet_amount);
                            let _ = msg.channel_id.say(&ctx.http, format!("Bet successfully submitted, your bet number is {}\n",ticket_no.to_string())).await;
                        }
                        else {
                            let _ = msg.channel_id.say(&ctx.http, "Error forming the ticket :/".to_string()).await;
                        }
                    }
                }
                if guild_channel.name == "tekken-tracker" && msg.content.starts_with("!winner") {
                    let author = msg.author.to_string();
                    let cleaned = author
                        .trim_start_matches('<')
                        .trim_end_matches('>')
                        .strip_prefix('@')
                        .unwrap_or("Bad request"); 
                    if BET_HANDLER.lock().unwrap().is_trusted(&cleaned) {
                        let db_connection = self.db.clone();
                        let parts: Vec<&str> = msg.content.split_whitespace().collect();
                        let winner = parts[1].to_string();
                        let parsed_winner = winner
                            .trim_start_matches('<')
                            .trim_end_matches('>')
                            .strip_prefix('@')
                            .unwrap_or("Bad request"); 
                        let ticket = parts[2].parse::<i32>().unwrap_or(-1);
                        let resolution = BET_HANDLER.lock().unwrap().handle_bet_resolution(db_connection, ticket, parsed_winner.to_string());
                        if resolution {
                            let _ = msg.channel_id.say(&ctx.http, "Bet successfully resolved").await;
                        }
                        else {
                            println!("Error!!!");
                        }
                    }
                }
                if guild_channel.name == "tekken-tracker" && msg.content.starts_with("!list-bets") {
                    let http = ctx.http.clone();
                    let message = BET_HANDLER.lock().unwrap().list_bets();
                    let channel_id = ChannelId::new(1404935148419682304);
                    let _ = channel_id.say(&http, message).await;
                }
                if guild_channel.name == "tekken-tracker" && msg.content.starts_with("!debts") {
                    let http = ctx.http.clone();
                    let db_connection = self.db.clone();
                    let message = daily_task::get_user_debts(db_connection);
                    let channel_id = ChannelId::new(1404935148419682304);
                    let _ = channel_id.say(&http, message).await;
                }
                if guild_channel.name == "tekken-tracker" && msg.content.starts_with("!help") {
                    let http = ctx.http.clone();
                    let message = "Reminder for the commands !bet and !winner please @ the user who you are trying to initiate/resolve the bet for\n\
                    Commands:\n\
                    !bet [bet receiever] [hours bet] - only users who are registered in the system can place bets. If you try to place a bet with a higher hour amount than either player can bet, the bot will reject the creation of the bet.\n\
                    !winner [bet winner] [ticket number] - this command can only be used by trusted users to clear a bet, please provide the bet number that was given at the bet creation you wish to clear. If you do not remember use the command !list-bets\n\
                    !list-bets - any user can use this command and it will show a list of the outstanding bets which include the users invovled as well as the bet number\n\
                    !debts - any user can use this command and it will show the users in the system who still have outstanding debt\n";
                    let channel_id = ChannelId::new(1404935148419682304);
                    let _ = channel_id.say(&http, message).await;
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
