use anyhow::Error;
use mongodb::{Collection, Database};
use serenity::prelude::TypeMapKey;

use crate::application::{
    models::{
        dto::{command_dto::CommandUseDto, user_services::AddUserDto},
        entities::command_use::CommandUse,
    },
    services::mongo::user_services::UserServices,
};

impl TypeMapKey for CommandServices {
    type Value = CommandServices;
}

#[derive(Clone)]
pub struct CommandServices {
    commands_use: Collection<CommandUse>,
    user_services: UserServices,
}

impl CommandServices {
    pub fn new(database: &Database, user_services: UserServices) -> Self {
        Self {
            user_services,
            commands_use: database.collection("CommandsUse"),
        }
    }

    pub async fn add_command_use(&self, command_use_dto: CommandUseDto) -> Result<(), Error> {
        let add = AddUserDto {
            guild_id: command_use_dto.guild_id,
            user_id: command_use_dto.user_id,
            nickname: command_use_dto.user_nickname,
            guild_name: command_use_dto.guild_name,
            date: command_use_dto.date,
        };

        self.user_services.add_user(add).await?;

        let command_use = CommandUse {
            guild_id: command_use_dto.guild_id,
            user_id: command_use_dto.user_id,
            date: command_use_dto.date,
            name: command_use_dto.command,
            args: command_use_dto.args,
        };

        self.commands_use.insert_one(command_use, None).await?;

        Ok(())
    }
}
