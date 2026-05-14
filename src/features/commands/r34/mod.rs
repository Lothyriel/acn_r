use crate::extensions::serenity::Command;

mod hentai;
mod random;
mod spam;

pub fn group() -> Vec<Command> {
    vec![random::random(), hentai::hentai(), spam::spam()]
}
