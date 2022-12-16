use serenity::async_trait;
use serenity::framework::StandardFramework;

use crate::commands::misc;

pub trait FrameworkExtensions {
    fn register_groups(self) -> StandardFramework;
}
#[async_trait]
impl FrameworkExtensions for StandardFramework {
    fn register_groups(self) -> StandardFramework {
        self.group(&misc::MISC_GROUP)
    }
}
