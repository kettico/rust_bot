use crate::{Data, Error};
use poise::Command;
use tracing::info;

mod info;
mod pause;
mod play;
mod queue;
mod resume;
mod skip;
mod stop;

pub fn commands() -> Vec<Command<Data, Error>> {
    let cmds: Vec<Command<Data, Error>> = vec![
        play::play(),
        stop::stop(),
        pause::pause(),
        resume::resume(),
        skip::skip(),
        queue::queue(),
        info::info(),
        help(),
    ];

    println!("[audio] Loaded {} command(s):", cmds.len());
    for cmd in &cmds {
        info!(" - {}", cmd.name);
    }

    cmds
}

#[poise::command(slash_command, prefix_command, guild_only, category = "Music")]
pub async fn help(ctx: crate::Context<'_>) -> Result<(), crate::Error> {
    let mut msg = String::from("Available commands:\n");
    for cmd in commands() {
        let desc = cmd.description.unwrap_or("No description".to_string());
        msg.push_str(&format!("**{}**: {}\n", cmd.name, desc));
    }
    ctx.reply(msg).await?;
    Ok(())
}
