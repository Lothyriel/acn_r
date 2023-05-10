use crate::{
    extensions::serenity_ext::Command,
    features::commands::misc::{att::att, debug::debug, stats::stats},
};

pub fn misc_group<'a>() -> Vec<Command> {
    vec![att(), stats(), debug()]
}
