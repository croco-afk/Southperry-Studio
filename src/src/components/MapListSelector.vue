<template>
  <div class="map-list-selector">
    <div class="header">
      <h2>{{ $t('mapSelector.title') }}</h2>
      <input 
        type="text" 
        :placeholder="$t('mapSelector.searchPlaceholder')" 
        v-model="searchTerm"
      >
    </div>
    <div class="map-list-container">
      <div v-if="loading" class="loading">{{ $t('mapSelector.loading') }}</div>
      <div v-else-if="error" class="error">{{ $t('mapSelector.error') }}: {{ error }}</div>
      <ul v-else-if="filteredMaps.length > 0" class="map-list">
        <li 
          v-for="map in filteredMaps" 
          :key="map.id" 
          :class="{ 'selected': selectedMap && map.id === selectedMap.id }"
          @click="selectMap(map)"
        >
          {{ map.id }} {{ map.name || 'null' }}
        </li>
      </ul>
      <div v-else class="no-maps">{{ $t('mapSelector.noMapsFound') }}</div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'MapListSelector',
  props: {
    selectedMap: {
      type: Object,
      default: null
    },
    maps: {
      type: Array,
      default: () => []
    },
    loading: {
      type: Boolean,
      default: false
    },
    error: {
      type: String,
      default: ''
    }
  },
  data() {
    return {
      searchTerm: '',
    };
  },
  computed: {
    filteredMaps() {
      if (!this.searchTerm) {
        return this.maps;
      }
      const lowerCaseSearchTerm = this.searchTerm.toLowerCase();
      return this.maps.filter(map => 
        (map.name && map.name.toLowerCase().includes(lowerCaseSearchTerm)) ||
        (map.region && map.region.toLowerCase().includes(lowerCaseSearchTerm)) ||
        String(map.id).toLowerCase().includes(lowerCaseSearchTerm)
      );
    }
  },
  methods: {
    selectMap(map) {
      // 当用户点击地图列表中的某个地图时，发出一个事件
      this.$emit('map-selected', map);
    }
  }
};
</script>

<style scoped>
/* 样式与之前相同 */
.map-list-selector {
  width: 300px; /* Adjust as needed */
  border-right: 1px solid #eee;
  padding: 15px;
  display: flex;
  flex-direction: column;
  background-color: #fcfcfc;
}

.header h2 {
  margin-top: 0;
  margin-bottom: 10px;
  font-size: 1.2em;
  color: #333;
}

.header input {
  width: 100%;
  padding: 8px;
  margin-bottom: 15px;
  border: 1px solid #ccc;
  border-radius: 4px;
  box-sizing: border-box; /* Include padding in width */
  font-size: 0.9em;
}

.map-list-container {
  flex-grow: 1;
  overflow-y: auto; /* Make the list scrollable */
  border: 1px solid #ddd; 
  border-radius: 4px;
  background-color: #fff;
}

.map-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.map-list li {
  padding: 8px 10px;
  cursor: pointer;
  border-bottom: 1px solid #eee;
  font-size: 0.9em;
  color: #555;
}

.map-list li:last-child {
  border-bottom: none;
}

.map-list li:hover {
  background-color: #f0f0f0;
}

.map-list li.selected {
  background-color: #e0e0e0;
  font-weight: bold;
  color: #333;
}

.loading, .error, .no-maps {
  padding: 10px;
  text-align: center;
  color: #666;
  font-size: 0.9em;
}
</style>