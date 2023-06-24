use crate::extensions::serenity::Command;

mod add_react;
mod list_react;
mod react;

pub fn group() -> Vec<Command> {
    vec![
        react::react(),
        add_react::add_react(),
        list_react::list_react(),
    ]
}
