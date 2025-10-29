# 📆 当前迭代记录（2025-10）

> 本文档用于跟踪 2025 年 10 月迭代的目标、范围、任务、验收标准与风险。本次迭代聚焦于"布局体系补齐：Grid + Layout/Sider 增强"。

## 🎯 迭代目标

- 完成响应式栅格系统（Grid / GridItem）
- 增强 Layout/Sider 能力，对齐 Naive UI 关键 API（部分）
- 在文档站新增示例与对照文档，形成可验收的 Demo 页

## 📦 迭代范围

- 代码库：`helixui-lib`（layout 模块）、`helixui-website`（文档与示例）
- 组件/模块：`Grid`、`GridItem`、`Layout`、`Sider`、`Space`
- 文档：新增迭代说明、使用指南与示例页面

## 🛠️ 任务清单

### 1) 栅格系统（Grid）

- 新增 `layout/grid.rs`：`Grid { cols, x_gap, y_gap, responsive_cols }`
- 新增 `GridItem { span, offset, responsive }`
- 在 `layout/mod.rs` 导出 Grid API
- 响应式断点沿用 `tokens::Breakpoint`

### 2) Layout 增强

- 新增 `inverted: bool`、`bordered: bool`、`has_sider?: bool`（默认自动识别）
- 新增 `scroll_strategy: Layout|Content`、`native_scrollbar: bool`

### 3) Sider 增强（基础）

- 新增 `collapse_mode: Width|Transform`
- 新增触发器：`show_trigger: None|Bar|Arrow`、`trigger_placement: Top|Bottom`
- 受控/非受控：`default_collapsed` + `collapsed` + `on_update_collapsed`

### 4) Space 能力补齐（本迭代做基础）

- `size` 支持 `(horizontal, vertical)` 形式（向下兼容现有 `i32`）
- 统一 `split` 的可访问性属性占位（A11y 标签）

### 5) 文档与示例（helixui-website）

- 在 `views/components/layout/layout_page.rs` 增加章节：
  - 栅格基础/响应式示例（以 24 列为例）
  - 固定 Header + 内容区滚动策略对比（Layout vs Content）
  - Sider 折叠与触发器示例
- 新增独立 Demo：三列响应式页面

## ✅ 验收标准

### Grid

- 支持 `cols`、`span/offset` 与断点配置
- 具备 `x_gap/y_gap`，不同断点下能够正确换行与占位
- 提供 2 个以上响应式演示（移动/桌面）

### Layout/Sider

- `inverted/bordered` 生效
- 提供 `scroll_strategy` 切换示例
- 触发器工作正常，`collapse_mode` 至少实现 Width 模式（Transform 暂留接口与样式占位）

### Space

- `(h, v)` 间距在横向/纵向方向示例可见且正确

### 文档

- 迭代说明页（本文）与布局组件文档新增章节已上线
- 示例可复现，构建通过

## 🔍 非目标（Out of Scope）

- Sider 拖拽 `resizable`
- `Scrollbar` 自定义滚动条组件
- `Affix/BackTop/Anchor` 附属布局组件
- 深入主题/令牌扩展（仅保留必要占位）

## 🗓️ 排期（1 周）

- **D1-D2**：Grid + GridItem 初版 & 基础用例
- **D3**：Layout/Sider API 扩展与样式落位
- **D4**：文档与示例补齐；Space `(h,v)` 支持
- **D5**：联调整改与验收

## ⚠️ 风险与缓解

### 断点映射不一致

- **风险**：响应式规则与断点映射不一致
- **缓解**：在 `tokens::Breakpoint` 上封装统一映射与工具函数

### 样式冲突

- **风险**：现有 Tailwind 类与 `inverted/bordered` 组合冲突
- **缓解**：以行内样式为主，类名为辅，必要时引入命名空间 `hx-*`

### Demo 滚动容器混淆

- **风险**：Demo 页面滚动容器选择混淆
- **缓解**：示例文档显式标注滚动容器与 CSS

## 📎 参考

- **Naive UI**：`Layout`/`Sider`/`Grid`/`Space` 文档（对齐 API 思路）
- **现有文件**：`helixui-lib/src/components/layout/*`, `helixui-website/src/views/components/layout/layout_page.rs`

## 📊 进度跟踪

### 已完成

- [x] 迭代文档创建
- [x] 任务清单制定
- [x] 验收标准定义
- [x] 新增通用组件：Tag（支持语义色/变体/尺寸/可关闭）
- [x] 文档站新增 Tag 演示与 API 表格
- [x] DemoBox 复制提示改用 dioxus_time 定时器
- [x] Sidebar 分类间距统一（修复第一个分类与其他不一致问题）
- [x] Tag 暗色模式颜色对比度优化；关闭按钮改用自研 Button（纯图标）

### 进行中

- [ ] Grid 组件实现（文档与示例补全）
- [ ] Layout 增强
- [ ] Sider 增强
- [ ] Space 能力补齐
- [ ] 文档与示例更新

### 待开始

- [ ] 代码审查
- [ ] 集成测试
- [ ] 文档最终检查

---

**创建时间**：2025-10-15
**负责人**：开发团队
**状态**：进行中
