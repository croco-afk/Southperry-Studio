<template>
  <div class="player-card">
    <!-- Header: Info & Controls merged for density -->
    <div class="player-bar">
      <!-- 修改 title -->
      <div class="info-group" @click="toggleCollapse" :title="$t('animationPlayer.toggleView')"> 
        <span class="indicator" :class="{ open: !isCollapsed }">›</span>
        <span class="name">{{ name }}</span>
      </div>
      
      <!-- Toolbar (Only visible when expanded) -->
      <div class="toolbar-group" v-if="!isCollapsed">
         <!-- BG Color -->
        <!-- 修改 title -->
        <select v-model="bgColor" class="mini-select" :title="$t('animationPlayer.background')">
          <option value="transparent">{{ $t('animationPlayer.bgCheck') }}</option> <!-- 修改文本 -->
          <option value="#ffffff">{{ $t('animationPlayer.bgWhite') }}</option>   <!-- 修改文本 -->
          <option value="#333333">{{ $t('animationPlayer.bgDark') }}</option>    <!-- 修改文本 -->
          <option value="#00ff00">{{ $t('animationPlayer.bgGreen') }}</option>   <!-- 修改文本 -->
        </select>
        <!-- Speed -->
        <!-- 修改 title 和选项 -->
        <select v-model="playbackRate" class="mini-select" :title="$t('animationPlayer.speed')">
          <option :value="0.25">0.25x</option>
          <option :value="0.5">0.5x</option>
          <option :value="1.0">1.0x</option>
          <option :value="2.0">2.0x</option>
          <option :value="4.0">4.0x</option>
          <option :value="8.0">8.0x</option>
        </select>
        <!-- 修改按钮文本 -->
        <button @click="$emit('export')" class="mini-btn btn-primary">{{ $t('animationPlayer.exportBtn') }}</button> 
      </div>
    </div>

    <!-- Main View -->
    <div v-show="!isCollapsed" class="player-body">
      <!-- Canvas -->
      <div 
        class="canvas-area" 
        :class="{ 'bg-checker': bgColor === 'transparent' }"
        :style="{ backgroundColor: bgColor }"
      >
        <canvas ref="canvasRef"></canvas>
      </div>

      <!-- Footer Controls -->
      <div class="player-footer">
        <div class="scrubber-row">
          <input 
            type="range" 
            min="0" 
            :max="totalFrames - 1" 
            v-model.number="currentFrameIdx" 
            class="scrubber"
            @pointerdown="startScrub" 
            @pointerup="endScrub"
          />
        </div>
        <div class="ctrl-row">
          <div class="frame-counter">
            {{ currentFrameIdx + 1 }}/{{ totalFrames }}
            <span class="ms">({{ anim.totalDuration }}ms)</span>
          </div>
          <div class="ctrl-btns">
            <button class="icon-btn" @click="step(-1)">l</button>
            <button class="icon-btn" @click="togglePlay">{{ isPlaying ? '||' : '>' }}</button>
            <button class="icon-btn" @click="step(1)">r</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onUnmounted, watch, toRaw, nextTick } from 'vue';
import { useI18n } from 'vue-i18n'; // <--- 新增

const props = defineProps({
  anim: { type: Object, required: true },
  name: { type: String, required: true },
  showOrigin: { type: Boolean, default: false }
});

const emit = defineEmits(['export']);
const { t } = useI18n(); // <--- 新增

// --- 状态 ---
const canvasRef = ref(null);
const bgColor = ref('#cccccc');
const timerId = ref(null);

// 播放控制状态
const isPlaying = ref(true);     
const currentFrameIdx = ref(0);  
const playbackRate = ref(1.0);   
const wasPlayingBeforeScrub = ref(false); 

const isCollapsed = ref(false); 

// 计算属性
const totalFrames = computed(() => props.anim?.frames?.length || 0);

const finalOffsetX = computed(() => props.anim.offset_x || 0);
const finalOffsetY = computed(() => props.anim.offset_y || 0);

// 计算容纳所有帧和原点的最小边界
const viewBox = computed(() => {
    const anim = props.anim;
    if (!anim || !anim.frames || anim.frames.length === 0) {
        return { width: 1, height: 1, viewOffsetX: 0, viewOffsetY: 0 };
    }
    
    let minX = 0;
    let minY = 0;
    // anim.width/height 通常是整个动画的边界
    
    minX = Math.min(minX, finalOffsetX.value);
    minY = Math.min(minY, finalOffsetY.value);
    
    const viewOffsetX = (minX < 0) ? -minX : 0;
    const viewOffsetY = (minY < 0) ? -minY : 0;
    
    const newWidth = anim.width + viewOffsetX;
    const newHeight = anim.height + viewOffsetY;
    
    const padding = 5; 

    return {
        width: newWidth + padding * 2,
        height: newHeight + padding * 2,
        viewOffsetX: viewOffsetX + padding, 
        viewOffsetY: viewOffsetY + padding,
    };
});

// --- 核心逻辑：绘制单帧 ---
const drawFrame = (index) => {
    const canvas = canvasRef.value;
    if (!canvas || !props.anim || !props.anim.frames.length) return;
    
    const { width: newW, height: newH, viewOffsetX, viewOffsetY } = viewBox.value;

    // 设置 Canvas 的【物理分辨率】，这里不进行缩小，保证原图清晰度
    if (canvas.width !== newW) canvas.width = newW;
    if (canvas.height !== newH) canvas.height = newH;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    ctx.imageSmoothingEnabled = false;
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    const frame = props.anim.frames[index];
    if (!frame || !frame.imgObj) return;
    
    const rawImg = toRaw(frame.imgObj);
    
    const drawX = Math.round(frame.raw_x + props.anim.offset_x + viewOffsetX);
    const drawY = Math.round(frame.raw_y + props.anim.offset_y + viewOffsetY);

    try {
        if (rawImg.width > 0) {
            ctx.drawImage(rawImg, drawX, drawY);
        }
    } catch (e) {
        console.warn('Draw error:', e);
    }

    if (props.showOrigin) {
        const originX = Math.round(props.anim.offset_x + viewOffsetX);
        const originY = Math.round(props.anim.offset_y + viewOffsetY);

        ctx.save();
        ctx.strokeStyle = '#007bff';
        ctx.lineWidth = 1;
        ctx.setLineDash([4, 2]); 
        
        ctx.beginPath(); ctx.moveTo(0, originY); ctx.lineTo(canvas.width, originY); ctx.stroke();
        ctx.beginPath(); ctx.moveTo(originX, 0); ctx.lineTo(originX, canvas.height); ctx.stroke();
        
        ctx.fillStyle = 'red';
        ctx.beginPath(); ctx.arc(originX, originY, 2, 0, Math.PI * 2); ctx.fill();
        ctx.restore();
    }
};

// --- 核心逻辑：循环控制 ---
const playLoop = () => {
  if (timerId.value) clearTimeout(timerId.value);
  if (!isPlaying.value) return;

  const frames = props.anim.frames;
  if (!frames || frames.length === 0) return;

  const frame = frames[currentFrameIdx.value];
  const delay = (frame.delay || 100) / playbackRate.value;

  timerId.value = setTimeout(() => {
    currentFrameIdx.value = (currentFrameIdx.value + 1) % frames.length;
    playLoop(); 
  }, delay);
};

// --- 交互动作 ---

const toggleCollapse = () => {
    isCollapsed.value = !isCollapsed.value;
};

const togglePlay = () => {
  isPlaying.value = !isPlaying.value;
  if (isPlaying.value) {
    playLoop();
  } else {
    if (timerId.value) clearTimeout(timerId.value);
  }
};

const step = (direction) => {
  isPlaying.value = false;
  if (timerId.value) clearTimeout(timerId.value);

  let next = currentFrameIdx.value + direction;
  if (next >= totalFrames.value) next = 0;
  if (next < 0) next = totalFrames.value - 1;
  
  currentFrameIdx.value = next;
};

const startScrub = () => {
  wasPlayingBeforeScrub.value = isPlaying.value; 
  
  if (isPlaying.value) {
    isPlaying.value = false;
    if (timerId.value) clearTimeout(timerId.value);
  }
};

const endScrub = () => {
  if (wasPlayingBeforeScrub.value) {
    isPlaying.value = true;
    playLoop(); 
  }
  wasPlayingBeforeScrub.value = false;
};


// --- 监听器 ---
watch(currentFrameIdx, (newIdx) => {
  requestAnimationFrame(() => drawFrame(newIdx));
});

watch(() => props.showOrigin, () => {
  drawFrame(currentFrameIdx.value);
});

watch(() => props.anim, () => {
  if (timerId.value) clearTimeout(timerId.value);
  currentFrameIdx.value = 0;
  isPlaying.value = true;
  nextTick(() => {
    const canvas = canvasRef.value;
    const { width: newW, height: newH } = viewBox.value; 
    if (canvas) {
        canvas.width = newW;
        canvas.height = newH;
    }
    drawFrame(0);
    playLoop();
  });
}, { deep: true });

onMounted(() => {
  if (props.anim && props.anim.frames.length) {
    const canvas = canvasRef.value;
    const { width: newW, height: newH } = viewBox.value; 
    if (canvas) {
        canvas.width = newW;
        canvas.height = newH;
    }
    drawFrame(0);
    playLoop();
  }
});

onUnmounted(() => {
  if (timerId.value) clearTimeout(timerId.value);
});
</script>

<style scoped>
.player-card {
  border: 1px solid #ddd;
  border-radius: 4px; /* 更锐利 */
  background: #fff;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  box-shadow: 0 1px 3px rgba(0,0,0,0.05);
  font-family: sans-serif;
  user-select: none;
}

/* Header Bar */
.player-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 4px 8px;
  background: #f1f3f5;
  border-bottom: 1px solid #eee;
  height: 28px;
  box-sizing: border-box;
}

.info-group {
  display: flex;
  align-items: center;
  cursor: pointer;
  flex: 1;
  overflow: hidden;
}
.indicator {
  font-family: monospace;
  font-weight: bold;
  margin-right: 6px;
  font-size: 12px;
  color: #666;
  transition: transform 0.1s;
  display: inline-block;
}
.indicator.open { transform: rotate(90deg); }
.name { font-size: 11px; font-weight: 600; color: #333; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }

.toolbar-group { display: flex; gap: 4px; align-items: center; }

.mini-select {
  font-size: 10px;
  padding: 0 2px;
  height: 20px;
  border: 1px solid #ccc;
  border-radius: 2px;
  background: #fff;
}
.mini-btn {
  font-size: 10px;
  height: 20px;
  padding: 0 6px;
  border: 1px solid #bbb;
  border-radius: 2px;
  background: #fff;
  cursor: pointer;
}
.btn-primary { border-color: #28a745; color: #28a745; font-weight: 600; }
.btn-primary:hover { background: #28a745; color: #fff; }

/* Canvas Area */
.canvas-area {
  padding: 10px;
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100px; /* 减小最小高度 */
  background-clip: padding-box;
}
.bg-checker {
  background-image: linear-gradient(45deg, #eee 25%, transparent 25%), 
                    linear-gradient(-45deg, #eee 25%, transparent 25%), 
                    linear-gradient(45deg, transparent 75%, #eee 75%), 
                    linear-gradient(-45deg, transparent 75%, #eee 75%);
  background-size: 10px 10px; /* 更细的网格 */
}
.canvas-area canvas {
  max-width: 100%;
  max-height: 400px; /* 限制最大高度 */
  width: auto; height: auto;
  image-rendering: pixelated; /* 像素风格 */
}

/* Footer Controls */
.player-footer {
  border-top: 1px solid #eee;
  padding: 4px 8px;
  background: #fff;
}
.scrubber-row { margin-bottom: 4px; display: flex; align-items: center; }
.scrubber { width: 100%; height: 4px; cursor: pointer; display: block; }

.ctrl-row { display: flex; justify-content: space-between; align-items: center; font-size: 10px; color: #666; }
.ms { color: #999; margin-left: 2px; }

.ctrl-btns { display: flex; gap: 4px; }
.icon-btn {
  width: 20px; height: 18px;
  border: 1px solid #ddd;
  background: #f9f9f9;
  border-radius: 2px;
  font-size: 9px;
  cursor: pointer;
  display: flex; align-items: center; justify-content: center;
  font-family: monospace;
}
.icon-btn:hover { background: #eee; color: #000; }
</style>