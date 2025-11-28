import { createApp } from 'vue'
import './style.css' // 你可以在这里写一些基础 CSS
import App from './App.vue'
import i18n from './i18n' // 引入配置

const app = createApp(App)
app.use(i18n) // 挂载
app.mount('#app')