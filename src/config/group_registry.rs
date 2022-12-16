use serenity::framework::StandardFramework;

use crate::commands::misc;

pub trait FrameworkExtensions {
    fn register_groups(self) -> StandardFramework;
    //fn register_buckets(self) -> StandardFramework;
}

impl FrameworkExtensions for StandardFramework {
    //fn register_buckets(self) -> StandardFramework {
        //self.bucket("pirocudo",|b| b.check()));
    //}

    fn register_groups(self) -> StandardFramework {
        self.group(&misc::MISC_GROUP)
    }
}