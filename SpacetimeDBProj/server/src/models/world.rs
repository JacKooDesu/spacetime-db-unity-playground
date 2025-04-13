use crate::types::Vector3;

// World 是一個平面
#[spacetimedb::table(name = world)]
pub struct World {
    #[primary_key]
    #[unique]
    #[auto_inc]
    id: u32,
    size: u32,
    center: Vector3,
}