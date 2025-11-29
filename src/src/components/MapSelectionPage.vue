<template>
  <div class="map-selection-page">
    <MapListSelector 
      :maps="availableMaps" 
      :selectedMap="currentSelectedMap" 
      :loading="isLoadingMaps" 
      :error="mapLoadingError"
      @map-selected="handleMapSelected" 
    />
    <MapViewer 
      :selectedMap="currentSelectedMap" 
      @close="handleCloseViewer" 
    />
  </div>
</template>

<script>
import MapListSelector from './MapListSelector.vue';
import MapViewer from './MapViewer.vue';

export default {
  name: 'MapSelectionPage',
  components: {
    MapListSelector,
    MapViewer
  },
  props: { 
    serverUrl: {
      type: String,
      required: true 
    }
  },
  data() {
    return {
      availableMaps: [], 
      currentSelectedMap: null, 
      isLoadingMaps: false,
      mapLoadingError: ''
    };
  },
  methods: {
    getMapFolder(mapId) {
      const firstDigit = mapId.charAt(0);
      return `Map${firstDigit}`;
    },
    getMinimapUrl(mapId) {
      const mapFolder = this.getMapFolder(mapId);
      return `${this.serverUrl}/node/image_unparsed/Map/Map/${mapFolder}/${mapId}.img/miniMap/canvas`;
    },
    handleMapSelected(map) {
      this.currentSelectedMap = map;
      // 如果需要，可以在这里发起另一个API请求以获取所选地图的更详细数据
    },
    handleCloseViewer() {
      // 发出 'close' 事件通知父组件（App.vue）关闭地图选择视图
      this.$emit('close'); 
      this.currentSelectedMap = null; 
    },
    async fetchMaps() {
      this.isLoadingMaps = true;
      this.mapLoadingError = '';
      try {
        // 构建API请求URL。根据后端路由，地图列表在 /string/map
        const apiUrl = new URL('/string/map', this.serverUrl).href; 
        console.log('Fetching maps from:', apiUrl); 

        const response = await fetch(apiUrl); // <<-- 使用 fetch 发送请求
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        const data = await response.json(); // 后端返回的是 JSON 数组的数组

        // 将后端返回的 `[ID, Name, StreetName]` 格式转换为前端所需的 `[{ id, name, region, minimapUrl }]` 格式
        this.availableMaps = data.map(mapData => {
          const mapId = String(mapData[0]);
          const mapName = mapData[1] || null;
          const streetName = mapData[2] || 'N/A'; // streetName 在前端显示为 region
          
          return {
            id: mapId,
            name: mapName,
            region: streetName,
            minimapUrl: this.getMinimapUrl(mapId)
          };
        });

        // 根据原始图片中的信息，默认选中 ID 为 '103010000' 的地图
        const defaultMapId = '103010000';
        const defaultMap = this.availableMaps.find(map => map.id === defaultMapId);
        
        if (defaultMap) {
            // 这里可以直接使用从 API 获取到的 defaultMap 对象
            this.currentSelectedMap = defaultMap;
        } else if (this.availableMaps.length > 0) {
          // 如果没有默认地图，则默认选中列表中的第一个地图
          this.currentSelectedMap = this.availableMaps[0];
        }

      } catch (e) {
        this.mapLoadingError = `Failed to load maps: ${e.message}`;
        console.error('Error fetching maps:', e);
      } finally {
        this.isLoadingMaps = false;
      }
    }
  },
  created() {
    // 确保 serverUrl 存在后才发起请求
    if (this.serverUrl) {
      this.fetchMaps();
    } else {
      this.mapLoadingError = 'Server URL is not provided to MapSelectionPage.';
      console.warn('MapSelectionPage: serverUrl prop is missing.');
    }
  }
};
</script>

<style scoped>
.map-selection-page {
  display: flex;
  height: 100%; /* 调整高度和宽度以适应App.vue的main-content */
  width: 100%; 
  /* 移除外部边框和阴影，这些应该由App.vue的main-content来管理 */
  border: none; 
  box-shadow: none;
  background-color: #fff;
  border-radius: 0; /* 移除圆角，由App.vue管理 */
  overflow: hidden; 
}
</style>