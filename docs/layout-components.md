## 布局组件设计（Flex / Layout / Legacy Grid / Grid / Space / Split）

### 目标与设计原则
- 高内聚、低耦合：每个组件聚焦单一职责，通过上下文轻耦合（如 gutter、断点共享）。
- 响应式优先：核心尺寸与排列行为均支持断点（sm/md/lg/xl/xxl），SSR 友好。
- 受控/非受控双模式：如 `Split` 比例、`Sider` 折叠，同时提供回调事件。
- 语义化与可访问性：使用语义标签与 ARIA 属性，键盘可达。
- 可主题化：依赖统一设计令牌（tokens）生成样式，支持暗色/尺寸系。
- 性能友好：样式尽量静态拼接；上下文避免深层 props 传递。

### 目录结构
```
helixui-lib/src/components/layout/
  mod.rs
  tokens.rs          # 断点/间距/层级/圆角等设计令牌
  context.rs         # Layout/Gutter/Breakpoint 上下文
  flex.rs            # Flex 组件
  layout.rs          # Layout / Header / Sider / Content / Footer
  legacy_grid.rs     # 旧版栅格（兼容语义）
  grid.rs            # 新栅格（现代 API）
  space.rs           # Space（间距）
  split.rs           # Split（可拖拽分割）
  utils.rs           # 断点解析/样式拼接/受控桥接
```

### 设计令牌（tokens）
- 断点：`sm=640`, `md=768`, `lg=1024`, `xl=1280`, `xxl=1536`
- 间距：`xs=4`, `sm=8`, `md=12`, `lg=16`, `xl=24`, `xxl=32`
- 容器最大宽：`sm=640`, `md=768`, `lg=1024`, `xl=1280`, `xxl=1536`
- 层级：`affix=1000`, `sider=900`, `header=800`, `split-handle=1100`
- 圆角/边框/阴影：与现有主题系统对齐，从 tokens 映射样式类

### 上下文模型
- BreakpointContext：当前断点、SSR 初始断点、断点计算工具。
- GutterContext：`Row` 提供 `gutter_x/gutter_y` 给 `Col`。
- LayoutContext：`has_sider`、`sider_collapsed`、`sider_width/collapsed_width`、`header_height/footer_height`、`is_rtl`。

---

## 组件一：Flex（弹性布局）
用途：在容器内进行一维（横/纵）分布、对齐与间距控制，替代部分简单栅格场景。

核心属性（草案）：
- direction: `Row | Column`
- wrap: `bool`
- gap: `Spacing | (x, y)`，支持响应式 map
- justify: `Start | Center | End | SpaceBetween | SpaceAround | SpaceEvenly`
- align: `Start | Center | End | Stretch | Baseline`
- inline: `bool`（inline-flex）
- reverse: `bool`

行为要点：
- 优先使用原生 CSS `gap`；不支持时回退为子元素 margin（降级策略在 utils）。
- 所有对齐语义与 Tailwind/Naive 保持一致，提供 class 透传。

---

## 组件二：Layout 家族
包含：`Layout`、`LayoutHeader`、`LayoutSider`、`LayoutContent`、`LayoutFooter`

关键属性（选摘）：
- Layout：`direction`（水平/垂直）、`full_height`、`gap`、`padded`、`bordered`、`has_sider`（自动/显式）
- Header/Footer：`height`、`sticky`、`bordered`、`z_index`
- Sider：`width`、`collapsed_width`、`default_collapsed`、`collapsed(Option)`、`on_collapsed_change`、`collapsible`、`breakpoint`、`resizable`、`min_width/max_width`
- Content：`padded`、`scroll`

交互设计：
- Sider 折叠：受控/非受控双模式；当视口低于 `breakpoint` 自动折叠并在恢复时还原。
- Sider 可拖拽：在 `min/max` 范围内改变宽度；`collapsed_width` 优先级最高。
- Header sticky：优先使用 `position: sticky`，在不支持环境降级为 `fixed`。

可访问性：
- 使用 `header/nav/main/footer/aside` 语义标签；折叠按钮使用 `aria-expanded` 和 `aria-controls`。

---

## 组件三：Legacy Grid（旧版栅格）
目标：兼容历史栅格 API，平滑迁移到新 Grid。默认 24 列。

Row 属性：`gutter`(x,y|响应式)、`justify`、`align`、`wrap`

Col 属性：`span/offset/push/pull`（1..=24），以及 `sm/md/lg/xl/xxl` 的断点规格（每个规格包含相同字段）。

实现要点：
- 通过 GutterContext 将间距传给子列；断点优先级从大到小回退。
- 仅修复兼容问题，不新增复杂语义。

---

## 组件四：Grid（新栅格）
目标：提供更现代的 API 与可维护实现，支持自动/显式列宽、等比与固定混排、内置响应式。

Grid 容器属性（草案）：
- cols: `Auto | Fixed(u16) | Template(Vec<Track>)`（如 fr/px/minmax）
- gap: `Spacing | (x,y)`（可响应式）
- auto_rows/auto_cols：自动轨道尺寸（如 minmax）
- align/justify：容器对齐

GridItem 属性：
- col: `RangeInclusive<u16>` 或 `Span(u16)`
- row: 同上
- order: `i32`

实现要点：
- 以 CSS Grid 为基础；在不支持 CSS Grid 的环境，提供最小化降级（可选）。
- 与 Legacy Grid 并存，后续通过 codemod/文档引导迁移。

---

## 组件五：Space（间距）
用途：控制一组子元素间的间距、对齐与换行，支持分隔符。

属性：
- direction: `Horizontal | Vertical`
- size: `Spacing | (x,y)` | 响应式 map
- wrap: `bool`
- align: 可选对齐
- split: 可选分隔符 vnode

实现要点：
- 优先使用 CSS `gap`；不支持时回退 margin。
- `split` 作为伪元素插入或通过子节点间插入（根据渲染限制选择）。

---

## 组件六：Split（面板分割）
用途：在水平/垂直方向将区域按比例分割，支持拖拽调整。

属性：
- direction: `Horizontal | Vertical`
- initial: `f32`（0.0..=1.0）
- min_a/min_b: 最小尺寸
- resizable: `bool`
- on_change: 回调（拖拽中节流触发）；on_commit（释放时）可选

实现要点：
- 采用绝对定位手柄；支持鼠标与触控事件；考虑 iframe/选择文本等边界。
- z-index 使用 tokens；手柄可自定义节点。
- 受控/非受控两种模式；SSR 初值与水合一致。

可访问性：
- 手柄提供 `aria-valuenow/min/max`，支持键盘箭头与 PageUp/Down 调整比例。

---

### 可访问性与国际化
- RTL 支持：`is_rtl` 由 LayoutContext 提供；在 push/pull、对齐与拖拽方向上进行反转。
- 语义标签：优先选择 `header/main/footer/aside/nav` 等；必要时补充 `role`。

### 工程与性能
- 样式缓存：基于 props 生成 key，避免重复计算。
- 事件节流：Split 拖拽、窗口 resize 使用节流/防抖。
- 类名拼接：尽可能静态化，动态部分控制在少数分支。

### 落地路线
1. 搭建 `tokens.rs`、`context.rs` 与工具函数。
2. 实现 Flex 与 Space，作为基础能力（gap/对齐/换行）。
3. 实现 Legacy Grid，保证兼容性；随后实现新 Grid。
4. 实现 Layout 家族与 Sider 折叠/断点/可拖拽能力。
5. 实现 Split（拖拽与键盘交互）。
6. 完成文档与示例页面，提供迁移指南与最佳实践。

### 示例（伪代码片段）
```rust
// 页面常见布局
rsx! {
    Layout { full_height: true,
        LayoutSider { collapsible: true, breakpoint: Some(Breakpoint::md) }
        Layout { direction: LayoutDirection::Vertical,
            LayoutHeader { sticky: true }
            LayoutContent { padded: true,
                Space { direction: Axis::Vertical, size: Spacing::Lg, wrap: false }
            }
            LayoutFooter {}
        }
    }
}
```

本设计文档将作为实现与评审的依据，具体 API 在开发阶段可能微调，但总体思想与契约保持不变。

---

## SSR Fallback 逻辑
场景：在 SSR 渲染阶段无法获知真实视口与容器尺寸，涉及断点判断（Layout/Sider/Row/Col）、`position: sticky/fixed` 行为、Split 初始比例等。

策略：
- 提供 `BreakpointContext` 的 `initial_breakpoint`，SSR 时由服务端注入（可配置为 `md` 或根据 UA 估计）。
- 客户端水合后，读取真实尺寸重新计算断点；如与 SSR 值不同，进行一次无动画的样式同步（避免闪烁）。
- Sider 折叠：若 `breakpoint` 生效导致状态变化，水合后以“受控优先”原则更新；对非受控模式，内部状态以真实断点覆盖 SSR 初值。
- Sticky/Header：SSR 阶段均按静态定位渲染；水合后再切换为 `sticky` 或 `fixed`，并在切换帧取消过渡。
- Split：SSR 使用 `initial` 比例渲染静态布局；水合后绑定事件，若受控则与外部值对齐。

实现要点：
- 在 `utils` 提供 `ssr_fallback_class(strategy)`：可插入占位 class，水合后移除。
- 对需尺寸参与计算的组件（如 Grid 的模板列）优先使用 CSS 功能（fr/minmax），减少 JS 参与并降低 SSR 差异。

## Controlled 泛型封装
为统一受控/非受控双模式，提供通用桥接：

```rust
// 伪代码：统一管理受控/非受控值与回调
pub struct Controlled<T> {
    pub value: Option<T>,              // 受控值
    pub default_value: Option<T>,      // 非受控初值
    pub on_change: Option<EventHandler<T>>,
}

impl<T: Clone + PartialEq> Controlled<T> {
    pub fn resolve(&self, inner_state: &T) -> T {
        self.value.clone().unwrap_or_else(|| inner_state.clone())
    }

    pub fn set_and_emit(&self, next: T, set_inner: impl FnOnce(T)) {
        if self.value.is_some() {
            if let Some(cb) = &self.on_change { cb.call(next); }
        } else {
            set_inner(next.clone());
            if let Some(cb) = &self.on_change { cb.call(next); }
        }
    }
}
```

应用：
- `Sider.collapsed`、`Split.ratio`、`GridItem.order` 等均通过 `Controlled<T>` 实现一致行为。

## 基础 Hooks

### use_drag（拖拽）
职责：统一鼠标/触控拖拽事件，提供节流与边界控制；在 Split、可改变宽度的 Sider 等复用。

接口（草案）：
- 入参：`on_start`, `on_move(DragState)`, `on_end`, `axis: Horizontal|Vertical|Both`, `bounds`、`capture_pointer`、`throttle_ms`。
- 出参：`bind`（要绑定在手柄上的事件属性集合），`is_dragging` 状态。

实现关键点：
- 处理 Pointer/Mouse/Touch 三类事件；在文本选择、iframe 场景使用透明遮罩层防误选。
- 支持 `prevent_default` 与 `stop_propagation` 开关；支持 RTL 方向修正。

### use_resize_observer（尺寸监听）
职责：监听元素尺寸变化，驱动断点与自适应布局（如 Container、Grid 自动轨道）。

接口（草案）：
- 入参：`node_ref`、`on_resize(ResizeEntry)`、`debounce_ms`。
- 出参：`content_rect`（最新尺寸）、`disconnect()`。

实现关键点：
- 原生 `ResizeObserver` 优先；不支持时退化为 `window.resize + getBoundingClientRect`，并进行防抖。
- 与 `BreakpointContext` 协作：当 `Container` 指定为断点参考容器时，按其宽度计算断点而非全局视口。

## tokens → theme config 抽象
目标：将静态 tokens 抽象为可配置主题，支持多主题与动态切换。

分层：
1. tokens（设计值集合）：断点、间距、半径、阴影、色板引用键等。
2. theme config（运行态主题）：在 tokens 基础上，结合品牌色/暗色等生成最终样式参数。
3. theme provider：在应用树注入 `ThemeContext`，组件只依赖 provider 输出，不直接依赖静态常量。

接口（草案）：
- `ThemeTokens`：纯数据结构；
- `ThemeConfig`：从 tokens 派生，提供 `spacing(scale) -> px`、`z_index(key) -> i32`、`breakpoint(name) -> px` 等方法；
- `ThemeProvider::new(config)`：注入到组件树；`use_theme()` 获取引用；
- 主题切换：`set_theme(config)` 或通过外层状态驱动 provider 更新；组件根据 `PartialEq` 差异进行最小化重渲染。

落地建议：
- 先实现只读主题（单主题）；随后扩展为可切换主题并支持按需合并（如 `ThemeConfig::extend`）。
- 在组件样式生成处统一从 `use_theme()` 取值，避免直接依赖常量。


