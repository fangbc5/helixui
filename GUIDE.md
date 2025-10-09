# Helix UI - Dioxus 组件库

Dioxus 组件库，提供优雅的组件和文档。

## 项目结构

```
src/
├── components/          # UI 组件
│   ├── button.rs       # Button 组件
│   ├── hero.rs
│   ├── logo.rs
│   └── mod.rs
├── views/              # 页面视图
│   ├── components/     # 组件文档页面
│   │   ├── button_page.rs
│   │   └── mod.rs
│   ├── layout.rs       # 文档布局（左侧导航、右侧目录）
│   ├── home.rs         # 首页
│   ├── navbar.rs       # 导航栏
│   └── mod.rs
└── main.rs             # 应用入口和路由
```

## 运行项目

### 1. 启动 Tailwind CSS 监听

在一个终端窗口中运行：

```bash
npm run css:watch
```

### 2. 启动开发服务器

在另一个终端窗口中运行：

#### Web 版本
```bash
dx serve --platform web
```

#### Desktop 版本
```bash
dx serve --platform desktop
```

## 访问页面

- **首页**: http://localhost:8080/
- **Button 组件文档**: http://localhost:8080/components/button

## 组件列表

### 已实现
- ✅ Button - 按钮组件，支持多种类型和尺寸

### 计划中
- ⏳ Avatar - 头像组件
- ⏳ Card - 卡片组件
- ⏳ Input - 输入框组件
- ⏳ Select - 选择器组件

## 添加新组件

### 1. 创建组件文件

在 `src/components/` 目录下创建新的组件文件，例如 `avatar.rs`:

```rust
use dioxus::prelude::*;

#[component]
pub fn Avatar(src: String, size: Option<String>) -> Element {
    rsx! {
        img {
            src: src,
            class: "rounded-full",
            // ... 其他属性
        }
    }
}
```

### 2. 导出组件

在 `src/components/mod.rs` 中添加：

```rust
mod avatar;
pub use avatar::Avatar;
```

### 3. 创建文档页面

在 `src/views/components/` 目录下创建文档页面，例如 `avatar_page.rs`。

### 4. 添加路由

在 `src/main.rs` 中添加新路由：

```rust
#[layout(DocLayout)]
    #[route("/components/avatar")]
    AvatarPage {},
```

### 5. 更新导航

在 `src/views/layout.rs` 的 `LeftSidebar` 组件中添加导航项。

## 技术栈

- **Dioxus 0.6.0** - Rust GUI 框架
- **Tailwind CSS 3.x** - CSS 框架
- **Asset Macro** - 静态资源管理

## 开发建议

1. 使用 `asset!` 宏引用所有静态资源（图片、CSS 等）
2. 组件应该使用 `#[component]` 宏
3. 在 `rsx!` 宏外部处理复杂逻辑
4. 保持组件的属性类型简单清晰

## 特色功能

- 📱 **多平台支持**: Web 和 Desktop
- 🎨 **优雅设计**: 参考 Naive UI 的设计风格
- 📖 **完整文档**: 每个组件都有详细的文档和示例
- ⚡ **高性能**: 基于 Dioxus 的虚拟 DOM

## 贡献

欢迎提交 Pull Request 来添加新组件或改进现有组件！

