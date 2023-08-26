use anyhow::{anyhow, Error};

pub fn join_errors<T, I>(input: I) -> Result<impl Iterator<Item = T>, Error>
where
    I: IntoIterator<Item = Result<T, Error>>,
{
    let mut iter = input.into_iter();

    match iter.any(|r| r.is_err()) {
        false => Ok(iter.filter_map(Result::ok)),
        true => {
            let error_messages = iter.filter_map(Result::err).map(|e| format!("{e}"));

            Err(anyhow!("Errors: {}", error_messages.join(" | ")))
        }
    }
}

pub trait JoinString {
    fn join(self, separator: &str) -> String;
}

impl<S: Iterator<Item = String>> JoinString for S {
    fn join(self, separator: &str) -> String {
        self.fold(String::new(), |acc, segment| {
            if acc.is_empty() {
                segment.to_string()
            } else {
                acc + separator + &segment
            }
        })
    }
}
