<template>
  <div class="map-viewer">
    <button class="close-button" @click="$emit('close')">×</button>
    <div class="minimap-preview-section">
      <h3>{{ $t('mapViewer.minimapPreviewTitle') }}</h3>
      <div v-if="selectedMap" class="minimap-content">
          <div class="map-details">
          <p>
            <strong>{{ $t('mapViewer.mapRegionLabel') }}</strong> {{ selectedMap.region }} 
            <span class="copy-icon" @click="copyToClipboard(selectedMap.region)">📋</span>
          </p>
          <p>
            <strong>{{ $t('mapViewer.mapNameLabel') }}</strong> {{ selectedMap.name }} 
            <span class="copy-icon" @click="copyToClipboard(selectedMap.name)">📋</span>
          </p>
          <p>
            <strong>{{ $t('mapViewer.mapIdLabel') }}</strong> {{ selectedMap.id }} 
            <span class="copy-icon" @click="copyToClipboard(selectedMap.id)">📋</span>
          </p>
        </div>
        
        <template v-if="selectedMap.minimapUrl && !imageLoadError">
          <img 
            :src="selectedMap.minimapUrl" 
            :alt="selectedMap.name || 'Minimap Preview'" 
            class="minimap-image"
            @error="handleImageError"
          >
        </template>
        <div v-else-if="imageLoadError" class="no-minimap-placeholder">
          Minimap not found or cannot be loaded
        </div>
        <div v-else class="no-minimap-placeholder">
          {{ $t('mapViewer.noMinimapAvailable') }}
        </div>
      </div>
      <div v-else class="no-map-selected">
        <p>{{ $t('mapViewer.noMapSelected') }}</p>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'MapViewer',
  props: {
    selectedMap: {
      type: Object,
      default: null
    }
  },
  data() {
    return {
      imageLoadError: false
    };
  },
  watch: {
    selectedMap() {
      this.imageLoadError = false;
    }
  },
  methods: {
    handleImageError() {
      console.error('Failed to load minimap image for map:', this.selectedMap?.id);
      this.imageLoadError = true;
    },
    async copyToClipboard(text) {
      if (navigator.clipboard) {
        try {
          await navigator.clipboard.writeText(String(text));
          alert('Copied to clipboard!');
        } catch (err) {
          console.error('Failed to copy: ', err);
          alert('Failed to copy to clipboard.');
        }
      } else {
        const textArea = document.createElement("textarea");
        textArea.value = String(text);
        textArea.style.position = "fixed";
        textArea.style.opacity = "0";
        document.body.appendChild(textArea);
        textArea.focus();
        textArea.select();
        try {
          document.execCommand('copy');
          alert('Copied to clipboard!');
        } catch (err) {
          console.error('Fallback: Failed to copy: ', err);
          alert('Fallback: Failed to copy to clipboard.');
        }
        document.body.removeChild(textArea);
      }
    }
  }
};
</script>

<style scoped>
/* 样式与之前相同 */
.map-viewer {
  flex-grow: 1; /* Take remaining space */
  padding: 15px;
  position: relative;
  background-color: #fff;
}

.close-button {
  position: absolute;
  top: 10px;
  right: 10px;
  background: none;
  border: none;
  font-size: 1.5em;
  cursor: pointer;
  color: #666;
  padding: 5px;
  line-height: 1;
}

.close-button:hover {
  color: #333;
}

.minimap-preview-section h3 {
  margin-top: 0;
  margin-bottom: 15px;
  font-size: 1.2em;
  color: #333;
}

.minimap-content {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

/* 新增的无小地图占位符样式 */
.no-minimap-placeholder {
  text-align: center;
  padding: 50px 20px;
  background-color: #f0f0f0;
  color: #888;
  border: 1px dashed #ccc;
  border-radius: 4px;
  margin-bottom: 10px; /* 与下方 map-details 保持间距 */
  font-size: 0.9em;
}


.minimap-image {
  max-width: 200px;
  max-height: 150px;
  border: 1px solid #ddd;
  border-radius: 4px;
  display: block;
}

.map-details p {
  margin: 5px 0;
  display: flex;
  align-items: center;
  font-size: 0.95em;
  color: #444;
}

.map-details strong {
  margin-right: 8px;
  min-width: 90px; 
  color: #222;
}

.copy-icon {
  cursor: pointer;
  margin-left: 8px;
  font-size: 0.9em; 
  user-select: none; 
  opacity: 0.7;
  transition: opacity 0.2s ease;
}

.copy-icon:hover {
  opacity: 1;
  color: #007bff;
}

.no-map-selected {
  text-align: center;
  color: #999;
  padding: 20px;
  border: 1px dashed #ccc;
  border-radius: 4px;
  margin-top: 20px;
}
</style>