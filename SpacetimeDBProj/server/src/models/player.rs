use spacetimedb::Identity;

// 玩家資料
#[spacetimedb::table(name = player, public)]
pub struct Player {
    #[primary_key]
    identity: Identity,
    #[unique]
    #[auto_inc]
    id: u32,
    name: String,
}