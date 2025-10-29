# 通用组件设计（Dropdown / Collapsible / Button / Icon / Divider）

> 本文档聚焦“通用（Common）”类组件的设计目标、API 约定与可访问性策略，优先覆盖当前已落地的 Dropdown、Collapsible，并为后续通用组件提供统一规范。

## 目标与设计原则

- 一致的 API 与属性命名：触发器 trigger、内容 content、禁用 disabled、受控 open/on_open_change
- 可组合，可嵌套：支持在内容中放任意元素（表单/列表/自定义面板）
- 无障碍默认可达：键盘导航、焦点环、ARIA 属性
- 可主题化：明暗主题、边框与阴影、圆角、悬浮与聚焦态

---

## Dropdown（下拉菜单）

### 使用场景

- 操作菜单（编辑、删除、分享）
- 选择类的基础能力（Select/Combobox/Cascader 的基石）
- 用户菜单、右键菜单、过滤器面板等浮层组件的通用外壳

### 组成与语义

- `Dropdown`：根容器，管理展开/收起状态与焦点漫游
- `DropdownTrigger`：触发器（button/任意元素），键盘 Enter/Space 打开
- `DropdownContent`：浮层内容容器（listbox/菜单）
- `DropdownItem<T>`：条目，支持 `disabled`、`on_select(value)`

### 最小示例

```rust
rsx! {
    Dropdown { default_open: false,
        DropdownTrigger { "打开菜单" }
        DropdownContent {
            DropdownItem::<&'static str> { value: "edit",   index: 0usize, "编辑" }
            DropdownItem::<&'static str> { value: "delete", index: 1usize, "删除" }
            DropdownItem::<&'static str> { value: "share",  index: 2usize, "分享" }
        }
    }
}
```

### Props（关键）

- `Dropdown`
  - `open: Option<bool>`（受控）/ `default_open: bool`（非受控）
  - `on_open_change: Option<EventHandler<bool>>`
  - `disabled: Signal<bool>`：禁用交互
  - `roving_loop: bool`：焦点循环
- `DropdownTrigger`
  - 无强制子元素要求（内部会渲染 button 角色与可聚焦）
- `DropdownContent`
  - 自动附带 `data-state=open|closed`，用于动画与可见性样式
- `DropdownItem<T>`
  - `value: T`、`index: usize`、`disabled: bool`、`on_select: EventHandler<T>`

### 样式与定位

- 视觉样式（Tailwind）：
  - Trigger：边框化按钮，hover/focus ring
  - Content：卡片化白底、阴影、圆角、进出场过渡
  - Item：hover/focus 高亮、禁用态灰色
- 定位策略（仅样式，无 JS 计算）：
  - `Dropdown` 根容器建议 `relative inline-block`
  - `DropdownContent` 使用 `absolute top-full left-0 mt-1`
  - 当存在父容器裁剪（`overflow: hidden`）时：
    - 推荐将浮层放至不裁剪的容器（如 Demo 容器取消 `overflow-hidden`）
    - 或在后续版本采用 `Portal` 将内容渲染至文档根部

### 可访问性（A11y）

- `role="listbox"` + `aria-labelledby` 关联触发器
- 键盘：ArrowUp/ArrowDown 导航，Enter 选择，Esc 关闭
- 焦点：打开时将焦点委托至条目，关闭回到触发器

### 扩展方向

- 子菜单（Submenu）、分组（Group/Label）、复选/单选项（CheckboxItem/RadioGroupItem）
- 右键菜单（ContextMenu）、选择器（Select/Combobox）、过滤器面板（FilterDropdown）

---

## Collapsible（折叠面板）

### 使用场景

- 常见 FAQ/帮助面板
- 表单高级设置的折叠区域

### 最小示例

```rust
rsx! {
    Collapsible { default_open: true,
        Collapsible::Trigger { "显示/隐藏" }
        Collapsible::Content { "内容区域" }
    }
}
```

### 关键点

- 受控/非受控双模式；`data-state` 驱动过渡动画
- 与 Dropdown 共享设计语言（触发器、内容、动画切换）

---

## 目录与实现位置

```
helixui-lib/src/components/common/
  dropdown.rs      # Dropdown 家族（Trigger/Content/Item）
  collapsible.rs   # Collapsible 家族
  mod.rs
```

---

## 后续规划（Common）

- Select / Combobox / Cascader（基于 Dropdown 的选择类）
- ContextMenu（右键菜单，坐标触发）
- UserMenu（头像下拉）
- FilterDropdown（表单筛选面板）

> 注：新增组件文档请复用本文档结构（目标 → API → 示例 → A11y → 样式/定位 → 扩展）。
