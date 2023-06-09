mod bot_test;

use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use songbird::SerenityInit;
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!play" {
            let guild_id = msg.guild_id.unwrap();
            let voice_manager = songbird::get(&ctx).await.unwrap().clone();

            let song_url = "URL_TO_LOFI_SONG"; // Replace with the URL of a lofi song

            if let Some(handler_lock) = voice_manager.join(guild_id, msg.author.id).await {
                let mut handler = handler_lock.lock().await;
                let source = match songbird::ffmpeg(song_url).await {
                    Ok(source) => source,
                    Err(why) => {
                        println!("Error sourcing ffmpeg: {:?}", why);
                        return;
                    }
                };
                handler.play_source(source);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Failed to create client");

    let songbird = songbird::SerenityInit::default();
    client.register_songbird(songbird);

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}