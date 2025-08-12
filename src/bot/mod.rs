use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::Client;

pub struct Bot {
    token: String,
}

impl Bot {
    pub fn new(token: String) -> Self {
        Bot { token }
    }

    pub async fn run(&self) {
        // Define a simple event handler
        struct Handler;

        #[serenity::async_trait]
        impl EventHandler for Handler {
            async fn message(&self, ctx: Context, msg: Message) {
                if msg.content == "!ping" {
                    let _ = msg.channel_id.say(&ctx.http, "Pong!").await;
                }
            }
        }

        let mut client = Client::builder(&self.token)
            .event_handler(Handler)
            .await
            .expect("Error creating client");

        if let Err(why) = client.start().await {
            println!("Client error: {:?}", why);
        }
    }
}
