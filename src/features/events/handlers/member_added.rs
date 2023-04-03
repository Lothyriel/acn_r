use anyhow::Error;
use serenity::{
    model::prelude::Member,
    prelude::{Context, Mentionable},
};

use crate::extensions::guild_ext::GuildExt;

pub async fn handler(ctx: Context, new_member: Member) -> Result<(), Error> {
    let response = format!("Novo random detectado: {}", new_member.mention());

    new_member
        .guild_id
        .say_on_main_text_channel(&ctx.http, &response)
        .await?;

    Ok(())
}
