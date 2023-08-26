use anyhow::Error;
use log::error;
pub trait LogExt {
    fn log(self);
}

impl LogExt for Result<(), Error> {
    fn log(self) {
        match self {
            Ok(_) => (),
            Err(error) => error!("{}", error),
        }
    }
}
