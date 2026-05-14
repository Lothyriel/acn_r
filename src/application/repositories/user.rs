use anyhow::Result;
use sqlx::{Pool, Row, Sqlite};

use crate::application::{
    infra::sqlite::{decode_datetime, encode_datetime, encode_id, encode_optional_id},
    models::{
        dto::user::{UpdateNickDto, UserActivityDto},
        entities::{
            nickname::NicknameChange,
            user::{Signature, User},
        },
    },
    repositories::guild::GuildRepository,
};

#[derive(Clone)]
pub struct UserRepository {
    db: Pool<Sqlite>,
    guild_repository: GuildRepository,
}

impl UserRepository {
    pub fn new(database: &Pool<Sqlite>, guild_repository: GuildRepository) -> Self {
        Self {
            db: database.clone(),
            guild_repository,
        }
    }

    pub async fn get_last_signature(&self, user_id: u64) -> Result<Option<Signature>> {
        let signature = sqlx::query(
            "SELECT emojis, date FROM signatures WHERE user_id = ? ORDER BY date DESC LIMIT 1",
        )
        .bind(encode_id(user_id)?)
        .fetch_optional(&self.db)
        .await?;

        signature
            .map(|row| {
                Ok(Signature {
                    emojis: row.try_get("emojis")?,
                    user_id,
                    date: decode_datetime(&row.try_get::<String, _>("date")?)?,
                })
            })
            .transpose()
    }

    pub async fn add_signature(&self, signature: Signature) -> Result<()> {
        sqlx::query("INSERT INTO signatures (user_id, emojis, date) VALUES (?, ?, ?)")
            .bind(encode_id(signature.user_id)?)
            .bind(signature.emojis)
            .bind(encode_datetime(signature.date))
            .execute(&self.db)
            .await?;

        Ok(())
    }

    pub async fn get_last_name(&self, user_id: u64) -> Result<Option<String>> {
        let possible_last_change = sqlx::query_scalar::<_, String>(
            "SELECT nickname FROM nickname_changes WHERE user_id = ? ORDER BY date DESC LIMIT 1",
        )
        .bind(encode_id(user_id)?)
        .fetch_optional(&self.db)
        .await?;

        Ok(possible_last_change)
    }

    pub async fn update_user(&self, user_activity: &UserActivityDto) -> Result<()> {
        if let Some(guild_info) = &user_activity.guild_info {
            self.guild_repository
                .add_guild(
                    guild_info.guild_id,
                    guild_info.guild_name.as_str(),
                    user_activity.date,
                )
                .await?;

            let update_dto = UpdateNickDto {
                user_id: user_activity.user_id,
                guild_id: Some(guild_info.guild_id),
                new_nickname: user_activity.nickname.to_owned(),
                date: user_activity.date,
            };

            self.update_nickname(update_dto).await?;
        }

        let user = User {
            id: user_activity.user_id,
        };

        sqlx::query("INSERT OR IGNORE INTO users (id) VALUES (?)")
            .bind(encode_id(user.id)?)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    pub async fn update_nickname(&self, update_dto: UpdateNickDto) -> Result<()> {
        if let Some(last_name) = self.get_last_name(update_dto.user_id).await?
            && last_name == update_dto.new_nickname
        {
            return Ok(());
        }

        let nick = NicknameChange {
            guild_id: update_dto.guild_id,
            user_id: update_dto.user_id,
            nickname: update_dto.new_nickname,
            date: update_dto.date,
        };

        sqlx::query(
            "INSERT INTO nickname_changes (user_id, guild_id, nickname, date) VALUES (?, ?, ?, ?)",
        )
        .bind(encode_id(nick.user_id)?)
        .bind(encode_optional_id(nick.guild_id)?)
        .bind(nick.nickname)
        .bind(encode_datetime(nick.date))
        .execute(&self.db)
        .await?;

        Ok(())
    }
}
