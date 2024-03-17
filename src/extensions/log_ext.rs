use anyhow::Result;
use log::error;

pub trait LogExt {
    fn log(self);
}

impl LogExt for Result<()> {
    fn log(self) {
        match self {
            Ok(_) => (),
            Err(error) => error!("{}", error),
        }
    }
}
