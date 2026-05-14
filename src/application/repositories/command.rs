use anyhow::Result;
use sqlx::{Pool, Sqlite};

use crate::application::{
    infra::sqlite::{encode_datetime, encode_id, encode_optional_id},
    models::{
        dto::{command_use::CommandUseDto, user::UserActivityDto},
        entities::command::{CommandError, CommandUse},
    },
    repositories::user::UserRepository,
};

#[derive(Clone)]
pub struct CommandRepository {
    db: Pool<Sqlite>,
    user_repository: UserRepository,
}

impl CommandRepository {
    pub fn new(database: &Pool<Sqlite>, user_repository: UserRepository) -> Self {
        Self {
            db: database.clone(),
            user_repository,
        }
    }

    pub async fn add_command_use(&self, command_use_dto: CommandUseDto) -> Result<()> {
        let command_use = CommandUse {
            guild_id: command_use_dto.guild_info.as_ref().map(|g| g.guild_id),
            user_id: command_use_dto.user_id,
            date: command_use_dto.date,
            name: command_use_dto.command,
            args: command_use_dto.args,
        };

        sqlx::query(
            "INSERT INTO command_uses (guild_id, user_id, date, name, args) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(encode_optional_id(command_use.guild_id)?)
        .bind(encode_id(command_use.user_id)?)
        .bind(encode_datetime(command_use.date))
        .bind(command_use.name)
        .bind(command_use.args)
        .execute(&self.db)
        .await?;

        let add = UserActivityDto {
            guild_info: command_use_dto.guild_info,
            user_id: command_use_dto.user_id,
            nickname: command_use_dto.user_nickname,
            date: command_use_dto.date,
            activity: None,
        };

        self.user_repository.update_user(&add).await?;

        Ok(())
    }

    pub async fn add_command_error(&self, dto: CommandUseDto, error: &str) -> Result<()> {
        let command_error = CommandError {
            guild_id: dto.guild_info.map(|g| g.guild_id),
            user_id: dto.user_id,
            date: dto.date,
            name: dto.command,
            args: dto.args,
            error: error.to_owned(),
        };

        sqlx::query(
            "INSERT INTO command_errors (guild_id, user_id, date, name, args, error) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(encode_optional_id(command_error.guild_id)?)
        .bind(encode_id(command_error.user_id)?)
        .bind(encode_datetime(command_error.date))
        .bind(command_error.name)
        .bind(command_error.args)
        .bind(command_error.error)
        .execute(&self.db)
        .await?;

        Ok(())
    }
}
