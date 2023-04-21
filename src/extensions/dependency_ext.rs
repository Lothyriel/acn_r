use anyhow::{anyhow, Error};
use serenity::{
    async_trait,
    prelude::{Context, TypeMapKey},
};

#[async_trait]
pub trait Dependencies {
    async fn get_dependency<T: TypeMapKey>(&self) -> Result<<T as TypeMapKey>::Value, Error>
    where
        T::Value: Clone;
}

#[async_trait]
impl Dependencies for Context {
    async fn get_dependency<T: TypeMapKey>(&self) -> Result<<T as TypeMapKey>::Value, Error>
    where
        T::Value: Clone,
    {
        self.data
            .read()
            .await
            .get::<T>()
            .cloned()
            .ok_or_else(|| anyhow!("Não tem {} cadastrado vei...", std::any::type_name::<T>()))
    }
}
