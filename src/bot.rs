use serenity::{
    async_trait
};

pub struct Bot{
    id_pirocudo: u64,
    id_mito: u64,
}

#[async_trait]
impl EventHandler for Bot {

}

impl Default for Bot{
    fn default() -> Self {
        Self { id_pirocudo: 244922703667003392, id_mito: 892942296566358066 }
    }
}