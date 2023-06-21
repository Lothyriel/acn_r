use crate::extensions::serenity::serenity_structs::Command;

mod random;

pub fn group() -> Vec<Command> {
    vec![random::random()]
}
