use crate::db;
use crate::bet;
use std::sync::{Arc, Mutex};
use std::env;
use reqwest::Client as ReqwestClient;
use chrono::prelude::*;
use serde_json::Value;
use rusqlite::Connection;
use bet::BetOverlord;

async fn send_steam_request(request: &str) -> Option<Value> {
    let client = ReqwestClient::new();
    let resp = client.get(request).send().await.ok()?;
    let json: Value = resp.json().await.ok()?;
    Some(json)
}

async fn update_debt_hours(db: Arc<Mutex<Connection>>, bet_handler:&mut BetOverlord) -> String {
    dotenv::dotenv().ok();
    let api_key = env::var("API_KEY").expect("Expected a token in the environment");
    let _tekken_id = 1778820;
    let mut message = String::from("Tekken debtors:\n");
    let db_connection = db.lock().unwrap();
    let mut new_week = false;
    let mut new_month = false;
    match db::get_time(&db_connection) {
        Ok(mut time_wizard) => {
            for time in &mut time_wizard {
                let now = Local::now();
                let current_month = now.month();
                let current_year = now.year();
                let week = time.get_week();
                let month = time.get_month();
                let year = time.get_year();
                if week == 7 {
                    time.set_week(1);
                    new_week = true;
                }
                else {
                    let bumped = week + 1;
                    time.set_week(bumped);
                }
                if month != current_month {
                    time.set_month(month);
                }
                if year != current_year {
                    time.set_year(year);
                }
                if month < current_month || (month >= current_month && year < current_year) {
                    new_month = true;
                }
                let _ = db::update_time(&db_connection, time.clone());
            }
        }
        Err(e) => {
            println!("Database error: {:?}", e);
        }
    }
    // "https://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/?key={}&steamid={}&format=json"
    match db::get_users(&db_connection) {
        Ok(mut users) => {
            for user in &mut users {
                // Assuming CSV columns: name, hours
                let name = user.get_id().to_string();
                let hours = user.get_playtime();
                let total_hours = user.get_hours_owed();
                let steam_id = user.get_steamid().to_string();
                let mut playtime_outer = hours;
                if hours < total_hours {
                    let request = format!(
                        "https://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/?key={}&steamid={}&format=json",
                        api_key,
                        steam_id,
                    );
                    if let Some(json) = send_steam_request(&request).await {
                        // Safely get the games array
                        if let Some(games) = json.get("response")
                                                .and_then(|r| r.get("games"))
                                                .and_then(|g| g.as_array()) {
                            // Find the game with appid 1778820
                            if let Some(tekken_game) = games.iter().find(|game| {
                                game.get("appid").and_then(|id| id.as_u64()) == Some(1778820)
                            }) {
                                // Get playtime_forever in minutes
                                let playtime = tekken_game.get("playtime_forever")
                                                        .and_then(|p| p.as_u64())
                                                        .unwrap_or(0);
                                playtime_outer = ((playtime as f32 / 60.0) * 100.0).trunc() / 100.0;
                                // Convert to hours for readability
                            } else {
                                println!("Tekken not found for this user.");
                            }
                        } else {
                            println!("No games found in response.");
                        }
                    }
                    let hours_left =  total_hours - hours;
                    if hours == playtime_outer {
                        message.push_str(&format!("<@{}> has played {} hours and has {} hours left to go!\nThey have played ZERO tekken hours within the last 24 hours :(\n", name, hours, hours_left));
                    }
                    else {
                        user.set_hours_owed(playtime_outer);
                        message.push_str(&format!("<@{}> has played {} hours and has {} hours left to go!\nThey have played {} tekken hours since last time, way to go :D!!!\n", name, playtime_outer, hours_left, playtime_outer - hours));
                    }
                    // If its a new month and we need to see if interest should be added
                    if new_month {
                        let monthy_hours = user.get_monthly_hours();
                        if monthy_hours < 5.0 {
                            playtime_outer = hours_left + (hours_left * 0.05);
                            user.set_hours_owed(playtime_outer);
                            message.push_str(&format!("<@{}> has not played their 5 monthly tekken hours and has incurred the 5% interest penalty. They now owe {} more hours D:", name, (hours_left*0.05)));
                        }
                        // reset monthly play counter
                        user.set_monthly_hours(0.0);
                    }
                }
                // Check to see if its a new week and if so reset available betting hours
                if new_week {
                    bet_handler.update_bet_hours(name.to_string(), 0.0);
                    user.set_bet_hours_available(0.0);
                }
                else {
                    user.set_bet_hours_available(bet_handler.get_bet_hours(&name));
                }
                //TODO: update user now in db
                let _ = db::update_user(&db_connection, user.clone());
            }
        }
        Err(e) => {
            println!("Database error: {:?}", e);
        }
    }
    message
}

pub async fn daily_check(db: Arc<Mutex<Connection>>, bet_handler:&mut BetOverlord) -> String {
    let message = update_debt_hours(db.clone(), bet_handler).await;
    message
}

pub fn get_user_debts(db: Arc<Mutex<Connection>>) -> String {
    let mut message = String::from("Tekken debtors:\n");
    let db_connection = db.lock().unwrap();
    match db::get_users(&db_connection) {
        Ok(users) => {
            for user in users {
                let name = user.get_name();
                let playtime = user.get_playtime();
                let hours_owed = user.get_hours_owed();
                let time_left = hours_owed - playtime;
                if time_left > 0.0 {
                    message.push_str(&format!("{} has played {} hours and still has {} hours left. As a reminder {} has {} total hours owed.\n", name, playtime, time_left, name, hours_owed));
                }
            }
        }
        Err(e) => {
            println!("Database error: {:?}", e);
        }
    }
    message
}
