use crate::{models, utils, AppStore, Error, Result}; // 移除 handlers (如果没用到)
use serde::Deserialize; 
use serde_json::{Map, Value}; // 移除 to_string (如果没用到)
use tauri::{command, ipc, AppHandle, Runtime, State, Window};
use wz_reader::{util::node_util, version::WzMapleVersion, WzNodeCast};

// 引入 WebP 编码所需的库
use webp_animation::{Encoder, EncoderOptions};
// 修正：直接使用 image crate，不需要 use image::self
use image; 

#[derive(Deserialize)]
pub struct WebPFrame {
    // Rust 应该接收一个 u8 数组（来自 JS 的 Array<number> 或 Uint8Array）
    pub data: Vec<u8>, 
    pub delay: i32,    // 延迟，单位 ms
}

#[command]
pub(crate) async fn encode_webp_anim<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    frames: Vec<WebPFrame>,
    width: u32,
    height: u32,
) -> Result<Vec<u8>> {
    if frames.is_empty() {
        return Ok(Vec::new());
    }

    // 1. 初始化 WebP 编码器
    let mut options = EncoderOptions::default();
    options.anim_params.loop_count = 0;
    
    // 使用 ImageProcessingError
    let mut encoder = Encoder::new_with_options((width, height), options)
        .map_err(|e| Error::ImageProcessingError(format!("Encoder init failed: {}", e)))?; 

    let mut timestamp_ms = 0;

    for frame in frames {
        // 2. 解码前端传来的 PNG buffer
        let img = image::load_from_memory(&frame.data)
            // 使用 ImageProcessingError
            .map_err(|e| Error::ImageProcessingError(format!("Failed to load image buffer: {}", e)))?;

        let rgba_img = img.to_rgba8(); 

        // 检查尺寸是否匹配
        if img.width() != width || img.height() != height {
             // 对于静态字符串，也封装成 String
             return Err(Error::ImageProcessingError("Frame size mismatch".to_string()));
        }

        // 3. 添加帧
        encoder.add_frame(rgba_img.as_raw(), timestamp_ms)
             // 使用 ImageProcessingError
             .map_err(|e| Error::ImageProcessingError(format!("Add frame failed: {}", e)))?;

        timestamp_ms += frame.delay;
    }

    // 4. 结束并生成最终 WebP 二进制
    let webp_data = encoder.finalize(timestamp_ms)
        // 使用 ImageProcessingError
        .map_err(|e| Error::ImageProcessingError(format!("Finalize failed: {}", e)))?;

    Ok(webp_data.to_vec())
}

#[command]
pub(crate) async fn get_server_url<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    state: State<'_, AppStore>,
) -> Result<Value> {
    let mut map = Map::new();
    let root = state.node.read().unwrap();
    map.insert(
        "url".to_string(),
        Value::String(format!("http://localhost:{}", state.port)),
    );
    map.insert("is_initialized".to_string(), Value::Bool(!root.is_null()));
    map.insert(
        "is_load_items".to_string(),
        Value::Bool(
            root.at("Item")
                .and_then(|e| Some(e.read().unwrap().children.len() > 0))
                == Some(true),
        ),
    );
    let patch_version = root.try_as_file().map(|f| f.wz_file_meta.patch_version);
    map.insert(
        "patch_version".to_string(),
        patch_version.map_or(Value::Null, |v| Value::Number(v.into())),
    );
    Ok(Value::Object(map))
}

#[command]
pub(crate) async fn init<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    state: State<'_, AppStore>,
    path: String,
    version: Option<String>,
) -> Result<i32> {
    let current_root_path = state
        .node
        .read()
        .unwrap()
        .try_as_file()
        .map(|f| f.wz_file_meta.path.clone());

    if let Some(current_root_path) = current_root_path {
        if current_root_path == path {
            return Ok(0);
        }
    }

    let version = version.map(|s| match s.as_str() {
        "GMS" => WzMapleVersion::GMS,
        "EMS" => WzMapleVersion::EMS,
        "BMS" => WzMapleVersion::BMS,
        _ => WzMapleVersion::UNKNOWN,
    });

    let base_node = utils::resolve_base(&path, version)
        .await
        .map_err(|_| crate::Error::InitWzFailed)?;

    state.replace_root(&base_node);

    let version = state
        .node
        .read()
        .unwrap()
        .try_as_file()
        .map(|f| f.wz_file_meta.patch_version)
        .unwrap_or(0);

    Ok(version)
}

#[command]
pub(crate) async fn parse_node<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    state: State<'_, AppStore>,
    path: String,
) -> Result<()> {
    let node_read = state.node.read().unwrap();

    let _ = node_read
        .at_path(&path)
        .map(|n| node_util::parse_node(&n))
        .ok_or(Error::NodeNotFound)?;

    Ok(())
}

#[command]
pub(crate) async fn unparse_node<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    state: State<'_, AppStore>,
    path: String,
) -> Result<()> {
    let node = state.node.read().unwrap();

    node.at_path(&path)
        .map(|n| n.write().unwrap().unparse())
        .ok_or(Error::NodeNotFound)?;

    Ok(())
}

#[command]
pub(crate) async fn get_node_info<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    state: State<'_, AppStore>,
    path: String,
) -> Result<models::NodeInfo> {
    let node = if path.is_empty() {
        state.node.clone()
    } else {
        state
            .node
            .read()
            .unwrap()
            .at_path(&path)
            .ok_or(Error::NodeNotFound)?
    };

    let node_read = node.read().unwrap();

    // 修复 to_string 引用问题，直接用 serde_json::to_string
    Ok(models::NodeInfo {
        name: node_read.name.to_string(),
        _type: serde_json::to_string(&node_read.object_type)?,
        has_child: !node_read.children.is_empty(),
    })
}

#[command]
pub(crate) async fn get_childs_info<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    state: State<'_, AppStore>,
    path: String,
) -> Result<Vec<models::NodeInfo>> {
    let node = if path.is_empty() {
        state.node.clone()
    } else {
        state
            .node
            .read()
            .unwrap()
            .at_path(&path)
            .ok_or(Error::NodeNotFound)?
    };

    let node_read = node.read().unwrap();

    Ok(node_read
        .children
        .values()
        .map(|node| {
            let node_read = node.read().unwrap();
            models::NodeInfo {
                name: node_read.name.to_string(),
                _type: serde_json::to_string(&node_read.object_type).unwrap(),
                has_child: !node_read.children.is_empty(),
            }
        })
        .collect())
}

#[command]
pub(crate) fn encode_webp(request: ipc::Request) -> ipc::Response {
    let ipc::InvokeBody::Raw(webp_data) = request.body() else {
        return ipc::Response::new(vec![]);
    };
    // 依然引用 handlers::webp (旧代码)
    let data = crate::handlers::webp::encode_wep_animation(&webp_data);
    if let Ok(data) = data {
        ipc::Response::new(data.to_vec())
    } else {
        ipc::Response::new(vec![])
    }
}