use anyhow::Error;
use log::error;
pub trait LogExt {
    fn log(self);
}

pub trait LogErrorsExt<T> {
    fn log_errors(self) -> LogResult<T>;
}

pub struct LogResult<T> {
    pub successes: Vec<T>,
    pub errors_count: usize,
}

impl<T> LogResult<T> {
    pub fn new(successes: Vec<T>, errors_count: usize) -> Self {
        Self {
            successes,
            errors_count,
        }
    }
}

impl<T> LogErrorsExt<T> for Vec<Result<T, Error>> {
    fn log_errors(self) -> LogResult<T> {
        let (values, errors): (Vec<_>, Vec<_>) = self.into_iter().partition(|r| r.is_ok());

        let errors_count = errors.len();

        let errors = errors.into_iter().filter_map(|e| e.err());

        for err in errors {
            error!("{}", err);
        }

        LogResult::new(
            values.into_iter().filter_map(|r| r.ok()).collect(),
            errors_count,
        )
    }
}

impl<T> LogExt for Result<T, Error> {
    fn log(self) {
        match self {
            Ok(_) => (),
            Err(error) => error!("{}", error),
        }
    }
}
