use crate::{Context, Error, HttpKey};
use ::serenity::all::{CreateButton, CreateSelectMenuKind};
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::ButtonStyle;
use tracing::{error, info};
#[poise::command(slash_command, prefix_command)]
pub async fn button(
    ctx: Context<'_>,
    #[description = "Search or URL"] query: String,
) -> Result<(), Error> {
    use serenity::builder::{CreateActionRow, CreateSelectMenu, CreateSelectMenuOption};

    let button1 = CreateButton::new("button1")
        .label("Option 1")
        .style(ButtonStyle::Primary);
    let button2 = CreateButton::new("button2")
        .label("Option 2")
        .style(ButtonStyle::Secondary);

    let action_row = CreateActionRow::Buttons(vec![button1, button2]);

    ctx.send(
        poise::CreateReply::default()
            .content("Pick a video to play:")
            .components(vec![action_row]),
    )
    .await?;
    Ok(())
}
