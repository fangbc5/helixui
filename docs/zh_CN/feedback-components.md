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

## 开发进度状态

### ✅ 已完成组件
- **Badge**：✅ 完成，支持多种类型和形状
- **Message**：✅ 完成，支持全局消息管理
- **Dialog**：✅ 完成，支持自定义内容和操作
- **Modal**：✅ 完成，基于 Dialog 扩展

### ⏳ 待开发组件
- **Notification**：⏳ 待开发
- **Alert**：⏳ 待开发

### 📊 完成度统计
- **已完成组件**：4/6 (67%)
- **文档演示**：100%
- **可访问性**：基础支持
- **主题集成**：100%

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
