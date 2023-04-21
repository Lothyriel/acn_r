use log::info;
use serenity::{model::prelude::Message, prelude::Context};

use crate::{
    application::models::allowed_ids::AllowedIds, extensions::dependency_ext::Dependencies,
};

pub async fn eh_mito(context: &Context, message: &Message) -> bool {
    let user_id = message.author.id.0;
    let allowed_ids = context
        .get_dependency::<AllowedIds>()
        .await
        .expect("Impossível exception");

    info!("O mano: {} tentou usar um método", user_id);
    allowed_ids.into_iter().any(|x| x == user_id)
}
