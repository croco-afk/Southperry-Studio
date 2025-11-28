<!-- src/components/SkillList.vue -->
<template>
  <div class="sidebar">
    <!-- Toolbar -->
    <div class="toolbar">
      <!-- Job Filter -->
      <div class="toolbar-row" v-if="skills.length > 0 && availableJobList.length > 0">
        <select v-model="selectedJobId" class="compact-select">
          <!-- 多语言化 -->
          <option value="">{{ $t('skillList.allJobs') }}</option>
          <option v-for="job in availableJobList" :key="job.id" :value="String(job.id)">
            {{ job.id }} - {{ job.name }}
          </option>
        </select>
      </div>
      
      <!-- Search & Refresh -->
      <div class="toolbar-row search-row">
        <!-- 多语言化 placeholder -->
        <input type="text" v-model="searchQuery" :placeholder="$t('skillList.searchPlaceholder')" class="compact-input" />
        <!-- 多语言化 title -->
        <button @click="fetchSkills" :disabled="loadingList" class="btn-refresh" :title="$t('skillList.refreshBtn')">R</button>
      </div>
    </div>
    
    <!-- Status Bar -->
    <div class="status-bar">
      {{ listStatus }}
    </div>

    <!-- List -->
    <div class="skill-list-container">
      <template v-if="groupedSkills.length > 0">
        <div v-for="group in groupedSkills" :key="group.id" class="skill-group">
          <!-- Group Header -->
          <div class="group-header" @click="toggleGroup(group.id)">
            <span class="arrow">{{ collapsedGroups.has(group.id) ? '▶' : '▼' }}</span>
            <span class="group-name">{{ group.name }}</span>
            <span class="group-count">{{ group.list.length }}</span>
          </div>

          <!-- Group Items -->
          <ul v-show="!collapsedGroups.has(group.id)" class="group-items">
            <li 
              v-for="skill in group.list" 
              :key="skill[0]" 
              @click="selectSkill(skill)"
              :class="{ active: currentSelectedId === skill[0] }"
            >
              <div class="list-icon">
                 <img :src="getIconUrl(skill)" loading="lazy" @error="handleImgError" />
              </div>
              <div class="list-content">
                <div class="list-title">{{ skill[2] }}</div>
                <div class="list-meta">
                  <span class="id-tag">{{ skill[0] }}</span>
                </div>
              </div>
            </li>
          </ul>
        </div>
      </template>
      
      <div v-if="!loadingList && groupedSkills.length === 0" class="empty-list">
        <!-- 多语言化 -->
        {{ $t('skillList.noMatches') }}
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n'; // <-- 新增

const props = defineProps({
  serverUrl: String,
  currentSelectedId: String // 接收当前选中的ID，用于高亮显示
});
const { t } = useI18n(); // <-- 新增

const emit = defineEmits(['select']);

// --- Data & State ---
const skills = ref([]);
const loadingList = ref(false);
const searchQuery = ref('');
const selectedJobId = ref('');

// --- Job Data ---
const RAW_JOB_DATA = [
  { "id": 100, "name": "Swordsman (1st)" },
  { "id": 1000, "name": "Noblesse" },
  { "id": 110, "name": "Fighter (2nd)" },
  { "id": 1100, "name": "Dawn Warrior (1st)" },
  { "id": 111, "name": "Crusader (3rd)" },
  { "id": 1110, "name": "Dawn Warrior (2nd)" },
  { "id": 1111, "name": "Dawn Warrior (3rd)" },
  { "id": 1112, "name": "Dawn Warrior (4th)" },
  { "id": 112, "name": "Hero (4th)" },
  { "id": 11212, "name": "Beast Tamer" },
  { "id": 120, "name": "Page (2nd)" },
  { "id": 1200, "name": "Blaze Wizard (1st)" },
  { "id": 121, "name": "Knight (3rd)" },
  { "id": 1210, "name": "Blaze Wizard (2nd)" },
  { "id": 1211, "name": "Blaze Wizard (3rd)" },
  { "id": 1212, "name": "Blaze Wizard (4th)" },
  { "id": 122, "name": "Paladin (4th)" },
  { "id": 130, "name": "Spearman (2nd)" },
  { "id": 1300, "name": "Wind Archer (1st)" },
  { "id": 131, "name": "Dragon Knight (3rd)" },
  { "id": 1310, "name": "Wind Archer (2nd)" },
  { "id": 1311, "name": "Wind Archer (3rd)" },
  { "id": 1312, "name": "Wind Archer (4th)" },
  { "id": 132, "name": "Dark Knight (4th)" },
  { "id": 1400, "name": "Night Walker (1st)" },
  { "id": 1410, "name": "Night Walker (2nd)" },
  { "id": 1411, "name": "Night Walker (3rd)" },
  { "id": 1412, "name": "Night Walker (4th)" },
  { "id": 14200, "name": "Kinesis (1st)" },
  { "id": 14210, "name": "Kinesis (2nd)" },
  { "id": 14211, "name": "Kinesis (3rd)" },
  { "id": 14212, "name": "Kinesis (4th)" },
  { "id": 1500, "name": "Thunder Breaker (1st)" },
  { "id": 1510, "name": "Thunder Breaker (2nd)" },
  { "id": 1511, "name": "Thunder Breaker (3rd)" },
  { "id": 1512, "name": "Thunder Breaker (4th)" },
  { "id": 15200, "name": "Illium (1st)" },
  { "id": 15210, "name": "Illium (2nd)" },
  { "id": 15211, "name": "Illium (3rd)" },
  { "id": 15212, "name": "Illium (4th)" },
  { "id": 15500, "name": "Ark (1st)" },
  { "id": 15510, "name": "Ark (2nd)" },
  { "id": 15511, "name": "Ark (3rd)" },
  { "id": 15512, "name": "Ark (4th)" },
  { "id": 200, "name": "Magician (1st)" },
  { "id": 2000, "name": "Aran" },
  { "id": 2001, "name": "Evan (Beginner)" },
  { "id": 210, "name": "Wizard - Fire/Poison (2nd)" },
  { "id": 2100, "name": "Aran (1st)" },
  { "id": 211, "name": "Mage - Fire/Poison (3rd)" },
  { "id": 2110, "name": "Aran (2nd)" },
  { "id": 2111, "name": "Aran (3rd)" },
  { "id": 2112, "name": "Aran (4th)" },
  { "id": 212, "name": "Arch Mage F/P (4th)" },
  { "id": 220, "name": "Wizard - Ice/Lightning (2nd)" },
  { "id": 2200, "name": "Evan (1st)" },
  { "id": 221, "name": "Mage - Ice/Lightning (3rd)" },
  { "id": 2211, "name": "Evan (2nd)" },
  { "id": 2214, "name": "Evan (3rd)" },
  { "id": 2217, "name": "Evan (4th)" },
  { "id": 222, "name": "Arch Mage I/L (4th)" },
  { "id": 230, "name": "Cleric (2nd)" },
  { "id": 2300, "name": "Mercedes (1st)" },
  { "id": 231, "name": "Priest (3rd)" },
  { "id": 2310, "name": "Mercedes (2nd)" },
  { "id": 2311, "name": "Mercedes (3rd)" },
  { "id": 2312, "name": "Mercedes (4th)" },
  { "id": 232, "name": "Bishop (4th)" },
  { "id": 2400, "name": "Phantom (1st)" },
  { "id": 2410, "name": "Phantom (2nd)" },
  { "id": 2411, "name": "Phantom (3rd)" },
  { "id": 2412, "name": "Phantom (4th)" },
  { "id": 2510, "name": "Shade (2nd)" },
  { "id": 2511, "name": "Shade (3rd)" },
  { "id": 2512, "name": "Shade (4th)" },
  { "id": 2700, "name": "Luminous (1st)" },
  { "id": 2710, "name": "Luminous (2nd)" },
  { "id": 2711, "name": "Luminous (3rd)" },
  { "id": 2712, "name": "Luminous (4th)" },
  { "id": 300, "name": "Archer (1st)" },
  { "id": 3000, "name": "Citizen" },
  { "id": 310, "name": "Hunter (2nd)" },
  { "id": 3100, "name": "Demon Slayer (1st)" },
  { "id": 3101, "name": "Demon Avenger (1st)" },
  { "id": 311, "name": "Ranger (3rd)" },
  { "id": 3110, "name": "Demon Slayer (2nd)" },
  { "id": 3111, "name": "Demon Slayer (3rd)" },
  { "id": 3112, "name": "Demon Slayer (4th)" },
  { "id": 312, "name": "Bow Master (4th)" },
  { "id": 3120, "name": "Demon Avenger (2nd)" },
  { "id": 3121, "name": "Demon Avenger (3rd)" },
  { "id": 3122, "name": "Demon Avenger (4th)" },
  { "id": 320, "name": "Crossbowman (2nd)" },
  { "id": 3200, "name": "Battle Mage (1st)" },
  { "id": 321, "name": "Sniper (3rd)" },
  { "id": 3210, "name": "Battle Mage (2nd)" },
  { "id": 3211, "name": "Battle Mage (3rd)" },
  { "id": 3212, "name": "Battle Mage (4th)" },
  { "id": 322, "name": "Marksman (4th)" },
  { "id": 3300, "name": "Wild Hunter (1st)" },
  { "id": 3310, "name": "Wild Hunter (2nd)" },
  { "id": 3311, "name": "Wild Hunter (3rd)" },
  { "id": 3312, "name": "Wild Hunter (4th)" },
  { "id": 3500, "name": "Mechanic (1st)" },
  { "id": 3510, "name": "Mechanic (2nd)" },
  { "id": 3511, "name": "Mechanic (3rd)" },
  { "id": 3512, "name": "Mechanic (4th)" },
  { "id": 3600, "name": "Xenon (1st)" },
  { "id": 3610, "name": "Xenon (2nd)" },
  { "id": 3611, "name": "Xenon (3rd)" },
  { "id": 3612, "name": "Xenon (4th)" },
  { "id": 3700, "name": "Blaster (1st)" },
  { "id": 3710, "name": "Blaster (2nd)" },
  { "id": 3711, "name": "Blaster (3rd)" },
  { "id": 3712, "name": "Blaster (4th)" },
  { "id": 400, "name": "Rogue (1st)" },
  { "id": 410, "name": "Assassin (1st)" },
  { "id": 4100, "name": "Hayato (1st)" },
  { "id": 411, "name": "Hermit (3rd)" },
  { "id": 4110, "name": "Hayato (2nd)" },
  { "id": 4111, "name": "Hayato (3rd)" },
  { "id": 4112, "name": "Hayato(4th)" },
  { "id": 412, "name": "Night Lord (4th)" },
  { "id": 420, "name": "Bandit (2nd)" },
  { "id": 4200, "name": "Kanna (1st)" },
  { "id": 421, "name": "Chief Bandit (3rd)" },
  { "id": 4210, "name": "Kanna (2nd)" },
  { "id": 4211, "name": "Kanna (3rd)" },
  { "id": 4212, "name": "Kanna(4th)" },
  { "id": 422, "name": "Shadower (4th)" },
  { "id": 430, "name": "Blade Recruit (2nd)" },
  { "id": 431, "name": "Blade Acolyte (3rd)" },
  { "id": 432, "name": "Blade Specialist (4th)" },
  { "id": 433, "name": "Blade Lord (5th)" },
  { "id": 434, "name": "Dual Blade (4th)" },
  { "id": 500, "name": "Pirate (1st)" },
  { "id": 501, "name": "Cannoneer (1st)" },
  { "id": 508, "name": "Jett (1st)" },
  { "id": 510, "name": "Brawler (2nd)" },
  { "id": 5100, "name": "Mihile (1st)" },
  { "id": 511, "name": "Marauder (3rd)" },
  { "id": 5110, "name": "Mihile (2nd)" },
  { "id": 5111, "name": "Mihile (3rd)" },
  { "id": 5112, "name": "Mihile (4th)" },
  { "id": 512, "name": "Buccaneer (4th)" },
  { "id": 520, "name": "Gunslinger (2nd)" },
  { "id": 521, "name": "Outlaw (3rd)" },
  { "id": 522, "name": "Corsair (4th)" },
  { "id": 530, "name": "Cannoneer (2nd)" },
  { "id": 531, "name": "Cannon Trooper (3rd)" },
  { "id": 532, "name": "Cannon Master (4th)" },
  { "id": 570, "name": "Jett (2nd)" },
  { "id": 571, "name": "Jett (3rd)" },
  { "id": 572, "name": "Jett (4th)" },
  { "id": 6100, "name": "Kaiser (1st)" },
  { "id": 6110, "name": "Kaiser (2nd)" },
  { "id": 6111, "name": "Kaiser (3rd)" },
  { "id": 6112, "name": "Kaiser (4th)" },
  { "id": 6400, "name": "Cadena (1st)" },
  { "id": 6410, "name": "Cadena (2nd)" },
  { "id": 6411, "name": "Cadena (3rd)" },
  { "id": 6412, "name": "Cadena (4th)" },
  { "id": 6500, "name": "Angelic Buster (1st)" },
  { "id": 6510, "name": "Angelic Buster (2nd)" },
  { "id": 6511, "name": "Angelic Buster (3rd)" },
  { "id": 6512, "name": "Angelic Buster (4th)" }
];

const jobList = ref(RAW_JOB_DATA.sort((a, b) => a.id - b.id));
const jobMap = new Map(RAW_JOB_DATA.map(j => [String(j.id), j.name]));


// --- Grouping Logic (New) ---
const collapsedGroups = ref(new Set()); // 存储被折叠的组ID，默认空集合表示全部展开

const toggleGroup = (groupId) => {
  if (collapsedGroups.value.has(groupId)) {
    collapsedGroups.value.delete(groupId);
  } else {
    collapsedGroups.value.add(groupId);
  }
};

const groupedSkills = computed(() => {
  // 基于 filteredSkills 进行分组
  const list = filteredSkills.value;
  if (!list.length) return [];

  const groups = {};

  for (const skill of list) {
    // 获取 job id (prefix)
    let jobId = skill[1];
    if (jobId.endsWith('.img')) jobId = jobId.slice(0, -4);

    if (!groups[jobId]) {
      groups[jobId] = {
        id: jobId,
        name: getJobName(jobId) || `Job ${jobId}`, // 使用已有的 getJobName helper
        list: []
      };
    }
    groups[jobId].list.push(skill);
  }

  // 转为数组并按 Job ID 排序
  return Object.values(groups).sort((a, b) => Number(a.id) - Number(b.id));
});


// --- Logic ---
const fetchSkills = async () => {
  if (!props.serverUrl) return;
  loadingList.value = true;
  try {
    const res = await fetch(`${props.serverUrl}/string/skill`);
    const data = await res.json();
    skills.value = data; 
  } catch (e) { 
    console.error(e); 
  } finally { 
    loadingList.value = false; 
  }
};

const skillJobIds = computed(() => {
  const ids = new Set();
  skills.value.forEach(s => {
    let folder = s[1];
    if (folder.endsWith('.img')) folder = folder.slice(0, -4);
    ids.add(folder);
  });
  return ids;
});

const availableJobList = computed(() => {
  if (skills.value.length === 0) return [];
  return jobList.value.filter(job => skillJobIds.value.has(String(job.id)));
});

const filteredSkills = computed(() => {
  let result = skills.value;

  if (selectedJobId.value) {
    result = result.filter(s => {
      let folder = s[1];
      if (folder.endsWith('.img')) folder = folder.slice(0, -4);
      return folder === selectedJobId.value;
    });
  }

  if (searchQuery.value) {
    const lower = searchQuery.value.toLowerCase();
    result = result.filter(s => 
      s[0].includes(lower) || (s[2] && s[2].toLowerCase().includes(lower))
    );
  }

  return result.slice(0, 500); 
});

const listStatus = computed(() => {
  if (loadingList.value) return t('skillList.loading'); // 多语言化
  if (skills.value.length === 0) return t('skillList.noData'); // 多语言化
  
  const total = skills.value.length;
  const filtered = filteredSkills.value.length;
  
  if (filtered === 0) return t('skillList.noMatches'); // 多语言化

  if (filtered < total) {
    // 使用命名参数的多语言化
    return t('skillList.showingStatus', { filtered, total });
  }
  // 使用命名参数的多语言化
  return t('skillList.totalStatus', { total });
});

const selectSkill = (skill) => {
  emit('select', skill);
};

// --- Helpers ---
const getWzPath = (skill) => {
  let folder = skill[1];
  if (!folder.endsWith('.img')) folder += '.img';
  return `Skill/${folder}/skill/${skill[0]}`;
};
const getIconUrl = (skill) => `${props.serverUrl}/node/image/${getWzPath(skill)}/icon`;
const handleImgError = (e) => e.target.style.opacity = 0.3;

const getJobName = (folderName) => {
  let id = folderName;
  if (id.endsWith('.img')) id = id.slice(0, -4);
  return jobMap.get(id) || id;
};

//用于解决后端 Sound/Skill.img 的懒加载问题
const preloadAudioStructure = async () => {
    if (!props.serverUrl) return;
    const audioRootPath = "Sound/Skill.img"; 
    try {
        // 请求 JSON 接口，因为 JSON 接口通常会触发后端对 WZ 目录结构的解析和缓存。
        const res = await fetch(`${props.serverUrl}/node/raw/${audioRootPath}`);
        if (res.ok) {
            console.log("Audio structure preload successful. Audio nodes are now cached on the backend.");
        } else {
            console.warn(`Audio structure preload failed (${res.status}). Audio access might be slow.`);
        }
    } catch (e) {
        console.error("Audio structure preload network error:", e);
    }
};

onMounted(() => {
    if (props.serverUrl) {
        // 1. 优先加载技能列表
        fetchSkills();
        
        // 2. 异步请求预加载 Sound/Skill.img 结构，不阻塞主 UI
        preloadAudioStructure();
    }
});

</script>

<style scoped>
/* Sidebar styling taken from previous SkillViewer */
.sidebar { 
  width: 260px;
  border-right: 1px solid #ddd; 
  display: flex; 
  flex-direction: column; 
  background: #fcfcfc; 
  height: 100%; /* 确保填满 Viewer */
}

.toolbar {
  padding: 8px;
  border-bottom: 1px solid #eee;
  display: flex;
  flex-direction: column;
  gap: 6px;
}
.toolbar-row { display: flex; gap: 4px; }
.search-row { margin-top: 2px; }

.compact-select, .compact-input {
  flex: 1;
  padding: 4px 6px;
  font-size: 11px;
  border: 1px solid #ccc;
  border-radius: 3px;
  outline: none;
}
.compact-input:focus, .compact-select:focus { border-color: #999; }

.btn-refresh {
  width: 24px; padding: 0;
  font-size: 10px; font-weight: bold;
  border: 1px solid #ccc; background: #eee;
  border-radius: 3px; cursor: pointer;
}

.status-bar {
  padding: 4px 8px;
  background: #eee;
  font-size: 10px;
  color: #666;
  border-bottom: 1px solid #ddd;
}

.skill-list { 
  flex: 1; 
  overflow-y: auto; 
  list-style: none; 
  padding: 0; margin: 0; 
}

.skill-list li { 
  display: flex; align-items: flex-start; 
  padding: 6px 8px; 
  border-bottom: 1px solid #f0f0f0; 
  cursor: pointer; 
}
.skill-list li:hover { background: #f5f5f5; }
.skill-list li.active { background: #e6f7ff; border-right: 2px solid #1890ff; }

.list-icon { 
  width: 28px; height: 28px; 
  display: flex; justify-content: center; align-items: center; 
  margin-right: 8px; border: 1px solid #eee; background: #fff; border-radius: 3px;
  flex-shrink: 0;
}
.list-icon img { max-width: 24px; max-height: 24px; }

.list-content { flex: 1; overflow: hidden; }
.list-title { font-size: 12px; font-weight: 600; color: #333; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }

.list-meta { display: flex; gap: 4px; margin-top: 2px; }
.id-tag { font-size: 10px; color: #888; font-family: monospace; background: #f5f5f5; padding: 0 3px; border-radius: 2px; }
.job-tag { font-size: 9px; color: #1890ff; border: 1px solid #e6f7ff; padding: 0 3px; border-radius: 2px; white-space: nowrap; overflow: hidden; max-width: 80px; text-overflow: ellipsis; }

.empty-list { padding: 15px; text-align: center; color: #ccc; font-size: 11px; }

/* 修改容器样式，允许内部滚动 */
.skill-list-container {
  flex: 1;           /* 自动占据 Toolbar 和 Statusbar 之外的空间 */
  overflow-y: auto;  /* 开启垂直滚动 */
  min-height: 0;     /* 【关键】防止 Flex 子元素内容过多时撑破容器 */
  
  padding: 0;
  margin: 0;
}

/* 确保内部元素没有额外的 margin 撑开 */
.group-items, .empty-list {
  width: 100%;
}

/* 分组标题样式 */
.group-header {
  position: sticky;
  top: 0;
  z-index: 10;
  background: #f1f5f9;
  border-bottom: 1px solid #e2e8f0;
  border-top: 1px solid #e2e8f0;
  padding: 6px 10px;
  font-size: 11px;
  font-weight: 600;
  color: #475569;
  cursor: pointer;
  display: flex;
  align-items: center;
  user-select: none;
}
.group-header:hover {
  background: #e2e8f0;
}

.arrow {
  font-size: 9px;
  margin-right: 6px;
  width: 10px;
  display: inline-block;
}

.group-name {
  flex: 1;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.group-count {
  background: #cbd5e1;
  color: #fff;
  font-size: 9px;
  padding: 1px 5px;
  border-radius: 8px;
  margin-left: 6px;
}

/* 列表项容器 */
.group-items {
  list-style: none;
  padding: 0;
  margin: 0;
}

/* 保持原有 li 样式，微调缩进或背景 */
.group-items li {
  display: flex; 
  align-items: flex-start; 
  padding: 6px 8px 6px 16px; /* 增加左侧 padding 以体现层级 */
  border-bottom: 1px solid #f0f0f0; 
  cursor: pointer; 
  background: #fff;
  align-items: center;
}
.group-items li:hover { background: #f8fafc; }
.group-items li.active { background: #e0f2fe; border-right: 3px solid #0284c7; }
</style>