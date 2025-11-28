mod chair;
mod equip;
mod image_map;
mod item;
pub mod json;
mod map;
mod mount;
mod mount_skill_id;
pub mod path;
mod png;
mod skill;
mod smap;
mod string;
pub mod webp;
mod zmap;
pub mod audio; // <--- 必须添加这行：声明 audio 模块存在 (对应文件 handlers/audio.rs)

pub use chair::*;
pub use equip::*;
pub use image_map::*;
pub use map::*;
pub use mount::*;
pub use png::*;
pub use skill::*;
pub use smap::*;
pub use string::*;
pub use zmap::*;
pub use audio::*; // <--- 然后才能导出