use mongodb::{Collection, Database};

use crate::application::models::entities::reaction::Reaction;

#[derive(Clone)]
pub struct ReactionServices {
    reactions: Collection<Reaction>
}

impl ReactionServices {
    pub fn new(db: &Database) -> Self {
        Self { reactions: db.collection("Reactions") }
    }

    pub fn add_reaction(file: Reaction) {

    }
}