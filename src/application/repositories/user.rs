use std::borrow::Borrow;

use anyhow::Error;
use mongodb::{bson::doc, options::FindOneOptions, Collection, Database};

use crate::application::{
    models::{
        dto::user::{AddUserDto, UpdateActivityDto, UpdateNickDto},
        entities::{nickname::NicknameChange, user::User, user_activity::UserActivity},
    },
    repositories::guild::GuildRepository,
};

#[derive(Clone)]
pub struct UserRepository {
    users: Collection<User>,
    user_activity: Collection<UserActivity>,
    nickname_changes: Collection<NicknameChange>,
    guild_repository: GuildRepository,
}

impl UserRepository {
    pub fn new(database: &Database, guild_repository: GuildRepository) -> Self {
        Self {
            guild_repository,
            users: database.collection("Users"),
            nickname_changes: database.collection("NicknameChanges"),
            user_activity: database.collection("UserActivity"),
        }
    }

    pub async fn get_last_name(&self, user_id: u64) -> Result<Option<String>, Error> {
        let filter = doc! {"user_id": user_id as i64};
        let options = FindOneOptions::builder().sort(doc! { "date": -1 }).build();

        let possible_last_change = self.nickname_changes.find_one(filter, options).await?;

        Ok(possible_last_change.map(|n| n.nickname))
    }

    pub async fn update_user_activity(&self, update_dto: UpdateActivityDto) -> Result<(), Error> {
        self.add_user(update_dto.borrow().into()).await?;
        self.add_activity(update_dto.borrow().into()).await?;

        Ok(())
    }

    pub async fn add_user(&self, add_user_dto: AddUserDto) -> Result<(), Error> {
        if let Some(guild_info) = &add_user_dto.guild_info {
            self.guild_repository
                .add_guild(
                    guild_info.guild_id,
                    guild_info.guild_name.to_owned(),
                    add_user_dto.date,
                )
                .await?;
        }

        let update_dto = UpdateNickDto {
            user_id: add_user_dto.user_id,
            guild_id: add_user_dto.guild_info.map(|g| g.guild_id),
            new_nickname: add_user_dto.nickname,
            date: add_user_dto.date,
        };

        let user_id = update_dto.user_id;
        self.update_nickname(update_dto).await?;

        if self.user_exists(user_id).await? {
            return Ok(());
        }

        let user = User { id: user_id };

        self.users.insert_one(user, None).await?;
        Ok(())
    }

    pub async fn update_nickname(&self, update_dto: UpdateNickDto) -> Result<(), Error> {
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

    pub async fn add_activity(&self, activity: UserActivity) -> Result<(), Error> {
        self.user_activity.insert_one(activity, None).await?;

        Ok(())
    }

    async fn user_exists(&self, guild_id: u64) -> Result<bool, Error> {
        Ok(self.get_user(guild_id).await?.is_some())
    }

    async fn get_user(&self, id: u64) -> Result<Option<User>, Error> {
        let doc = doc! {"id": id as i64};
        Ok(self.users.find_one(doc, None).await?)
    }
}