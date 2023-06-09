use crate::{
    extensions::serenity::serenity_structs::Command, features::commands::jukebox::play::play,
};

pub fn jukebox_group<'a>() -> Vec<Command> {
    vec![play()]
}
