use chrono::Utc;
use serenity::Error;
use std::future::Future;

pub async fn log<F>(function: F)
where
    F: Future<Output = Result<(), Error>>,
{
    match function.await {
        Ok(_) => return,
        Err(error) => eprintln!(
            "Handler: {} Error: {} {:?}",
            std::any::type_name::<F>(),
            error,
            Utc::now()
        ),
    }
}

type Sexo = Sized + (impl Cu);
type Cu = dyn (Future<Output = Result<(), Error>>);
trait Log<T> {
    fn log(function: Cu);
}

impl Log<Result<(), Error>> for Cu {
    fn log(function: Cu) {
        todo!()
    }
}
