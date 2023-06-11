use anyhow::Error;
use poise::serenity_prelude::Member;

use crate::application::{
    dependency_configuration::DependencyContainer, models::dto::user::UpdateNickDto,
};

pub async fn handler(new: &Member, data: &DependencyContainer) -> Result<(), Error> {
    let now = chrono::Utc::now();
    let user_services = &data.user_services;

    let dto = UpdateNickDto {
        user_id: new.user.id.0,
        guild_id: Some(new.guild_id.0),
        new_nickname: new.display_name().to_string(),
        date: now,
    };

    user_services.update_nickname(dto).await?;

    Ok(())
}
