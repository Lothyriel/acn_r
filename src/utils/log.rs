use serenity::Error;

pub trait LogErrors {
    fn log(self);
}

impl LogErrors for Result<(), Error> {
    fn log(self) {
        match self {
            Ok(_) => return,
            Err(error) => println!("{}", error),
        }
    }
}
