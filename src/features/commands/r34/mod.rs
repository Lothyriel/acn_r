use crate::extensions::serenity::serenity_structs::Command;

mod random;

pub fn r34_group() -> Vec<Command> {
    vec![random::random()]
}
