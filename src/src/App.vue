<template>
  <div class="app-container">
    <div class="header-bar">
      <div class="header-left">
        <!-- 直接使用全局 $t -->
        <p>{{ $t('home.appTitle') }}</p>
        <span class="subtitle">{{ $t('home.subtitle') }}</span>
      </div>
      
      <div class="header-actions">
        <button 
          @click="switchToHome" 
          class="btn-icon" 
          :class="{ active: currentView === 'home' }"
          :title="$t('home.homeBtn')"
        >
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"/>
          </svg>
        </button>
        
        <!-- 2. Readme Button (新增：切换到文档视图) -->
        <button 
          @click="currentView = 'readme'" 
          class="btn-icon" 
          :class="{ active: currentView === 'readme' }"
          :title="$t('home.readmeBtn')"
        >
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
             <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"/>
          </svg>
        </button>

        <!-- 3. Settings Button -->
        <button @click="showSettings = true" class="btn-icon" :title="$t('home.settingsBtn')">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
          </svg>
        </button>
      </div>
    </div>
    
    <!-- 内容区域 -->
    <div class="main-content">
      <!-- Readme 视图 -->
      <ReadmeViewer v-if="currentView === 'readme'" /> 

      <!-- 主功能视图 (使用 v-else 保持原逻辑) -->
      <template v-else>
        <WzLoader @wz-loaded="handleLoaded" v-if="!serverUrl" />
        
        <div v-else class="viewer-wrapper">
           <div class="status-indicator">
              <div class="status-dot"></div>
              <span>{{ $t('home.status', { url: serverUrl }) }}</span>
           </div>
           <SkillViewer 
             :serverUrl="serverUrl" 
             :defaultExportPath="defaultExportPath"
             :showOriginGlobal="showOriginGlobal"
             :useEqualDimensions="useEqualDimensions" 
           />
        </div>
      </template>
    </div>

    <!-- Settings Modal: 传入 appLanguage -->
    <SettingsModal 
      v-model:isOpen="showSettings" 
      :currentPath="defaultExportPath"
      :isOriginVisible="showOriginGlobal"
      :useEqualDimensions="useEqualDimensions"
      v-model:language="appLanguage" 
      @save-path="updateExportPath"
      @save-settings="updateSettings"
    />
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import WzLoader from './components/WzLoader.vue';
import SkillViewer from './components/SkillViewer.vue';
import SettingsModal from './components/SettingsModal.vue';
import ReadmeViewer from './components/ReadmeViewer.vue'; // 引入新组件

const serverUrl = ref('');
const showSettings = ref(false);
const defaultExportPath = ref('./');
const showOriginGlobal = ref(false);
const useEqualDimensions = ref(true);

// --- 新增状态 ---
const currentView = ref('home'); // 'home' | 'readme'
const appLanguage = ref('en');

onMounted(() => {
  const savedPath = localStorage.getItem('default_export_path');
  if (savedPath) defaultExportPath.value = savedPath;
  
  const savedOrigin = localStorage.getItem('setting_show_origin');
  if (savedOrigin) {
    showOriginGlobal.value = (savedOrigin === 'true');
  }

  // 读取新设置
  const savedEqual = localStorage.getItem('setting_equal_dims');
  if (savedEqual) useEqualDimensions.value = (savedEqual === 'true');
  
  const savedLang = localStorage.getItem('app_language');
  if (savedLang) appLanguage.value = savedLang; // 只要 localStorage 能存 ko 就没问题
});

const handleLoaded = (url) => {
  console.log("WZ Loaded, Server URL:", url);
  serverUrl.value = url;
};

const updateExportPath = (path) => {
  defaultExportPath.value = path;
};

const updateSettings = (settings) => {
  if (settings.hasOwnProperty('showOrigin')) showOriginGlobal.value = settings.showOrigin;
  if (settings.hasOwnProperty('equalDimensions')) useEqualDimensions.value = settings.equalDimensions;
  
  // 保存语言 (SettingsModal 会 emit 完整的 settings 对象，或者我们利用 v-model 自动同步)
  if (settings.hasOwnProperty('language')) {
    appLanguage.value = settings.language;
    localStorage.setItem('app_language', settings.language);
  }
};

const switchToHome = () => {
  if (currentView.value === 'readme') {
    currentView.value = 'home';
  } else {
    // 如果已经在 Home，点击 Home 按钮可以触发重置 WZ (可选)
    resetApp();
  }
};


const resetApp = () => {
  serverUrl.value = '';
  console.log("Returning to WZ selection.");
};
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
  background: #f5f7fa;
  color: #1f2937;
  margin: 0;
  padding: 0;
  overflow: hidden; /* 禁止浏览器默认滚动条 */
}

.app-container {
  height: 100vh; /* 强制 100% 视口高度 */
  display: flex;
  flex-direction: column; /* 上下布局 */
}

.header-bar {
  flex-shrink: 0; /* 防止 Header 被压缩 */
  background: white;
  border-bottom: 1px solid #e5e7eb;
  padding: 0 10px 10px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
  max-width: 1400px;
  margin: 0 auto;
  width: 100%;
}

.header-left {
  display: flex;
  align-items: baseline;
  gap: 12px;
}

.header-left p {
  margin: 0;
}


h1 {
  font-size: 24px;
  font-weight: 700;
  color: #111827;
  margin: 0;
}

.subtitle {
  color: #6b7280;
  font-weight: 600;
}

.header-actions {
    display: flex;
    gap: 8px;
    align-items: center;
}

/* 新增或修改为 btn-icon 样式 (更紧凑的图标按钮) */
.btn-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 8px;
    width: 34px;
    height: 30px;
    color: #374151;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
    flex-shrink: 0;
}

.btn-icon:hover {
  background: #e5e7eb;
  border-color: #d1d5db;
}

.btn-icon svg {
  width: 18px; /* 图标保持不变 */
  height: 18px;
  stroke-width: 2;
}

.main-content {
    flex: 1;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    position: relative;
    max-width: 1400px;
    width: 100%;
    margin: 0 auto;
}

.status-indicator {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 1px 8px;
    background: #ecfdf5;
    border: 1px solid #d1fae5;
    font-size: 11px;
    color: #065f46;
}

.status-dot {
  width: 8px;
  height: 8px;
  background: #10b981;
  border-radius: 50%;
  animation: pulse 2s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}

.btn-icon.active {
  background: #e5e7eb; /* 灰色背景表示激活 */
  color: #111827;
  border-color: #d1d5db;
}

.viewer-wrapper {
  display: flex;
  flex-direction: column;
  height: 100%;
}
</style>