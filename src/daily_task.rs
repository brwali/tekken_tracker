use crate::db;
use crate::bet;
use crate::db::Time;
use std::sync::{Arc, Mutex};
use std::env;
use reqwest::Client as ReqwestClient;
use chrono::prelude::*;
use chrono::Duration;
use serde_json::Value;
use rusqlite::Connection;
use bet::BetOverlord;

// Global consts
const TEKKEN_APP_ID: u64 = 1778820;

async fn get_request(request: &str, token: Option<&str>) -> Option<Value> {
    let client = ReqwestClient::new();
    let mut req = client.get(request);
    if let Some(auth) = token {
        req = req.bearer_auth(auth);
    }
    let resp = req.send().await.ok()?;
    let json: Value = resp.json().await.ok()?;
    Some(json)
}

pub async fn match_analysis(polaris_id: &str, token: &str, daily_playtime: f32, name: &str) -> String {
    let request = format!(
        "https://api.ewgf.gg/external/battles/{}",
        polaris_id,
    );
    let json_response = get_request(&request, Some(token)).await;
    let now = Utc::now();
    let cutoff = now - Duration::hours(48);
    if let Some(json) = json_response {
        if let Some(data) = json.get("data") {
            if let Ok(battles) = serde_json::from_value::<Vec<Value>>(data.clone()) {
            let mut win_no = 0;
            let mut round_count = 0;
            let mut loss_no = 0;
            let mut draw_no = 0;
            for battle in battles {
                if let Some(battle_at_str) = battle.get("battle_at").and_then(|v| v.as_str()) {
                    if let Ok(battle_at) = DateTime::parse_from_rfc3339(battle_at_str) {
                        if battle_at > cutoff {
                            let cleaned_polaris_id = polaris_id.replace("-", "");
                            let winner = battle.get("winner").and_then(|v| v.as_u64()).unwrap_or(0);
                            let winner_id;
                            if winner == 1 {
                                winner_id =  battle.get("p1_tekken_id").and_then(|v| v.as_str()).unwrap_or("");
                            } else if winner == 2 {
                                winner_id = battle.get("p2_tekken_id").and_then(|v| v.as_str()).unwrap_or("");
                            } else {
                                winner_id = "Draw";
                            }
                            let p1_rounds = battle.get("p1_rounds_won").and_then(|v| v.as_u64()).unwrap_or(0);
                            let p2_rounds = battle.get("p2_rounds_won").and_then(|v| v.as_u64()).unwrap_or(0);
                            round_count += p1_rounds + p2_rounds;
                            if winner_id == cleaned_polaris_id {
                                win_no += 1;
                            } else if winner_id == "Draw" {
                                draw_no += 1;
                            } else {
                                loss_no += 1;
                            }
                        }
                    }
                }
            }
            let mut message = String::new();
            if round_count == 0 {
                message = format!("No games were played within the last 48 hours. This could be the result of the debtor labbing, or story mode, but do you really believe that they would lab for {} hours :thinking:\n", daily_playtime).to_string();
            } else {
                message.push_str(&format!("Within the last 48 hours, {} matches were played!! {} wins, {} losses, {} draws\n", win_no + loss_no + draw_no, win_no, loss_no, draw_no));
                // Assume 1 round == 1 minute
                let playtime_percentage = round_after_math((round_count as f32) / round_after_math(daily_playtime / 60.0));
                if  playtime_percentage < 0.50 && name == "Mason" {
                    message.push_str(&format!("Using our advanced analytical algorithm :) the debtor did not meet the normal playtime threshold.\n\
                    This could be the result of afk'd hours, labbing, or waiting in a custom lobby. The Tekken Bank would like to give the benefit of the doubt, but given past behavior of this debtor we have flagged this tekken session.\n"));
                }
            }
            return message;
        }
    }
}
    "Failed to fetch or parse battles.\n".to_string()
}

async fn update_debt_hours(db: Arc<Mutex<Connection>>, bet_handler:&mut BetOverlord) -> (String, bool) {
    dotenv::dotenv().ok();
    let api_key = env::var("API_KEY").expect("Expected a token in the environment");
    let ewgf_key = env::var("EWGF_KEY").expect("Expected a token in the environment");
    let _tekken_id = 1778820;
    let mut message = String::from("Tekken debtors:\n");
    let db_connection = db.lock().unwrap();
    let mut new_week = false;
    let mut new_month = false;
    let mut time = Time::new();
    // Keeps track of # of days since last tekken game
    let mut zero_day_streak = 0;
    let mut total_hours_today = 0.0;
    match db::get_time(&db_connection) {
        Ok(mut time_wizard) => {
            for t in &mut time_wizard {
                let now = Local::now();
                let current_month = now.month();
                let current_year = now.year();
                let week = t.get_week();
                let month = t.get_month();
                let year = t.get_year();
                zero_day_streak = t.get_zero_day_streak();
                if week == 7 {
                    t.set_week(1);
                    new_week = true;
                }
                else {
                    let bumped = week + 1;
                    t.set_week(bumped);
                }
                if month != current_month {
                    t.set_month(current_month);
                }
                if year != current_year {
                    t.set_year(current_year);
                }
                if month < current_month || (month >= current_month && year < current_year) {
                    new_month = true;
                }
                time = t.clone();
            }
        }
        Err(e) => {
            return (format!("Database error: {:?}", e).to_string(), false);
        }
    }
    match db::get_users(&db_connection) {
        Ok(mut users) => {
            for user in &mut users {
                let name = user.get_id().to_string();
                let hours = user.get_playtime();
                let total_hours = user.get_hours_owed();
                let steam_id = user.get_steamid().to_string();
                let birth_name = user.get_name().to_string();
                let mut playtime_outer = hours;
                if hours < total_hours {
                    let request = format!(
                        "https://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/?key={}&steamid={}&format=json",
                        api_key,
                        steam_id,
                    );
                    if let Some(json) = get_request(&request, None).await {
                        // Safely get the games array
                        if let Some(games) = json.get("response")
                                                .and_then(|r| r.get("games"))
                                                .and_then(|g| g.as_array()) {
                            if let Some(tekken_game) = games.iter().find(|game| {
                                game.get("appid").and_then(|id| id.as_u64()) == Some(TEKKEN_APP_ID)
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
                    let hours_left = round_after_math(total_hours - playtime_outer);
                    let daily_playtime = round_after_math(playtime_outer - hours);
                    let mut played_today = false;
                    if hours == playtime_outer {
                        if new_week {
                            message.push_str(&format!("<@{}> has played {} hours and has {} hours left to go!\nThey have played ZERO tekken hours within the last 24 hours :(\n", name, playtime_outer, hours_left));
                        }
                        else {
                            message.push_str(&format!("{} has played {} hours and has {} hours left to go!\nThey have played ZERO tekken hours within the last 24 hours :(\n\n", birth_name, playtime_outer, hours_left));
                        }
                    }
                    else {
                        played_today = true;
                        total_hours_today += daily_playtime;
                        user.set_playtime(playtime_outer);
                        let monthly_hours = user.get_monthly_hours();
                        user.set_monthly_hours(round_after_math(monthly_hours + daily_playtime));
                        if new_week {
                            message.push_str(&format!("<@{}> has played {} hours and has {} hours left to go!\nThey have played {} tekken hours since last time, way to go :D!!!\n", name, playtime_outer, hours_left, daily_playtime));
                        }
                        else {
                            message.push_str(&format!("{} has played {} hours and has {} hours left to go!\nThey have played {} tekken hours since last time, way to go :D!!!\n\n", birth_name, playtime_outer, hours_left, daily_playtime));
                        }
                    }
                    // Due to API restraints we can only get matches with a 24 hour delay
                    // so if they played yesterday get match history
                    if user.get_played_yesterday() == 1 {
                        let matches = match_analysis(user.get_polar_id(), &ewgf_key, daily_playtime, &birth_name).await;
                        message.push_str(&matches);
                        message.push_str("\n");
                        user.set_played_yesterday(0);
                    } else if played_today && user.get_played_yesterday() == 0 {
                        user.set_played_yesterday(1);
                    }
                    // If its a new month and we need to see if interest should be added
                    if new_month {
                        let monthy_hours = user.get_monthly_hours();
                        if monthy_hours < 5.0 {
                            playtime_outer = total_hours + (hours_left * 0.05);
                            playtime_outer = round_after_math(playtime_outer);
                            user.set_hours_owed(playtime_outer);
                            message.push_str(&format!("<@{}> has not played their 5 monthly tekken hours and has incurred the 5% interest penalty. They now owe {} more hours D:\n\n", name, round_after_math(hours_left*0.05)));
                        }
                        // reset monthly play counter
                        user.set_monthly_hours(0.0);
                    }
                }
                // Check to see if its a new week and if so reset available betting hours
                if new_week {
                    let mut hours_earned = bet_handler.get_hours_change(&name);
                    let change;
                    if hours_earned > 0.0 {
                        change = "gained";
                    } else {
                        hours_earned = hours_earned * -1.0;
                        change = "lost";
                    };
                    if hours_earned == 0.0 {
                        message.push_str(&format!("This week {} has not engaged in any bets.\n\n", birth_name));
                    } else {
                        message.push_str(&format!("This week {} has {} {} hours through bets.\n\n", birth_name, change, hours_earned));
                    }
                    bet_handler.update_bet_hours(name.to_string(), 10.0);
                    user.set_bet_hours_available(10.0);
                    bet_handler.update_hour_change(name, 0.0);
                }
                else {
                    user.set_bet_hours_available(bet_handler.get_bet_hours(&name));
                }
                match db::update_user(&db_connection, user.clone()){
                    Ok(_) => println!("Update successful"),
                    Err(e) => println!("Update failed: {:?}", e),
                }
            }
            if total_hours_today > 0.0 {
                zero_day_streak = 0;
                message.push_str(&format!("The debtors have played {} hours of tekken today! POG\n\n", total_hours_today));
            } else {
                zero_day_streak += 1;
                message.push_str(&format!("The debtors have played ZERO hours of tekken today.\n\n"));
                message.push_str(&format!("It has been {} days since any tekken has been played.\n\n", zero_day_streak));
            }
        }
        Err(e) => {
            println!("Database error: {:?}", e);
        }
    }
    time.set_zero_day_streak(zero_day_streak);
    // Update time DB table
    let _ = db::update_time(&db_connection, time.clone());
    (message, total_hours_today == 0.0)
}

pub async fn daily_check(db: Arc<Mutex<Connection>>, bet_handler:&mut BetOverlord) -> (String, bool) {
    let (message, no_one_played_today) = update_debt_hours(db.clone(), bet_handler).await;
    (message, no_one_played_today)
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
                let time_left = round_after_math(hours_owed - playtime);
                if time_left > 0.0 {
                    message.push_str(&format!("{} has played {} hours and still has {} hours left. As a reminder {} has {} total hours owed.\n\n", name, playtime, time_left, name, hours_owed));
                }
            }
        }
        Err(e) => {
            println!("Database error: {:?}", e);
        }
    }
    message
}

pub fn round_after_math(val: f32) -> f32 {
    (val * 100.0).trunc() / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_after_math_positive() {
        assert_eq!(round_after_math(1.2345), 1.23);
        assert_eq!(round_after_math(0.0), 0.0);
        assert_eq!(round_after_math(2.9999), 2.99);
    }

    #[test]
    fn test_round_after_math_negative() {
        // truncation moves toward zero for negatives
        assert_eq!(round_after_math(-1.239), -1.23);
        assert_eq!(round_after_math(-0.001), 0.0);
    }
}
