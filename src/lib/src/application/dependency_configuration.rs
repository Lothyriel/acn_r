use anyhow::Error;
use lavalink_rs::LavalinkClient;
use mongodb::Database;
use poise::serenity_prelude::{Http, UserId};
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
        songbird_listener::VoiceController,
    },
    repositories::{
        command::CommandRepository, guild::GuildRepository, jukebox::JukeboxRepository,
        reaction::ReactionRepository, stats::StatsRepository, user::UserRepository,
        voice::VoiceRepository,
    },
};

pub struct DependencyContainer {
    pub services: ServicesContainer,
    pub repositories: RepositoriesContainer,
}

impl DependencyContainer {
    pub async fn build(
        settings: AppSettings,
        http: Arc<Http>,
        songbird: Arc<Songbird>,
        id: UserId,
    ) -> Result<Self, Error> {
        let repositories = RepositoriesContainer::build(&settings.mongo_settings).await?;

        let services =
            ServicesContainer::build(&repositories, settings, http, songbird, id).await?;

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
    pub voice_controller: Arc<VoiceController>,
}

impl ServicesContainer {
    async fn build(
        repositories: &RepositoriesContainer,
        settings: AppSettings,
        http: Arc<Http>,
        songbird: Arc<Songbird>,
        bot_id: UserId,
    ) -> Result<Self, Error> {
        let http_client = Client::new();

        let lava_client = lavalink_ctx::get_lavalink_client(&settings, songbird).await?;

        let github_client = Arc::new(GithubClient::new(http_client, settings.github_settings));

        let app_configurations = Arc::new(RwLock::new(AppConfigurations::new()));

        let deploy_services = DeployServices::new(github_client, app_configurations.to_owned());

        let voice_controller = Arc::new(VoiceController::new(repositories.voice.to_owned(), http));

        Ok(Self {
            deploy_services,
            lava_client,
            bot_id,
            allowed_ids: settings.allowed_ids,
            app_configurations,
            voice_controller,
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
    pub voice: VoiceRepository,
}

impl RepositoriesContainer {
    pub async fn build(settings: &MongoSettings) -> Result<Self, Error> {
        let db = Self::database(&settings).await?;
        Ok(Self::build_with_db(db))
    }

    pub fn build_with_db(db: Database) -> Self {
        let guild = GuildRepository::new(&db);

        let user = UserRepository::new(&db, guild.to_owned());

        let command = CommandRepository::new(&db, user.to_owned());

        let stats = StatsRepository::new(&db);

        let jukebox = JukeboxRepository::new(&db);

        let reaction = ReactionRepository::new(&db);

        let voice = VoiceRepository::new(&db);

        Self {
            user,
            guild,
            command,
            stats,
            jukebox,
            reaction,
            voice,
        }
    }

    pub async fn database(settings: &MongoSettings) -> Result<Database, Error> {
        Ok(create_mongo_client(settings).await?.database("acn_r"))
    }
}
