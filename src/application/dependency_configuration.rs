use anyhow::Error;
use lavalink_rs::LavalinkClient;
use mongodb::Database;
use poise::serenity_prelude::UserId;
use reqwest::Client;
use songbird::Songbird;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::application::{
    infra::{
        appsettings::{AppConfigurations, AppSettings, MongoSettings},
        deploy_service::DeployServices,
        http_clients::github_client::GithubClient,
        lavalink_ctx,
        mongo_client::create_mongo_client,
    },
    repositories::{
        command::CommandRepository, guild::GuildRepository, jukebox::JukeboxRepository,
        reaction::ReactionRepository, stats::StatsRepository, user::UserRepository,
    },
};

pub struct DependencyContainer {
    pub services: ServicesContainer,
    pub repositories: RepositoriesContainer,
}

impl DependencyContainer {
    pub async fn build(
        settings: AppSettings,
        songbird: Arc<Songbird>,
        id: UserId,
        deploy_file: &str,
    ) -> Result<Self, Error> {
        let repositories = RepositoriesContainer::build(&settings.mongo_settings).await?;

        let services = ServicesContainer::build(settings, songbird, id, deploy_file).await?;

        Ok(Self {
            services,
            repositories,
        })
    }
}

pub struct ServicesContainer {
    pub bot_id: UserId,
    pub allowed_ids: Vec<u64>,
    pub app_configurations: Arc<RwLock<AppConfigurations>>,
    pub lava_client: LavalinkClient,
    pub deploy_services: DeployServices,
    pub songbird: Arc<Songbird>,
}

impl ServicesContainer {
    async fn build(
        settings: AppSettings,
        songbird: Arc<Songbird>,
        bot_id: UserId,
        deploy_file: &str,
    ) -> Result<Self, Error> {
        let http_client = Client::new();

        let lava_client = lavalink_ctx::get_lavalink_client(&settings, songbird.to_owned()).await?;

        let github_client = Arc::new(GithubClient::new(http_client, settings.github_settings));

        let app_configurations = Arc::new(RwLock::new(Default::default()));

        let deploy_services =
            DeployServices::new(github_client, app_configurations.to_owned(), deploy_file);

        Ok(Self {
            deploy_services,
            lava_client,
            bot_id,
            allowed_ids: settings.allowed_ids,
            app_configurations,
            songbird,
        })
    }
}

pub struct RepositoriesContainer {
    pub user: UserRepository,
    pub command: CommandRepository,
    pub guild: GuildRepository,
    pub stats: StatsRepository,
    pub jukebox: JukeboxRepository,
    pub reaction: ReactionRepository,
}

impl RepositoriesContainer {
    pub async fn build(settings: &MongoSettings) -> Result<Self, Error> {
        let db = Self::database(settings).await?;
        Ok(Self::build_with_db(db))
    }

    pub fn build_with_db(db: Database) -> Self {
        let guild = GuildRepository::new(&db);

        let user = UserRepository::new(&db, guild.to_owned());

        let command = CommandRepository::new(&db, user.to_owned());

        let stats = StatsRepository::new(&db);

        let jukebox = JukeboxRepository::new(&db);

        let reaction = ReactionRepository::new(&db);

        Self {
            user,
            guild,
            command,
            stats,
            jukebox,
            reaction,
        }
    }

    pub async fn database(settings: &MongoSettings) -> Result<Database, Error> {
        Ok(create_mongo_client(settings).await?.database("acn_r"))
    }
}
