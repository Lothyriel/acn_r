use anyhow::Error;
use mongodb::{Collection, Database};

use crate::application::{
    models::{
        dto::{command_use::CommandUseDto, user::AddUserDto},
        entities::command::{CommandError, CommandUse},
    },
    services::user_services::UserServices,
};

#[derive(Clone)]
pub struct CommandServices {
    commands_use: Collection<CommandUse>,
    commands_errors: Collection<CommandError>,
    user_services: UserServices,
}

impl CommandServices {
    pub fn new(database: &Database, user_services: UserServices) -> Self {
        Self {
            user_services,
            commands_use: database.collection("CommandsUse"),
            commands_errors: database.collection("CommandsErrors"),
        }
    }

    pub async fn add_command_use(&self, command_use_dto: CommandUseDto) -> Result<(), Error> {
        let command_use = CommandUse {
            guild_id: command_use_dto.guild_info.as_ref().map(|g| g.guild_id),
            user_id: command_use_dto.user_id,
            date: command_use_dto.date,
            name: command_use_dto.command,
            args: command_use_dto.args,
        };

        self.commands_use.insert_one(command_use, None).await?;

        let add = AddUserDto {
            guild_info: command_use_dto.guild_info,
            user_id: command_use_dto.user_id,
            nickname: command_use_dto.user_nickname,
            date: command_use_dto.date,
        };

        self.user_services.add_user(add).await?;

        Ok(())
    }

    pub async fn add_command_error(&self, dto: CommandUseDto, error: String) -> Result<(), Error> {
        let command_error = CommandError {
            guild_id: dto.guild_info.map(|g| g.guild_id),
            user_id: dto.user_id,
            date: dto.date,
            name: dto.command,
            args: dto.args,
            error,
        };

        self.commands_errors.insert_one(command_error, None).await?;

        Ok(())
    }
}
