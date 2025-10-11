# HelixUI

一个现代化的 Dioxus UI 组件库，提供美观、响应式的组件。

## 项目结构

这是一个 Rust 工作空间项目，包含两个主要部分：

### 📦 helixui-lib

独立的 UI 组件库，可以被其他项目引用使用。

**特性：**
- 🎨 现代化设计
- 🌙 深色模式支持
- 📱 响应式设计
- 🚀 基于 Dioxus 构建
- 🎯 类型安全且高性能

**组件分类：**
- `common/` - 通用组件（Button, Icon 等）
- `feedback/` - 反馈组件（Badge, Dialog, Message 等）
- `layout/` - 布局组件（Logo, Footer, Navbar 等）

### 🌐 helixui-website

官方文档网站，展示所有组件的使用方法和示例。

**功能：**
- 📚 完整的组件文档
- 🎮 交互式示例
- 🔍 组件搜索和分类
- 📖 使用指南和快速开始

## 快速开始

### 使用组件库

在你的 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
helixui = { path = "path/to/helixui-lib" }
dioxus = { version = "0.6.0", features = ["web"] }
```

### 基本用法

```rust
use helixui::components::{Button, ButtonType};
use dioxus::prelude::*;

fn App() -> Element {
    rsx! {
        Button {
            button_type: ButtonType::Primary,
            "Hello HelixUI!"
        }
    }
}
```

### 运行文档网站

```bash
# 运行网站
cargo run -p helixui-website

# 或者构建
cargo build -p helixui-website
```

## 开发

### 构建所有项目

```bash
cargo build
```

### 检查所有项目

```bash
cargo check
```

### 运行测试

```bash
cargo test
```

## 许可证

MIT License