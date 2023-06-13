use crate::extensions::serenity::serenity_structs::Command;

mod play;
mod queue;
mod skip;

pub fn jukebox_group<'a>() -> Vec<Command> {
    vec![play::play(), skip::skip(), queue::queue()]
}
