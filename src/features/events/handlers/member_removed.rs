use anyhow::Error;
use serenity::{
    model::{
        prelude::{GuildId, Member},
        user::User,
    },
    prelude::Context,
};

use crate::extensions::guild_ext::GuildExt;

pub async fn handler(
    ctx: Context,
    id: GuildId,
    user: User,
    member: Option<Member>,
) -> Result<(), Error> {
    let name = member
        .map(|m| m.display_name().to_string())
        .unwrap_or_else(|| user.name);
    
    let msg = format!("Eis que o mano '{}' foi de base", name);
    id.say_on_main_text_channel(&ctx.http, &msg).await?;

    Ok(())
}
