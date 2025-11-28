<template>
  <div class="loader-wrapper">
    <div class="loader-card">
      <!-- 1. Drop Zone -->
      <div 
        class="drop-zone" 
        :class="{ 'is-loading': loading }"
        @click="!loading && selectFile()"
      >
        <div class="icon-box">
          <!-- Loading Spinner or File Icon -->
          <svg v-if="loading" class="spinner" viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M12 6v4m0 4v4m-4-8h8M6 12h12" opacity="0"/>
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"/>
          </svg>
          <svg v-else class="upload-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"/>
          </svg>
        </div>
        
        <div class="zone-text">
          <!-- 多语言化 -->
          <h3 v-if="loading">{{ $t('loader.parsing') }}</h3>
          <h3 v-else>{{ $t('loader.selectFile') }}</h3>
          <!-- 多语言化，使用 || $t() 作为 fallback -->
          <p class="sub-text">{{ statusMsg || $t('loader.dropZoneHelp') }}</p>
        </div>
      </div>

      <!-- 2. History List -->
      <div v-if="history.length > 0" class="history-section">
        <div class="section-title">{{ $t('loader.recentFiles') }}</div>
        <ul class="history-list">
          <li v-for="(path, index) in history" :key="index" class="history-item" @click="!loading && initWz(path)">
            <svg class="file-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
            </svg>
            <span class="path-text" :title="path">{{ path }}</span>
            <button @click.stop="removeFromHistory(index)" class="del-btn" :title="$t('loader.removeHistory')">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
              </svg>
            </button>
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { useI18n } from 'vue-i18n'; // <-- 新增: 引入 i18n hook

const emit = defineEmits(['wz-loaded']);
const statusMsg = ref('');
const loading = ref(false);
const history = ref([]);
const { t } = useI18n(); // <-- 新增: 获取翻译函数

onMounted(() => {
  const saved = localStorage.getItem('wz_history');
  if (saved) {
    history.value = JSON.parse(saved);
  }
});

const addToHistory = (path) => {
  const newHistory = [path, ...history.value.filter(h => h !== path)].slice(0, 5);
  history.value = newHistory;
  localStorage.setItem('wz_history', JSON.stringify(newHistory));
};

const removeFromHistory = (index) => {
  history.value.splice(index, 1);
  localStorage.setItem('wz_history', JSON.stringify(history.value));
};

const selectFile = async () => {
  const selected = await open({
    multiple: false,
    // 尽管是系统弹窗，但可以为 filter name 提供一个通用的多语言键 (common.wzFile)
    filters: [{ name: t('common.wzFile'), extensions: ['wz'] }] 
  });
  if (selected) initWz(selected);
};

const initWz = async (path) => {
  if (!path.endsWith('Base.wz')) {
    // 多语言化错误信息
    statusMsg.value = t('loader.errorSelectBase');
    return;
  }
  loading.value = true;
  // 多语言化加载前缀
  statusMsg.value = `${t('loader.loadingPrefix')} ${path.split(/[\\/]/).pop()}`; 
  
  try {
    await invoke('init', { path, version: null });
    addToHistory(path); 
    const serverInfo = await invoke('get_server_url');
    emit('wz-loaded', serverInfo.url);
  } catch (err) {
    // 多语言化失败前缀
    statusMsg.value = `${t('loader.failedPrefix')} ${err}`;
  } finally {
    loading.value = false;
  }
};
</script>

<style scoped>
/* 容器：垂直居中，稍微偏上 */
.loader-wrapper {
  display: flex;
  justify-content: center;
  align-items: flex-start;
  padding-top: 8vh; 
  height: 100%;
}

.loader-card {
  width: 100%;
  max-width: 480px;
  background: #fff;
  border-radius: 12px;
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
  border: 1px solid #e5e7eb;
  padding: 24px;
  display: flex;
  flex-direction: column;
  gap: 24px;
}

/* 1. Drop Zone */
.drop-zone {
  border: 2px dashed #e5e7eb;
  border-radius: 8px;
  padding: 40px 20px;
  text-align: center;
  cursor: pointer;
  background: #f9fafb;
  transition: all 0.2s ease;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
}

.drop-zone:hover:not(.is-loading) {
  border-color: #3b82f6;
  background: #eff6ff;
}

.drop-zone.is-loading {
  cursor: wait;
  opacity: 0.8;
  border-style: solid;
  background: #fff;
}

.icon-box {
  width: 48px;
  height: 48px;
  color: #9ca3af;
  display: flex;
  align-items: center;
  justify-content: center;
}

.drop-zone:hover .icon-box {
  color: #3b82f6;
}

.upload-icon, .spinner {
  width: 100%;
  height: 100%;
}

.spinner {
  animation: spin 1s linear infinite;
  color: #3b82f6;
}

.zone-text h3 {
  margin: 0 0 4px 0;
  font-size: 16px;
  font-weight: 600;
  color: #374151;
}

.sub-text {
  margin: 0;
  font-size: 13px;
  color: #6b7280;
}

/* 2. History Section */
.history-section {
  border-top: 1px solid #f3f4f6;
  padding-top: 20px;
}

.section-title {
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: #9ca3af;
  font-weight: 600;
  margin-bottom: 10px;
}

.history-list {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.history-item {
  display: flex;
  align-items: center;
  padding: 8px 10px;
  border-radius: 6px;
  cursor: pointer;
  transition: background 0.15s;
  border: 1px solid transparent;
}

.history-item:hover {
  background: #f3f4f6;
  border-color: #e5e7eb;
}

.file-icon {
  width: 16px;
  height: 16px;
  margin-right: 10px;
  color: #9ca3af;
  flex-shrink: 0;
}

.path-text {
  flex: 1;
  font-size: 13px;
  color: #4b5563;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  /* 显示路径左侧或中间比较重要，RTL 可以让长路径显示末尾文件名 */
  direction: rtl; 
  text-align: left;
}

.del-btn {
  background: none;
  border: none;
  padding: 4px;
  margin-left: 8px;
  border-radius: 4px;
  cursor: pointer;
  color: #d1d5db;
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0; /* 默认隐藏 */
  transition: all 0.2s;
}

.history-item:hover .del-btn {
  opacity: 1; /* Hover 显示 */
}

.del-btn:hover {
  background: #fee2e2;
  color: #ef4444;
}

.del-btn svg {
  width: 14px;
  height: 14px;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}
</style>