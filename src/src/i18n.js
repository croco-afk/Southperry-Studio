import { createI18n } from 'vue-i18n'
import en from './locales/en.json'
import zhCN from './locales/zh-CN.json'
import ko from './locales/ko.json'

// 获取本地存储的语言，如果没有则默认 'en'
const savedLang = localStorage.getItem('app_language') || 'en'

const i18n = createI18n({
  legacy: false, // 必须为 false 才能在 Composition API (script setup) 中使用
  locale: savedLang, 
  fallbackLocale: 'en',
  messages: {
    en: en,
    'zh-CN': zhCN, // 繁体中文
    ko: ko,       // 韩文
  }
})

export default i18n