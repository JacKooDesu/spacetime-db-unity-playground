// World 內的物件
#[spacetimedb::table(name = entity, public)]
pub struct Entity {
    #[primary_key]
    #[unique]
    #[auto_inc]
    id: u32,
    name: String,
}