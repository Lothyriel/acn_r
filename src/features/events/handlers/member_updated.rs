use anyhow::Error;
use serenity::model::prelude::Member;

use crate::application::{
    models::dto::user_services::UpdateNickDto,
    services::dependency_configuration::DependencyContainer,
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
