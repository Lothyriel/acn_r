use crate::{ extensions::serenity_ext::Command, features::commands::misc::{att::att, stats::stats, debug::debug}};

pub fn misc_group<'a>() -> Vec<Command> {
    vec![att(), stats(), debug()]
}
