### MapleLens

这是一个用于预览并且提取 Maplestory Wz 文件中技能动图的工具，主要供《Southperry》玩家自定义技能时获取素材使用。

DIY 技能地址：http://southperry.site/custom-skills

**核心功能**

- WZ 解析与检索： 支持拖拽加载 Base.wz，可通过职业、ID或名称快速查找技能。
- 预览： 还原技能动画与音效，支持显示原点（Anchor）
- 一键资源提取： 批量导出技能图标、音频及 WebP 动图，可自选保存目录。
- 开发适配导出： 导出的动图附带包含坐标 (origin) 与延迟 (delay) 的 JSON 文件，支持“最小裁剪”或“等尺寸”模式。
- 多语言支持： 完整支持中文、英文、韩文界面切换。

**WZ 资源获取**

安装任意版本的 Maplestory 客户端，并将安装目录下的 Base.wz 文件拖拽至 MapleLens 程序中即可。
寻找不同版本的 Maplestory 客户端下载，可以参考下面这几篇帖子：
https://forum.ragezone.com/threads/maplestory-client-localhost-archive.1101897/
https://archive.org/details/twms-maplestory

**技术栈**
- 前端: Vue 3, Vite
- 后端: Rust (Tauri v2)

**致谢与来源 (Acknowledgement & Source):**

本项目的**核心数据处理后端**（Wz 文件解析、节点查询、图片/音频提取等功能）**直接使用了 [MapleSalon2](https://github.com/spd789562/MapleSalon2) 的 Rust 后端代码。**

感谢原项目作者 Leo Lin 的杰出工作。

**原项目链接:** https://github.com/spd789562/MapleSalon2
