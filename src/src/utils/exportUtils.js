// src/utils/exportUtils.js
import { mkdir, writeFile } from '@tauri-apps/plugin-fs';
import { join } from '@tauri-apps/api/path';

/**
 * 辅助函数：将 Canvas 转为 Uint8Array
 */
const canvasToBuffer = (canvas) => {
  return new Promise((resolve, reject) => {
    canvas.toBlob(async (blob) => {
      if (!blob) return reject(new Error('Canvas blob failed'));
      const arrayBuffer = await blob.arrayBuffer();
      resolve(new Uint8Array(arrayBuffer));
    }, 'image/png');
  });
};

/**
 * 辅助函数：计算所有帧的全局边界 (Bounding Box)
 */
const calculateGlobalBounds = (frames) => {
  let min_x = Infinity, max_x = -Infinity;
  let min_y = Infinity, max_y = -Infinity;

  frames.forEach(frame => {
    if (!frame.imgObj) return;
    // WZ 坐标系: origin 是图片内的点，对应世界坐标 (0,0)
    // 所以图片左上角的世界坐标是: (-origin.x, -origin.y)
    const left = -frame.shift_left;
    const top = -frame.shift_up;
    const right = left + frame.imgObj.naturalWidth;
    const bottom = top + frame.imgObj.naturalHeight;

    if (left < min_x) min_x = left;
    if (top < min_y) min_y = top;
    if (right > max_x) max_x = right;
    if (bottom > max_y) max_y = bottom;
  });

  if (min_x === Infinity) return null;

  const width = Math.ceil(max_x - min_x);
  const height = Math.ceil(max_y - min_y);

  return {
    min_x, min_y,
    width, height,
    // 全局原点相对于新画布左上角的坐标
    anchor_x: -min_x, 
    anchor_y: -min_y
  };
};

/**
 * 模式 A: 原始/修剪模式 (Legacy)
 * 特点：每一帧尺寸可能不同（只做偶数修正），JSON 记录每一帧相对于中心的偏移
 */
export const exportTrimmedMode = async (frames, frameDir) => {
  const jsonOutput = {};

  for (const frame of frames) {
    if (!frame.imgObj) continue;

    const rawW = frame.imgObj.naturalWidth;
    const rawH = frame.imgObj.naturalHeight;

    // 强制偶数宽高 (为了某些视频编码器兼容性)
    const targetW = rawW % 2 !== 0 ? rawW + 1 : rawW;
    const targetH = rawH % 2 !== 0 ? rawH + 1 : rawH;

    // 创建 Canvas 绘制
    const canvas = document.createElement('canvas');
    canvas.width = targetW;
    canvas.height = targetH;
    const ctx = canvas.getContext('2d', { willReadFrequently: true });
    ctx.clearRect(0, 0, targetW, targetH);
    ctx.drawImage(frame.imgObj, 0, 0);

    // 保存图片
    const fileName = `${frame.id}.png`;
    const filePath = await join(frameDir, fileName);
    const pngData = await canvasToBuffer(canvas);
    await writeFile(filePath, pngData);

    // 计算坐标 (相对于图片中心的偏移)
    const centerX = targetW / 2;
    const centerY = targetH / 2;
    const newX = frame.shift_left - centerX;
    const newY = frame.shift_up - centerY;

    jsonOutput[frame.id] = {
      x: -newX,
      y: newY,
      delay: frame.delay
    };
  }

  return jsonOutput;
};

/**
 * 模式 B: 等宽等高模式 (Equal Dimensions)
 * 特点：所有帧画布尺寸一致，图像已根据原点对齐。
 * JSON 记录的 x, y 是固定的（即全局原点位置）。
 */
export const exportEqualMode = async (frames, frameDir) => {
  const bounds = calculateGlobalBounds(frames);
  if (!bounds) throw new Error("无法计算边界");

  const jsonOutput = {};
  
  // 确保画布尺寸也是偶数 (可选，但推荐)
  const canvasW = bounds.width % 2 !== 0 ? bounds.width + 1 : bounds.width;
  const canvasH = bounds.height % 2 !== 0 ? bounds.height + 1 : bounds.height;

  for (const frame of frames) {
    if (!frame.imgObj) continue;

    const canvas = document.createElement('canvas');
    canvas.width = canvasW;
    canvas.height = canvasH;
    const ctx = canvas.getContext('2d', { willReadFrequently: true });
    
    // 清空
    ctx.clearRect(0, 0, canvasW, canvasH);

    // 计算绘制位置
    // 图片左上角世界坐标 = (-shift_left, -shift_up)
    // 画布左上角世界坐标 = (min_x, min_y)
    // 绘制坐标 = 图片左上角 - 画布左上角
    const drawX = (-frame.shift_left) - bounds.min_x;
    const drawY = (-frame.shift_up) - bounds.min_y;

    ctx.drawImage(frame.imgObj, drawX, drawY);

    // 保存图片
    const fileName = `${frame.id}.png`;
    const filePath = await join(frameDir, fileName);
    const pngData = await canvasToBuffer(canvas);
    await writeFile(filePath, pngData);

    // JSON 输出
    // 在等宽模式下，每一帧的 "原点" 在画布上的位置是固定的
    // 这里我们输出原点相对于画布左上角的坐标
    jsonOutput[frame.id] = {
      x: -bounds.anchor_x + canvasW / 2,
      y: bounds.anchor_y - canvasH / 2,
      delay: frame.delay,
      // 额外标记，方便使用者知道这是画布尺寸
      canvas_w: canvasW,
      canvas_h: canvasH
    };
  }

  // 也可以选择在 JSON 根目录添加全局信息，取决于你的后端需求
  // jsonOutput._info = { mode: "equal", width: canvasW, height: canvasH, anchor_x: bounds.anchor_x, anchor_y: bounds.anchor_y };

  return jsonOutput;
};