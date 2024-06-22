use crate::extensions::serenity::Command;

mod play;
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
    ]
}
