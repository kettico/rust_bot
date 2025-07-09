use crate::{Context, Error, HttpKey};
use ::serenity::all::CreateSelectMenuKind;
use poise::serenity_prelude as serenity;
use tracing::{error, info};

#[poise::command(slash_command, prefix_command)]
pub async fn select(
    ctx: Context<'_>,
    #[description = "Search or URL"] query: String,
) -> Result<(), Error> {
    use serenity::builder::{CreateActionRow, CreateSelectMenu, CreateSelectMenuOption};

    let options = vec![
        CreateSelectMenuOption::new("Option1", "video1").description("Descrition 1"),
        CreateSelectMenuOption::new("Option2", "video2").description("Descrition 2"),
    ];

    let select_menu =
        CreateSelectMenu::new("select_menu", CreateSelectMenuKind::String { options });

    let action_row = CreateActionRow::SelectMenu(select_menu);
    ctx.reply("Select Command").await?;

    ctx.send(
        poise::CreateReply::default()
            .content("Pick a video to play:")
            .components(vec![action_row]),
    )
    .await?;
    Ok(())
}
