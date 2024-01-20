use anyhow::Error;

pub trait JoinString {
    fn join(self, separator: &str) -> String;
}

impl<S: Iterator<Item = String>> JoinString for S {
    fn join(self, separator: &str) -> String {
        self.fold(String::new(), |acc, segment| {
            if acc.is_empty() {
                segment
            } else {
                acc + separator + &segment
            }
        })
    }
}

pub fn collapse_errors<T, V>(values: V) -> Result<Vec<T>, Error>
where
    V: Iterator<Item = Result<T, Error>>,
{
    let result: Result<_, _> = values.into_iter().collect();

    Ok(result?)
}
