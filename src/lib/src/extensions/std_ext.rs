use anyhow::{anyhow, Error};

pub fn join_errors<T, I>(input: I) -> Result<impl Iterator<Item = T>, Error>
where
    I: IntoIterator<Item = Result<T, Error>>,
{
    let mut iter = input.into_iter();

    match iter.any(|r| r.is_err()) {
        false => Ok(iter.filter_map(Result::ok)),
        true => {
            let error_messages: Vec<_> = iter
                .filter_map(Result::err)
                .map(|e| format!("{e}"))
                .collect();

            Err(anyhow!("Errors: {}", error_messages.join(" | ")))
        }
    }
}
