use anyhow::Result;
use poise::serenity_prelude::{Context, GuildId, Member, User};

use crate::extensions::serenity::guild_ext::GuildExt;

pub async fn handler(
    ctx: &Context,
    id: &GuildId,
    user: &User,
    member: &Option<Member>,
) -> Result<()> {
    let name = member
        .as_ref()
        .map(|m| m.display_name().to_string())
        .unwrap_or_else(|| user.name.to_owned());

    let msg = format!("Eis que o mano '{}' foi de base", name);
    id.say_on_main_text_channel(&ctx.http, &msg).await?;

    Ok(())
}
