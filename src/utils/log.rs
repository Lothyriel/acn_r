use chrono::Utc;
use serenity::{Error};
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

// type Sexo = Sized + (impl Cu);

// type Cu = impl (Future<Output = Result<(), Error>>);

// #[async_trait(?Send)]
// trait Log {
//     async fn log(&self);
// }

// #[async_trait(?Send)]
// impl Log for dyn Future<Output = Result<(), Error>> {
//     async fn log(&self) {
//         match self.await {
//             Ok(_) => return,
//             Err(error) => eprintln!(
//                 "Handler: {} Error: {} {:?}",
//                 std::any::type_name::<F>(),
//                 error,
//                 Utc::now()
//             ),
//         }
//     }
// }
