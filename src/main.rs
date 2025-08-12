use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::Client;
use std::env;
use std::fs::File;
use std::io::BufReader;
use csv::Reader;

struct Handler;

fn format_tekken_debtors(csv_path: &str) -> String {
    let file = match File::open(csv_path) {
        Ok(f) => f,
        Err(_) => return "CSV file not found.".to_string(),
    };
    let mut rdr = Reader::from_reader(BufReader::new(file));
    let mut message = String::from("Tekken debtors:\n");

    for result in rdr.records() {
        if let Ok(record) = result {
            // Assuming CSV columns: name, hours
            let name = record.get(0).unwrap_or("Unknown");
            let hours = record.get(1).unwrap_or("0.0").trim().parse::<f32>().unwrap_or(0.0);
            let total_hours = record.get(2).unwrap_or("0.0").trim().parse::<f32>().unwrap_or(0.0);
            if hours < total_hours {
                let hours_left =  total_hours - hours;
                message.push_str(&format!("<@{}> has played {} hours and has {} hours left to go!\n", name, hours, hours_left));
            }
        }
    }

    message
}

#[serenity::async_trait]
impl EventHandler for Handler {

    async fn ready(&self, ctx: Context, _data: Ready) {
        // Replace with your channel ID
        let channel_id = ChannelId::new(1404935148419682304);

        // Read from CSV (example: first row, first column)
        let message = format_tekken_debtors("tekken_hours.csv");
        let _ = channel_id.say(&ctx.http, message).await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        // Fetch the channel object
        if let Ok(channel) = msg.channel_id.to_channel(&ctx).await {
            if let serenity::model::channel::Channel::Guild(guild_channel) = channel {
                if guild_channel.name == "tekken-tracker" && msg.content == "!ping" {
                    let _ = msg.channel_id.say(&ctx.http, "<@451064565963161611>").await;
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
