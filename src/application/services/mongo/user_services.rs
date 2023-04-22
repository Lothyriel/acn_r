use anyhow::Error;
use mongodb::{bson::doc, options::FindOneOptions, Collection, Database};
use serenity::prelude::TypeMapKey;

use crate::application::{
    models::{
        dto::user_services::{AddUserDto, UpdateActivityDto, UpdateNickDto},
        entities::{nickname::NicknameChange, user::User, user_activity::UserActivity},
    },
    services::mongo::guild_services::GuildServices,
};

impl TypeMapKey for UserServices {
    type Value = UserServices;
}

#[derive(Clone)]
pub struct UserServices {
    users: Collection<User>,
    user_activity: Collection<UserActivity>,
    nickname_changes: Collection<NicknameChange>,
    guild_services: GuildServices,
}

impl UserServices {
    pub fn new(database: &Database, guild_services: GuildServices) -> Self {
        Self {
            guild_services,
            users: database.collection("Users"),
            nickname_changes: database.collection("NicknameChanges"),
            user_activity: database.collection("UserActivity"),
        }
    }

    pub async fn update_user_activity(&self, update_dto: UpdateActivityDto) -> Result<(), Error> {
        self.add_activity(&update_dto).await?;
        self.add_user(update_dto.into()).await?;

        Ok(())
    }

    async fn add_user(&self, add_user_dto: AddUserDto) -> Result<(), Error> {
        self.guild_services
            .add_guild(
                add_user_dto.guild_id,
                add_user_dto.guild_name,
                add_user_dto.date,
            )
            .await?;

        let update_dto = UpdateNickDto {
            user_id: add_user_dto.user_id,
            guild_id: add_user_dto.guild_id,
            new_nickname: add_user_dto.nickname,
            date: add_user_dto.date,
        };

        self.update_nickname(update_dto).await?;

        let user = User {
            id: add_user_dto.user_id,
        };

        self.users.insert_one(user, None).await?;
        Ok(())
    }

    async fn update_nickname(&self, update_dto: UpdateNickDto) -> Result<(), Error> {
        match self.get_last_name(update_dto.user_id).await? {
            Some(last_name) => {
                if last_name == update_dto.new_nickname {
                    return Ok(());
                }
            }
            None => (),
        }

        let nick = NicknameChange {
            guild_id: update_dto.guild_id,
            user_id: update_dto.user_id,
            nickname: update_dto.new_nickname,
            date: update_dto.date,
        };

        self.nickname_changes.insert_one(nick, None).await?;

        Ok(())
    }

    async fn get_last_name(&self, user_id: u64) -> Result<Option<String>, Error> {
        let filter = doc! {"user_id": user_id as i64};
        let options = FindOneOptions::builder().sort(doc! { "date": -1 }).build();

        let possible_last_change = self.nickname_changes.find_one(filter, options).await?;

        Ok(possible_last_change.map(|n| n.nickname))
    }

    async fn add_activity(&self, update_dto: &UpdateActivityDto) -> Result<(), Error> {
        let activity = UserActivity {
            guild_id: update_dto.guild_id,
            user_id: update_dto.user_id,
            date: update_dto.date,
            activity_type: update_dto.activity,
        };

        self.user_activity.insert_one(activity, None).await?;

        Ok(())
    }
}
