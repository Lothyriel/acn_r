use crate::extensions::serenity::Command;

mod get_voice;

pub fn group() -> Vec<Command> {
    vec![get_voice::get_voice()]
}
