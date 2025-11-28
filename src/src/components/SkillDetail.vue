<template>
  <div class="detail-container">
    <!-- Empty State -->
    <div v-if="!skill" class="empty-state">
      <div class="empty-content">
        <div class="empty-icon">&larr;</div>
        <!-- 多语言化 -->
        <p>{{ $t('skillDetail.selectSkill') }}</p>
      </div>
    </div>
    
    <div v-else class="main-layout">
      <!-- Header: Compact -->
      <div class="header">
        <div class="header-left">
          <div class="icon-wrapper">
            <img :src="getIconUrl(skill)" class="skill-icon" @error="handleImgError"/>
          </div>
          <div class="header-info">
            <div class="title-row">
              <h2>{{ skill[2] }}</h2>
              <span class="badge">{{ skill[0] }}</span>
            </div>
            <div class="path-row">
              <span class="code-text">{{ getWzPath(skill) }}</span>
            </div>
          </div>
        </div>
        
        <div class="header-right">
          <button 
            class="btn-primary" 
            @click="exportAll" 
            :disabled="isExporting || (Object.keys(animations).length === 0 && Object.keys(sounds).length === 0)"
            >
            <span v-if="isExporting" class="loading-dots">{{ $t('skillDetail.exportingPrefix') }} {{ exportProgress }}</span>
            <!-- 多语言化 -->
            <span v-else>{{ $t('skillDetail.exportAllBtn') }}</span>
          </button>
        </div>
      </div>

      <!-- Scrollable Content -->
      <div class="scroll-content">
        <!-- 多语言化 -->
        <div v-if="loading" class="loading-state">{{ $t('skillDetail.loading') }}</div>

        <!-- Sounds Section -->
        <div v-if="Object.keys(sounds).length > 0" class="section-container">
          <div class="section-header">
            <!-- 多语言化并传递计数参数 -->
            <h3>{{ $t('skillDetail.audioSection', { count: Object.keys(sounds).length }) }}</h3>
          </div>
          <div class="sound-grid">
            <div v-for="(sound, key) in sounds" :key="key" class="sound-item">
              <div class="sound-info" :title="key">
                <span class="type-icon">♪</span>
                <span class="sound-name">{{ key }}</span>
              </div>
              <div class="sound-actions">
                <!-- 多语言化 title -->
                <button class="btn-icon" @click="playSound(sound.url)" :title="$t('skillDetail.playBtn')">▶</button>
                <!-- 多语言化 title -->
                <button class="btn-icon" @click="exportSound(key, sound.url)" :title="$t('skillDetail.exportBtn')">↓</button>
              </div>
            </div>
          </div>
        </div>

        <!-- Animations Section -->
        <div class="section-container full-height">
          <div class="section-header">
            <!-- 多语言化并传递计数参数 -->
            <h3>{{ $t('skillDetail.animationsSection', { count: Object.keys(animations).length }) }}</h3>
            <!-- 多语言化 -->
            <!-- <span style="margin-left:auto; font-size:9px; color:#999;">{{ $t('skillDetail.dragDropHint') }}</span> -->
          </div>

          <div v-if="Object.keys(animations).length === 0 && !loading" class="no-data-msg">
            <!-- 多语言化 -->
            {{ $t('skillDetail.noAnimData') }}
          </div>
          
          <div 
            class="animations-grid form-group main-animation-box extra-anims-wrapper"
            @dragover.prevent
            @drop.prevent="handleDrop"
          >
            <AnimationPlayer
              v-for="(anim, name) in animations"
              :key="name"
              :name="name"
              :anim="anim"
              :showOrigin="showOriginGlobal" 
              @export="exportAnim(name, anim)"
            />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue';
import { mkdir, writeFile } from '@tauri-apps/plugin-fs';
import { join } from '@tauri-apps/api/path'; 
import { open, ask } from '@tauri-apps/plugin-dialog';
import AnimationPlayer from './AnimationPlayer.vue';
import { invoke } from '@tauri-apps/api/core'; 
import { exportTrimmedMode, exportEqualMode } from '../utils/exportUtils';
import { useI18n } from 'vue-i18n'; // <-- 新增
import { open as openShell } from '@tauri-apps/plugin-shell'; 

const props = defineProps(['skill', 'serverUrl', 'exportPath', 'showOriginGlobal', 'useEqualDimensions']);
const loading = ref(false);
const animations = ref({});
const sounds = ref({}); 

const isExporting = ref(false);
const exportProgress = ref(''); 
const { t } = useI18n(); // <-- 新增

// ==========================================
// 新增：拖拽处理逻辑
// ==========================================
const handleDrop = async (event) => {
  const files = Array.from(event.dataTransfer.files).filter(file => file.type.startsWith('image/'));
  
  if (files.length === 0) return;

  // 1. 自然排序文件名 (e.g. walk.0.png, walk.1.png, walk.10.png)
  files.sort((a, b) => a.name.localeCompare(b.name, undefined, { numeric: true, sensitivity: 'base' }));

  // 2. 准备帧数据
  const framesData = [];
  const promises = files.map((file, index) => {
    return new Promise((resolve) => {
      const img = new Image();
      const objectUrl = URL.createObjectURL(file);
      img.src = objectUrl;
      img.onload = () => {
        // 默认将原点设为底部中心，或者根据图片大小设为 (0,0)
        // 这里默认设为底部中心，模拟角色站立点
        const originX = Math.floor(img.width / 2);
        const originY = img.height;

        framesData.push({
          id: index,
          src: objectUrl, // 保存 URL 以便 AnimationPlayer 使用
          delay: 120,     // 默认延迟
          shift_left: originX, 
          shift_up: originY, 
          imgObj: img,
          // 记录排序用的索引，防止异步加载导致乱序
          _sortIndex: index 
        });
        resolve();
      };
      img.onerror = () => resolve(); // 忽略错误
    });
  });

  await Promise.all(promises);

  if (framesData.length === 0) return;

  // 3. 重新按原始文件顺序排序帧 (因为 onload 是异步的)
  framesData.sort((a, b) => a._sortIndex - b._sortIndex);

  // 4. 生成唯一的动画名称
  // 使用第一个文件的名字前缀作为基础，或者 Custom_时间戳
  let baseName = files[0].name.split('.')[0] || 'Custom';
  const animName = `${baseName}_Drop_${Date.now().toString().slice(-4)}`;

  // 5. 计算 Metrics 并加入到 animations 列表
  // 注意：calculateMetrics 是现有的函数，复用它来生成 canvas 尺寸
  const animMetrics = calculateMetrics(framesData);
  
  // Vue Reactivity: 添加新属性
  animations.value = {
    ...animations.value,
    [animName]: animMetrics
  };
};

// ... [原有代码: processAnimExport, processSoundExport, exportAll 等保持不变] ...

// --- processAnimExport (保持不变) ---
const processAnimExport = async (animName, animData, targetRootDir) => {
  const metrics = calculatePythonNaming(animData.frames);
  if (!metrics) return; 

  const skillId = props.skill[0];
  const suffix = `${skillId}_${animName}`; 
  const finalFileName = `${metrics.finalFileNamePrefix}_${suffix}`;

  const frameDirName = suffix; 
  const frameDir = await join(targetRootDir, frameDirName);
  await mkdir(frameDir, { recursive: true });

  let jsonOutput = {};
  
  try {
    if (props.useEqualDimensions) {
      jsonOutput = await exportEqualMode(animData.frames, frameDir);
    } else {
      jsonOutput = await exportTrimmedMode(animData.frames, frameDir);
    }
    const jsonString = JSON.stringify(jsonOutput, null, 2);
    const encoder = new TextEncoder();
    await writeFile(await join(frameDir, 'origin.json'), encoder.encode(jsonString));
  } catch (err) {
    console.error("Frame Export Error:", err);
    alert(`${t('skillDetail.errorExportFrames')} ${err.message}`); 
    return; 
  }

  try {
    const framesForBackend = [];
    for (const frame of animData.frames) {
      if (!frame.imgObj) continue;
      const frameCanvas = drawCompositeFrame(frame, metrics);
      const buffer = await canvasToBytes(frameCanvas);
      framesForBackend.push({
        data: Array.from(buffer), 
        delay: frame.delay
      });
    }
    const webpBytes = await invoke('encode_webp_anim', {
      frames: framesForBackend,
      width: metrics.width,
      height: metrics.height
    });
    const webpPath = await join(targetRootDir, `${finalFileName}.webp`);
    await writeFile(webpPath, new Uint8Array(webpBytes));
  } catch (backendErr) {
    console.error(`Backend WebP encoding failed for ${animName}:`, backendErr);
  }
};

const getOggUrl = (originalUrl) => {
  return originalUrl.replace('/node/raw/', '/node/sound_ogg/');
};

const processSoundExport = async (soundName, soundUrl, targetDir, fileNamePrefix = "", format = "ogg") => {
  try {
    let targetUrl = soundUrl;
    if (format === 'ogg') {
      targetUrl = getOggUrl(soundUrl);
    }
    const res = await fetch(targetUrl);
    if (!res.ok) throw new Error(`Network error: ${res.statusText}`);
    const buffer = await res.arrayBuffer();
    const extension = format === 'ogg' ? 'ogg' : 'wav';
    const finalName = fileNamePrefix 
      ? `${fileNamePrefix}_${soundName}.${extension}` 
      : `${soundName}.${extension}`;
    const savePath = await join(targetDir, finalName);
    await writeFile(savePath, new Uint8Array(buffer));
  } catch (e) {
    console.error(`Failed to save sound ${soundName} as ${format}:`, e);
    throw e;
  }
};

const exportIcon = async (skillId, targetDir) => {
  try {
    const iconUrl = getIconUrl(props.skill);
    
    // 1. 创建图片对象并加载
    const img = new Image();
    img.crossOrigin = "Anonymous"; // 防止跨域导致画布污染(Tainted Canvas)
    img.src = iconUrl;
    
    // 等待图片加载完成
    await new Promise((resolve, reject) => {
      img.onload = resolve;
      img.onerror = () => reject(new Error('Failed to load icon image'));
    });

    // 2. 创建画布并绘制
    const canvas = document.createElement('canvas');
    canvas.width = img.naturalWidth;
    canvas.height = img.naturalHeight;
    const ctx = canvas.getContext('2d');
    ctx.drawImage(img, 0, 0);

    // 3. 转换为二进制数据 (复用代码中已有的 canvasToBytes 函数)
    const buffer = await canvasToBytes(canvas);
    
    // 4. 写入文件
    const savePath = await join(targetDir, `${skillId}_icon.png`);
    await writeFile(savePath, buffer);
    console.log(`Icon exported to: ${savePath}`);

  } catch (e) {
    console.error(`Error exporting icon for ${skillId}:`, e);
  }
};

// 修改后的 exportAll 函数
const exportAll = async () => {
  if (isExporting.value) return;
  const skillId = props.skill[0];
  const soundCount = Object.keys(sounds.value).length;
  const animCount = Object.keys(animations.value).length;
  const defaultPath = localStorage.getItem('default_export_path');
  let saveDir = defaultPath;
  if (!saveDir) {
    saveDir = await open({
      directory: true,
      title: `Select Directory to Export All Assets for ${skillId}`
    });
  }
  if (!saveDir) return;
  isExporting.value = true;
  try {
    const rootSkillDir = await join(saveDir, skillId);
    await mkdir(rootSkillDir, { recursive: true });
    
    // ==========================================
    // MODIFIED: 导出 Icon
    // ==========================================
    exportProgress.value = t('common.icon'); // 多语言化
    await exportIcon(skillId, rootSkillDir); 
    
    if (soundCount > 0) {
      exportProgress.value = t('common.audio'); // 多语言化
      const audioDir = rootSkillDir
      await mkdir(audioDir, { recursive: true });
      for (const [key, sound] of Object.entries(sounds.value)) {
        await processSoundExport(key, sound.url, audioDir, skillId); 
      }
    }
    let processed = 0;
    for (const [name, anim] of Object.entries(animations.value)) {
      processed++;
      exportProgress.value = t('common.animProgress', { processed, total: animCount }); // 多语言化
      await processAnimExport(name, anim, rootSkillDir);
    }
    
    // ==========================================
    // MODIFIED: 移除 confirm 弹窗和 openPathInExplorer 调用
    //           改为直接显示成功信息和路径
    // ==========================================
    const successMessage = t('skillDetail.exportSuccessAll') + '\n' + t('common.path') + `: ${rootSkillDir}`;
    alert(successMessage);

  } catch (e) {
    console.error(e);
    alert(`${t('skillDetail.exportFailedAll')} ${e.message}`);
  } finally {
    isExporting.value = false;
    exportProgress.value = '';
  }
};

// 修改后的 exportAnim 函数
const exportAnim = async (animName, animData) => {
  const metrics = calculatePythonNaming(animData.frames);
  if (!metrics) { alert(t('common.noValidFrames')); return; } 
  const skillId = props.skill[0];
  const defaultPath = localStorage.getItem('default_export_path');
  let saveDir = defaultPath;
  if (!saveDir) {
    saveDir = await open({ directory: true, title: `Export ${animName}` });
  }
  if (!saveDir) return;
  const skillDir = await join(saveDir, skillId);
  await mkdir(skillDir, { recursive: true });
  try {
    await processAnimExport(animName, animData, skillDir);
    
    // ==========================================
    // MODIFIED: 移除 confirm 弹窗和 openPathInExplorer 调用
    //           改为直接显示成功信息和路径
    // ==========================================
    const successMessage = t('skillDetail.animExportSuccess', { name: animName }) + '\n' + t('common.path') + `: ${skillDir}`;
    alert(successMessage);
    
  } catch (e) {
    console.error(e);
    alert(`${t('common.exportFailed')} ${e.message}`);
  }
};

const exportSound = async (soundName, soundUrl, format = 'ogg') => {
  const skillId = props.skill[0];
  const defaultPath = localStorage.getItem('default_export_path');
  let saveDir = defaultPath;
  if (!saveDir) {
    saveDir = await open({ 
      directory: true, 
      title: `Export ${soundName} as ${format.toUpperCase()}` 
    });
  }
  if (!saveDir) return; 
  const skillDir = await join(saveDir, skillId); 
  await mkdir(skillDir, { recursive: true });
  try {
    await processSoundExport(soundName, soundUrl, skillDir, skillId, format);
    alert(`${t('skillDetail.soundExportSuccess', { format: format.toUpperCase() })}\n${t('common.path')}: ${skillDir}`);
  } catch (error) {
    alert(`${t('common.exportFailed')} ${error.message}`);
  }
};

const playSound = (soundUrl) => {
  const audio = new Audio(soundUrl);
  audio.play().catch(e => alert(`${t('common.cannotPlayAudio')} ${e}`)); // 多语言化
};

const getWzPath = (skill) => {
  let folder = skill[1];
  if (!folder.endsWith('.img')) folder += '.img';
  return `Skill/${folder}/skill/${skill[0]}`;
};
const getIconUrl = (skill) => `${props.serverUrl}/node/image/${getWzPath(skill)}/icon`;
const handleImgError = (e) => e.target.style.opacity = 0.3;
const getChildData = (children, prop) => children?.[prop]?.data ?? null;

const imageToPngBuffer = async (img) => { /* ...原代码... */ 
  const canvas = document.createElement('canvas');
  canvas.width = img.naturalWidth;
  canvas.height = img.naturalHeight;
  const ctx = canvas.getContext('2d', { willReadFrequently: true });
  ctx.drawImage(img, 0, 0);
  return new Promise((resolve, reject) => {
    canvas.toBlob(async (blob) => {
      if (!blob) { reject(new Error('Canvas conversion failed')); return; }
      const arrayBuffer = await blob.arrayBuffer();
      resolve(new Uint8Array(arrayBuffer));
    }, 'image/png');
  });
};
const canvasToBytes = (canvas) => { /* ...原代码... */ 
  return new Promise((resolve, reject) => {
    canvas.toBlob(async (blob) => {
      if (!blob) return reject("Canvas blob failed");
      const arrayBuffer = await blob.arrayBuffer();
      resolve(new Uint8Array(arrayBuffer)); 
    }, 'image/png');
  });
};
const drawCompositeFrame = (frame, metrics) => { /* ...原代码... */ 
  const canvas = document.createElement('canvas');
  canvas.width = metrics.width;
  canvas.height = metrics.height;
  const ctx = canvas.getContext('2d', { willReadFrequently: true });
  ctx.clearRect(0, 0, metrics.width, metrics.height);
  if (frame.imgObj) {
    const drawX = -frame.shift_left + metrics.offsetX;
    const drawY = -frame.shift_up + metrics.offsetY;
    ctx.drawImage(frame.imgObj, drawX, drawY);
  }
  return canvas;
};

// ... [watchters 和 parse 逻辑保持不变] ...

watch(() => props.skill, async (newSkill) => {
  if (!newSkill) return;
  animations.value = {};
  loading.value = true;
  try {
    const path = getWzPath(newSkill);
    let res = await fetch(`${props.serverUrl}/node/json/${path}`);
    if (!res.ok) throw new Error(`Failed`);
    const json = await res.json();
    await parseAndPrepareAnimations(json, path);

    const audioPath = "Sound/Skill.img/" + newSkill[0];
    try {
      let audioRes = await fetch(`${props.serverUrl}/node/json/${audioPath}`);
      if (audioRes.ok) {
        const audioJson = await audioRes.json();
        await parseAndPrepareSounds(audioJson, audioPath);
      } else {
        sounds.value = {}; 
      }
    } catch (e) { sounds.value = {}; }
  } finally {
    loading.value = false;
  }
});

const parseAndPrepareSounds = async (rootJson, rootPath) => { /* ...原代码... */ 
  const children = rootJson.children || {};
  const soundResult = {}; 
  for (const key of Object.keys(children)) {
    const node = children[key];
    if (node.type === 'Sound') {
      const soundPath = `${rootPath}/${key}`;
      const url = `${props.serverUrl}/node/raw/${soundPath}`;
      soundResult[key] = { name: key, url: url, path: soundPath };
    }
  }
  sounds.value = soundResult;
};

const parseAndPrepareAnimations = async (rootJson, rootPath) => {
  const result = {};

  // 1. 辅助判断：节点是否为图片帧
  const isFrameNode = (node) => {
    if (!node) return false;
    const isPng = node.type === 'PNG'; // 有些 WZ 是 Canvas，这里根据你的数据主要是 PNG
    // 检查是否有链接节点 (_outlink / _inlink)
    const hasLink = node.children && (node.children['_outlink'] || node.children['_inlink']);
    return isPng || hasLink;
  };

  // 2. 核心递归函数
  // node: 当前遍历到的节点对象
  // currentPath: 当前节点相对于根的路径，例如 "hit/0" 或 "effect"
  const findAnimationsRecursively = async (node, currentPath) => {
    if (!node || !node.children) return;

    // --- 检查 A：当前节点本身就是动画容器吗？ ---
    // 特征：拥有名为 "0" 的子节点，且 "0" 是图片
    if (node.children["0"] && isFrameNode(node.children["0"])) {
      const framesData = [];
      let i = 0;
      
      // 提取所有帧
      while (node.children[i.toString()]) {
        const frameNode = node.children[i.toString()];
        const origin = getChildData(frameNode.children, 'origin') || [0, 0];
        const rawDelay = getChildData(frameNode.children, 'delay');
        const delay = rawDelay ? parseInt(rawDelay, 10) : 100;

        // 构建完整的图片请求 URL
        // 注意：URL 路径 = 根路径(rootPath) + 当前层级路径(currentPath)
        const frameUrlPath = currentPath ? `${rootPath}/${currentPath}` : rootPath;

        framesData.push({
          id: i,
          src: `${props.serverUrl}/node/image/${frameUrlPath}/${i}`,
          delay: delay,
          shift_left: origin[0],
          shift_up: origin[1],
          imgObj: null
        });
        i++;
      }

      if (framesData.length > 0) {
        await preloadImages(framesData);
        // 将提取到的动画存入结果
        // key 就是路径，例如 result["effect"] 或 result["hit/0"]
        result[currentPath.replace("/", "_")] = calculateMetrics(framesData);
      }
      
      // 找到了动画序列后，通常不需要再往这个节点的子节点里找了（图片里不会包图片）
      return; 
    }

    // --- 步骤 B：如果当前节点不是动画容器，继续递归找子节点 ---
    for (const key of Object.keys(node.children)) {
      // 过滤掉非容器类型的节点（可选，例如 String/Int/Vector 不需要遍历）
      const childNode = node.children[key];
      
      // 只有当子节点有 children 时才值得继续挖
      if (childNode.children) {
        // 拼接路径：如果是根层级，路径就是 key；否则是 parent/key
        const nextPath = currentPath ? `${currentPath}/${key}` : key;
        // 等待递归完成（使用 await 确保加载顺序，也可以用 Promise.all 并发）
        await findAnimationsRecursively(childNode, nextPath);
      }
    }
  };

  // 3. 启动递归
  // 传入根节点的 children 作为起始，初始路径为空字符串 ""
  // 注意：这里传入 rootJson 还是 rootJson.children 取决于你的 rootJson 结构
  // 既然原代码是用 rootJson.children，我们保持一致，把它视为一个虚拟根节点
  await findAnimationsRecursively(rootJson, "");

  // 4. 赋值
  console.log("Parsed Animations Keys:", Object.keys(result));
  animations.value = result;
};

const preloadImages = async (frames) => { /* ...原代码... */ 
  const promises = frames.map(frame => {
    return new Promise((resolve) => {
      const img = new Image();
      img.crossOrigin = "Anonymous";
      img.src = frame.src;
      img.onload = () => { frame.imgObj = img; resolve(); };
      img.onerror = () => { frame.imgObj = null; resolve(); };
    });
  });
  await Promise.all(promises);
};

// 复用 calculateMetrics 
const calculateMetrics = (frames) => {
  let min_x = Infinity, max_x = -Infinity;
  let min_y = Infinity, max_y = -Infinity;
  let base_anchor_x = 0, base_anchor_y = 0;
  let totalDuration = 0;

  frames.forEach(frame => {
    if (!frame.imgObj) return;
    const raw_x = -frame.shift_left;
    const raw_y = -frame.shift_up;
    const w = frame.imgObj.width;
    const h = frame.imgObj.height;

    if (raw_x < min_x) { min_x = raw_x; base_anchor_x = frame.shift_left; }
    if (raw_y < min_y) { min_y = raw_y; base_anchor_y = frame.shift_up; }
    if ((raw_x + w) > max_x) max_x = raw_x + w;
    if ((raw_y + h) > max_y) max_y = raw_y + h;

    frame.raw_x = raw_x;
    frame.raw_y = raw_y;
    totalDuration += frame.delay;
  });

  const canvas_width = Math.ceil(max_x - min_x);
  const canvas_height = Math.ceil(max_y - min_y);
  
  if (canvas_width <= 0 || canvas_height <= 0) return { width: 100, height: 100, frames: [] };

  const cbec_x = canvas_width / 2;
  const cbec_y = canvas_height;
  const final_offset_x = -(cbec_x - base_anchor_x);
  const final_offset_y = cbec_y - base_anchor_y;
  
  return {
    frames,
    width: canvas_width,
    height: canvas_height,
    offset_x: -min_x,
    offset_y: -min_y,
    finalOffsetX: parseFloat(final_offset_x.toFixed(2)),
    finalOffsetY: parseFloat(final_offset_y.toFixed(2)),
    totalDuration,
    frameCount: frames.length
  };
};

// ... [calculatePythonNaming 等保持不变] ...
const calculatePythonNaming = (frames) => {
  let min_x = Infinity, max_x = -Infinity;
  let min_y = Infinity, max_y = -Infinity;
  let base_x_anchor_candidate = 0;
  let base_y_anchor_candidate = 0;
  let totalDuration = 0;

  frames.forEach(frame => {
    if (!frame.imgObj) return;
    const shift_left = frame.shift_left; 
    const shift_up = frame.shift_up;     
    const raw_x = -shift_left;
    const raw_y = -shift_up;
    const w = frame.imgObj.naturalWidth;
    const h = frame.imgObj.naturalHeight;

    if (raw_x < min_x) { min_x = raw_x; base_x_anchor_candidate = shift_left; }
    if (raw_y < min_y) { min_y = raw_y; base_y_anchor_candidate = shift_up; }
    if ((raw_x + w) > max_x) max_x = raw_x + w;
    if ((raw_y + h) > max_y) max_y = raw_y + h;
    totalDuration += frame.delay;
  });

  if (min_x === Infinity) return null;

  let canvas_width = Math.ceil(max_x - min_x);
  let canvas_height = Math.ceil(max_y - min_y);
  if (canvas_width % 2 !== 0) canvas_width++;
  if (canvas_height % 2 !== 0) canvas_height++;

  const offset_x = -min_x;
  const offset_y = -min_y;
  const base_anchor_x = base_x_anchor_candidate;
  const base_anchor_y = base_y_anchor_candidate;
  
  const final_offset_x = canvas_width / 2 - base_anchor_x;
  const final_offset_y = -canvas_height / 2 + base_anchor_y;

  const fmt = (num) => num.toFixed(2).replace(/\.?0+$/, "");
  const x_str = fmt(final_offset_x);
  const y_str = fmt(final_offset_y);

  return {
    width: canvas_width,
    height: canvas_height,
    offsetX: offset_x,
    offsetY: offset_y,
    finalFileNamePrefix: `${x_str}_${y_str}_${totalDuration}`,
    totalDuration
  };
};
</script>

<style scoped>
/* 新增样式：拖拽区域的高亮效果 */
.animations-grid.main-animation-box.extra-anims-wrapper {
  transition: background-color 0.2s, border-color 0.2s;
  border: 2px dashed transparent; /* 预留边框空间 */
}

/* 简单的拖拽反馈（当文件经过时） */
.animations-grid.main-animation-box.extra-anims-wrapper:active,
.animations-grid.main-animation-box.extra-anims-wrapper:focus-within {
  /* 注意：纯 CSS 无法完美检测 dragover，通常需要 JS 配合 class，
     但这里简单的 :active 有时能提供基本反馈 */
  border-color: #3b82f6;
  background-color: #f0f9ff;
}
.detail-container {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: #fff;
  color: #333;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  overflow: hidden; /* 防止双重滚动 */
}

.empty-state {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #aaa;
  font-size: 13px;
}
.empty-icon { font-size: 24px; margin-bottom: 8px; }

.main-layout { display: flex; flex-direction: column; height: 100%; }

/* Header 调整 */
.header {
  flex-shrink: 0;
  padding: 10px 16px; /* 稍微增加一点 padding */
  background: #f8f9fa;
  border-bottom: 1px solid #ddd;
  display: flex;
  justify-content: space-between; /* 确保左右分开 */
  align-items: center;
}

.header-left { 
  display: flex; 
  align-items: center; 
  gap: 10px; 
  overflow: hidden; 
}

/* Header Right Button */
.header-right .btn-primary {
  background-color: #3b82f6; /* 主色蓝 */
  color: white;
  border: none;
  border-radius: 4px;
  padding: 6px 12px;
  font-size: 12px;
  font-weight: 600;
  cursor: pointer;
  transition: background 0.2s;
  display: flex;
  align-items: center;
  gap: 6px;
  box-shadow: 0 1px 2px rgba(0,0,0,0.1);
}

.header-right .btn-primary:hover:not(:disabled) {
  background-color: #2563eb;
}

.header-right .btn-primary:disabled {
  background-color: #9ca3af;
  cursor: not-allowed;
  opacity: 0.8;
}

/* 简单的 Loading 动画文字 */
.loading-dots:after {
  content: '.';
  animation: dots 1.5s steps(5, end) infinite;
}
@keyframes dots {
  0%, 20% { content: '.'; }
  40% { content: '..'; }
  60% { content: '...'; }
  80%, 100% { content: ''; }
}

.icon-wrapper {
  width: 36px; height: 36px;
  background: #fff; border: 1px solid #ddd; border-radius: 4px;
  display: flex; align-items: center; justify-content: center;
}
.skill-icon { max-width: 32px; max-height: 32px; }

.header-info { display: flex; flex-direction: column; justify-content: center; overflow: hidden; }
.title-row { display: flex; align-items: baseline; gap: 8px; }
.title-row h2 { margin: 0; font-size: 14px; font-weight: 600; white-space: nowrap; }
.badge { font-size: 11px; color: #666; font-family: monospace; background: #eee; padding: 0 4px; border-radius: 2px; }
.path-row .code-text { font-size: 10px; color: #888; font-family: monospace; }

/* --- Content --- */
.scroll-content {
  flex: 1;
  overflow-y: auto;
  padding: 10px;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.section-container {
  border: 1px solid #eee;
  border-radius: 4px;
  background: #fff;
  display: flex;
  flex-direction: column;
}
.section-container.full-height { flex: 1; min-height: 0; }

.section-header {
  padding: 6px 10px;
  background: #fafafa;
  border-bottom: 1px solid #eee;
  font-size: 11px;
  font-weight: 600;
  color: #555;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}
.section-header h3 { margin: 0; font-size: inherit; }

/* --- Sound Grid (Compact) --- */
.sound-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(140px, 1fr)); /* 更窄 */
  gap: 6px;
  padding: 8px;
}
.sound-item {
  border: 1px solid #eee;
  border-radius: 3px;
  padding: 4px 8px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  background: #fff;
}
.sound-item:hover { border-color: #ccc; background: #fcfcfc; }

.sound-info { display: flex; align-items: center; gap: 6px; overflow: hidden; flex: 1; }
.type-icon { font-size: 10px; color: #999; }
.sound-name { font-size: 11px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }

.sound-actions { display: flex; gap: 2px; }
.btn-icon {
  background: none; border: 1px solid transparent; cursor: pointer;
  font-size: 10px; padding: 2px 6px; color: #555; border-radius: 3px;
}
.btn-icon:hover { background: #eee; border-color: #ddd; color: #000; }

/* --- Animation Grid --- */
.animations-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 10px;
  padding: 10px;
  align-items: start;
}
.no-data-msg { padding: 20px; text-align: center; color: #ccc; font-size: 12px; }
.loading-state { text-align: center; color: #999; font-size: 12px; padding: 10px; }
</style>