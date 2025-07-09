#![allow(unused_imports)]

use poise::serenity_prelude as serenity;
use reqwest::Client as HttpClient;
use serenity::prelude::TypeMapKey;
use songbird::SerenityInit;
use std::env::var;
use tracing::*;
use tracing_appender::rolling;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

mod error;
use crate::error::*;
mod audio;

type Context<'a> = poise::Context<'a, Data, Error>;
struct HttpKey;

impl TypeMapKey for HttpKey {
    type Value = HttpClient;
}
pub struct Data {}

#[tokio::main]
async fn main() {
    // 1. LOGGING
    let file_appender = rolling::daily("logs", "bot.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // Set up logging to both file and console
    let file_layer = fmt::layer().with_writer(non_blocking).with_ansi(false);
    let console_layer = fmt::layer();

    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .with(EnvFilter::new("info")) // Only show info, warn, error by default
        .init();

    // 2. BOT COMMAND SETUP
    let mut command_list = vec![];
    command_list.extend(audio::commands());

    let options = poise::FrameworkOptions {
        commands: command_list,
        on_error: |error| Box::pin(on_error(error)),
        pre_command: |ctx| {
            Box::pin(async move {
                tracing::info!(command = %ctx.command().qualified_name, "[START] Command invoked");
            })
        },
        post_command: |ctx| {
            Box::pin(async move {
                tracing::info!(command = %ctx.command().qualified_name, "[END] Command finished");
            })
        },
        command_check: Some(|ctx| {
            Box::pin(async move {
                if ctx.author().id == 123456789 {
                    return Ok(false);
                }
                Ok(true)
            })
        }),
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(async move {
                tracing::info!(event = %event.snake_case_name(), "[HANDLER] Event received");
                Ok(())
            })
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .options(options)
        .build();

    let token = var("DISCORD_TOKEN")
        .expect("Missing `DISCORD_TOKEN` env var, see README for more information.");

    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .register_songbird()
        .type_map_insert::<HttpKey>(HttpClient::new())
        .await
        .expect("Error creating client.");

    tokio::spawn(async move {
        let _ = client
            .start()
            .await
            .map_err(|why| println!("Client ended: {:?}", why));
    });

    let _signal_err = tokio::signal::ctrl_c().await;
    println!("Received Ctrl-C, shutting down.");
}
