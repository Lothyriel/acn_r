use crate::extensions::serenity::serenity_structs::Command;

mod play;
mod queue;
mod shuffle;
mod skip;
mod stop;
mod playlist;

pub fn jukebox_group<'a>() -> Vec<Command> {
    vec![
        play::play(),
        skip::skip(),
        queue::queue(),
        stop::stop(),
        shuffle::shuffle(),
        playlist::playlist(),
    ]
}
