mod db;
mod daily_task;
mod bet;
use crate::db::User;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::Client;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::env;
use tokio::time::Duration;
use rusqlite::Connection;

struct Handler {
    db: Arc<Mutex<Connection>>,
}

static BET_HANDLER: Lazy<Mutex<bet::BetOverlord>> = Lazy::new(|| {
    Mutex::new(bet::BetOverlord::new())
});

static DAILY_TASK_SPAWNED: AtomicBool = AtomicBool::new(false);

fn parse_id(id: String) -> String {
    let parsed_id = id
            .trim_start_matches('<')
            .trim_end_matches('>')
            .strip_prefix('@')
            .unwrap_or("Bad request");
    return parsed_id.to_string();
}

fn setup_betting_manager(db: Arc<Mutex<Connection>>) {
    let db_connection = db.lock().unwrap();
    match db::get_users(&db_connection) {
        Ok(users) => {
            for user in &users {
                BET_HANDLER.lock().unwrap().add_better(user.get_id().to_string());
                BET_HANDLER.lock().unwrap().add_relation(user.get_id().to_string(), user.get_name().to_string());
                BET_HANDLER.lock().unwrap().update_bet_hours(user.get_id().to_string(), 10.0);
                BET_HANDLER.lock().unwrap().update_hour_change(user.get_id().to_string(), 0.0);
                // Not ideal but for the time being should work, should probably be a schema change
                if user.get_name() == "Kwangwon" || user.get_name() == "Brandon" || user.get_name() == "Daniel" {
                    BET_HANDLER.lock().unwrap().add_trusted(user.get_id().to_string());
                }
            }
        }
        Err(e) => {
            println!("Database error: {:?}", e);
        }
    }
}

#[serenity::async_trait]
impl EventHandler for Handler {

    async fn ready(&self, ctx: Context, _data: Ready) {

        if DAILY_TASK_SPAWNED.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
            // already started
            return;
        }
        // we only want this copy of the db to exist for setting up
        // the bet manager, so set it in its own scope
        {
            let setup_db = self.db.clone();
            setup_betting_manager(setup_db);
        }

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
                    let author = msg.author.to_string();
                    let cleaned = parse_id(author);
                    if msg.content.starts_with("!bet") && BET_HANDLER.lock().unwrap().can_bet(&cleaned) {
                        let parts: Vec<&str> = msg.content.split_whitespace().collect();
                        let bet_amount = parts[2].parse::<f32>().unwrap_or(-1.0);
                        // after parsing the bet_amount first check that its a legal value
                        if bet_amount <= 0.0 || bet_amount > 10.0 {
                            let _ = msg.channel_id.say(&ctx.http, "Incorrect bet value, please try again".to_string()).await;
                        } else {
                            // If it is legal now start to parse the rest of the command
                            let bet_recp = parts[1].to_string();
                            let parsed_bet_recp = parse_id(bet_recp);
                            if cleaned == "Bad request" || parsed_bet_recp == "Bad request" {
                                let _ = msg.channel_id.say(&ctx.http, "Invalid users entered, please try again".to_string()).await;
                            } else {
                                // We now know all the values that were entered have been entered correctly, so now
                                // we need to check if the users are eligble to bet and also that they have the correct
                                // amount of hours to place the bet
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
                    if msg.content.starts_with("!winner") && BET_HANDLER.lock().unwrap().is_trusted(&cleaned) {
                        let db_connection = self.db.clone();
                        let parts: Vec<&str> = msg.content.split_whitespace().collect();
                        let winner = parts[1].to_string();
                        let parsed_winner = parse_id(winner);
                        let ticket = parts[2].parse::<i32>().unwrap_or(-1);
                        if ticket == -1 {
                            let _ = msg.channel_id.say(&ctx.http, "Invalid bet number please try again").await;
                        } else if parsed_winner == "Bad request" {
                                let _ = msg.channel_id.say(&ctx.http, "Invalid user entered, please try again".to_string()).await;
                        } else {
                            let (winner_res, loser_res, amount_res) = BET_HANDLER.lock().unwrap().handle_bet_resolution(db_connection, ticket, parsed_winner.to_string());
                            if winner_res != "Fake" {
                                let message = format!("<@{}> has won the bet losing {} hours from their debt while <@{}> has lost and nobly taking on {} hours of tekken", winner_res, amount_res, loser_res, amount_res);
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
                    if msg.content.starts_with("!cancel-bet") && BET_HANDLER.lock().unwrap().is_trusted(&cleaned) {
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
                    if msg.content.starts_with("!show-user-stats") {
                        let parts: Vec<&str> = msg.content.split_whitespace().collect();
                        let user = parts[1].to_string();
                        let parsed_user = parse_id(user);
                        if parsed_user != "Bad request" {
                            let http = ctx.http.clone();
                            let user;
                            // This database section needs its own scope so that we can send a message as a response after
                            // the amount has been retrieved from the database
                            {
                                let db = self.db.clone();
                                let db_connection = db.lock().unwrap();
                                // This is actually awful and needs to be fixed, but can be fixed later
                                user = db::get_user(&db_connection, &parsed_user).unwrap().unwrap();
                            }
                            let _ = msg.channel_id.say(&http, format!(
                                    "{} has played {:?} hours\n\nOwes a total of {:?} hours\n\nHas played {:?} hours this month\n\nAnd has {:?} hours to bet with this week",
                                    user.get_name(), user.get_playtime(), user.get_hours_owed(), user.get_monthly_hours(), BET_HANDLER.lock().unwrap().get_bet_hours(user.get_id())
                                )).await;
                        }
                    }
                    if msg.content.starts_with("!help") {
                        let http = ctx.http.clone();
                        let message = "Reminder for the commands !bet, !winner, and !show-user-stats please @ the user for each respective command\n\
                        Commands:\n\n\
                        !bet [bet receiever] [hours bet] - only users who are registered in the system can place bets. If you try to place a bet with a higher hour amount than either player can bet, the bot will reject the creation of the bet.\n\n\
                        !winner [bet winner] [bet number] - this command can only be used by trusted users to clear a bet, please provide the bet number that was given at the bet creation you wish to clear. If you do not remember use the command !list-bets\n\n\
                        !cancel-bet [bet number] - this command can only be used by trusted users to cancel a bet, please provide the bet number to remove the bet from the outstanding bets which will also refund each player their bet hours. If you do not remember use the command !list-bets\n\n\
                        !list-bets - any user can use this command and it will show a list of the outstanding bets which include the users invovled as well as the bet number\n\n\
                        !debts - any user can use this command and it will show the users in the system who still have outstanding debt\n\n\
                        !show-user-stats [tekken gamer] - any user can use this command and it will show the total hours played, hours owed, monthly hours, and bet hours available for the week\n\n";
                        let _ = msg.channel_id.say(&http, message).await;
                    }
                    // The following functions are only intended for admin use and so will not be added to the help message
                    // For now admins can either add or remove members from the trusted list and I imagine changing bet hours
                    // may be necessary but would undermine trust in the bot. So at launch it will not
                    let brandon_id:&str = &format!("{}", env::var("BRANDON_ID").unwrap());
                    let kwangwon_id:&str = &format!("{}", env::var("KWANGWON_ID").unwrap());
                    if msg.content.starts_with("!add-trusted") && (cleaned == brandon_id || cleaned == kwangwon_id) {
                        let parts: Vec<&str> = msg.content.split_whitespace().collect();
                        let winner = parts[1].to_string();
                        let new_trusted = parse_id(winner);
                        let http = ctx.http.clone();
                        let _ = BET_HANDLER.lock().unwrap().add_trusted(new_trusted.to_string());
                        let _ = msg.channel_id.say(&http, "Added new member to trusted list").await;
                    }
                    if msg.content.starts_with("!remove-trusted") && (cleaned == brandon_id || cleaned == kwangwon_id) {
                        let parts: Vec<&str> = msg.content.split_whitespace().collect();
                        let wizard = parts[1].to_string();
                        let new_wizard = parse_id(wizard);
                        let http = ctx.http.clone();
                        let _ = BET_HANDLER.lock().unwrap().remove_trusted(&new_wizard);
                        let _ = msg.channel_id.say(&http, "Removed member from the trusted list").await;
                    }
                    if msg.content.starts_with("!add-user") && (cleaned == brandon_id || cleaned == kwangwon_id) {
                        let parts: Vec<&str> = msg.content.split_whitespace().collect();
                        let wizard = parts[1].to_string();
                        let new_wizard = parse_id(wizard);
                        let name = parts[2].to_string();
                        let playtime = parts[3].parse::<f32>().unwrap_or(-1.0);
                        let hours_owed = parts[4].parse::<f32>().unwrap_or(-1.0);
                        let steam_id = parts[5].to_string();
                        // update the bet handler first
                        // This is bad code btw, I shouldn't be cloning each time I want to pass the value
                        // however, I also don't have time currently to properly fix this prime target for
                        // (Issue #11)
                        BET_HANDLER.lock().unwrap().add_better(new_wizard.clone());
                        BET_HANDLER.lock().unwrap().add_relation(new_wizard.clone(), name.clone());
                        BET_HANDLER.lock().unwrap().update_bet_hours(new_wizard.clone(), 10.0);
                        BET_HANDLER.lock().unwrap().update_hour_change(new_wizard.clone(), 0.0);
                        let newbie = User::new(new_wizard, name, playtime, hours_owed, steam_id, 0.0, 10.0);
                        // now update the db
                        {
                            let db = self.db.clone();
                            let db_connection = db.lock().unwrap();
                            let _ = db::add_user(&db_connection, newbie).unwrap();
                        }
                        // send a success message
                        let http = ctx.http.clone();
                        let _ = msg.channel_id.say(&http, "Added user to the bot").await;
                    }
                    if msg.content.starts_with("!adjust-hours") && (cleaned == brandon_id || cleaned == kwangwon_id) {
                        let parts: Vec<&str> = msg.content.split_whitespace().collect();
                        let wizard = parts[1].to_string();
                        let new_wizard = parse_id(wizard);
                        let new_hours = parts[2].parse::<f32>().unwrap_or(-1.0);
                        let http = ctx.http.clone();
                        if new_hours == -1.0 {
                            let _ = msg.channel_id.say(&http, "Invalid hour amount").await;
                        } else {
                            // Another instance of the db needing its own scope because we want to send a
                            // message after the db operation is successful
                            {
                                let db = self.db.clone();
                                let db_connection = db.lock().unwrap(); 
                                let _ = db::update_hours_owed(&db_connection, &new_wizard, new_hours).unwrap();
                            }
                            let _ = msg.channel_id.say(&http, "successfully updated hours").await;
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
