use anyhow::{anyhow, Error};

pub trait VecResultExt<T, E> {
    fn partition_results(self) -> (Vec<T>, Vec<E>);
}

impl<T, E> VecResultExt<T, E> for Vec<Result<T, E>> {
    fn partition_results(self) -> (Vec<T>, Vec<E>) {
        let (success, errors): (Vec<_>, Vec<_>) = self.into_iter().partition(Result::is_ok);
        (
            success.into_iter().filter_map(Result::ok).collect(),
            errors.into_iter().filter_map(Result::err).collect(),
        )
    }
}

pub trait VecResultErrorExt<T> {
    fn all_successes(self) -> Result<Vec<T>, Error>;
}

impl<T> VecResultErrorExt<T> for Vec<Result<T, Error>> {
    fn all_successes(self) -> Result<Vec<T>, Error> {
        let (success, errors) = self.partition_results();

        let mut error_message_map = errors.iter().map(|e| format!("{e}"));
        let failed = error_message_map.any(|_| true);

        match failed {
            false => Ok(success),
            true => {
                let error_messages: Vec<_> = error_message_map.collect();
                Err(anyhow!("Errors: {}", error_messages.join(" | ")))
            }
        }
    }
}
