# 反馈组件设计（Badge / Message / Dialog / Modal / Notification / Alert）

## 目标与设计原则

- **高内聚、低耦合**：每个组件聚焦单一职责，通过统一的主题系统保持视觉一致性
- **用户体验优先**：提供直观的反馈机制，支持多种交互方式
- **可访问性**：遵循 ARIA 标准，支持键盘导航和屏幕阅读器
- **主题化**：支持明暗主题切换，与整体设计系统保持一致
- **性能优化**：合理使用动画和过渡效果，避免性能问题

## 目录结构

```
helixui-lib/src/components/feedback/
  mod.rs
  badge.rs          # 标记组件
  message.rs        # 消息提示
  dialog.rs         # 对话框
  modal.rs          # 模态框
  notification.rs   # 通知（待实现）
  alert.rs          # 警告（待实现）

helixui-lib/src/overlay/
  mod.rs
  config/           # 配置系统
    overlay_config.rs       # 主配置
    z_index_config.rs       # Z-Index 配置
    responsive_config.rs    # 响应式配置
    simple_types.rs         # 简单类型定义
  core/             # 核心层
    state_manager.rs        # 状态管理
    event_system.rs         # 事件系统
    animation_manager.rs    # 动画管理
    theme_manager.rs        # 主题管理
    platform_adapter.rs     # 平台适配
  components/       # 组件层
    message_overlay.rs      # 消息覆盖层
    dialog_overlay.rs       # 对话框覆盖层
    modal_overlay.rs        # 模态框覆盖层
    notice_overlay.rs       # 通知覆盖层
    interactive_overlay.rs  # 交互式覆盖层
  plugins/          # 插件系统
    plugin_manager.rs       # 插件管理器
    animation_plugin.rs     # 动画插件
    theme_plugin.rs         # 主题插件
    accessibility_plugin.rs # 无障碍插件
  utils/            # 工具模块
    error_handler.rs        # 错误处理
    helpers.rs              # 辅助函数
```

## 组件设计

### 1. Badge（标记）

**用途**：用于显示状态标记或数量信息，通常附着在其他组件上。

**核心属性**：

- `value`: `Option<String>` - 显示的值
- `type`: `BadgeType` - 类型（Default, Success, Warning, Error, Info）
- `shape`: `BadgeShape` - 形状（Dot, Round, Square）
- `size`: `BadgeSize` - 尺寸（Small, Medium, Large）
- `max`: `Option<u32>` - 最大值，超过时显示 "max+"
- `processing`: `bool` - 是否显示处理中状态
- `offset`: `Option<(i32, i32)>` - 偏移量 (x, y)

**实现要点**：

- 使用绝对定位附着到父元素
- 支持数字徽章和点状徽章
- 处理中状态使用动画效果
- 支持自定义样式和位置

### 2. Message（消息）

**用途**：轻量级的全局提示反馈，用于显示操作结果或重要信息。

**核心属性**：

- `type`: `MessageType` - 类型（Info, Success, Warning, Error）
- `content`: `String` - 消息内容
- `duration`: `Option<u32>` - 显示时长（毫秒），None 表示不自动关闭
- `closable`: `bool` - 是否显示关闭按钮
- `icon`: `Option<Element>` - 自定义图标
- `on_close`: `Option<EventHandler<()>>` - 关闭回调

**实现要点**：

- 全局消息管理器，支持队列管理
- 自动计算位置，避免重叠
- 支持手动关闭和自动关闭
- 提供便捷的全局方法（`show_message`, `close_message`）

### 3. Dialog（对话框）

**用途**：模态对话框，在保留当前页面状态的情况下，告知用户并承载相关操作。

**核心属性**：

- `visible`: `bool` - 是否显示
- `title`: `Option<String>` - 标题
- `content`: `Element` - 内容
- `footer`: `Option<Element>` - 底部内容
- `width`: `Option<String>` - 宽度
- `closable`: `bool` - 是否显示关闭按钮
- `mask_closable`: `bool` - 点击遮罩是否关闭
- `on_close`: `Option<EventHandler<()>>` - 关闭回调

**实现要点**：

- 使用 Portal 渲染到 body
- 支持 ESC 键关闭
- 自动管理焦点和滚动锁定
- 支持自定义动画效果

### 4. Modal（模态框）

**用途**：模态对话框，用于显示重要信息或收集用户输入。

**核心属性**：

- `visible`: `bool` - 是否显示
- `title`: `Option<String>` - 标题
- `content`: `Element` - 内容
- `footer`: `Option<Element>` - 底部内容
- `width`: `Option<String>` - 宽度
- `centered`: `bool` - 是否垂直居中
- `closable`: `bool` - 是否显示关闭按钮
- `mask_closable`: `bool` - 点击遮罩是否关闭
- `on_ok`: `Option<EventHandler<()>>` - 确定回调
- `on_cancel`: `Option<EventHandler<()>>` - 取消回调

**实现要点**：

- 基于 Dialog 扩展，增加确定/取消按钮
- 支持表单验证和提交
- 提供确认对话框的便捷方法
- 支持自定义按钮文本和行为

### 5. Notification（通知）

**用途**：显示通知提醒消息，通常出现在页面角落。

**核心属性**：

- `type`: `NotificationType` - 类型（Info, Success, Warning, Error）
- `title`: `Option<String>` - 标题
- `content`: `String` - 内容
- `duration`: `Option<u32>` - 显示时长
- `closable`: `bool` - 是否显示关闭按钮
- `placement`: `NotificationPlacement` - 位置（TopLeft, TopRight, BottomLeft, BottomRight）
- `on_close`: `Option<EventHandler<()>>` - 关闭回调

**实现要点**：

- 全局通知管理器
- 支持多个通知同时显示
- 自动计算位置和动画
- 提供便捷的全局方法

### 6. Alert（警告）

**用途**：静态的信息提示，用于页面中的重要信息展示。

**核心属性**：

- `type`: `AlertType` - 类型（Info, Success, Warning, Error）
- `title`: `Option<String>` - 标题
- `content`: `Element` - 内容
- `closable`: `bool` - 是否显示关闭按钮
- `show_icon`: `bool` - 是否显示图标
- `banner`: `bool` - 是否为横幅样式
- `on_close`: `Option<EventHandler<()>>` - 关闭回调

**实现要点**：

- 内联组件，不占用额外层级
- 支持多种样式变体
- 可关闭和不可关闭两种模式
- 与页面内容自然融合

## 主题系统集成

### 颜色系统

```rust
pub enum FeedbackColor {
    Info,      // 信息 - 蓝色系
    Success,   // 成功 - 绿色系
    Warning,   // 警告 - 橙色系
    Error,     // 错误 - 红色系
    Default,   // 默认 - 灰色系
}
```

### 尺寸系统

```rust
pub enum FeedbackSize {
    Small,     // 小尺寸
    Medium,    // 中尺寸
    Large,     // 大尺寸
}
```

### 动画系统

- **淡入淡出**：用于消息和通知的显示/隐藏
- **滑动**：用于对话框的进入/退出
- **缩放**：用于模态框的打开/关闭
- **脉冲**：用于处理中状态的徽章

## 可访问性设计

### 键盘导航

- **Tab 键**：在可交互元素间切换焦点
- **Enter/Space**：激活按钮和链接
- **ESC 键**：关闭对话框和模态框
- **方向键**：在通知列表中导航

### ARIA 属性

- `role="alert"` - 用于重要消息
- `role="dialog"` - 用于对话框
- `aria-labelledby` - 关联标题
- `aria-describedby` - 关联描述
- `aria-expanded` - 表示展开状态
- `aria-hidden` - 隐藏装饰性元素

### 屏幕阅读器支持

- 提供有意义的文本描述
- 使用 `aria-live` 区域播报动态内容
- 确保焦点管理正确
- 提供关闭操作的明确指示

## 性能优化

### 渲染优化

- 使用 `Portal` 避免不必要的重渲染
- 实现虚拟化支持大量消息/通知
- 合理使用 `memo` 和 `use_memo`
- 避免在渲染过程中进行复杂计算

### 动画优化

- 使用 CSS 动画而非 JavaScript 动画
- 合理设置动画时长和缓动函数
- 支持 `prefers-reduced-motion` 媒体查询
- 避免同时触发多个动画

### 内存管理

- 及时清理事件监听器
- 合理管理全局状态
- 避免内存泄漏
- 提供清理方法

## 使用示例

### Badge 使用

```rust
rsx! {
    div {
        Button { "消息" }
        Badge {
            value: Some("5".to_string()),
            type: BadgeType::Error,
            shape: BadgeShape::Round,
        }
    }
}
```

### Message 使用

```rust
// 显示消息
show_message(MessageType::Success, "操作成功！".to_string());

// 自定义消息
rsx! {
    Message {
        type: MessageType::Info,
        content: "这是一条信息".to_string(),
        duration: Some(3000),
        closable: true,
    }
}
```

### Dialog 使用

```rust
rsx! {
    Dialog {
        visible: dialog_visible,
        title: Some("确认删除".to_string()),
        content: rsx! { "确定要删除这个项目吗？" },
        footer: rsx! {
            Button { onclick: move |_| set_dialog_visible(false), "取消" }
            Button { onclick: move |_| handle_delete(), "确定" }
        },
        on_close: move |_| set_dialog_visible(false),
    }
}
```

### Modal 使用

```rust
rsx! {
    Modal {
        visible: modal_visible,
        title: Some("编辑用户".to_string()),
        content: rsx! { /* 表单内容 */ },
        on_ok: move |_| handle_save(),
        on_cancel: move |_| set_modal_visible(false),
    }
}
```

## Overlay 系统架构

### 设计理念

Overlay 系统是一个通用的覆盖层管理系统，为所有反馈组件提供统一的底层支持。它采用**分层架构**设计，将配置、核心功能、组件实现和插件扩展分离，实现了高度的模块化和可扩展性。

### 核心特性

1. **统一状态管理**：全局状态管理器，跟踪所有 Overlay 的生命周期
2. **智能 Z-Index 管理**：自动分配和管理 Z-Index，避免层级冲突
3. **事件驱动架构**：基于事件总线的通信机制，组件间松耦合
4. **动画系统**：支持多种动画类型和缓动函数，可配置和扩展
5. **主题集成**：与全局主题系统深度集成，自动响应主题变化
6. **平台适配**：跨平台支持（Web、Desktop、Mobile），自动检测平台能力
7. **插件机制**：支持动画、主题、无障碍等插件的动态注册和扩展
8. **响应式设计**：支持断点配置，适配不同屏幕尺寸

### 架构层次

#### 1. 配置层（Config Layer）

负责提供各种配置项和默认值：

**OverlayConfig**：主配置类

```rust
pub struct OverlayConfig {
    pub z_index: ZIndexConfig,       // Z-Index 配置
    pub animation: AnimationConfig,  // 动画配置
    pub theme: ThemeConfig,          // 主题配置
    pub responsive: ResponsiveConfig,// 响应式配置
    pub platform: PlatformConfig,    // 平台配置
}
```

**ZIndexConfig**：管理层级顺序

- 基础层级：1000
- 每个 Overlay 自动递增：+10
- 支持自定义层级范围

**AnimationConfig**：动画配置

```rust
pub struct AnimationConfig {
    pub duration: u32,            // 持续时间（毫秒）
    pub delay: u32,               // 延迟时间
    pub easing: EasingType,       // 缓动类型
    pub enter: AnimationType,     // 进入动画
    pub exit: AnimationType,      // 退出动画
    pub fill_mode: FillMode,      // 填充模式
    pub iteration_count: u32,     // 迭代次数
}
```

**ResponsiveConfig**：响应式配置

- 支持多个断点配置
- 针对不同屏幕尺寸的布局策略
- 支持移动端优化

#### 2. 核心层（Core Layer）

提供 Overlay 系统的核心功能：

**GlobalStateManager**：全局状态管理

```rust
pub struct GlobalStateManager {
    overlays: HashMap<String, OverlayState>,  // 所有 Overlay 状态
    z_index_stack: Vec<u32>,                  // Z-Index 栈
    next_z_index: u32,                        // 下一个可用 Z-Index
}
```

功能：

- `add_overlay()`: 添加新的 Overlay
- `remove_overlay()`: 移除 Overlay
- `show_overlay()` / `hide_overlay()`: 显示/隐藏 Overlay
- `get_visible_overlays()`: 获取所有可见 Overlay
- `get_overlays_by_type()`: 按类型筛选 Overlay
- `clear_all()`: 清理所有 Overlay

**EventBus**：事件总线

```rust
pub enum EventType {
    OverlayShow,      // Overlay 显示
    OverlayHide,      // Overlay 隐藏
    OverlayClose,     // Overlay 关闭
    ZIndexChange,     // Z-Index 变化
    PlatformChange,   // 平台变化
    AnimationStart,   // 动画开始
    AnimationEnd,     // 动画结束
    ThemeChange,      // 主题变化
}
```

功能：

- 注册/移除事件监听器
- 发送事件到订阅者
- 支持事件过滤和优先级

**AnimationManager**：动画管理器

```rust
pub enum AnimationType {
    FadeIn / FadeOut,       // 淡入/淡出
    SlideIn / SlideOut,     // 滑入/滑出
    ScaleIn / ScaleOut,     // 缩放进入/退出
    BounceIn / BounceOut,   // 弹跳进入/退出
    Custom(String),         // 自定义动画
}

pub enum EasingType {
    Linear, EaseIn, EaseOut, EaseInOut,
    EaseInQuad, EaseOutQuad, EaseInOutQuad,
    EaseInCubic, EaseOutCubic, EaseInOutCubic,
    Custom(String),
}
```

功能：

- 生成动画类名
- 检测平台动画支持
- 创建进入/退出动画
- 支持自定义缓动函数

**ThemeManager**：主题管理器

- 管理明暗主题切换
- 提供组件样式类
- 与全局主题系统集成
- 支持自定义主题

**PlatformAdapter**：平台适配器

```rust
pub struct PlatformCapabilities {
    pub supports_gestures: bool,        // 是否支持手势
    pub supports_haptic_feedback: bool, // 是否支持触觉反馈
    pub supports_animations: bool,      // 是否支持动画
    pub supports_touch: bool,           // 是否支持触摸
    pub max_z_index: u32,              // 最大 Z-Index
    pub animation_duration: u32,       // 动画时长
}
```

功能：

- 自动检测平台类型（Web/Desktop/Mobile）
- 提供平台能力查询
- 适配不同平台的行为差异

#### 3. 组件层（Components Layer）

基于核心层实现的具体 Overlay 组件：

**MessageOverlay**：消息覆盖层

- 轻量级消息提示
- 自动定位和动画
- 支持多种消息类型
- 自动关闭和手动关闭

**DialogOverlay**：对话框覆盖层

- 模态对话框实现
- 支持遮罩层
- 焦点管理
- ESC 键关闭

**ModalOverlay**：模态框覆盖层

- 基于 DialogOverlay 扩展
- 支持确定/取消操作
- 表单集成
- 验证支持

**NoticeOverlay**：通知覆盖层

- 页面角落通知
- 支持堆叠显示
- 自定义位置
- 多个同时显示

**InteractiveOverlay**：交互式覆盖层

- Popover / Tooltip 基础
- 跟随目标元素
- 自动定位计算
- 碰撞检测

#### 4. 插件层（Plugins Layer）

提供可扩展的插件机制：

**PluginManager**：插件管理器

```rust
pub trait OverlayPlugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn initialize(&mut self, context: &OverlayContext) -> Result<(), String>;
    fn cleanup(&mut self) -> Result<(), String>;
    fn handle_event(&mut self, event: &str, data: &HashMap<String, String>) -> Result<(), String>;
}
```

**AnimationPlugin**：动画插件

- 扩展动画类型
- 自定义动画效果
- 动画队列管理

**ThemePlugin**：主题插件

- 自定义主题
- 动态主题切换
- 主题预设

**AccessibilityPlugin**：无障碍插件

- ARIA 属性自动添加
- 焦点管理增强
- 屏幕阅读器优化
- 键盘导航支持

### 使用示例

#### 基础使用

```rust
use helixui::overlay::*;

// 显示消息
show_message("操作成功".to_string(), MessageType::Success);

// 显示带持续时间的消息
show_message_with_duration("保存成功".to_string(), MessageType::Success, 3000);

// 显示指定位置的消息
show_message_with_position(
    "请注意".to_string(),
    MessageType::Warning,
    MessagePosition::TopCenter
);
```

#### 高级配置

```rust
use helixui::overlay::{config::*, core::*};

// 创建自定义配置
let config = OverlayConfig::new()
    .with_animation(AnimationConfig {
        duration: 300,
        easing: EasingType::EaseInOut,
        enter: AnimationType::SlideIn,
        exit: AnimationType::FadeOut,
        ..Default::default()
    })
    .with_z_index(ZIndexConfig {
        base: 2000,
        increment: 20,
        max: 9999,
    });

// 使用配置创建 Overlay
// ...
```

#### 插件扩展

```rust
use helixui::overlay::plugins::*;

// 创建插件管理器
let mut plugin_manager = PluginManager::new();

// 注册插件
plugin_manager.register_plugin(Box::new(AnimationPlugin::new()))?;
plugin_manager.register_plugin(Box::new(ThemePlugin::new()))?;
plugin_manager.register_plugin(Box::new(AccessibilityPlugin::new()))?;

// 初始化所有插件
plugin_manager.initialize_all()?;
```

### 技术优势

1. **分层架构**：清晰的职责划分，易于维护和扩展
2. **状态集中管理**：避免状态分散导致的不一致问题
3. **事件驱动**：组件间通过事件通信，降低耦合度
4. **平台无关**：统一的 API，跨平台一致的行为
5. **性能优化**：
   - 智能 Z-Index 分配，避免不必要的重排
   - 动画使用 CSS 实现，硬件加速
   - 懒加载和按需渲染
6. **可测试性**：模块化设计便于单元测试
7. **可扩展性**：插件机制支持功能扩展

### 与反馈组件的集成

Overlay 系统为所有反馈组件提供底层支持：

- **Message**: 使用 `MessageOverlay` + `GlobalStateManager`
- **Dialog**: 使用 `DialogOverlay` + `AnimationManager`
- **Modal**: 使用 `ModalOverlay` + `ThemeManager`
- **Notification**: 使用 `NoticeOverlay` + `EventBus`
- **Toast**: 使用 `MessageOverlay` 的轻量变体

所有组件共享：

- 统一的 Z-Index 管理
- 一致的动画效果
- 相同的主题系统
- 统一的事件处理

## 开发进度状态

### ✅ 已完成组件

- **Badge**：✅ 完成，支持多种类型和形状
- **Message**：✅ 完成，支持全局消息管理，已集成 Overlay 系统
- **Dialog**：✅ 完成，支持自定义内容和操作
- **Modal**：✅ 完成，基于 Dialog 扩展
- **Overlay System**：✅ 完成，提供完整的覆盖层管理架构

### ⏳ 待开发组件

- **Notification**：⏳ 待开发（Overlay 基础已就绪）
- **Alert**：⏳ 待开发

### 📊 完成度统计

- **已完成组件**：4/6 (67%)
- **Overlay 系统**：100%（架构完整）
- **文档演示**：100%
- **可访问性**：基础支持 + 插件增强
- **主题集成**：100%
- **跨平台支持**：100%

## 最佳实践

### 使用建议

1. **Badge**：用于状态指示和数量提醒，避免过度使用
2. **Message**：用于操作反馈，保持简洁明了
3. **Dialog**：用于重要确认，避免频繁弹出
4. **Modal**：用于复杂表单和重要操作
5. **Notification**：用于系统通知，控制数量
6. **Alert**：用于页面重要信息，保持可见性

### 设计原则

1. **一致性**：保持视觉和交互的一致性
2. **层次性**：合理使用不同级别的反馈
3. **及时性**：及时提供反馈，避免用户困惑
4. **可操作性**：提供明确的后续操作指引
5. **可关闭性**：允许用户关闭非关键信息

---

本设计文档将作为反馈组件实现与评审的依据，具体 API 在开发阶段可能微调，但总体思想与契约保持不变。
