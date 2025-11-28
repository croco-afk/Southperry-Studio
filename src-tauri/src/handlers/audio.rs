use std::io::Cursor;
use crate::{Error, Result};

// 引入 vorbis_rs 用于编码
use vorbis_rs::{VorbisBitrateManagementStrategy, VorbisEncoderBuilder};

// 引入 symphonia 用于通用解码 (支持 WAV 和 MP3)
use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

/// 将任意音频数据 (WAV/MP3) 转换为 Ogg Vorbis 格式
pub fn convert_audio_to_ogg(input_data: &[u8]) -> Result<Vec<u8>> {
    // ============================
    // 1. 解码阶段 (使用 Symphonia)
    // ============================

    // 创建媒体源流
    let cursor = Cursor::new(input_data.to_vec()); // 复制一份数据的所有权给 cursor
    let mss = MediaSourceStream::new(Box::new(cursor), Default::default());

    // 创建探测提示 (Hint)
    let hint = Hint::new(); // 让它自动探测格式

    // 探测格式
    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &FormatOptions::default(), &MetadataOptions::default())
        .map_err(|e| Error::AudioProcessingError(format!("Probe failed: {}", e)))?;

    let mut format = probed.format;

    // 找到默认音轨
    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .ok_or_else(|| Error::AudioProcessingError("No supported audio track found".to_string()))?;

    // 创建解码器
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &DecoderOptions::default())
        .map_err(|e| Error::AudioProcessingError(format!("Decoder creation failed: {}", e)))?;

    let track_id = track.id;
    let sample_rate = track.codec_params.sample_rate.ok_or_else(|| Error::AudioProcessingError("Unknown sample rate".to_string()))?;
    let channel_count = track.codec_params.channels.ok_or_else(|| Error::AudioProcessingError("Unknown channel count".to_string()))?.count();

    // 准备存储解码后的浮点数样本
    // 结构: Vec<Vec<f32>>，外层是声道，内层是样本
    let mut planar_samples: Vec<Vec<f32>> = vec![Vec::new(); channel_count];

    // 解码循环
    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(symphonia::core::errors::Error::IoError(_)) => break, // 读完了
            Err(symphonia::core::errors::Error::ResetRequired) => continue, // 需要重置
            Err(e) => return Err(Error::AudioProcessingError(format!("Packet decode error: {}", e))),
        };

        if packet.track_id() != track_id {
            continue;
        }

        match decoder.decode(&packet) {
            Ok(decoded) => {
                // 将解码后的 AudioBuffer 转换为 f32 并存入 planar_samples
                copy_buffer_to_planar(&decoded, &mut planar_samples);
            }
            Err(symphonia::core::errors::Error::IoError(_)) => break,
            Err(symphonia::core::errors::Error::DecodeError(_)) => continue, // 忽略损坏的帧
            Err(e) => return Err(Error::AudioProcessingError(format!("Decode failed: {}", e))),
        }
    }

    // ============================
    // 2. 编码阶段 (使用 Vorbis_rs)
    // ============================
    
    // 准备输出缓冲区
    let mut output_buffer = Vec::new();

    let mut encoder = VorbisEncoderBuilder::new(
        std::num::NonZeroU32::new(sample_rate).unwrap(),
        std::num::NonZeroU8::new(channel_count as u8).unwrap(),
        &mut output_buffer,
    )
    .map_err(|e| Error::AudioProcessingError(format!("Vorbis builder failed: {}", e)))?
    .bitrate_management_strategy(VorbisBitrateManagementStrategy::QualityVbr { target_quality: 0.4 })
    .build()
    .map_err(|e| Error::AudioProcessingError(format!("Vorbis init failed: {}", e)))?;

    encoder.encode_audio_block(&planar_samples)
        .map_err(|e| Error::AudioProcessingError(format!("Vorbis encode failed: {}", e)))?;

    encoder.finish()
        .map_err(|e| Error::AudioProcessingError(format!("Vorbis finish failed: {}", e)))?;

    Ok(output_buffer)
}

/// 辅助函数：将不同格式的 AudioBuffer 统一转换为 f32 Planar 格式
fn copy_buffer_to_planar(decoded: &AudioBufferRef, planar: &mut Vec<Vec<f32>>) {
    match decoded {
        AudioBufferRef::F32(buf) => {
            for (ch, channel_samples) in planar.iter_mut().enumerate() {
                channel_samples.extend_from_slice(buf.chan(ch));
            }
        }
        AudioBufferRef::U8(buf) => {
            for (ch, channel_samples) in planar.iter_mut().enumerate() {
                for &sample in buf.chan(ch) {
                    channel_samples.push((sample as f32 - 128.0) / 128.0);
                }
            }
        }
        AudioBufferRef::S16(buf) => {
            for (ch, channel_samples) in planar.iter_mut().enumerate() {
                for &sample in buf.chan(ch) {
                    channel_samples.push(sample as f32 / 32768.0);
                }
            }
        }
        AudioBufferRef::S24(buf) => {
            for (ch, channel_samples) in planar.iter_mut().enumerate() {
                for &sample in buf.chan(ch) {
                    channel_samples.push(sample.0 as f32 / 8388608.0);
                }
            }
        }
        AudioBufferRef::S32(buf) => {
            for (ch, channel_samples) in planar.iter_mut().enumerate() {
                for &sample in buf.chan(ch) {
                    channel_samples.push(sample as f32 / 2147483648.0);
                }
            }
        }
        _ => {
            // 对于不支持的格式（如 F64），暂不处理，实际在音频文件中很少见
        }
    }
}