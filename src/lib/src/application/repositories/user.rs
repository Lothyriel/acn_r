use anyhow::Error;
use mongodb::{bson::doc, options::FindOneOptions, Collection, Database};

use crate::application::{
    models::{
        dto::user::{UpdateNickDto, UpdateUserDto},
        entities::{
            nickname::NicknameChange,
            user::{Signature, User},
        },
    },
    repositories::guild::GuildRepository,
};

#[derive(Clone)]
pub struct UserRepository {
    users: Collection<User>,
    signatures: Collection<Signature>,
    nickname_changes: Collection<NicknameChange>,
    guild_repository: GuildRepository,
}

impl UserRepository {
    pub fn new(database: &Database, guild_repository: GuildRepository) -> Self {
        Self {
            guild_repository,
            users: database.collection("Users"),
            signatures: database.collection("Signatures"),
            nickname_changes: database.collection("NicknameChanges"),
        }
    }

    pub async fn get_last_signature(&self, user_id: u64) -> Result<Option<Signature>, Error> {
        let filter = doc! {"user_id": user_id as i64};

        let options = FindOneOptions::builder().sort(doc! { "date": -1 }).build();

        let user = self.signatures.find_one(filter, options).await?;

        Ok(user)
    }

    pub async fn add_signature(&self, signature: Signature) -> Result<(), Error> {
        self.signatures.insert_one(signature, None).await?;

        Ok(())
    }

    pub async fn get_last_name(&self, user_id: u64) -> Result<Option<String>, Error> {
        let filter = doc! {"user_id": user_id as i64};
        let options = FindOneOptions::builder().sort(doc! { "date": -1 }).build();

        let possible_last_change = self.nickname_changes.find_one(filter, options).await?;

        Ok(possible_last_change.map(|n| n.nickname))
    }

    pub async fn update_user(&self, update_user_dto: UpdateUserDto) -> Result<(), Error> {
        if let Some(guild_info) = &update_user_dto.guild_info {
            self.guild_repository
                .add_guild(
                    guild_info.guild_id,
                    guild_info.guild_name.to_owned(),
                    update_user_dto.date,
                )
                .await?;

            let update_dto = UpdateNickDto {
                user_id: update_user_dto.user_id,
                guild_id: Some(guild_info.guild_id),
                new_nickname: update_user_dto.nickname,
                date: update_user_dto.date,
            };

            self.update_nickname(update_dto).await?;
        }

        if self.user_exists(update_user_dto.user_id).await? {
            return Ok(());
        }

        let user = User {
            id: update_user_dto.user_id,
        };

        self.users.insert_one(user, None).await?;
        Ok(())
    }

    pub async fn update_nickname(&self, update_dto: UpdateNickDto) -> Result<(), Error> {
        if let Some(last_name) = self.get_last_name(update_dto.user_id).await? {
            if last_name == update_dto.new_nickname {
                return Ok(());
            }
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

    async fn user_exists(&self, guild_id: u64) -> Result<bool, Error> {
        Ok(self.get_user(guild_id).await?.is_some())
    }

    async fn get_user(&self, id: u64) -> Result<Option<User>, Error> {
        let doc = doc! {"id": id as i64};
        Ok(self.users.find_one(doc, None).await?)
    }
}
