這是一份為你的 GitHub 專案量身打造的 **`README.md`** 檔案。內容完整記錄了專案特色、技術棧、目錄結構、開發環境設定、建置部署流程，以及 PWA 版本更新的維護注意事項。

你可以直接複製下方內容並儲存為專案根目錄下的 `README.md`：

```markdown
# 🧬 Leptos + WebAssembly - Conway's Game of Life (PWA)

一個基於 **Rust**、**Leptos (CSR)** 與 **WebAssembly** 開發的高效能「康威生命遊戲（Conway's Game of Life）」網頁應用程式。支援響應式介面（RWD）與 PWA（Progressive Web App）離線存取，可直接安裝至手機與桌面，並具備自動偵測新版本更新機制。

---

## ✨ 專案亮點

- ⚡ **超高運算效能**：核心遊戲邏輯採用 Rust 編譯為 WebAssembly (WASM) 執行。
- 🎨 **現代化動態介面**：利用 Leptos 響應式框架（Signals）打造簡潔且高效的 UI 控制介面。
- 📱 **響應式與精準點擊**：畫布（Canvas）自動適應電腦與手機螢幕，並經過 CSS 座標轉置演算，確保手機觸控點擊細胞時精準無誤。
- 📲 **PWA 支援**：支援離線遊玩、安裝至手機主畫面獨立運行。
- 🚀 **版本更新提示**：內建 Service Worker 版本監聽，發布新版時自動跳出「🚀 發現新版本！」一鍵更新提示。

---

## 🛠️ 技術棧 (Tech Stack)

- **前端框架**：[Leptos](https://leptos.dev/) (Client-Side Rendering)
- **程式語言**：Rust / WebAssembly
- **打包工具**：[Trunk](https://trunkrs.dev/)
- **樣式與繪圖**：CSS3 / HTML5 Canvas API
- **離線與安裝**：PWA (Service Worker + Manifest)

---

## 📁 專案目錄結構

```text
.
├── src/
│   ├── components/
│   │   └── game_canvas.rs  # Canvas 繪圖邏輯、點擊座標轉換與操作控制按鈕
│   ├── game/               # 康威生命遊戲的核心資料結構與演算法 logic
│   ├── lib.rs              # 元件入口與主頁面元件
│   └── main.rs             # 應用程式啟動點
├── assets/
│   ├── style.css           # 樣式表 (深色模式與自適應排版)
│   ├── icon-192.png        # PWA 圖示 (小)
│   └── icon-512.png        # PWA 圖示 (大)
├── index.html              # 應用程式 HTML 載入點與 Service Worker 註冊腳本
├── manifest.json           # PWA 設定檔 (APP 名稱、圖示、主題顏色)
├── sw.js                   # Service Worker 快取與跳過等待機制
├── Cargo.toml              # Rust 依賴套件設定
└── Trunk.toml              # Trunk 建置設定 (可選)

```

---

## 🚀 本地開發環境設置 (Local Development)

### 1. 前置需求

請確保本機已安裝以下環境：

* **Rust**: [https://www.rust-lang.org/](https://www.rust-lang.org/)
* **WASM 構建目標**:
```bash
rustup target add wasm32-unknown-unknown

```


* **Trunk** (Rust WebApp 打包工具):
```bash
cargo install trunk

```



### 2. 啟動開發伺服器

```bash
# 複製專案
git clone [https://github.com/YOUR_USERNAME/YOUR_REPO_NAME.git](https://github.com/YOUR_USERNAME/YOUR_REPO_NAME.git)
cd YOUR_REPO_NAME

# 啟動 Trunk 開發伺服器 (具備 Hot Reload 功能)
trunk serve

```

開啟瀏覽器前往 `http://127.0.0.1:8080` 即可預覽應用程式。

---

## 📦 打包與部署 (Build & Deployment)

### 1. 手動生產環境打包

執行以下命令進行 release 優化編譯：

```bash
trunk build --release

```

編譯後的靜態檔案將會生成於 `dist/` 資料夾內，可直接部署至 GitHub Pages、Vercel 或 Cloudflare Pages。

### 2. 部署至 GitHub Pages 範例

若要發布至 GitHub Pages，確保將 `dist/` 資料夾內容推送至 `gh-pages` 分支即可。

---

## 🔧 專案維護與版本更新流程 (PWA Maintenance)

當你修改了程式碼並準備發布新版本時，為了確保手機/電腦上的 PWA 能順利抓到最新檔案而非持續使用舊快取，請遵循以下步驟：

1. **更新 `sw.js` 版本號**：
開啟 `sw.js`，修改頂部的 `CACHE_NAME`（例如由 `game-of-life-v1` 改為 `game-of-life-v2`）：
```javascript
const CACHE_NAME = 'game-of-life-v2';

```


2. **重新編譯與部署**：
執行 `trunk build --release` 並將成果更新至伺服器。
3. **使用者端效果**：
使用者再次開啟 App 時，Service Worker 會自動偵測到 `sw.js` 的變更，並在頁面下方跳出 **「🚀 發現新版本！ [立即更新]」** 提示按鈕。使用者點擊後頁面將無痛刷新並套用最新程式碼。

---

## 💡 開發經驗筆記 (Lessons Learned)

1. **Canvas 點擊座標還原**：
當 Canvas 經過 CSS 縮放 (Width 100%) 後，`client_x` / `client_y` 需乘以「真實解析度與 CSS 渲染尺寸的比例（Scale factor）」，才能正確換算回網格的行與列索引。
2. **Service Worker 作用域**：
`index.html` 註冊 Service Worker 時建議使用相對路徑 `./sw.js`，以避免在 GitHub Pages 等帶有子路徑（Subpath）的環境下出現 `404 Not Found` 註冊失敗。

```

```