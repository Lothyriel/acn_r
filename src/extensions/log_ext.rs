use anyhow::Error;
use log::error;

pub trait LogExt {
    fn log(self);
}

impl LogExt for Vec<Result<(), Error>> {
    fn log(self) {
        let errors: Vec<_> = self.into_iter().filter_map(|f| f.err()).collect();

        for err in errors {
            error!("{:?}", err);
        }
    }
}

impl<T> LogExt for Result<T, Error> {
    fn log(self) {
        match self {
            Ok(_) => return,
            Err(error) => error!("{}", error),
        }
    }
}
