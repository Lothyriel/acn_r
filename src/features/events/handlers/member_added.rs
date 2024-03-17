use anyhow::Result;
use poise::serenity_prelude::{Context, Member, Mentionable};

use crate::extensions::serenity::guild_ext::GuildExt;

pub async fn handler(ctx: &Context, new_member: &Member) -> Result<()> {
    let response = format!("Novo random detectado: {}", new_member.mention());

    new_member
        .guild_id
        .say_on_main_text_channel(&ctx.http, &response)
        .await?;

    Ok(())
}
