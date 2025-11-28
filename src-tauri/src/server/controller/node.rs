use crate::server::extractors::TargetNodeExtractor;
use crate::server::models::GetJsonParam;
use crate::{handlers, utils, Error, Result};

use std::io::{BufWriter, Cursor};

use axum::extract::{Path, Query};
use axum::{body::Body, extract::State, http::header, response::IntoResponse};
use image::ImageFormat;
use wz_reader::util::node_util;
use wz_reader::WzNodeCast;

use super::super::AppState;

pub async fn get_ogg_sound(
    TargetNodeExtractor(node): TargetNodeExtractor,
) -> Result<impl IntoResponse> {
    // 1. 获取原始数据
    let raw_sound_data = if let Some(raw) = node.read().unwrap().try_as_sound() {
        // === 关键修改：移除了对 Binary 类型的强制检查 ===
        // 只要是 Sound 节点，无论内部是 Binary(WAV) 还是 Mp3，我们都取出 buffer
        raw.get_buffer()
    } else {
        return Err(Error::NodeTypeMismatch("WzSound"));
    };

    // 2. 调用通用的转换函数 (支持自动探测 WAV/MP3)
    let ogg_buffer = handlers::audio::convert_audio_to_ogg(&raw_sound_data)?;

    Ok((
        [
            (header::CONTENT_TYPE, "audio/ogg"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        ogg_buffer,
    ))
}

pub async fn get_image(
    State(root): State<AppState>,
    TargetNodeExtractor(node): TargetNodeExtractor,
) -> Result<impl IntoResponse> {
    let image = handlers::resolve_png(&node, Some(&root.0))?;

    let mut buf = BufWriter::new(Cursor::new(Vec::new()));
    // maybe use ImageFormat::Webp is better it quicker and smaller.
    image
        .write_to(&mut buf, ImageFormat::WebP)
        .map_err(|_| Error::ImageSendError)?;

    let body = Body::from(buf.into_inner().unwrap().into_inner());

    Ok((
        [
            (header::CONTENT_TYPE, "image/webp"),
            (header::CACHE_CONTROL, "max-age=3600"),
        ],
        body,
    ))
}

pub async fn get_image_unparsed(
    State(root): State<AppState>,
    Path(path): Path<String>,
) -> Result<impl IntoResponse> {
    let (image_node, path) =
        node_util::get_image_node_from_path(&root.0, &path).ok_or(Error::NodeNotFound)?;

    let image = handlers::resolve_png_unparsed(&image_node, &path, Some(&root.0))?;

    let mut buf = BufWriter::new(Cursor::new(Vec::new()));
    // maybe use ImageFormat::Webp is better it quicker and smaller.
    image
        .write_to(&mut buf, ImageFormat::WebP)
        .map_err(|_| Error::ImageSendError)?;

    let body = Body::from(buf.into_inner().unwrap().into_inner());

    Ok((
        [
            (header::CONTENT_TYPE, "image/webp"),
            (header::CACHE_CONTROL, "max-age=3600"),
        ],
        body,
    ))
}

pub async fn get_raw(TargetNodeExtractor(node): TargetNodeExtractor) -> Result<impl IntoResponse> {
    let buffer: Vec<u8>;

    if let Some(raw) = node.read().unwrap().try_as_raw_data() {
        buffer = raw.get_buffer().to_vec();
    } else if let Some(raw) = node.read().unwrap().try_as_sound() {
        buffer = raw.get_buffer(); 
    } else {
        return Err(Error::NodeTypeMismatch("WzRaw or WzSound"));
    }

    Ok((
        [
            // 建议把这里改成 audio/wav 或者 audio/mp3，这样浏览器更容易识别
            // 但 application/octet-stream 也是勉强可以的
            (header::CONTENT_TYPE, "audio/wav"), 
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        buffer,
    ))
}

pub async fn get_json(
    Query(param): Query<GetJsonParam>,
    TargetNodeExtractor(node): TargetNodeExtractor,
) -> Result<impl IntoResponse> {
    let is_simple = param.simple.unwrap_or(false);
    let json = if is_simple {
        handlers::json::to_simple_json(&node.read().unwrap())
    } else {
        node.read().unwrap().to_json()
    }?;

    Ok((
        [
            (header::CONTENT_TYPE, "application/json"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        json.to_string(),
    ))
}

pub async fn load_extra_paths(
    State(root): State<AppState>,
    Query(param): Query<std::collections::HashMap<String, String>>,
) -> Result<impl IntoResponse> {
    let empty_string = String::new();
    let pathes = param.get("path").unwrap_or(&empty_string);
    let pathes = {
        let root = root.0.read().unwrap();
        pathes
            .split(',')
            .filter(|path| root.children.contains_key(*path))
            .collect::<Vec<&str>>()
    };

    utils::load_wz_by_base(root.0, &pathes, None, None).await?;

    Ok(())
}

pub async fn parse(
    State(root): State<AppState>,
    Path(path): Path<String>,
) -> Result<impl IntoResponse> {
    let node_read = root.0.read().unwrap();

    let _ = node_read
        .at_path(&path)
        .map(|n| node_util::parse_node(&n))
        .ok_or(Error::NodeNotFound)?;

    Ok(())
}

pub async fn unparse(
    State(root): State<AppState>,
    Path(path): Path<String>,
) -> Result<impl IntoResponse> {
    let node = root.0.read().unwrap();

    node.at_path(&path)
        .map(|n| n.write().unwrap().unparse())
        .ok_or(Error::NodeNotFound)?;

    Ok(())
}
