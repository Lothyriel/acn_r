use crate::extensions::serenity::serenity_structs::Command;

mod play;
mod queue;
mod skip;
mod stop;

pub fn jukebox_group<'a>() -> Vec<Command> {
    vec![play::play(), skip::skip(), queue::queue(), stop::stop()]
}
