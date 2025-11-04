# SEO 图片规范

## 📸 **og-image.png** (Open Graph Image)

**用途**: Facebook、LinkedIn、WhatsApp 等社交平台分享时的预览图

**规范要求**:

- **尺寸**: 1200×630 像素 (推荐)
- **宽高比**: 1.91:1
- **文件大小**: < 8MB (推荐 < 300KB)
- **文件格式**: PNG, JPG, GIF
- **最小尺寸**: 600×315 像素
- **最大尺寸**: 1200×630 像素

**设计建议**:

- 包含品牌 logo 和产品名称
- 简洁的背景和主要功能说明
- 文字要大且清晰（移动端也能看清）

## 🐦 **twitter-card.png** (Twitter Card Image)

**用途**: Twitter 分享时的预览图

**规范要求**:

- **尺寸**: 1200×675 像素 (推荐)
- **宽高比**: 16:9 (summary_large_image)
- **文件大小**: < 5MB
- **文件格式**: PNG, JPG, WEBP, GIF
- **最小尺寸**: 300×157 像素

**设计建议**:

- 可以与 og-image 相同，或针对 Twitter 优化
- 考虑 Twitter 的深色/浅色主题适配

## 🖼️ **screenshot.png** (App Screenshot)

**用途**: 结构化数据中的应用截图，展示实际界面

**规范要求**:

- **尺寸**: 建议 1280×720 或 1920×1080 像素
- **宽高比**: 16:9 或 16:10
- **文件大小**: < 1MB (推荐)
- **文件格式**: PNG (保持清晰度)

**设计建议**:

- 展示应用的主要界面
- 突出核心功能
- 高质量的实际使用截图

---

## 🎨 **为 peek 项目的建议设计内容**

基于您的 HTTP Inspector 工具，建议这些图片包含：

### **og-image.png & twitter-card.png**

- peek logo
- "Modern HTTP Inspector" 标题
- 简洁的 API 请求示例或界面预览
- 关键特性：Fast, Open Source, Cross-platform
- 品牌色彩：#7c3aed (紫色主题)

### **screenshot.png**

- peek 应用的主界面截图
- 显示一个 HTTP 请求的完整流程
- 包含请求面板、响应面板、headers 等
- 展示 JSON 格式化后的响应

## 🛠️ **快速生成建议**

如果您需要快速生成这些图片，可以：

1. **使用设计工具**: Figma, Canva, Adobe Creative Suite
2. **在线工具**:
   - Social Media Image Generator
   - Bannercreator.com
   - Crello
3. **程序化生成**: 使用 Puppeteer 或 Playwright 截图您的应用

需要我帮您创建一个简单的 HTML 模板来生成这些图片吗？
