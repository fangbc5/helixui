# HelixUI

一个现代化的 Dioxus UI 组件库，提供美观、响应式的组件。

## 特性

- 🎨 **现代化设计** - 简洁美观的组件设计
- 🌙 **深色模式支持** - 内置深色/浅色主题切换
- 📱 **响应式设计** - 适配各种屏幕尺寸
- 🚀 **基于 Dioxus** - 使用 Rust 构建的高性能前端框架
- 🎯 **类型安全** - 编译时类型检查，减少运行时错误
- ⚡ **高性能** - 优化的渲染性能和内存使用

## 快速开始

### 安装

在你的 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
helixui = "0.1.0"
dioxus = { version = "0.6.0", features = ["web"] }
```

### 基本用法

```rust
use helixui::components::{Button, ButtonType, Icon, IconType};
use dioxus::prelude::*;

fn App() -> Element {
    rsx! {
        div {
            class: "p-8 space-y-4",
            
            // 按钮组件
            Button {
                button_type: ButtonType::Primary,
                "点击我"
            }
            
            // 图标组件
            Icon { 
                icon: IconType::Home,
                size: IconSize::Large 
            }
        }
    }
}
```

## 组件

### 通用组件 (Common)

- **Button** - 按钮组件，支持多种类型和尺寸
- **Icon** - 图标组件，提供丰富的 SVG 图标库

### 反馈组件 (Feedback)

- **Badge** - 徽章组件，用于显示通知数量
- **Dialog** - 对话框组件，支持多种类型
- **Message** - 消息组件，用于显示提示信息

## 主题

HelixUI 支持深色和浅色主题，组件会自动适配当前主题：

```rust
// 组件会自动使用当前主题的颜色
Button {
    button_type: ButtonType::Primary,
    "主题适配按钮"
}
```

## 文档

- [在线文档](https://docs.rs/helixui)
- [组件示例](https://github.com/fangbaichun/helixui/tree/main/helixui-website)
- [GitHub 仓库](https://github.com/fangbaichun/helixui)

## 许可证

MIT License - 详见 [LICENSE](LICENSE) 文件

## 贡献

欢迎提交 Issue 和 Pull Request！

## 更新日志

### 0.1.0

- 初始版本发布
- 包含基础组件：Button, Icon, Badge, Dialog, Message
- 支持深色/浅色主题
- 完整的文档和示例
