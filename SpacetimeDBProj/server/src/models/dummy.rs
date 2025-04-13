use crate::types::Vector3;

// 玩家可操作的物件
#[spacetimedb::table(name = dummy, public)]
pub struct Dummy {
    #[primary_key]
    entity_id: u32,
    #[index(btree)]
    player_id:u32,
    pos: Vector3,
    vel: Vector3,
}