# 🏗️ HelixUI 组件架构设计

## 📋 概述

HelixUI 采用分层架构设计，通过组件复用和类型安全的方式，构建了一个现代化、可扩展的 UI 组件库。本文档详细介绍了组件的架构设计和实现原理。

## 🎯 设计原则

### 1. 分层架构
- **基础层** - 提供核心功能（Overlay）
- **中间层** - 提供通用功能（PopupBase, NoticeBase）
- **应用层** - 提供具体组件（Modal, Dialog, Message）

### 2. 组件复用
- 避免重复代码
- 保持 API 一致性
- 确保类型安全

### 3. 跨平台兼容
- 支持 Web/Desktop/App
- 响应式设计
- 平台适配

## 🏗️ 反馈组件架构

### 📊 架构图

```
Overlay（Portal + zIndex + 动画）
├── PopupBase（可交互类浮层）
│    ├── Dialog（对话框）
│    │    └── Modal（模态框）
│    └── Drawer / Popover / Tooltip（待实现）
└── NoticeBase（轻提示类浮层）
     ├── Message（消息）
     └── Notification / Snackbar（待实现）
```

### 🔧 各层职责

#### 1. Overlay 基础层
**文件位置**: `helixui-lib/src/components/feedback/overlay.rs`

**核心功能**:
- Portal 渲染
- z-index 管理
- 动画支持
- 跨平台适配

**关键特性**:
```rust
pub struct OverlayProps {
    pub visible: bool,
    pub z_index_offset: u32,
    pub show_mask: bool,
    pub mask_closable: bool,
    pub keyboard_enabled: bool,
    pub touch_enabled: bool,
    // ...
}
```

**平台适配**:
- Web: 使用 DOM API
- Desktop: 使用原生窗口 API
- Mobile: 支持手势操作

#### 2. PopupBase 可交互浮层
**文件位置**: `helixui-lib/src/components/feedback/popup_base.rs`

**核心功能**:
- 基于 Overlay 的可交互浮层
- 响应式尺寸控制
- 拖拽支持
- 手势关闭

**关键特性**:
```rust
pub struct PopupBaseProps {
    pub visible: bool,
    pub position: String,
    pub size: Option<ResponsiveSize>,
    pub draggable: bool,
    pub transform_origin: String,
    pub title: Option<String>,
    // ...
}
```

**响应式设计**:
```rust
pub struct ResponsiveSize {
    pub mobile: Option<String>,
    pub tablet: Option<String>,
    pub desktop: Option<String>,
    pub large: Option<String>,
    pub xlarge: Option<String>,
    pub xxlarge: Option<String>,
}
```

#### 3. Dialog 对话框
**文件位置**: `helixui-lib/src/components/feedback/dialog.rs`

**核心功能**:
- 基于 PopupBase 的对话框
- 类型图标支持
- 尺寸和位置控制

**类型系统**:
```rust
pub enum DialogType {
    Default,
    Confirm,
    Info,
    Success,
    Warning,
    Error,
}
```

#### 4. Modal 模态框
**文件位置**: `helixui-lib/src/components/feedback/modal.rs`

**核心功能**:
- 基于 Dialog 的模态框
- 确认/取消按钮
- 事件处理

**类型别名**:
```rust
pub type ModalSize = DialogSize;
pub type ModalPosition = DialogPosition;
pub type ModalType = DialogType;
```

#### 5. NoticeBase 轻提示浮层
**文件位置**: `helixui-lib/src/components/feedback/notice_base.rs`

**核心功能**:
- 基于 Overlay 的轻提示浮层
- 自动关闭
- 手势支持
- 震动反馈

**关键特性**:
```rust
pub struct NoticeBaseProps {
    pub visible: bool,
    pub position: String,
    pub duration: u32,
    pub closable: bool,
    pub notice_type: String,
    pub gesture_close: bool,
    pub haptic_feedback: bool,
    // ...
}
```

#### 6. Message 消息
**文件位置**: `helixui-lib/src/components/feedback/message.rs`

**核心功能**:
- 基于 NoticeBase 的消息组件
- 类型别名
- 管理器支持

**类型别名**:
```rust
pub type MessageType = NoticeType;
pub type MessagePosition = NoticePosition;
```

## 🔄 组件复用机制

### 类型别名
通过类型别名实现组件间的类型共享：

```rust
// Modal 使用 Dialog 的类型
pub type ModalSize = DialogSize;
pub type ModalPosition = DialogPosition;
pub type ModalType = DialogType;

// Message 使用 NoticeBase 的类型
pub type MessageType = NoticeType;
pub type MessagePosition = NoticePosition;
```

### 组件组合
通过组件组合实现功能复用：

```rust
// Modal 组件内部使用 Dialog
#[component]
pub fn Modal(props: ModalProps) -> Element {
    rsx! {
        Dialog {
            visible: props.visible,
            title: props.title.clone(),
            size: props.size,
            position: props.position,
            dialog_type: props.modal_type,
            // ... 其他属性
            children: rsx! {
                // Modal 特有的内容
                {props.children}
                
                // 按钮区域
                if props.show_confirm || props.show_cancel {
                    // 按钮实现
                }
            }
        }
    }
}
```

## 🌐 跨平台支持

### 平台检测
```rust
#[derive(Debug, Clone)]
pub enum Platform {
    Web,
    Desktop,
    Mobile,
}

pub struct PlatformAdapter {
    pub platform: Platform,
    pub z_index_base: u32,
    pub animation_duration: u32,
    pub mask_class: String,
}
```

### 响应式断点
```rust
#[derive(Debug, Clone, Copy)]
pub enum Breakpoint {
    Mobile,    // < 640px
    Tablet,    // 640px - 1024px
    Desktop,   // 1024px - 1280px
    Large,     // 1280px - 1536px
    XLarge,    // 1536px - 1920px
    XXLarge,   // > 1920px
}
```

## 📱 响应式设计

### 断点系统
```rust
impl Breakpoint {
    pub fn from_width(width: u32) -> Self {
        match width {
            0..=639 => Breakpoint::Mobile,
            640..=1023 => Breakpoint::Tablet,
            1024..=1279 => Breakpoint::Desktop,
            1280..=1535 => Breakpoint::Large,
            1536..=1919 => Breakpoint::XLarge,
            _ => Breakpoint::XXLarge,
        }
    }
}
```

### 响应式尺寸
```rust
impl ResponsiveSize {
    pub fn new() -> Self {
        Self {
            mobile: None,
            tablet: None,
            desktop: None,
            large: None,
            xlarge: None,
            xxlarge: None,
        }
    }
    
    pub fn with_responsive(
        mobile: Option<&str>,
        tablet: Option<&str>,
        desktop: Option<&str>,
        large: Option<&str>,
        xlarge: Option<&str>,
        xxlarge: Option<&str>,
    ) -> Self {
        Self {
            mobile: mobile.map(|s| s.to_string()),
            tablet: tablet.map(|s| s.to_string()),
            desktop: desktop.map(|s| s.to_string()),
            large: large.map(|s| s.to_string()),
            xlarge: xlarge.map(|s| s.to_string()),
            xxlarge: xxlarge.map(|s| s.to_string()),
        }
    }
}
```

## 🎨 样式系统

### 主题支持
- 深色模式自动适配
- 自定义主题变量
- 响应式样式类

### 动画系统
- CSS 过渡动画
- 变换原点控制
- 平台优化动画

## 🔧 扩展指南

### 添加新的 PopupBase 子组件
1. 创建新的组件文件
2. 定义组件 Props
3. 使用 PopupBase 作为基础
4. 添加特定的功能

### 添加新的 NoticeBase 子组件
1. 创建新的组件文件
2. 定义组件 Props
3. 使用 NoticeBase 作为基础
4. 添加特定的功能

### 自定义平台适配
1. 扩展 PlatformAdapter
2. 添加平台特定逻辑
3. 更新平台检测

## 📚 最佳实践

### 1. 组件设计
- 保持单一职责
- 使用组合而非继承
- 提供合理的默认值

### 2. 类型安全
- 使用类型别名
- 避免 Any 类型
- 提供编译时检查

### 3. 性能优化
- 按需加载
- 避免不必要的重渲染
- 使用 memo 优化

### 4. 可访问性
- 支持键盘导航
- 提供 ARIA 标签
- 支持屏幕阅读器

## 🚀 未来规划

### 短期目标
- [ ] 实现 Drawer 组件
- [ ] 实现 Popover 组件
- [ ] 实现 Tooltip 组件
- [ ] 实现 Notification 组件
- [ ] 实现 Snackbar 组件

### 长期目标
- [ ] 添加更多平台支持
- [ ] 优化动画性能
- [ ] 增强可访问性
- [ ] 添加主题定制工具

## 📖 相关文档

- [组件 API 文档](./docs/components.md)
- [主题定制指南](./docs/theming.md)
- [响应式设计指南](./docs/responsive.md)
- [可访问性指南](./docs/accessibility.md)
