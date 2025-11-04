# SEO Image Specifications

## 📸 **og-image.png** (Open Graph Image)

**Purpose**: Preview image when sharing on social platforms such as Facebook, LinkedIn, WhatsApp, etc.

**Specification Requirements**:

- **Dimensions**: 1200×630 pixels (recommended)
- **Aspect ratio**: 1.91:1
- **File size**: < 8MB (recommended < 300KB)
- **File format**: PNG, JPG, GIF
- **Minimum dimensions**: 600×315 pixels
- **Maximum dimensions**: 1200×630 pixels

**Design Suggestions**:

- Include brand logo and product name
- Simple background with key feature description
- Text should be large and clear (readable on mobile)

## 🐦 **twitter-card.png** (Twitter Card Image)

**Purpose**: Preview image when sharing on Twitter

**Specification Requirements**:

- **Dimensions**: 1200×675 pixels (recommended)
- **Aspect ratio**: 16:9 (summary_large_image)
- **File size**: < 5MB
- **File format**: PNG, JPG, WEBP, GIF
- **Minimum dimensions**: 300×157 pixels

**Design Suggestions**:

- Can be the same as og-image or optimized for Twitter
- Consider adaptation for Twitter's dark/light themes

## 🖼️ **screenshot.png** (App Screenshot)

**Purpose**: Application screenshot in structured data, showing actual interface

**Specification Requirements**:

- **Dimensions**: Recommended 1280×720 or 1920×1080 pixels
- **Aspect ratio**: 16:9 or 16:10
- **File size**: < 1MB (recommended)
- **File format**: PNG (maintains clarity)

**Design Suggestions**:

- Show the main application interface
- Highlight core features
- High-quality actual usage screenshot

---

## 🎨 **Recommended Design Content for the peek Project**

Based on your HTTP Inspector tool, these images should include:

### **og-image.png & twitter-card.png**

- peek logo
- "Modern HTTP Inspector" title
- Simple API request example or interface preview
- Key features: Fast, Open Source, Cross-platform
- Brand color: #7c3aed (purple theme)

### **screenshot.png**

- Main interface screenshot of the peek application
- Show a complete HTTP request flow
- Include request panel, response panel, headers, etc.
- Display formatted JSON response

## 🛠️ **Quick Generation Tips**

If you need to quickly generate these images:

1. **Use design tools**: Figma, Canva, Adobe Creative Suite
2. **Online tools**:
   - Social Media Image Generator
   - Bannercreator.com
   - Crello
3. **Programmatic generation**: Use Puppeteer or Playwright to screenshot your application

Would you like me to help create a simple HTML template to generate these images?
