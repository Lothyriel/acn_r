use anyhow::Result;
use poise::serenity_prelude::Member;

use crate::application::{
    dependency_configuration::DependencyContainer, models::dto::user::UpdateNickDto,
};

pub async fn handler(new: &Member, data: &DependencyContainer) -> Result<()> {
    let user_repository = &data.repositories.user;

    let dto = UpdateNickDto {
        user_id: new.user.id.get(),
        guild_id: Some(new.guild_id.get()),
        new_nickname: new.display_name().to_string(),
        date: chrono::Utc::now(),
    };

    user_repository.update_nickname(dto).await?;

    Ok(())
}
