use crate::application::models::entities::reaction::Reaction;

pub struct ReactionDto {
    pub reaction: Reaction,
    pub bytes: Vec<u8>,
}
