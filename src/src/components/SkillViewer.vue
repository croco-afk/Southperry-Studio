<!-- src/components/SkillViewer.vue -->
<template>
  <div class="viewer-layout">
    <!-- Sidebar Component -->
    <SkillList 
      :serverUrl="serverUrl"
      :currentSelectedId="selectedSkill ? selectedSkill[0] : null"
      @select="handleSelectSkill"
    />

    <!-- Main Content -->
    <div class="main-panel">
      <SkillDetail 
        :skill="selectedSkill" 
        :serverUrl="serverUrl" 
        :exportPath="defaultExportPath"
        :showOriginGlobal="showOriginGlobal"
        :useEqualDimensions="useEqualDimensions"
      />
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import SkillDetail from './SkillDetail.vue';
import SkillList from './SkillList.vue';

const props = defineProps({ 
    serverUrl: String,
    defaultExportPath: String,
    showOriginGlobal: Boolean, 
    useEqualDimensions: Boolean
});

const selectedSkill = ref(null);

const handleSelectSkill = (skill) => {
  selectedSkill.value = skill;
};
</script>

<style scoped>
.viewer-layout { 
  display: flex; 
  height: 100%; /* <-- 改为这行，直接填满父容器 */
  width: 100%;
  border: 1px solid #ddd; 
  background: #fff; 
  overflow: hidden; /* 确保不产生外层滚动条 */
}

.main-panel { 
  flex: 1; 
  overflow: hidden; 
  min-width: 0; 
  display: flex; 
  flex-direction: column;
}
</style>