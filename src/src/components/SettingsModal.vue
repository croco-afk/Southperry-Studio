<template>
  <div v-if="isOpen" class="modal-overlay" @click.self="close">
    <div class="modal-content">
      <!-- 1. 标题多语言化 -->
      <h3 class="modal-title">{{ $t('settings.title') }}</h3>

      <!-- 2. 语言选择（直接绑定到全局 i18n.locale） -->
      <div class="form-group">
        <label>{{ $t('settings.languageLabel') }}</label>
        <!-- 直接绑定到 $i18n.locale，并在 change 时同步保存到 localStorage -->
        <select 
          v-model="$i18n.locale" 
          @change="updateLocaleStorage" 
          class="full-width-select"
        >
          <!-- 选项的显示文本也多语言化，使用嵌套的 key -->
          <option value="en">{{ $t('languages.en') }}</option>
          <option value="zh-CN">{{ $t('languages.zh-CN') }}</option>
          <option value="ko">{{ $t('languages.ko') }}</option>
        </select>
      </div>

      <div class="form-group">
        <label>{{ $t('settings.exportPathLabel') }}</label>
        <div class="input-row">
          <!-- Placeholder 多语言化 -->
          <input type="text" v-model="localPath" readonly :placeholder="$t('settings.exportPathDefault')" />
          <button @click="selectPath">...</button>
        </div>
      </div>

      <div class="form-group row-group">
        <input type="checkbox" id="chkOrigin" v-model="localShowOrigin" />
        <!-- Label 多语言化 -->
        <label for="chkOrigin">{{ $t('settings.showOriginLabel') }}</label>
      </div>

      <!-- 新增：导出模式设置 -->
      <div class="form-group row-group">
        <input type="checkbox" id="chkEqual" v-model="localEqualDimensions" />
        <!-- Label 多语言化 -->
        <label for="chkEqual">{{ $t('settings.exportEqualLabel') }}</label>
      </div>

      <div class="modal-actions">
        <!-- 按钮多语言化 -->
        <button @click="close" class="btn-text">{{ $t('settings.cancelBtn') }}</button>
        <button @click="save" class="btn-primary">{{ $t('settings.saveBtn') }}</button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue';
import { useI18n } from 'vue-i18n'; // 引入 i18n hook
import { open } from '@tauri-apps/plugin-dialog';

// 移除 language prop 和 update:language emit
const props = defineProps(['isOpen', 'currentPath', 'isOriginVisible', 'useEqualDimensions']);
const emit = defineEmits(['update:isOpen', 'save-path', 'save-settings']);

const { locale } = useI18n(); // 获取当前语言环境的 ref

const localPath = ref('');
const localShowOrigin = ref(false);
const localEqualDimensions = ref(false);
// localLang 被移除，直接使用 locale.value

watch(() => props.isOpen, (newVal) => {
  if (newVal) {
    localPath.value = props.currentPath || '';
    localShowOrigin.value = props.isOriginVisible;
    localEqualDimensions.value = props.useEqualDimensions;
    // 不再需要同步 props.language，因为 App.vue 会在初次加载时从 localStorage 恢复 locale.value
    // 而且 App.vue 中语言选择器已经直接操作了 $i18n.locale
  }
});

// 在 select 改变时调用，确保语言设置被保存
const updateLocaleStorage = () => {
    localStorage.setItem('app_language', locale.value);
    // 注意: $i18n.locale 改变时，App.vue 里的 $i18n.locale 也会改变，无需 emit
}

const selectPath = async () => {
  const selected = await open({ directory: true, multiple: false, title: "Select Export Directory" }); // 标题也可以多语言化
  if (selected) localPath.value = selected;
};

const save = () => {
  // 1. 保存路径 (不变)
  localStorage.setItem('default_export_path', localPath.value);
  emit('save-path', localPath.value);

  // 2. 保存设置 (不变)
  localStorage.setItem('setting_show_origin', localShowOrigin.value);
  localStorage.setItem('setting_equal_dims', localEqualDimensions.value);

  // 3. 语言设置（由 updateLocaleStorage 或 v-model 触发，这里再检查一次以防万一）
  // localStorage.setItem('app_language', locale.value);

  // 4. 通知父组件保存设置（不再包含 language，因为 App.vue 可以在自身通过 $i18n.locale 访问）
  emit('save-settings', {
    showOrigin: localShowOrigin.value,
    equalDimensions: localEqualDimensions.value,
    // 移除 language 字段，因为它已经是全局状态
  });

  close();
};

const close = () => emit('update:isOpen', false);
</script>


<style scoped>
.modal-overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.4); display: flex; justify-content: center; align-items: center; z-index: 9999; }
.modal-content { background: white; padding: 16px; border-radius: 6px; width: 320px; box-shadow: 0 10px 25px rgba(0,0,0,0.15); font-family: sans-serif; }
.modal-title { margin: 0 0 12px 0; font-size: 14px; font-weight: 600; color: #333; border-bottom: 1px solid #eee; padding-bottom: 8px; }

.form-group { margin-bottom: 12px; }
.form-group label { display: block; font-size: 11px; font-weight: 600; color: #555; margin-bottom: 4px; }

.input-row { display: flex; gap: 4px; }
.input-row input { flex: 1; font-size: 11px; padding: 4px; border: 1px solid #ccc; border-radius: 3px; background: #f9f9f9; }
.input-row button { padding: 0 8px; border: 1px solid #ccc; background: #eee; cursor: pointer; border-radius: 3px; }

.row-group { display: flex; align-items: center; gap: 6px; }
.row-group label { margin: 0; font-weight: 400; cursor: pointer; }

.modal-actions { display: flex; justify-content: flex-end; gap: 8px; margin-top: 16px; }
.btn-text { background: none; border: none; font-size: 11px; color: #666; cursor: pointer; }
.btn-text:hover { color: #333; text-decoration: underline; }
.btn-primary { background: #007bff; color: white; border: none; font-size: 11px; padding: 5px 12px; border-radius: 3px; cursor: pointer; }
.btn-primary:hover { background: #0056b3; }

.full-width-select {
  width: 100%;
  padding: 4px;
  font-size: 11px;
  border: 1px solid #ccc;
  border-radius: 3px;
  background: #f9f9f9;
}
</style>