### MapleLens

This is a tool designed to preview and extract skill animations from Maplestory Wz files. It is primarily intended for "Southperry" players to acquire assets for creating custom skills.

DIY Skills URL: http://southperry.site/custom-skills

**Core Features**

- **WZ Parsing & Search:** Supports drag-and-drop loading of `Base.wz`. Quickly search for skills by Job, ID, or Name.
- **Preview:** Restores skill animations and sound effects, with support for displaying the Anchor point.
- **One-Click Resource Extraction:** Batch export skill icons, audio, and WebP animations. Supports custom save directories.
- **Developer-Ready Export:** Exported animations include a JSON file containing coordinates (`origin`) and delays (`delay`). Supports "Minimal Crop" or "Canvas Size" modes.
- **Multi-language Support:** Full support for switching between Chinese, English, and Korean interfaces.

**WZ Resource Acquisition**

Install any version of the Maplestory client and drag the `Base.wz` file from the installation directory into the MapleLens application.
To find different versions of the Maplestory client for download, you can refer to following threads:
https://forum.ragezone.com/threads/maplestory-client-localhost-archive.1101897/
https://archive.org/details/twms-maplestory

**Tech Stack**
- Frontend: Vue 3, Vite
- Backend: Rust (Tauri v2)

**Acknowledgement & Source:**

The **core backend data processing** of this project (Wz file parsing, node querying, image/audio extraction, etc.) **directly utilizes the Rust backend code from [MapleSalon2](https://github.com/spd789562/MapleSalon2).**

Special thanks to the original author, Leo Lin, for his outstanding work.

**Original Project Link:** https://github.com/spd789562/MapleSalon2