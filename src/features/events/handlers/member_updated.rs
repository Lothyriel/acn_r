use anyhow::Error;
use serenity::{model::prelude::Member, prelude::Context};

use crate::{
    application::{services::mongo::user_services::UserServices, models::dto::user_services::UpdateNickDto},
    extensions::dependency_ext::Dependencies,
};

pub async fn handler(ctx: Context, new: Member) -> Result<(), Error> {
    let now = chrono::Utc::now();
    let user_services = ctx.get_dependency::<UserServices>().await?;

    let dto = UpdateNickDto{
        user_id: new.user.id.0,
        guild_id: Some(new.guild_id.0),
        new_nickname: new.display_name().to_string(),
        date: now
    };
    
    user_services.update_nickname(dto).await?;

    Ok(())
}
