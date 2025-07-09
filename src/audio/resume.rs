use crate::{Context, Error};
use songbird::get;

#[poise::command(slash_command, prefix_command, guild_only, category = "Music")]
pub async fn resume(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx
        .guild_id()
        .ok_or("Command can only be used in a server")?;
    let manager = get(ctx.serenity_context())
        .await
        .ok_or("Songbird Voice Client should be initialized.")?;

    let handler_lock = manager.get(guild_id).ok_or("Not in a voice channel")?;
    let mut handler = handler_lock.lock().await;

    if handler.queue().is_empty() {
        ctx.reply("Nothing is currently paused.").await?;
        return Ok(());
    }

    match handler.queue().resume() {
        Ok(()) => {
            ctx.reply("Resumed playback!").await?;
        }
        Err(_) => {
            ctx.reply("Nothing was paused.").await?;
        }
    }

    Ok(())
}
