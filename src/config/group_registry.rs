use serenity::framework::StandardFramework;

#[path = "../commands/misc/misc.rs"]
mod misc;

pub trait FrameworkExtensions {
    fn register_groups(self) -> StandardFramework;
}

impl FrameworkExtensions for StandardFramework {
    fn register_groups(self) -> StandardFramework {
        self.group(&misc::MISC_GROUP)
    }
}
