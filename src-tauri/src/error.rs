use serde::{ser::Serializer, Serialize};
use wz_reader::{node, property};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    // === 核心基础错误 (必须保留) ===
    #[error(transparent)]
    Io(#[from] std::io::Error), // 修复大量 E0277 io 错误

    #[error("node error: {0}")]
    NodeError(#[from] node::Error), // 修复大量 E0277, E0631 node 错误

    #[error("json parse error")]
    JsonParseError(#[from] serde_json::Error),

    // === WZ 解析相关错误 (必须保留) ===
    #[error("image parse error")]
    ImageParseError(#[from] property::png::WzPngParseError), // 修复 handlers/png.rs 错误

    #[error("string parse error")]
    StringParseError(#[from] property::string::WzStringParseError), // 修复 handlers/smap.rs 等错误

    #[error("sound parse error")]
    SoundParseError(#[from] property::sound::WzSoundError), // 新增: 修复 sound 相关错误

    // === 业务逻辑错误 ===
    #[error("init wz failed")]
    InitWzFailed,

    #[error("root wz not yet initialized, please use init command first")]
    NotInitialized,

    #[error("node not found")]
    NodeNotFound,

    #[error("node type mismatch, can only use on {0}")]
    NodeTypeMismatch(&'static str),

    #[error("image sending error")]
    ImageSendError,

    // === 处理过程错误 ===
    #[error("image processing error: {0}")]
    ImageProcessingError(String), // 修复 E0599: 找回丢失的图片处理错误

    #[error("audio processing error: {0}")]
    AudioProcessingError(String), // 新增: 音频转换错误

    // === 移动端错误 ===
    #[cfg(mobile)]
    #[error(transparent)]
    PluginInvoke(#[from] tauri::plugin::mobile::PluginInvokeError),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
