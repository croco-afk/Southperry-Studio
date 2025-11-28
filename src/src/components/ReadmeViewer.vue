<!-- src/components/ReadmeViewer.vue -->
<template>
  <div class="readme-container">
    <!-- 增加一个容器 wrapper 用于居中和限制宽度 -->
    <div class="markdown-wrapper">
      <div class="markdown-body" v-html="renderedContent"></div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue';
import { marked } from 'marked';
import { useI18n } from 'vue-i18n';

const { locale } = useI18n();
const renderedContent = ref('');

// 1. 配置 marked
const renderer = new marked.Renderer();

// 🔴 修复点：在新版 marked 中，参数是一个对象，需要用 { } 进行解构
renderer.link = ({ href, title, text }) => {
  const titleAttr = title ? ` title="${title}"` : '';
  const relAttr = ' rel="noopener noreferrer"';
  const targetAttr = ' target="_blank"';
  
  // 确保 href 存在，防止极其边缘的情况
  const safeHref = href || '#';
  // 确保 text 存在
  const safeText = text || safeHref;

  return `<a href="${safeHref}"${titleAttr}${targetAttr}${relAttr}>${safeText}</a>`;
};

marked.setOptions({
  renderer: renderer,
  gfm: true,
  breaks: true, // 建议开启换行符支持
});

// 加载并解析 MD 文件
const loadContent = async (lang) => {
  try {
    const effectiveLang = lang || 'en';
    const fileName = `README.${effectiveLang}.md`;
    console.log(`Loading: ${fileName}`); // 调试用
    
    const response = await fetch(`/${fileName}`);
    
    if (!response.ok) {
        console.warn(`[ReadmeViewer] Failed to load ${fileName}. Falling back to README.en.md`);
        const fallbackResponse = await fetch('/README.en.md');
        if (fallbackResponse.ok) {
             const text = await fallbackResponse.text();
             renderedContent.value = marked.parse(text);
             return;
        }
      renderedContent.value = `<h3>Failed to load ${fileName}</h3>`;
      return;
    }
    
    const text = await response.text();
    renderedContent.value = marked.parse(text);
  } catch (e) {
    renderedContent.value = `<h3>Error loading readme: ${e}</h3>`;
  }
};

watch(locale, (newLang) => {
  loadContent(newLang);
}, { immediate: true });
</script>


<style scoped>
.readme-container {
  height: 100%;
  background: #ffffff;
  overflow-y: auto; /* 滚动条在最外层 */
  scroll-behavior: smooth;
}

/* 限制阅读宽度的容器 */
.markdown-wrapper {
  max-width: 800px; /* 黄金阅读宽度 */
  margin: 0 auto;
  padding: 40px 24px 80px; /* 底部留白多一点 */
}

/* --- 排版基础 --- */
.markdown-body {
  font-family: Inter, sans-serif;
  font-size: 16px;
  line-height: 1.3; /* 增加行高，提升阅读体验 */
  color: #24292f;   /* GitHub 经典的深灰，不是纯黑 */
}

/* --- 标题 --- */
.markdown-body :deep(h1),
.markdown-body :deep(h2),
.markdown-body :deep(h3) {
  margin-top: 24px;
  margin-bottom: 16px;
  font-weight: 600;
  line-height: 1.25;
  color: #1f2328;
}

.markdown-body :deep(h1) {
  font-size: 2em;
  padding-bottom: 0.3em;
  border-bottom: 1px solid #d0d7de;
}

.markdown-body :deep(h2) {
  font-size: 1.5em;
  padding-bottom: 0.3em;
  border-bottom: 1px solid #d0d7de;
}

/* --- 段落与列表 --- */
.markdown-body :deep(p) {
  margin-bottom: 16px;
  color: black;
}

.markdown-body :deep(ul),
.markdown-body :deep(ol) {
  padding-left: 2em;
  margin-bottom: 16px;
}

.markdown-body :deep(li) {
  margin-top: 0.25em;
}

/* --- 链接 --- */
.markdown-body :deep(a) {
  color: #0969da;
  text-decoration: none;
}
.markdown-body :deep(a:hover) {
  text-decoration: underline;
}

/* --- 引用块 --- */
.markdown-body :deep(blockquote) {
  margin: 0 0 16px;
  padding: 0 1em;
  color: #57606a;
  border-left: 0.25em solid #d0d7de;
  background-color: #ffffff; /* 或者淡灰色 #f8f9fa */
}

/* --- 代码块 (重点优化) --- */
/* 行内代码 */
.markdown-body :deep(code) {
  padding: 0.2em 0.4em;
  margin: 0;
  font-size: 85%;
  font-family: "SFMono-Regular", Consolas, "Liberation Mono", Menlo, monospace;
  background-color: #eff1f3; /* 浅灰色背景 */
  border-radius: 6px;
  color: #24292f;
}

/* 多行代码块 */
.markdown-body :deep(pre) {
  padding: 16px;
  overflow: auto;
  font-size: 85%;
  line-height: 1.45;
  background-color: #161b22; /* 深色背景，更专业 */
  border-radius: 8px; /* 圆角 */
  margin-bottom: 16px;
  box-shadow: 0 4px 6px rgba(0,0,0,0.05); /* 轻微阴影 */
}

.markdown-body :deep(pre code) {
  background-color: transparent; /* 移除 pre 内部 code 的背景 */
  color: #e6edf3; /* 代码文字颜色变白 */
  padding: 0;
  border-radius: 0;
}

/* --- 图片 --- */
.markdown-body :deep(img) {
  max-width: 100%;
  box-sizing: content-box;
  background-color: #ffffff;
  border-radius: 6px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.1); /* 给图片加点阴影和质感 */
  margin: 16px 0;
}

/* --- 表格 --- */
.markdown-body :deep(table) {
  border-spacing: 0;
  border-collapse: collapse;
  margin-bottom: 16px;
  width: 100%;
  overflow: auto;
}
.markdown-body :deep(th),
.markdown-body :deep(td) {
  padding: 6px 13px;
  border: 1px solid #d0d7de;
}
.markdown-body :deep(tr:nth-child(2n)) {
  background-color: #f6f8fa; /* 斑马纹 */
}
</style>