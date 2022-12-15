use serenity::framework::StandardFramework;

pub trait FrameworkExtensions {
    fn RegisterGroups(self) -> StandardFramework;
}

impl FrameworkExtensions for StandardFramework {
    fn RegisterGroups(self) -> StandardFramework {
        self.group(&misc::MISC_GROUP)
    }
}
