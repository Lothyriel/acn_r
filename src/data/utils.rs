use serenity::model::user::User;

const ID_PIROCUDO: u64 = 244922703667003392;
const ID_MITO: u64 = 892942296566358066;

pub async fn eh_mito(user: &User) -> bool {
    let id = user.id;
    id == ID_MITO || id == ID_PIROCUDO
}
