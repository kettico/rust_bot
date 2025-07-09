use crate::{Context, Error};
use songbird::get;

#[poise::command(slash_command, prefix_command, guild_only, category = "Music")]
pub async fn info(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx
        .guild_id()
        .ok_or("Command can only be used in a server")?;
    let manager = get(ctx.serenity_context())
        .await
        .ok_or("Songbird Voice Client should be initialized.")?;

    let handler_lock = manager.get(guild_id).ok_or("Not in a voice channel")?;
    let handler = handler_lock.lock().await;

    Ok(())
}
