# 贡献指南

感谢您对 HelixUI 项目的关注！我们欢迎所有形式的贡献，包括但不限于：

- 🐛 报告 Bug
- ✨ 提出新功能
- 📝 改进文档
- 🎨 优化设计
- 🧪 编写测试
- 🔧 代码优化

## 🚀 快速开始

### 环境准备

1. **安装 Rust** (版本 1.70+)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **克隆项目**
   ```bash
   git clone https://github.com/fangbc5/helixui.git
   cd helixui
   ```

3. **安装依赖**
   ```bash
   cargo build
   ```

### 开发流程

1. **Fork 项目**
   - 点击 GitHub 页面右上角的 "Fork" 按钮

2. **创建分支**
   ```bash
   git checkout -b feature/your-feature-name
   # 或者
   git checkout -b fix/your-bug-fix
   ```

3. **进行开发**
   - 编写代码
   - 添加测试
   - 更新文档

4. **提交更改**
   ```bash
   git add .
   git commit -m "feat: add new component"
   ```

5. **推送分支**
   ```bash
   git push origin feature/your-feature-name
   ```

6. **创建 Pull Request**
   - 在 GitHub 上创建 PR
   - 填写详细的描述

## 📋 代码规范

### Rust 代码规范

- 遵循 [Rust 官方代码规范](https://doc.rust-lang.org/1.0.0/style-guide.html)
- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量

```bash
# 格式化代码
cargo fmt

# 检查代码质量
cargo clippy

# 运行测试
cargo test
```

### 提交信息规范

我们使用 [Conventional Commits](https://www.conventionalcommits.org/) 规范：

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

**类型 (type):**
- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `style`: 代码格式调整
- `refactor`: 代码重构
- `test`: 测试相关
- `chore`: 构建过程或辅助工具的变动

**示例:**
```
feat(button): add dashed border support
fix(avatar): handle image loading errors
docs: update README with new features
```

## 🎨 组件开发指南

### 组件结构

每个组件都应该遵循以下结构：

```rust
use dioxus::prelude::*;

/// 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct ComponentProps {
    /// 属性描述
    #[props(default = DefaultValue)]
    pub prop: PropType,
    
    /// 子元素
    pub children: Element,
}

/// 组件实现
#[component]
pub fn Component(props: ComponentProps) -> Element {
    rsx! {
        div {
            class: "component-class",
            {props.children}
        }
    }
}
```

### 组件要求

1. **类型安全** - 充分利用 Rust 的类型系统
2. **可访问性** - 支持键盘导航和屏幕阅读器
3. **响应式** - 适配不同屏幕尺寸
4. **主题支持** - 支持深色/浅色模式
5. **文档完整** - 包含使用示例和 API 文档

### 测试要求

每个组件都应该包含：

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::prelude::*;

    #[test]
    fn test_component_renders() {
        // 测试组件渲染
    }

    #[test]
    fn test_component_props() {
        // 测试组件属性
    }
}
```

## 📚 文档规范

### 组件文档

每个组件都应该有完整的文档：

```rust
/// 组件名称
/// 
/// 组件的详细描述和使用场景
/// 
/// # 示例
/// 
/// ```rust
/// use helixui::components::{Component, ComponentProps};
/// 
/// rsx! {
///     Component {
///         prop: "value",
///         "内容"
///     }
/// }
/// ```
/// 
/// # Props
/// 
/// | 属性 | 类型 | 默认值 | 描述 |
/// |------|------|--------|------|
/// | prop | String | "" | 属性描述 |
```

### README 更新

如果添加了新组件或功能，请更新：

1. **组件列表** - 在 README 中添加新组件
2. **使用示例** - 提供基本用法示例
3. **特性列表** - 更新项目特性

## 🐛 Bug 报告

在报告 Bug 时，请包含以下信息：

1. **环境信息**
   - Rust 版本
   - Dioxus 版本
   - 操作系统
   - 浏览器（如果是 Web 应用）

2. **复现步骤**
   - 详细的操作步骤
   - 期望的结果
   - 实际的结果

3. **代码示例**
   - 最小化的复现代码
   - 错误信息或截图

## ✨ 功能请求

在提出新功能时，请：

1. **检查现有 Issue** - 避免重复
2. **详细描述** - 说明使用场景和预期效果
3. **提供示例** - 如果可能，提供代码示例
4. **考虑兼容性** - 考虑对现有 API 的影响

## 🏷️ 发布流程

### 版本号规范

我们使用 [语义化版本](https://semver.org/lang/zh-CN/)：

- **主版本号** - 不兼容的 API 修改
- **次版本号** - 向下兼容的功能性新增
- **修订号** - 向下兼容的问题修正

### 发布检查清单

- [ ] 所有测试通过
- [ ] 文档已更新
- [ ] CHANGELOG 已更新
- [ ] 版本号已更新
- [ ] 标签已创建

## 💬 社区

- 🐛 **Bug 报告**: [GitHub Issues](https://github.com/fangbc5/helixui/issues)
- 💡 **功能建议**: [GitHub Discussions](https://github.com/fangbc5/helixui/discussions)
- 💬 **交流讨论**: [GitHub Discussions](https://github.com/fangbc5/helixui/discussions)

## 📄 许可证

通过贡献代码，您同意您的贡献将在 MIT 许可证下发布。

---

感谢您的贡献！🎉
