use anyhow::Error;
use mongodb::Database;

use crate::application::{
    infra::{appsettings::MongoSettings, mongo_client::create_mongo_client},
    repositories::{
        command::CommandRepository, guild::GuildRepository, jukebox::JukeboxRepository,
        reaction::ReactionRepository, stats::StatsRepository, user::UserRepository,
    },
};

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
        Ok(create_mongo_client(&settings).await?.database("acn_r"))
    }
}
