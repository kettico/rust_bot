use crate::{Context, Error, HttpKey};
use serenity::all::ChannelId;
use serenity::futures::channel;
use songbird::input::Compose;
use songbird::{get, input::YoutubeDl};

use poise::CreateReply;
use poise::serenity_prelude::CreateEmbed;
use tracing::{error, info};

/// Play a song from YouTube or another source. Provide a search term or a direct URL to a song.
#[poise::command(slash_command, prefix_command, guild_only, category = "Music")]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Search or URL"] query: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    /* SECTION -- GET VARS */

    /* !SECTION */

    // SECTION -- Join Voice
    if let Err(e) = join_vc(ctx).await {
        error!("{:?}", e);
        ctx.reply(format!("Error: {e}")).await?;
        return Ok(());
    }
    /* !SECTION */

    /* SECTION -- Search for songs */
    if let Err(e) = search_song(ctx, query).await {
        error!("{:?}", e);
        ctx.reply(format!("Error: {e}")).await?;
        return Ok(());
    }

    /* !SECTION */
    Ok(())
}

async fn join_vc(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx
        .guild_id()
        .ok_or("Command can only be used in a server")?;

    let author_id = ctx.author().id;

    // Find the user's current voice channel
    let channel_id = ctx
        .serenity_context()
        .cache
        .guild(guild_id)
        .and_then(|guild| {
            guild
                .voice_states
                .get(&author_id)
                .and_then(|vs| vs.channel_id)
        })
        .ok_or("You must be in a voice channel to use this command")?;

    // Get the Songbird manager
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialization");

    // Check if already connected to a voice channel
    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        if handler.current_channel() == Some(channel_id.into()) {
            return Ok(());
        }
    }

    // Join the user's voice channel
    let _handler_lock = manager.join(guild_id, channel_id).await;
    info!(
        "User {} joined voice channel in guild {}",
        author_id, guild_id
    );
    Ok(())
}

async fn search_song(ctx: Context<'_>, query: String) -> Result<(), Error> {
    let http_client = {
        let data = ctx.serenity_context().data.read().await;
        data.get::<HttpKey>()
            .cloned()
            .expect("Guaranteed to exist in the typemap.")
    };

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialization");

    let guild_id = ctx
        .guild_id()
        .ok_or("Command can only be used in a server")?;

    // TODO: Maybe rewrite
    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        let mut src = if query.starts_with("http") {
            YoutubeDl::new(http_client, query.clone())
        } else {
            YoutubeDl::new_search(http_client, query.clone())
        };
        let _ = handler.enqueue(src.clone().into()).await;

        let meta = src.aux_metadata().await;
        let (title, thumbnail) = match &meta {
            Ok(m) => (
                m.title
                    .clone()
                    .unwrap_or_else(|| "Unknown title".to_string()),
                m.thumbnail.clone().unwrap_or_else(|| "".to_string()),
            ),
            Err(e) => {
                error!("Could not fetch song info for '{}': {}", query, e);
                ctx.say("Could not fetch song info").await?;
                return Ok(());
            }
        };
        info!("Now playing: {} (thumbnail: {})", title, thumbnail);
        ctx.send(CreateReply {
            content: Some(format!("Now playing: **{}**", title)),
            embeds: vec![CreateEmbed::new().title(title).thumbnail(thumbnail)],
            ..Default::default()
        })
        .await?;
    }
    Ok(())
}
