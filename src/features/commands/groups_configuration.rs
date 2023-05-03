use serenity::framework::StandardFramework;

use crate::{
    extensions::group_registry::FrameworkExtensions,
    features::commands::{misc::misc_group::MISC_GROUP, r34::r34_group::R34_GROUP},
};

impl FrameworkExtensions for StandardFramework {
    fn register_groups(self) -> StandardFramework {
        self.group(&MISC_GROUP).group(&R34_GROUP)
    }
}
