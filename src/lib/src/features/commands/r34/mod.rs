use crate::extensions::serenity::Command;

mod random;

pub fn group() -> Vec<Command> {
    vec![random::random()]
}
