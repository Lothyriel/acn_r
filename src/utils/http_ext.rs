use serenity::http::{Http, request::Request};

pub trait HttpExt {
    fn get_all_guilds(&self);
}

impl HttpExt for Http{
    fn get_all_guilds(&self) {
        self.request(req);
    }
}