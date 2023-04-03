use serenity::prelude::TypeMapKey;

pub struct AllowedIds;

impl TypeMapKey for AllowedIds {
    type Value = Vec<u64>;
}