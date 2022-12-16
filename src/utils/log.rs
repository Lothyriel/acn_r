use std::time::SystemTime;
use serenity::Error;
use chrono::Utc;

pub trait LogErrors {
    fn log(self);
}

impl LogErrors for Result<(), Error> {
    fn log(self) {
        match self {
            Ok(_) => return,
            Err(error) => println!("{} {:?}", error, Utc::now()),
        }
    }
}
