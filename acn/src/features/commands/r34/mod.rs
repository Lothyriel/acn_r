use crate::application::Command;

mod random;

pub fn group() -> Vec<Command> {
    vec![random::random()]
}
