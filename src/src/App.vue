<template>
  <div class="app-container">
    <div class="header-bar">
      <div class="header-left">
        <p>{{ $t('home.appTitle') }}</p>
        <span class="subtitle">{{ $t('home.subtitle') }}</span>
      </div>
      
      <!-- 中间功能菜单 -->
      <div class="header-menu">

        <!-- Map Button -->
        <button 
          @click="switchToView('map-selection')" 
          class="btn-icon" 
          :class="{ active: currentView === 'map-selection' }"
          :title="$t('home.mapViewerBtn')"
        >
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.828 0l-4.243-4.243a8 8 0 1111.314 0z" />
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
          </svg>
        </button>

      </div>
      
      <!-- 右侧工具按钮 -->
      <div class="header-actions">
        <button 
          @click="switchToView('home')"
          class="btn-icon" 
          :class="{ active: currentView === 'home' }"
          :title="$t('home.homeBtn')"
        >
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"/>
          </svg>
        </button>

        <!-- Settings Button -->
        <button @click="showSettings = true" class="btn-icon" :title="$t('home.settingsBtn')">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
          </svg>
        </button>

        <!-- Readme Button -->
        <button 
          @click="switchToView('readme')" 
          class="btn-icon" 
          :class="{ active: currentView === 'readme' }"
          :title="$t('home.readmeBtn')"
        >
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
             <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"/>
          </svg>
        </button>
      </div>
    </div>
    
    <!-- 内容区域 -->
    <div class="main-content">
      <!-- Readme 视图 -->
      <ReadmeViewer v-if="currentView === 'readme'" /> 

      <MapSelectionPage 
         v-else-if="currentView === 'map-selection'"
         :serverUrl="serverUrl"  
         @close="switchToView('home')" 
       />

      <!-- 主功能视图 -->
      <template v-else-if="currentView === 'home'">
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
import ReadmeViewer from './components/ReadmeViewer.vue';
import MapSelectionPage from './components/MapSelectionPage.vue'; 

const serverUrl = ref('');
const showSettings = ref(false);
const defaultExportPath = ref('./');
const showOriginGlobal = ref(false);
const useEqualDimensions = ref(true);

const currentView = ref('home');
const appLanguage = ref('en');

onMounted(() => {
  const savedPath = localStorage.getItem('default_export_path');
  if (savedPath) defaultExportPath.value = savedPath;
  
  const savedOrigin = localStorage.getItem('setting_show_origin');
  if (savedOrigin) {
    showOriginGlobal.value = (savedOrigin === 'true');
  }

  const savedEqual = localStorage.getItem('setting_equal_dims');
  if (savedEqual) useEqualDimensions.value = (savedEqual === 'true');
  
  const savedLang = localStorage.getItem('app_language');
  if (savedLang) appLanguage.value = savedLang;
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
  
  if (settings.hasOwnProperty('language')) {
    appLanguage.value = settings.language;
    localStorage.setItem('app_language', settings.language);
  }
};

// 统一视图切换逻辑
const switchToView = (viewName) => {
  if (currentView.value === viewName) {
    if (viewName === 'home') {
        resetApp(); 
    }
    return;
  }
  currentView.value = viewName;
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
  overflow: hidden; 
}

.app-container {
  height: 100vh; 
  display: flex;
  flex-direction: column; 
}

.header-bar {
  flex-shrink: 0; 
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
  gap: 20px;
}

.header-left {
  display: flex;
  align-items: baseline;
  gap: 12px;
  flex-shrink: 0;
}

.header-left p {
  margin: 0;
}

.header-menu {
  display: flex;
  gap: 8px;
  align-items: center;
  flex: 1;
  justify-content: center;
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
  width: 18px; 
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
  background: #e5e7eb; 
  color: #111827;
  border-color: #d1d5db;
}

.viewer-wrapper {
  display: flex;
  flex-direction: column;
  height: 100%;
}
</style>