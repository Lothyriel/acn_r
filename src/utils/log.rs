use log::error;
use serenity::{async_trait, Error};
use std::{future::Future};

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

#[async_trait]
pub trait FutureExt {
    async fn log<F>(self)
    where
        F: Future<Output = Result<(), Error>>;
}

// #[async_trait]
// impl FutureExt for Box<dyn Future<Output = u32>> {
//     async fn log<F>(self)
//     where
//         F: Future<Output = Result<(), Error>>,
//     {
//         match self.await {
//             Ok(_) => return,
//             Err(error) => error!("Handler: {} Error: {}", std::any::type_name::<F>(), error),
//         }
//     }
// }

pub async fn log<F>(function: F)
where
    F: Future<Output = Result<(), Error>>,
{
    match function.await {
        Ok(_) => return,
        Err(error) => error!("Handler: {} Error: {}", std::any::type_name::<F>(), error),
    }
}