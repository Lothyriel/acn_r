use anyhow::Error;
use log::error;
pub trait LogExt {
    fn log(self);
}

pub trait LogErrorsExt<T> {
    fn log_errors(self) -> Vec<T>;
}

impl<T> LogErrorsExt<T> for Vec<Result<T, Error>> {
    fn log_errors(self) -> Vec<T> {
        let (values, errors): (Vec<_>, Vec<_>) = self.into_iter().partition(|r| r.is_ok());

        let errors = errors.into_iter().filter_map(|e| e.err());

        for err in errors {
            error!("{:?}", err);
        }

        values.into_iter().filter_map(|r| r.ok()).collect()
    }
}

impl<T> LogExt for Result<T, Error> {
    fn log(self) {
        match self {
            Ok(_) => (),
            Err(error) => error!("{error}"),
        }
    }
}
