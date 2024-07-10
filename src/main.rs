mod commands;

use std::env;
use serenity::model::gateway::Ready;
use serenity::{async_trait, Client};
use serenity::all::{EventHandler};
use serenity::model::application::{Command, Interaction};
use serenity::client::Context;
use serenity::prelude::GatewayIntents;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing::{debug, error, info, Level, warn};
use serenity::model::id::GuildId;
use tracing_subscriber::EnvFilter;


struct AMECA; // Im not exactly sure if anything is even supposed to go in here later...


#[async_trait]
impl EventHandler for AMECA {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);


        let guild_token =  env::var("GUILD_ID")
            .expect("Expected GUILD_ID in environment")
            .parse()
            .expect("GUILD_ID must be an integer");

        debug!(guild_token);

        let guild_id = GuildId::new(
            guild_token
        );


        let commands = guild_id
            .set_commands(&ctx.http, vec![
                commands::hello::register(),
            ])
            .await;

        debug!("Registering the following commands: {commands:#?}");
    }

    
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("NO .ENV file found");

    let debug_file =
        tracing_appender::rolling::hourly("./logs/", "debug")
            .with_max_level(tracing::Level::INFO);

    let warn_file = tracing_appender::rolling::hourly("./logs/", "warnings")
        .with_max_level(tracing::Level::WARN);
    let all_files = debug_file.and(warn_file);

    tracing_subscriber::registry()
        .with(EnvFilter::from_env("AMECA_LOG_LEVEL"))
        // .with(console_layer)
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(all_files)
                .with_ansi(false),
        )
        .with(
            tracing_subscriber::fmt::Layer::new()
                .with_ansi(true)
                .with_writer(std::io::stdout.with_max_level(Level::DEBUG))
                .with_file(true)
                .with_line_number(true),
        )
        .init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token.");

    let mut client = Client::builder(&token,GatewayIntents::privileged()).event_handler(AMECA).await;

    match client{
        Ok(mut client) => {
            if let Err(why) = client.start().await {
                error!("Client error: {why:?}");
            }
        },
        Err(err) =>{
            error!("{}",format!("Error in creating bot: {:?}",err));
            panic!();
        }
    }


    debug!("Loaded token {}",token);
}
