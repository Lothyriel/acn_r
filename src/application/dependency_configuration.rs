use anyhow::Result;
use lavalink_rs::client::LavalinkClient;
use mongodb::Database;
use poise::serenity_prelude::UserId;
use reqwest::Client;

use super::{
    infra::appsettings::{create_mongo_client, AppSettings},
    repositories::{
        command::CommandRepository, guild::GuildRepository, jukebox::JukeboxRepository,
        stats::StatsRepository, user::UserRepository,
    },
};

pub struct DependencyContainer {
    pub bot_id: UserId,
    pub allowed_ids: Vec<UserId>,
    pub repositories: RepositoriesContainer,
    pub http_client: Client,
    pub lavalink_client: LavalinkClient,
}

impl DependencyContainer {
    pub async fn build(
        settings: AppSettings,
        lavalink_client: LavalinkClient,
        id: UserId,
    ) -> Result<Self> {
        let repositories = RepositoriesContainer::build().await?;

        Ok(Self {
            repositories,
            bot_id: id,
            allowed_ids: settings.allowed_ids.to_owned(),
            lavalink_client,
            http_client: Client::new(),
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
    pub async fn build() -> Result<Self> {
        let db = create_mongo_client().await?.database("acn_r");

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
}
