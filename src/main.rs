use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::Client;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use std::env;
use std::collections::{HashSet, HashMap};
use std::fs::{File, OpenOptions};
use std::io::{BufReader};
use csv::{Reader, Writer};
use reqwest::Client as ReqwestClient;
use chrono::prelude::*;
use tokio::time::{sleep, Duration};
use serde_json::Value;

struct Handler;
#[derive(Clone)]
struct Bet {
    user1: &str,
    user2: String,
    bet: f32,
    ticket_no: i32,
}
struct BetOverlord {
    betters: HashSet<String>,
    trusted_users: HashSet<String>,
    hours_available: HashMap<String, f32>,
    bet_house: Arc<Mutex<HashMap<i32, Bet>>>,
    counter: i32,
}

impl Bet {
    pub fn new(u1: &str, u2: &str, b: f32, tno: i32) -> Self {
        Bet {
            user1: u1,
            user2: u2,
            bet: b,
            ticket_no: tno,
        }
    }
}

impl BetOverlord {
    pub fn new() -> Self {
        BetOverlord {
            betters: HashSet::<String>::new(),
            trusted_users: HashSet::<String>::new(),
            hours_available: HashMap::<String, f32>::new(),
            bet_house: Arc::new(Mutex::new(HashMap::<i32, Bet>::new())),
            counter: 0,
        }
    }
    fn can_bet(&self, id: &str) -> bool {
        self.betters.contains(&id)
    }
    fn is_trusted(&self, id: &str) -> bool {
        self.trusted_users.contains(&id)
    }
    fn add_better(&mut self, id: String) {
        self.betters.insert(id);
    }
    fn add_trusted(&mut self, id: String) {
        self.trusted_users.insert(id);
    }
    fn update_bet_hours(&mut self, id: String, hours: f32) {
        self.hours_available.insert(id, hours);
    }
    fn hour_check(&self, id1: &str, id2: &str, bet: f32) -> bool {
        let p1_hours = self.hours_available.get(&id1);
        let p2_hours = self.hours_available.get(&id2);
        *p1_hours.unwrap() + bet <= 10.0 && *p2_hours.unwrap() + bet <= 10.0
    }
    fn get_bet_hours(&self, id: &str) -> f32 {
        match self.hours_available.get(&id) {
            Some(&val) => val,
            None => -1.0,
        }
    }
    fn handle_bet_creation(&mut self, id1: &str, id2: &str, bet: f32) -> i32 {
        let ticket_no: i32 = self.counter;
        let bet_ticket: Bet = Bet::new(id1, id2, bet, ticket_no);
        self.counter += 1;
        self.bet_house.lock().unwrap().insert(ticket_no, bet_ticket);
        let p1_hours = self.hours_available.get(&id1).unwrap() - bet;
        let p2_hours = self.hours_available.get(&id2).unwrap() - bet;
        let _ = self.update_bet_hours(id1.to_string(), p1_hours);
        let _ = self.update_bet_hours(id2.to_string(), p2_hours);
        ticket_no
    }
    fn handle_bet_resolution(&mut self, ticket_no: i32) {

    }
}
static KW: Lazy<Mutex<BetOverlord>> = Lazy::new(|| {
    Mutex::new(BetOverlord::new())
});
async fn send_steam_request(request: &str) -> Option<Value> {
    let client = ReqwestClient::new();
    let resp = client.get(request).send().await.ok()?;
    let json: Value = resp.json().await.ok()?;
    Some(json)
}

fn write_records_to_csv(csv_path: &str, header: &[&str], records: &[Vec<String>]) -> std::io::Result<()> {
    let mut wtr = Writer::from_writer(
        OpenOptions::new().write(true).truncate(true).open(csv_path)?
    );
    wtr.write_record(header)?;
    for record in records {
        wtr.write_record(record)?;
    }
    wtr.flush()?;
    Ok(())
}

fn time_check(time_path: &str) -> (String, String, String) {
    let file = match File::open(time_path) {
        Ok(f) => f,
        Err(_) => return ("CSV file not found.".to_string(), "".to_string(), "".to_string()),
    };
    let mut rdr = Reader::from_reader(BufReader::new(file));
    let first_row = match rdr.records().next() {
        Some(result) => match result {
            Ok(r) => r,
            Err(_) => return ("Error reading row".to_string(), "".to_string(), "".to_string()),
        },
        None => return ("CSV is empty".to_string(), "".to_string(), "".to_string()),
    };

    let month = first_row.get(0).unwrap_or("").to_string();
    let week  = first_row.get(1).unwrap_or("").to_string();
    let year = first_row.get(2).unwrap_or("").to_string();
    (month, week, year)
    }

async fn format_tekken_debtors(csv_path: &str) -> (String, Vec<Vec<String>>, Vec<Vec<String>>) {
    dotenv::dotenv().ok();
    let api_key = env::var("API_KEY").expect("Expected a token in the environment");
    let _tekken_id = 1778820;
    let file = match File::open(csv_path) {
        Ok(f) => f,
        Err(_) => return ("CSV file not found.".to_string(), Vec::new(), Vec::new()),
    };
    let mut rdr = Reader::from_reader(BufReader::new(file));
    let mut message = String::from("Tekken debtors:\n");
    let mut updated_records: Vec<Vec<String>> = Vec::new();
    let mut updated_time_record: Vec<Vec<String>> = Vec::new();
    let (month, week, year) = time_check("time_info.csv");
    let now = Local::now();
    let current_month = now.month();
    let current_year = now.year();
    let mut week_int = week.parse::<i32>().unwrap_or(0);
    let month_int = month.parse::<u32>().unwrap_or(0);
    let year_int = year.parse::<i32>().unwrap_or(0);
    let mut new_week = false;
    if week_int == 7 {
        week_int = 0;
        new_week = true;
    }
    else {
        week_int += 1;
    }
    let mut updated_timerow:Vec<String> = [month, week, year].to_vec();
    if month_int != current_month {
        updated_timerow[0] = current_month.to_string();
    }
    if year_int != current_year {
        updated_timerow[2] = current_year.to_string();
    }
    updated_timerow[1] = week_int.to_string();
    // "https://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/?key={}&steamid={}&format=json"
    for result in rdr.records() {
        if let Ok(record) = result {
            // Assuming CSV columns: name, hours
            let name = record.get(0).unwrap_or("Unknown");
            let hours = record.get(1).unwrap_or("0.0").trim().parse::<f32>().unwrap_or(0.0);
            let total_hours = record.get(2).unwrap_or("0.0").trim().parse::<f32>().unwrap_or(0.0);
            let mut playtime_outer = hours;
            let mut updated_rowrecord:Vec<String> = record.iter().map(|s| s.to_string()).collect();
            if hours < total_hours {
                let request = format!(
                    "https://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/?key={}&steamid={}&format=json",
                    api_key,
                    record.get(3).unwrap_or("Unknown").trim()
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
                    message.push_str(&format!("<@{}> has played {} hours and has {} hours left to go!\nThey have played ZERO tekken hours since last time :(\n", name, hours, hours_left));
                }
                else {
                    let playtime_string = playtime_outer.to_string();
                    updated_rowrecord[1] = playtime_string;
                    message.push_str(&format!("<@{}> has played {} hours and has {} hours left to go!\nThey have played {} tekken hours since last time, way to go :D!!!\n", name, playtime_outer, hours_left, playtime_outer - hours));
                }
                // If its a new month and we need to see if interest should be added
                if month_int < current_month || (month_int >= current_month && year_int < current_year) {
                    let monthy_hours = record.get(4).unwrap_or("0").parse::<i32>().unwrap_or(0);
                    if monthy_hours < 5 {
                        playtime_outer = hours_left + (hours_left * 0.05);
                        updated_rowrecord[1] = playtime_outer.to_string();
                        message.push_str(&format!("<@{}> has not played their 5 monthly tekken hours and has incurred the 5% interest penalty. They now owe {} more hours D:", name, (hours_left*0.05)));
                    }
                    // reset monthly play counter
                    updated_rowrecord[4] = "0".to_string();
                }
            }
            // Check to see if its a new week and if so reset available betting hours
            if new_week {
                KW.lock().unwrap().update_bet_hours(name.to_string(), 0.0);
                updated_rowrecord[5] = "0".to_string();
            }
            else {
                updated_rowrecord[5] = KW.lock().unwrap().get_bet_hours(name.to_string()).to_string();
            }
            updated_records.push(updated_rowrecord);
        }
    }
    updated_time_record.push(updated_timerow);
    (message, updated_records, updated_time_record)
}

async fn daily_check() -> String {
    let (message, records, time_records) = format_tekken_debtors("tekken_hours.csv").await;
    let main_header = ["discord_id","playtime","hours_owed","steam_id", "month_hours", "bet_hours_available"];
    let _ = write_records_to_csv("tekken_hours.csv", &main_header, &records);
    let time_header = ["month", "week", "year"];
    let _ = write_records_to_csv("time_info.csv", &time_header,  &time_records);
    message
}

fn setup_betting_manager() {
    // First we need to add the people who can bet
    // Add Jackson
    KW.lock().unwrap().add_better("<@259508260082548747>".to_string());
    // Add Mason
    KW.lock().unwrap().add_better("<@236622475612389377>".to_string());
    // Add Jonathan
    KW.lock().unwrap().add_better("<@489595366174490624>".to_string());
    // Add Logan
    KW.lock().unwrap().add_better("<@258772151585341440>".to_string());
    // Add Brandon
    KW.lock().unwrap().add_better("<@451064565963161611>".to_string());
    // Add Wyatt
    KW.lock().unwrap().add_better("<@303219081941614592>".to_string());
    // Add Bryan
    KW.lock().unwrap().add_better("<@259826437022810112>".to_string());
    //TODO: add KWangwon's id
    //Now we need to add the trusted third party members
    KW.lock().unwrap().add_trusted("<@451064565963161611>".to_string())
    //TODO: add KWangwon's id
    //TODO: add Daniel's id
}

#[serenity::async_trait]
impl EventHandler for Handler {

    async fn ready(&self, ctx: Context, _data: Ready) {

        setup_betting_manager();

        let http = ctx.http.clone();
        // Read from CSV (example: first row, first column)
        tokio::spawn(async move {
            loop {
                let message = daily_check().await;
                // TODO: Replace with tree id when ready
                let channel_id = ChannelId::new(1404935148419682304);
                let _ = channel_id.say(&http, message).await;
                sleep(Duration::from_secs(60 * 60 * 24)).await;
            }
        });
    }

    async fn message(&self, ctx: Context, msg: Message) {
        // Fetch the channel object
        //TODO: message validation because you cannot trust the people who are going to use this bot
        if let Ok(channel) = msg.channel_id.to_channel(&ctx).await {
            if let serenity::model::channel::Channel::Guild(guild_channel) = channel {
                if guild_channel.name == "tekken-tracker" && msg.content.starts_with("!bet") && KW.lock().unwrap().can_bet(&msg.author.to_string()) {
                    let parts: Vec<&str> = msg.content.split_whitespace().collect();
                    // the type is def wrong here for the bets argh
                    let bet_amount = parts[2].parse::<f32>().unwrap_or(-1.0);
                    let bet_init = msg.author.to_string();
                    let bet_recp = parts[1].to_string();
                    if KW.lock().unwrap().can_bet(&bet_recp) && KW.lock().unwrap().hour_check(&bet_init, &bet_recp, bet_amount) {
                        let ticket_no = KW.lock().unwrap().handle_bet_creation(&bet_init, &bet_recp, bet_amount);
                        let _ = msg.channel_id.say(&ctx.http, ticket_no.to_string()).await;
                    }
                }
                if guild_channel.name == "tekken-tracker" && msg.content.starts_with("!winner") && KW.lock().unwrap().is_trusted(&msg.author.to_string()) {
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
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
