// title info
// userdata/{user_id}/config/librarycache/{game_id}.json

// gamerecordings
// userdata/{user_id}/gamerecordings/clips/clip_{game_id}_{YYYYMMDD}_{HHmmSS}/video/

pub mod clip_info;
pub mod exporter;
mod librarycache;
mod proto;
mod steam_root;

pub use clip_info::ClipInfo;
pub use steam_root::ClipPath;
pub use steam_root::GameRecordingsRoots;
pub use steam_root::SteamRoot;
