use anyhow::Result;
use poise::serenity_prelude::GuildMemberUpdateEvent;

use crate::application::{
    dependency_configuration::DependencyContainer, models::dto::user::UpdateNickDto,
};

pub async fn handler(new: &GuildMemberUpdateEvent, data: &DependencyContainer) -> Result<()> {
    let user_repository = &data.repositories.user;

    let dto = UpdateNickDto {
        user_id: new.user.id.get(),
        guild_id: Some(new.guild_id.get()),
        new_nickname: new.nick.as_ref().unwrap_or_else(|| &new.user.name).clone(),
        date: chrono::Utc::now(),
    };

    user_repository.update_nickname(dto).await?;

    Ok(())
}
