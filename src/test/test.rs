use crate::{Context, Error};
#[poise::command(slash_command, prefix_command)]
pub async fn test(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    ctx.say("Saying hi").await?;
    ctx.reply("Replying hi").await?;
    ctx.send(
        poise::CreateReply::default()
            .content("Personal response")
            .ephemeral(true),
    )
    .await?;

    Ok(())
}

/*
In poise (a Rust Discord bot framework), command handlers typically return Result<(), Error> (or similar), and you interact with Discord by calling methods on the context (`ctx`). Here are the main response methods you can use:

1. ctx.say("message").await
   - Sends a simple message to the channel where the command was invoked.

2. ctx.reply("message").await
   - Replies directly to the user's message (threaded reply in Discord).

3. ctx.send(builder).await
   - Sends a message with more customization (embeds, files, components, etc.) using a poise::CreateReply builder.

4. ctx.defer().await
   - Defers the response, useful for long-running commands to avoid Discord's timeout.

You always return Ok(()) (or an appropriate Result) from the command function. The return value is not the message itself, but an indication of success or error. The actual message is sent via the ctx methods above.

Summary of each:
- say: Simple message to channel.
- reply: Threaded reply to user.
- send: Fully customizable message (embeds, files, etc.).
- defer: Tells Discord you’ll respond later.
- send_ephemeral: Private message to user.
- send_persistent: Message that persists after interaction.

You can chain or combine these as needed, but you must always return Ok(()) or an error from your command function.
*/
