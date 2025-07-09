use crate::{Context, Error};
use songbird::get;

#[poise::command(slash_command, prefix_command, guild_only, category = "Music")]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx
        .guild_id()
        .ok_or("Command can only be used in a server")?;
    let manager = get(ctx.serenity_context())
        .await
        .ok_or("Songbird Voice Client should be initialized.")?;

    let handler_lock = manager.get(guild_id).ok_or("Not in a voice channel")?;
    let mut handler = handler_lock.lock().await;

    if let Some(track) = handler.queue().current() {
        let res = track.stop();
        if res.is_ok() {
            ctx.reply("Skipped the current track!").await?;
        } else {
            ctx.reply("Failed to skip the track.").await?;
        }
    } else {
        ctx.reply("Nothing is currently playing.").await?;
    }

    Ok(())
}
