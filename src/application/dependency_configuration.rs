use anyhow::Result;
use mongodb::Database;
use poise::serenity_prelude::UserId;
use reqwest::Client;

use super::{
    infra::{
        appsettings::{AppSettings, MongoSettings},
        audio::manager::AudioManager,
        mongo_client::create_mongo_client,
    },
    repositories::{
        command::CommandRepository, guild::GuildRepository, jukebox::JukeboxRepository,
        stats::StatsRepository, user::UserRepository,
    },
};

pub struct DependencyContainer {
    pub services: ServicesContainer,
    pub repositories: RepositoriesContainer,
}

impl DependencyContainer {
    pub async fn build(settings: &AppSettings, id: UserId) -> Result<Self> {
        let repositories = RepositoriesContainer::build(&settings).await?;

        let services = ServicesContainer::build(settings, id);

        Ok(Self {
            services,
            repositories,
        })
    }
}

pub struct ServicesContainer {
    pub bot_id: UserId,
    pub allowed_ids: Vec<UserId>,
    pub http_client: Client,
    pub audio_manager: AudioManager,
}

impl ServicesContainer {
    fn build(settings: &AppSettings, bot_id: UserId) -> Self {
        let audio_manager = AudioManager::new();

        audio_manager.start();

        Self {
            http_client: Client::new(),
            bot_id,
            allowed_ids: settings.allowed_ids,
            audio_manager,
        }
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
    pub async fn build(settings: &AppSettings) -> Result<Self> {
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

    pub async fn database(settings: &MongoSettings) -> Result<Database> {
        Ok(create_mongo_client(settings).await?.database("acn_r"))
    }
}
