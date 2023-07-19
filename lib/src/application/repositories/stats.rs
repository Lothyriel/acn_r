use anyhow::Error;
use mongodb::{Collection, Database};

use crate::application::models::entities::russian_roulette::RussianRoulette;

#[derive(Clone)]
pub struct StatsRepository {
    russian_roulette: Collection<RussianRoulette>,
}

impl StatsRepository {
    pub fn new(database: &Database) -> Self {
        Self {
            russian_roulette: database.collection("RussianRoulette"),
        }
    }

    pub async fn add_russian_roulette(&self, attempt: RussianRoulette) -> Result<(), Error> {
        self.russian_roulette.insert_one(attempt, None).await?;

        Ok(())
    }
}
