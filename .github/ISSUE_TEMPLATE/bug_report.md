---
name: 🐛 Bug 报告
about: 创建一个 Bug 报告来帮助我们改进
title: '[BUG] '
labels: ['bug', 'needs-triage']
assignees: ''

---

## 🐛 Bug 描述
简要描述这个 Bug。

## 🔄 复现步骤
描述如何复现这个问题：

1. 进入 '...'
2. 点击 '....'
3. 滚动到 '....'
4. 看到错误

## 🎯 期望行为
清楚简洁地描述你期望发生的事情。

## 📸 截图
如果适用，添加截图来帮助解释你的问题。

## 🖥️ 环境信息
请填写以下信息：

- **操作系统**: [例如 Windows 10, macOS 12.0, Ubuntu 20.04]
- **Rust 版本**: [例如 1.70.0]
- **Dioxus 版本**: [例如 0.6.0]
- **HelixUI 版本**: [例如 0.1.0]
- **浏览器**: [如果是 Web 应用，例如 Chrome 91, Firefox 89]

## 📝 代码示例
如果可能，提供一个最小化的代码示例来复现问题：

```rust
use helixui::components::{Button, ButtonType};
use dioxus::prelude::*;

fn App() -> Element {
    rsx! {
        Button {
            button_type: ButtonType::Primary,
            "测试按钮"
        }
    }
}
```

## 📋 额外信息
添加任何其他关于问题的上下文信息。

## ✅ 检查清单
- [ ] 我已经搜索了现有的 Issues
- [ ] 我已经确认这是一个 Bug 而不是功能请求
- [ ] 我已经提供了足够的信息来复现问题
