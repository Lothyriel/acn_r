use anyhow::Error;
use lib::application::models::dto::user::UpdateNickDto;
use poise::serenity_prelude::Member;

use crate::application::{
    dependency_configuration::DependencyContainer,
};

pub async fn handler(new: &Member, data: &DependencyContainer) -> Result<(), Error> {
    let now = chrono::Utc::now();
    let user_repository = &data.repositories.user;

    let dto = UpdateNickDto {
        user_id: new.user.id.0,
        guild_id: Some(new.guild_id.0),
        new_nickname: new.display_name().to_string(),
        date: now,
    };

    user_repository.update_nickname(dto).await?;

    Ok(())
}
