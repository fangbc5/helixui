use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Language {
    ZhCN,
    EnUS,
}

impl Language {
    #[allow(dead_code)]
    pub fn code(&self) -> &'static str {
        match self {
            Language::ZhCN => "zh-CN",
            Language::EnUS => "en-US",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Language::ZhCN => "简体中文",
            Language::EnUS => "English",
        }
    }

    pub fn toggle(&self) -> Self {
        match self {
            Language::ZhCN => Language::EnUS,
            Language::EnUS => Language::ZhCN,
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::ZhCN
    }
}

/// 全局语言状态
pub static LANGUAGE: GlobalSignal<Language> = Signal::global(Language::default);

/// 翻译文本
pub fn t(key: &str) -> String {
    let lang = *LANGUAGE.read();
    let text = match (lang, key) {
        // 导航
        (Language::ZhCN, "nav.home") => "首页",
        (Language::EnUS, "nav.home") => "Home",
        (Language::ZhCN, "nav.docs") => "文档",
        (Language::EnUS, "nav.docs") => "Docs",
        (Language::ZhCN, "nav.components") => "组件",
        (Language::EnUS, "nav.components") => "Components",
        
        // 首页
        (Language::ZhCN, "home.title") => "Helix UI",
        (Language::EnUS, "home.title") => "Helix UI",
        (Language::ZhCN, "home.subtitle") => "一个优雅的 Dioxus 组件库",
        (Language::EnUS, "home.subtitle") => "An elegant Dioxus component library",
        (Language::ZhCN, "home.description") => "为 Dioxus 框架打造的现代化组件库。提供丰富的组件和优雅的设计。",
        (Language::EnUS, "home.description") => "A modern component library for the Dioxus framework. Provides rich components and elegant design.",
        (Language::ZhCN, "home.get_started") => "开始使用",
        (Language::EnUS, "home.get_started") => "Get Started",
        
        // 特性
        (Language::ZhCN, "feature.elegant.title") => "优雅设计",
        (Language::EnUS, "feature.elegant.title") => "Elegant Design",
        (Language::ZhCN, "feature.elegant.desc") => "精心设计的组件，提供现代化的用户界面",
        (Language::EnUS, "feature.elegant.desc") => "Carefully designed components with modern UI",
        (Language::ZhCN, "feature.performance.title") => "高性能",
        (Language::EnUS, "feature.performance.title") => "High Performance",
        (Language::ZhCN, "feature.performance.desc") => "基于 Dioxus 框架，提供卓越的性能",
        (Language::EnUS, "feature.performance.desc") => "Built on Dioxus framework for excellent performance",
        (Language::ZhCN, "feature.easy.title") => "易于使用",
        (Language::EnUS, "feature.easy.title") => "Easy to Use",
        (Language::ZhCN, "feature.easy.desc") => "简洁的 API，快速上手开发",
        (Language::EnUS, "feature.easy.desc") => "Simple API for quick development",
        
        // 侧边栏
        (Language::ZhCN, "sidebar.docs") => "文档",
        (Language::EnUS, "sidebar.docs") => "Documentation",
        (Language::ZhCN, "sidebar.components") => "组件",
        (Language::EnUS, "sidebar.components") => "Components",
        (Language::ZhCN, "sidebar.general") => "通用组件",
        (Language::EnUS, "sidebar.general") => "General",
        (Language::ZhCN, "sidebar.feedback") => "反馈组件",
        (Language::EnUS, "sidebar.feedback") => "Feedback",
        (Language::ZhCN, "sidebar.data-input") => "数据输入",
        (Language::EnUS, "sidebar.data-input") => "Data Input",
        
        // 文档页面
        (Language::ZhCN, "docs.introduction") => "介绍",
        (Language::EnUS, "docs.introduction") => "Introduction",
        (Language::ZhCN, "docs.quick-start") => "快速上手",
        (Language::EnUS, "docs.quick-start") => "Quick Start",
        (Language::ZhCN, "docs.guide") => "指南",
        (Language::EnUS, "docs.guide") => "Guide",
        (Language::ZhCN, "docs.version") => "版本",
        (Language::EnUS, "docs.version") => "Version",
        
        // 介绍页面
        (Language::ZhCN, "intro.title") => "介绍",
        (Language::EnUS, "intro.title") => "Introduction",
        (Language::ZhCN, "intro.welcome") => "欢迎使用 Helix UI",
        (Language::EnUS, "intro.welcome") => "Welcome to Helix UI",
        (Language::ZhCN, "intro.desc") => "Helix UI 是一个为 Dioxus 框架设计的现代化组件库。",
        (Language::EnUS, "intro.desc") => "Helix UI is a modern component library designed for the Dioxus framework.",
        (Language::ZhCN, "intro.what") => "什么是 Helix UI？",
        (Language::EnUS, "intro.what") => "What is Helix UI?",
        (Language::ZhCN, "intro.what.desc") => "Helix UI 提供了一套完整的 UI 组件库，帮助您快速构建漂亮的 Dioxus 应用程序。",
        (Language::EnUS, "intro.what.desc") => "Helix UI provides a complete set of UI components to help you quickly build beautiful Dioxus applications.",
        (Language::ZhCN, "intro.features") => "特性",
        (Language::EnUS, "intro.features") => "Features",
        (Language::ZhCN, "intro.features.modern") => "现代化设计",
        (Language::EnUS, "intro.features.modern") => "Modern Design",
        (Language::ZhCN, "intro.features.rust") => "Rust 原生",
        (Language::EnUS, "intro.features.rust") => "Rust Native",
        (Language::ZhCN, "intro.features.customizable") => "高度可定制",
        (Language::EnUS, "intro.features.customizable") => "Highly Customizable",
        
        // 快速上手页面
        (Language::ZhCN, "quick.title") => "快速上手",
        (Language::EnUS, "quick.title") => "Quick Start",
        (Language::ZhCN, "quick.install") => "安装",
        (Language::EnUS, "quick.install") => "Installation",
        (Language::ZhCN, "quick.usage") => "使用",
        (Language::EnUS, "quick.usage") => "Usage",
        
        // 指南页面
        (Language::ZhCN, "guide.title") => "指南",
        (Language::EnUS, "guide.title") => "Guide",
        (Language::ZhCN, "guide.getting-started") => "开始使用",
        (Language::EnUS, "guide.getting-started") => "Getting Started",
        (Language::ZhCN, "guide.theming") => "主题定制",
        (Language::EnUS, "guide.theming") => "Theming",
        
        // 版本页面
        (Language::ZhCN, "version.title") => "版本",
        (Language::EnUS, "version.title") => "Version",
        (Language::ZhCN, "version.current") => "当前版本",
        (Language::EnUS, "version.current") => "Current Version",
        
        // 组件页面
        (Language::ZhCN, "component.avatar") => "头像 Avatar",
        (Language::EnUS, "component.avatar") => "Avatar",
        (Language::ZhCN, "component.badge") => "标记 Badge",
        (Language::EnUS, "component.badge") => "Badge",
        (Language::ZhCN, "component.button") => "按钮 Button",
        (Language::EnUS, "component.button") => "Button",
        (Language::ZhCN, "component.card") => "卡片 Card",
        (Language::EnUS, "component.card") => "Card",
        (Language::ZhCN, "component.divider") => "分割线 Divider",
        (Language::EnUS, "component.divider") => "Divider",
        (Language::ZhCN, "component.icon") => "图标 Icon",
        (Language::EnUS, "component.icon") => "Icon",
        (Language::ZhCN, "component.message") => "消息 Message",
        (Language::EnUS, "component.message") => "Message",
        (Language::ZhCN, "component.dialog") => "对话框 Dialog",
        (Language::EnUS, "component.dialog") => "Dialog",
        (Language::ZhCN, "component.modal") => "模态框 Modal",
        (Language::EnUS, "component.modal") => "Modal",

        // 默认
        _ => key,
    };
    text.to_string()
}
