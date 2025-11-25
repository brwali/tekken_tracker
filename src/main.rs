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
    let brandon_id:&str = &format!("{}", env::var("BRANDON_ID").unwrap());
    let kwangwon_id:&str = &format!("{}", env::var("KWANGWON_ID").unwrap());
    // First we need to add the people who can bet
    // Add Jackson
    let jackson_id:&str = &format!("{}", env::var("JACKSON_ID").unwrap());
    BET_HANDLER.lock().unwrap().add_better(jackson_id.to_string());
    BET_HANDLER.lock().unwrap().add_relation(jackson_id.to_string(), "Jackson".to_string());
    BET_HANDLER.lock().unwrap().update_bet_hours(jackson_id.to_string(), 10.0);
    BET_HANDLER.lock().unwrap().update_hour_change(jackson_id.to_string(), 0.0);
    // Add Mason
    let mason_id:&str = &format!("{}", env::var("MASON_ID").unwrap());
    BET_HANDLER.lock().unwrap().add_better(mason_id.to_string());
    BET_HANDLER.lock().unwrap().add_relation(mason_id.to_string(), "Mason".to_string());
    BET_HANDLER.lock().unwrap().update_bet_hours(mason_id.to_string(), 10.0);
    BET_HANDLER.lock().unwrap().update_hour_change(mason_id.to_string(), 0.0);
    // Add Jonathan
    let jon_id:&str = &format!("{}", env::var("JON_ID").unwrap());
    BET_HANDLER.lock().unwrap().add_better(jon_id.to_string());
    BET_HANDLER.lock().unwrap().add_relation(jon_id.to_string(), "Jonathan".to_string());
    BET_HANDLER.lock().unwrap().update_bet_hours(jon_id.to_string(), 10.0);
    BET_HANDLER.lock().unwrap().update_hour_change(jon_id.to_string(), 0.0);
    // Add Logan
    let logan_id:&str = &format!("{}", env::var("LOGAN_ID").unwrap());
    BET_HANDLER.lock().unwrap().add_better(logan_id.to_string());
    BET_HANDLER.lock().unwrap().add_relation(logan_id.to_string(), "Logan".to_string());
    BET_HANDLER.lock().unwrap().update_bet_hours(logan_id.to_string(), 10.0);
    BET_HANDLER.lock().unwrap().update_hour_change(logan_id.to_string(), 0.0);
    // Add Brandon
    BET_HANDLER.lock().unwrap().add_better(brandon_id.to_string());
    BET_HANDLER.lock().unwrap().add_relation(brandon_id.to_string(), "Brandon".to_string());
    BET_HANDLER.lock().unwrap().update_bet_hours(brandon_id.to_string(), 10.0);
    BET_HANDLER.lock().unwrap().update_hour_change(brandon_id.to_string(), 0.0);
    // Add Wyatt
    let wyatt_id:&str = &format!("{}", env::var("WYATT_ID").unwrap());
    BET_HANDLER.lock().unwrap().add_better(wyatt_id.to_string());
    BET_HANDLER.lock().unwrap().add_relation(wyatt_id.to_string(), "Wyatt".to_string());
    BET_HANDLER.lock().unwrap().update_bet_hours(wyatt_id.to_string(), 10.0);
    BET_HANDLER.lock().unwrap().update_hour_change(wyatt_id.to_string(), 0.0);
    // Add Bryan
    let bryan_id:&str = &format!("{}", env::var("BRYAN_ID").unwrap());
    BET_HANDLER.lock().unwrap().add_better(bryan_id.to_string());
    BET_HANDLER.lock().unwrap().add_relation(bryan_id.to_string(), "Bryan".to_string());
    BET_HANDLER.lock().unwrap().update_bet_hours(bryan_id.to_string(), 10.0);
    BET_HANDLER.lock().unwrap().update_hour_change(bryan_id.to_string(), 0.0);
    // Add Kwangwon
    BET_HANDLER.lock().unwrap().add_better(kwangwon_id.to_string());
    BET_HANDLER.lock().unwrap().add_relation(kwangwon_id.to_string(), "Kwangwon".to_string());
    BET_HANDLER.lock().unwrap().update_bet_hours(kwangwon_id.to_string(), 10.0);
    BET_HANDLER.lock().unwrap().update_hour_change(kwangwon_id.to_string(), 0.0);
    //Now we need to add the trusted third party members\
    // Add Brandon
    BET_HANDLER.lock().unwrap().add_trusted(brandon_id.to_string());
    // Add Kwangwon
    BET_HANDLER.lock().unwrap().add_trusted(kwangwon_id.to_string());
    // Add Daniel
    let daniel_id:&str = &format!("{}", env::var("DANIEL_ID").unwrap_or("".to_string()));
    BET_HANDLER.lock().unwrap().add_trusted(daniel_id.to_string());
    BET_HANDLER.lock().unwrap().add_relation(daniel_id.to_string(), "Daniel".to_string());
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

                    let tree_id = ChannelId::new(1433474989365002342);
                    let _ = tree_id.say(&http, message.clone()).await;
                    let kazoo_id = ChannelId::new(1319106712313135116);
                    let _ = kazoo_id.say(&http, message.clone()).await;

                    tokio::time::sleep(Duration::from_secs(60 * 60 * 24)).await;
                }
            }
        });
    }

    async fn message(&self, ctx: Context, msg: Message) {
        // Fetch the channel object
        if let Ok(channel) = msg.channel_id.to_channel(&ctx).await {
            if let serenity::model::channel::Channel::Guild(guild_channel) = channel {
                if guild_channel.name == "tekken" {
                    if msg.content.starts_with("!bet") {
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
                            if bet_amount <= 0.0 || bet_amount > 10.0 {
                                let _ = msg.channel_id.say(&ctx.http, "Incorrect bet value, please try again".to_string()).await;
                            } else {
                                let bet_recp = parts[1].to_string();
                                let parsed_bet_recp = bet_recp
                                    .trim_start_matches('<')
                                    .trim_end_matches('>')
                                    .strip_prefix('@')
                                    .unwrap_or("Bad request");
                                if cleaned == "Bad request" || parsed_bet_recp == "Bad request" {
                                    let _ = msg.channel_id.say(&ctx.http, "Invalid users entered, please try again".to_string()).await;
                                } else {
                                    if BET_HANDLER.lock().unwrap().can_bet(&parsed_bet_recp) && BET_HANDLER.lock().unwrap().hour_check(&cleaned, &parsed_bet_recp, bet_amount) {
                                        let ticket_no = BET_HANDLER.lock().unwrap().handle_bet_creation(cleaned.to_string(), parsed_bet_recp.to_string(), bet_amount);
                                        let _ = msg.channel_id.say(&ctx.http, format!("Bet successfully submitted, your bet number is {}\n",ticket_no.to_string())).await;
                                    }
                                    else {
                                        let _ = msg.channel_id.say(&ctx.http, "Error forming the ticket :/".to_string()).await;
                                    }
                                }
                            }
                        }
                    }
                    if msg.content.starts_with("!winner") {
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
                            if ticket == -1 {
                                let _ = msg.channel_id.say(&ctx.http, "Invalid bet number please try again").await;
                            } else if parsed_winner == "Bad request" {
                                    let _ = msg.channel_id.say(&ctx.http, "Invalid user entered, please try again".to_string()).await;
                            } else {
                                let (winner_res, loser_res, amount_res) = BET_HANDLER.lock().unwrap().handle_bet_resolution(db_connection, ticket, parsed_winner.to_string());
                                if winner_res != "Fake" {
                                    let message = format!("<@{}> has won the bet losing {} hours from their debt while <@{}> has lost and nobly takens on {} hours of tekken", winner_res, amount_res, loser_res, amount_res);
                                    // Change for when bot is ready, the bot should send the message to tree and kazoo regardless of where the command was entered
                                    let tree_id = ChannelId::new(1433474989365002342);
                                    let _ = tree_id.say(&ctx.http, message.clone()).await;
                                    let kazoo_id = ChannelId::new(1319106712313135116);
                                    let _ = kazoo_id.say(&ctx.http, message).await;
                                }
                                else {
                                    let _ = msg.channel_id.say(&ctx.http, "Error :(").await;
                                }
                            }
                        }
                    }
                    if msg.content.starts_with("!cancel-bet") {
                        let author = msg.author.to_string();
                        let cleaned = author
                            .trim_start_matches('<')
                            .trim_end_matches('>')
                            .strip_prefix('@')
                            .unwrap_or("Bad request");
                        if BET_HANDLER.lock().unwrap().is_trusted(&cleaned) {
                            let parts: Vec<&str> = msg.content.split_whitespace().collect();
                            let ticket = parts[1].parse::<i32>().unwrap_or(-1);
                            if ticket == -1 {
                                let _ = msg.channel_id.say(&ctx.http, "Invalid bet number please try again").await;
                            } else {
                                let amount = BET_HANDLER.lock().unwrap().cancel_bet(ticket);
                                if amount > 0.0 {
                                    let message = format!("Bet number {} has been removed and both debtors have recieved a bet refund of {} hours", ticket, amount);
                                    let _ = msg.channel_id.say(&ctx.http, message).await;
                                }
                                else {
                                    let _ = msg.channel_id.say(&ctx.http, "Error :(").await;
                                }
                            }
                        }
                    }
                    if msg.content.starts_with("!list-bets") {
                        let http = ctx.http.clone();
                        let message = BET_HANDLER.lock().unwrap().list_bets();
                        let _ = msg.channel_id.say(&http, message).await;
                    }
                    if msg.content.starts_with("!debts") {
                        let http = ctx.http.clone();
                        let db_connection = self.db.clone();
                        let message = daily_task::get_user_debts(db_connection);
                        let _ = msg.channel_id.say(&http, message).await;
                    }
                    if msg.content.starts_with("!monthly-hours") {
                        let parts: Vec<&str> = msg.content.split_whitespace().collect();
                        let user = parts[1].to_string();
                        let parsed_winner = user
                            .trim_start_matches('<')
                            .trim_end_matches('>')
                            .strip_prefix('@')
                            .unwrap_or("Bad request");
                        if user != "Bad request" {
                            let http = ctx.http.clone();
                            let amount;
                            {
                                let db = self.db.clone();
                                let db_connection = db.lock().unwrap(); 
                                amount = db::get_monthly_hours(&db_connection, &parsed_winner).unwrap();
                            }
                            let _ = msg.channel_id.say(&http, format!("They have played {:?} hours this month", amount.unwrap())).await;
                        }
                    }
                    if msg.content.starts_with("!help") {
                        let http = ctx.http.clone();
                        let message = "Reminder for the commands !bet and !winner please @ the user who you are trying to initiate/resolve the bet for\n\
                        Commands:\n\n\
                        !bet [bet receiever] [hours bet] - only users who are registered in the system can place bets. If you try to place a bet with a higher hour amount than either player can bet, the bot will reject the creation of the bet.\n\n\
                        !winner [bet winner] [bet number] - this command can only be used by trusted users to clear a bet, please provide the bet number that was given at the bet creation you wish to clear. If you do not remember use the command !list-bets\n\n\
                        !cancel-bet [bet number] - this command can only be used by trusted users to cancel a bet, please provide the bet number to remove the bet from the outstanding bets which will also refund each player their bet hours. If you do not remember use the command !list-bets\n\n\
                        !list-bets - any user can use this command and it will show a list of the outstanding bets which include the users invovled as well as the bet number\n\n\
                        !debts - any user can use this command and it will show the users in the system who still have outstanding debt\n\n";
                        let _ = msg.channel_id.say(&http, message).await;
                    }
                    // The following functions are only intended for admin use and so will not be added to the help message
                    // For now admins can either add or remove members from the trusted list and I imagine changing bet hours
                    // may be necessary but would undermine trust in the bot. So at launch it will not
                    let brandon_id:&str = &format!("{}", env::var("BRANDON_ID").unwrap());
                    let kwangwon_id:&str = &format!("{}", env::var("KWANGWON_ID").unwrap());
                    if msg.content.starts_with("!add-trusted") {
                        let author = msg.author.to_string();
                        let cleaned = author
                            .trim_start_matches('<')
                            .trim_end_matches('>')
                            .strip_prefix('@')
                            .unwrap_or("Bad request");
                        if cleaned == brandon_id || cleaned == kwangwon_id {
                            let parts: Vec<&str> = msg.content.split_whitespace().collect();
                            let winner = parts[1].to_string();
                            let new_trusted = winner
                                .trim_start_matches('<')
                                .trim_end_matches('>')
                                .strip_prefix('@')
                                .unwrap_or("Bad request");
                            let http = ctx.http.clone();
                            let _ = BET_HANDLER.lock().unwrap().add_trusted(new_trusted.to_string());
                            let _ = msg.channel_id.say(&http, "Added new member to trusted list").await;
                        }
                    }
                    if msg.content.starts_with("!remove-trusted") {
                        let author = msg.author.to_string();
                        let cleaned = author
                            .trim_start_matches('<')
                            .trim_end_matches('>')
                            .strip_prefix('@')
                            .unwrap_or("Bad request");
                        if cleaned == brandon_id || cleaned == kwangwon_id {
                            let parts: Vec<&str> = msg.content.split_whitespace().collect();
                            let wizard = parts[1].to_string();
                            let new_wizard = wizard
                                .trim_start_matches('<')
                                .trim_end_matches('>')
                                .strip_prefix('@')
                                .unwrap_or("Bad request");
                            let http = ctx.http.clone();
                            let _ = BET_HANDLER.lock().unwrap().remove_trusted(new_wizard);
                            let _ = msg.channel_id.say(&http, "Removed member from the trusted list").await;
                        }
                    }
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
