use crate::{
    extensions::serenity::serenity_structs::Command, features::commands::r34::random::random,
};

pub fn r34_group() -> Vec<Command> {
    vec![random()]
}
