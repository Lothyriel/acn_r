use crate::application::Command;

mod play;
mod playlist;
mod queue;
mod shuffle;
mod skip;
mod stop;

pub fn group() -> Vec<Command> {
    vec![
        play::play(),
        skip::skip(),
        queue::queue(),
        stop::stop(),
        shuffle::shuffle(),
        playlist::playlist(),
    ]
}
