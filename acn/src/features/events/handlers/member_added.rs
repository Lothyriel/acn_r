use anyhow::Error;
use lib::extensions::serenity::guild_ext::GuildExt;
use poise::serenity_prelude::{Context, Member, Mentionable};

pub async fn handler(ctx: &Context, new_member: &Member) -> Result<(), Error> {
    let response = format!("Novo random detectado: {}", new_member.mention());

    new_member
        .guild_id
        .say_on_main_text_channel(&ctx.http, &response)
        .await?;

    Ok(())
}
