use anyhow::Error;
use lavalink_rs::LavalinkClient;
use mongodb::Database;
use poise::serenity_prelude::UserId;
use songbird::Songbird;
use std::sync::Arc;

use crate::application::{
    infra::{
        appsettings::{AppSettings, MongoSettings},
        lavalink_ctx,
        mongo_client::create_mongo_client,
    },
    repositories::{
        command::CommandRepository, guild::GuildRepository, jukebox::JukeboxRepository,
        user::UserRepository,
    },
};

use super::repositories::stats::StatsRepository;

pub struct DependencyContainer {
    pub services: ServicesContainer,
    pub repositories: RepositoriesContainer,
}

impl DependencyContainer {
    pub async fn build(
        settings: AppSettings,
        songbird: Arc<Songbird>,
        id: UserId,
    ) -> Result<Self, Error> {
        let repositories = RepositoriesContainer::build(&settings).await?;

        let services = ServicesContainer::build(settings, songbird, id).await?;

        Ok(Self {
            services,
            repositories,
        })
    }
}

pub struct ServicesContainer {
    pub bot_id: UserId,
    pub allowed_ids: Vec<u64>,
    pub lava_client: LavalinkClient,
    pub songbird: Arc<Songbird>,
}

impl ServicesContainer {
    async fn build(
        settings: AppSettings,
        songbird: Arc<Songbird>,
        bot_id: UserId,
    ) -> Result<Self, Error> {
        let lava_client = lavalink_ctx::get_lavalink_client(&settings, songbird.to_owned()).await?;

        Ok(Self {
            lava_client,
            bot_id,
            allowed_ids: settings.allowed_ids,
            songbird,
        })
    }
}

pub struct RepositoriesContainer {
    pub user: UserRepository,
    pub command: CommandRepository,
    pub guild: GuildRepository,
    pub jukebox: JukeboxRepository,
    pub stats: StatsRepository,
}

impl RepositoriesContainer {
    pub async fn build(settings: &AppSettings) -> Result<Self, Error> {
        let db = Self::database(&settings.mongo_settings).await?;

        Ok(Self::build_with_db(db))
    }

    pub fn build_with_db(db: Database) -> Self {
        let guild = GuildRepository::new(&db);

        let user = UserRepository::new(&db, guild.to_owned());

        let command = CommandRepository::new(&db, user.to_owned());

        let stats = StatsRepository::new(&db);

        let jukebox = JukeboxRepository::new(&db);

        Self {
            user,
            guild,
            command,
            stats,
            jukebox,
        }
    }

    pub async fn database(settings: &MongoSettings) -> Result<Database, Error> {
        Ok(create_mongo_client(settings).await?.database("acn_r"))
    }
}
