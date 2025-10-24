use crate::views::layout::ComponentsSidebar;
use crate::views::layout::TocItem;
use crate::views::DocPage;
use dioxus::prelude::*;
use helixui::components::DemoBox;
use helixui::components::{
    Button, ButtonShape, ButtonSize, ButtonType, DialogContent, DialogDescription, DialogRoot,
    DialogTitle, Icon, IconType,
};

/// Dialog 演示页面
#[component]
pub fn DialogPage() -> Element {
    // 定义目录项
    let toc_items = vec![
        TocItem {
            id: "basic".to_string(),
            title: "基础用法".to_string(),
            level: 1,
        },
        TocItem {
            id: "variants".to_string(),
            title: "不同变体".to_string(),
            level: 1,
        },
        TocItem {
            id: "custom-content".to_string(),
            title: "自定义内容".to_string(),
            level: 1,
        },
        TocItem {
            id: "modal".to_string(),
            title: "模态对话框".to_string(),
            level: 1,
        },
        TocItem {
            id: "advanced".to_string(),
            title: "高级用法".to_string(),
            level: 1,
        },
        TocItem {
            id: "api".to_string(),
            title: "API".to_string(),
            level: 1,
        },
    ];

    rsx! {
        DocPage {
            sidebar: rsx! { ComponentsSidebar {} },
            toc_items,

            div {
                class: "component-doc",

                // 标题
                div {
                    class: "mb-8",
                    h1 {
                        class: "text-4xl font-bold text-gray-900 dark:text-white mb-2",
                        "Dialog 对话框"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300",
                        "Dialog 组件用于显示模态对话框，支持多种变体、自定义内容和丰富的交互功能。"
                    }
                }

                // 基础用法
                section {
                    id: "basic",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "基础用法"
                    }

                    DemoBox {
                        title: "基础用法".to_string(),
                        description: "最简单的 Dialog 用法，显示一个基本的对话框。".to_string(),
                        code: r#"use helixui::components::{DialogRoot, DialogContent, DialogTitle, DialogDescription, Button, ButtonType};

#[component]
fn DialogDemo() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        div {
            class: "space-y-4",
            Button {
                button_type: ButtonType::Primary,
                onclick: move |_| open.set(true),
                "打开对话框"
            }
            DialogRoot {
                open: open(),
                on_open_change: move |v| open.set(v),
                DialogContent {
                    DialogTitle {
                        "项目信息"
                    }
                    DialogDescription {
                        "这里是一些关于项目的额外信息。"
                    }
                }
            }
        }
    }
}"#.to_string(),

                        children: rsx! {
                            DialogDemo {}
                        }
                    }
                }

                // 不同变体
                section {
                    id: "variants",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "不同变体"
                    }

                    DemoBox {
                        title: "不同变体".to_string(),
                        description: "Dialog 支持多种变体，适应不同的使用场景。".to_string(),
                        code: r#"use helixui::components::{DialogRoot, DialogContent, DialogTitle, DialogDescription, Button, ButtonType};

#[component]
fn DialogVariantsDemo() -> Element {
    let mut open1 = use_signal(|| false);
    let mut open2 = use_signal(|| false);
    let mut open3 = use_signal(|| false);

    rsx! {
        div {
            class: "flex flex-wrap gap-4",
            div {
                class: "space-y-2",
                Button {
                    button_type: ButtonType::Primary,
                    onclick: move |_| open1.set(true),
                    "信息对话框"
                }
                DialogRoot {
                    open: open1(),
                    on_open_change: move |v| open1.set(v),
                    DialogContent {
                        div {
                            class: "flex justify-between items-start mb-2",
                        DialogTitle {
                            "📋 信息提示"
                        }
                        Button {
                            button_type: ButtonType::Tertiary,
                            size: ButtonSize::Small,
                            shape: ButtonShape::Circle,
                            class: "text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 flex-shrink-0",
                            onclick: move |_| open1.set(false),
                            Icon {
                                icon: IconType::Close,
                            }
                        }
                        }
                        DialogDescription {
                            "这是一条信息提示，用于告知用户当前状态或提供帮助信息。"
                        }
                    }
                }
            }
            div {
                class: "space-y-2",
                Button {
                    button_type: ButtonType::Warning,
                    onclick: move |_| open2.set(true),
                    "警告对话框"
                }
                DialogRoot {
                    open: open2(),
                    on_open_change: move |v| open2.set(v),
                    DialogContent {
                        div {
                            class: "flex justify-between items-start mb-2",
                        DialogTitle {
                            "⚠️ 警告提示"
                        }
                        Button {
                            button_type: ButtonType::Tertiary,
                            size: ButtonSize::Small,
                            shape: ButtonShape::Circle,
                            class: "text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 flex-shrink-0",
                            onclick: move |_| open2.set(false),
                            Icon {
                                icon: IconType::Close,
                            }
                        }
                        }
                        DialogDescription {
                            "这是一条警告提示，提醒用户注意潜在的风险或问题。"
                        }
                    }
                }
            }
            div {
                class: "space-y-2",
                Button {
                    button_type: ButtonType::Error,
                    onclick: move |_| open3.set(true),
                    "错误对话框"
                }
                DialogRoot {
                    open: open3(),
                    on_open_change: move |v| open3.set(v),
                    DialogContent {
                        div {
                            class: "flex justify-between items-start mb-2",
                        DialogTitle {
                            "❌ 错误提示"
                        }
                        Button {
                            button_type: ButtonType::Tertiary,
                            size: ButtonSize::Small,
                            shape: ButtonShape::Circle,
                            class: "text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 flex-shrink-0",
                            onclick: move |_| open3.set(false),
                            Icon {
                                icon: IconType::Close,
                            }
                        }
                        }
                        DialogDescription {
                            "操作失败，请检查输入信息后重试。"
                        }
                    }
                }
            }
        }
    }
}"#.to_string(),

                        children: rsx! {
                            DialogVariantsDemo {}
                        }
                    }
                }

                // 自定义内容
                section {
                    id: "custom-content",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "自定义内容"
                    }

                    DemoBox {
                        title: "自定义内容".to_string(),
                        description: "Dialog 可以包含丰富的自定义内容，如图标、列表、表单等。".to_string(),
                        code: r#"use helixui::components::{DialogRoot, DialogContent, DialogTitle, DialogDescription, Button, ButtonType};

#[component]
fn DialogCustomContentDemo() -> Element {
    let mut open1 = use_signal(|| false);
    let mut open2 = use_signal(|| false);
    
    rsx! {
        div {
            class: "flex flex-wrap gap-4",
            div {
                class: "space-y-2",
                Button {
                    button_type: ButtonType::Primary,
                    onclick: move |_| open1.set(true),
                    "复杂内容"
                }
                DialogRoot {
                    open: open1(),
                    on_open_change: move |v| open1.set(v),
                    DialogContent {
                        div {
                            class: "flex justify-between items-start mb-2",
                        DialogTitle {
                            "🎯 系统更新"
                        }
                        Button {
                            button_type: ButtonType::Tertiary,
                            size: ButtonSize::Small,
                            shape: ButtonShape::Circle,
                            class: "text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 flex-shrink-0",
                            onclick: move |_| open1.set(false),
                            Icon {
                                icon: IconType::Close,
                            }
                        }
                        }
                        DialogDescription {
                            div {
                                class: "space-y-3",
                                p {
                                    "系统将进行重要更新，更新内容包括："
                                }
                                ul {
                                    class: "list-disc list-inside space-y-1 text-sm",
                                    li { "修复安全漏洞" }
                                    li { "优化性能表现" }
                                    li { "新增功能特性" }
                                    li { "改进用户体验" }
                                }
                                p {
                                    class: "text-sm text-gray-600 dark:text-gray-400",
                                    "更新过程大约需要 5-10 分钟，期间系统将暂时不可用。"
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "space-y-2",
                Button {
                    button_type: ButtonType::Info,
                    onclick: move |_| open2.set(true),
                    "表单内容"
                }
                DialogRoot {
                    open: open2(),
                    on_open_change: move |v| open2.set(v),
                    DialogContent {
                        div {
                            class: "flex justify-between items-start mb-2",
                        DialogTitle {
                            "📝 用户反馈"
                        }
                        Button {
                            button_type: ButtonType::Tertiary,
                            size: ButtonSize::Small,
                            shape: ButtonShape::Circle,
                            class: "text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 flex-shrink-0",
                            onclick: move |_| open2.set(false),
                            Icon {
                                icon: IconType::Close,
                            }
                        }
                        }
                        DialogDescription {
                            div {
                                class: "space-y-3",
                                p {
                                    "请告诉我们您遇到的问题："
                                }
                                textarea {
                                    class: "w-full p-2 border border-gray-300 dark:border-gray-600 rounded-md resize-none",
                                    placeholder: "请描述您遇到的问题...",
                                    rows: 3
                                }
                                div {
                                    class: "flex items-center gap-2",
                                    input {
                                        r#type: "checkbox",
                                        id: "urgent"
                                    }
                                    label {
                                        r#for: "urgent",
                                        class: "text-sm",
                                        "紧急问题"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}"#.to_string(),

                        children: rsx! {
                            DialogCustomContentDemo {}
                        }
                    }
                }

                // 模态对话框
                section {
                    id: "modal",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "模态对话框"
                    }

                    DemoBox {
                        title: "模态对话框".to_string(),
                        description: "展示模态对话框的焦点捕获和背景点击关闭功能。".to_string(),
                        code: r#"use helixui::components::{DialogRoot, DialogContent, DialogTitle, DialogDescription, Button, ButtonType};

#[component]
fn DialogModalDemo() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        div {
            class: "space-y-4",
            Button {
                button_type: ButtonType::Primary,
                onclick: move |_| open.set(true),
                "打开模态对话框"
            }
            DialogRoot {
                open: open(),
                on_open_change: move |v| open.set(v),
                DialogContent {
                    div {
                        class: "flex justify-between items-start mb-2",
                        DialogTitle {
                            "模态对话框"
                        }
                        Button {
                            button_type: ButtonType::Tertiary,
                            size: ButtonSize::Small,
                            shape: ButtonShape::Circle,
                            class: "text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 flex-shrink-0",
                            onclick: move |_| open.set(false),
                            Icon {
                                icon: IconType::Close,
                            }
                        }
                    }
                    DialogDescription {
                        "这是一个模态对话框，点击背景或按 ESC 键可以关闭。"
                    }
                }
            }
        }
    }
}"#.to_string(),

                        children: rsx! {
                            DialogModalDemo {}
                        }
                    }
                }

                // 高级用法
                section {
                    id: "advanced",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "高级用法"
                    }

                    DemoBox {
                        title: "高级用法".to_string(),
                        description: "展示 Dialog 的高级用法，包括动态内容、条件渲染和复杂交互。".to_string(),
                        code: r#"use helixui::components::{DialogRoot, DialogContent, DialogTitle, DialogDescription, Button, ButtonType};

#[component]
fn DialogAdvancedDemo() -> Element {
    let mut open1 = use_signal(|| false);
    let mut open2 = use_signal(|| false);

    rsx! {
        div {
            class: "flex flex-wrap gap-4",
            div {
                class: "space-y-2",
                Button {
                    button_type: ButtonType::Primary,
                    onclick: move |_| open1.set(true),
                    "多步骤对话框"
                }
                DialogRoot {
                    open: open1(),
                    on_open_change: move |v| open1.set(v),
                    DialogContent {
                        div {
                            class: "flex justify-between items-start mb-2",
                        DialogTitle {
                            "🚀 发布新版本"
                        }
                        Button {
                            button_type: ButtonType::Tertiary,
                            size: ButtonSize::Small,
                            shape: ButtonShape::Circle,
                            class: "text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 flex-shrink-0",
                            onclick: move |_| open1.set(false),
                            Icon {
                                icon: IconType::Close,
                            }
                        }
                        }
                        DialogDescription {
                            div {
                                class: "space-y-4",
                                div {
                                    class: "flex items-center gap-2",
                                    div {
                                        class: "w-8 h-8 rounded-full bg-blue-100 dark:bg-blue-900 flex items-center justify-center text-sm font-medium",
                                        "1"
                                    }
                                    span { "检查代码质量" }
                                }
                                div {
                                    class: "flex items-center gap-2",
                                    div {
                                        class: "w-8 h-8 rounded-full bg-blue-100 dark:bg-blue-900 flex items-center justify-center text-sm font-medium",
                                        "2"
                                    }
                                    span { "运行测试套件" }
                                }
                                div {
                                    class: "flex items-center gap-2",
                                    div {
                                        class: "w-8 h-8 rounded-full bg-blue-100 dark:bg-blue-900 flex items-center justify-center text-sm font-medium",
                                        "3"
                                    }
                                    span { "构建生产版本" }
                                }
                                div {
                                    class: "flex items-center gap-2",
                                    div {
                                        class: "w-8 h-8 rounded-full bg-blue-100 dark:bg-blue-900 flex items-center justify-center text-sm font-medium",
                                        "4"
                                    }
                                    span { "部署到服务器" }
                                }
                                p {
                                    class: "text-sm text-gray-600 dark:text-gray-400",
                                    "请确认您已完成所有必要的准备工作。"
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "space-y-2",
                Button {
                    button_type: ButtonType::Info,
                    onclick: move |_| open2.set(true),
                    "动态内容"
                }
                DialogRoot {
                    open: open2(),
                    on_open_change: move |v| open2.set(v),
                    DialogContent {
                        div {
                            class: "flex justify-between items-start mb-2",
                        DialogTitle {
                            "📊 数据分析"
                        }
                        Button {
                            button_type: ButtonType::Tertiary,
                            size: ButtonSize::Small,
                            shape: ButtonShape::Circle,
                            class: "text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 flex-shrink-0",
                            onclick: move |_| open2.set(false),
                            Icon {
                                icon: IconType::Close,
                            }
                        }
                        }
                        DialogDescription {
                            div {
                                class: "space-y-3",
                                p {
                                    "正在分析您的数据，请稍候..."
                                }
                                div {
                                    class: "w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2",
                                    div {
                                        class: "bg-blue-600 h-2 rounded-full transition-all duration-300",
                                        style: "width: 75%"
                                    }
                                }
                                div {
                                    class: "flex justify-between text-sm text-gray-600 dark:text-gray-400",
                                    span { "进度: 75%" }
                                    span { "预计剩余时间: 30秒" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}"#.to_string(),

                        children: rsx! {
                            DialogAdvancedDemo {}
                        }
                    }
                }

                // API 文档
                section {
                    id: "api",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "API"
                    }

                    div {
                        class: "space-y-8",

                        // DialogRoot Props
                        div {
                            h3 {
                                class: "text-xl font-semibold text-gray-900 dark:text-white mb-4",
                                "DialogRoot Props"
                            }
                            div {
                                class: "overflow-x-auto",
                                table {
                                    class: "w-full border-collapse border border-gray-200 dark:border-gray-700",
                                    thead {
                                        tr {
                                            class: "bg-gray-50 dark:bg-gray-800",
                                            th { class: "p-3 text-left text-sm font-medium text-gray-900 dark:text-white border-b border-gray-200 dark:border-gray-700", "属性" }
                                            th { class: "p-3 text-left text-sm font-medium text-gray-900 dark:text-white border-b border-gray-200 dark:border-gray-700", "类型" }
                                            th { class: "p-3 text-left text-sm font-medium text-gray-900 dark:text-white border-b border-gray-200 dark:border-gray-700", "默认值" }
                                            th { class: "p-3 text-left text-sm font-medium text-gray-900 dark:text-white border-b border-gray-200 dark:border-gray-700", "说明" }
                                        }
                                    }
                                    tbody {
                                        tr {
                                            class: "border-b border-gray-200 dark:border-gray-700",
                                            td { class: "p-3 text-sm text-gray-900 dark:text-white font-mono", "id" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "ReadSignal<Option<String>>" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "None" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "对话框的 ID" }
                                        }
                                        tr {
                                            class: "border-b border-gray-200 dark:border-gray-700",
                                            td { class: "p-3 text-sm text-gray-900 dark:text-white font-mono", "is_modal" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "ReadSignal<bool>" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "true" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "是否为模态对话框" }
                                        }
                                        tr {
                                            class: "border-b border-gray-200 dark:border-gray-700",
                                            td { class: "p-3 text-sm text-gray-900 dark:text-white font-mono", "open" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "ReadSignal<Option<bool>>" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "None" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "控制对话框的显示状态" }
                                        }
                                        tr {
                                            class: "border-b border-gray-200 dark:border-gray-700",
                                            td { class: "p-3 text-sm text-gray-900 dark:text-white font-mono", "default_open" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "bool" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "false" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "默认是否打开" }
                                        }
                                        tr {
                                            class: "border-b border-gray-200 dark:border-gray-700",
                                            td { class: "p-3 text-sm text-gray-900 dark:text-white font-mono", "on_open_change" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "Callback<bool>" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "-" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "打开状态改变时的回调" }
                                        }
                                        tr {
                                            class: "border-b border-gray-200 dark:border-gray-700",
                                            td { class: "p-3 text-sm text-gray-900 dark:text-white font-mono", "children" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "Element" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "-" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "对话框的子元素" }
                                        }
                                    }
                                }
                            }
                        }

                        // DialogContent Props
                        div {
                            h3 {
                                class: "text-xl font-semibold text-gray-900 dark:text-white mb-4",
                                "DialogContent Props"
                            }
                            div {
                                class: "overflow-x-auto",
                                table {
                                    class: "w-full border-collapse border border-gray-200 dark:border-gray-700",
                                    thead {
                                        tr {
                                            class: "bg-gray-50 dark:bg-gray-800",
                                            th { class: "p-3 text-left text-sm font-medium text-gray-900 dark:text-white border-b border-gray-200 dark:border-gray-700", "属性" }
                                            th { class: "p-3 text-left text-sm font-medium text-gray-900 dark:text-white border-b border-gray-200 dark:border-gray-700", "类型" }
                                            th { class: "p-3 text-left text-sm font-medium text-gray-900 dark:text-white border-b border-gray-200 dark:border-gray-700", "默认值" }
                                            th { class: "p-3 text-left text-sm font-medium text-gray-900 dark:text-white border-b border-gray-200 dark:border-gray-700", "说明" }
                                        }
                                    }
                                    tbody {
                                        tr {
                                            class: "border-b border-gray-200 dark:border-gray-700",
                                            td { class: "p-3 text-sm text-gray-900 dark:text-white font-mono", "id" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "ReadSignal<Option<String>>" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "None" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "内容区域的 ID" }
                                        }
                                        tr {
                                            class: "border-b border-gray-200 dark:border-gray-700",
                                            td { class: "p-3 text-sm text-gray-900 dark:text-white font-mono", "class" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "Option<String>" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "None" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "自定义 CSS 类名" }
                                        }
                                        tr {
                                            class: "border-b border-gray-200 dark:border-gray-700",
                                            td { class: "p-3 text-sm text-gray-900 dark:text-white font-mono", "children" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "Element" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "-" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "内容区域的子元素" }
                                        }
                                    }
                                }
                            }
                        }

                        // DialogTitle Props
                        div {
                            h3 {
                                class: "text-xl font-semibold text-gray-900 dark:text-white mb-4",
                                "DialogTitle Props"
                            }
                            div {
                                class: "overflow-x-auto",
                                table {
                                    class: "w-full border-collapse border border-gray-200 dark:border-gray-700",
                                    thead {
                                        tr {
                                            class: "bg-gray-50 dark:bg-gray-800",
                                            th { class: "p-3 text-left text-sm font-medium text-gray-900 dark:text-white border-b border-gray-200 dark:border-gray-700", "属性" }
                                            th { class: "p-3 text-left text-sm font-medium text-gray-900 dark:text-white border-b border-gray-200 dark:border-gray-700", "类型" }
                                            th { class: "p-3 text-left text-sm font-medium text-gray-900 dark:text-white border-b border-gray-200 dark:border-gray-700", "默认值" }
                                            th { class: "p-3 text-left text-sm font-medium text-gray-900 dark:text-white border-b border-gray-200 dark:border-gray-700", "说明" }
                                        }
                                    }
                                    tbody {
                                        tr {
                                            class: "border-b border-gray-200 dark:border-gray-700",
                                            td { class: "p-3 text-sm text-gray-900 dark:text-white font-mono", "id" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "ReadSignal<Option<String>>" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "None" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "标题的 ID" }
                                        }
                                        tr {
                                            class: "border-b border-gray-200 dark:border-gray-700",
                                            td { class: "p-3 text-sm text-gray-900 dark:text-white font-mono", "children" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "Element" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "-" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "标题内容" }
                                        }
                                    }
                                }
                            }
                        }

                        // DialogDescription Props
                        div {
                            h3 {
                                class: "text-xl font-semibold text-gray-900 dark:text-white mb-4",
                                "DialogDescription Props"
                            }
                            div {
                                class: "overflow-x-auto",
                                table {
                                    class: "w-full border-collapse border border-gray-200 dark:border-gray-700",
                                    thead {
                                        tr {
                                            class: "bg-gray-50 dark:bg-gray-800",
                                            th { class: "p-3 text-left text-sm font-medium text-gray-900 dark:text-white border-b border-gray-200 dark:border-gray-700", "属性" }
                                            th { class: "p-3 text-left text-sm font-medium text-gray-900 dark:text-white border-b border-gray-200 dark:border-gray-700", "类型" }
                                            th { class: "p-3 text-left text-sm font-medium text-gray-900 dark:text-white border-b border-gray-200 dark:border-gray-700", "默认值" }
                                            th { class: "p-3 text-left text-sm font-medium text-gray-900 dark:text-white border-b border-gray-200 dark:border-gray-700", "说明" }
                                        }
                                    }
                                    tbody {
                                        tr {
                                            class: "border-b border-gray-200 dark:border-gray-700",
                                            td { class: "p-3 text-sm text-gray-900 dark:text-white font-mono", "id" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "ReadSignal<Option<String>>" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "None" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "描述的 ID" }
                                        }
                                        tr {
                                            class: "border-b border-gray-200 dark:border-gray-700",
                                            td { class: "p-3 text-sm text-gray-900 dark:text-white font-mono", "children" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "Element" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "-" }
                                            td { class: "p-3 text-sm text-gray-600 dark:text-gray-300", "描述内容" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// 演示组件
#[component]
fn DialogDemo() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        div {
            class: "space-y-4",
            Button {
                button_type: ButtonType::Primary,
                onclick: move |_| open.set(true),
                "打开对话框"
            }
            DialogRoot {
                open: open(),
                on_open_change: move |v| open.set(v),
                DialogContent {
                    div {
                        class: "flex justify-between items-start mb-2",
                        DialogTitle {
                            "项目信息"
                        }
                        Button {
                            button_type: ButtonType::Tertiary,
                            size: ButtonSize::Small,
                            shape: ButtonShape::Circle,
                            class: "text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 flex-shrink-0",
                            onclick: move |_| open.set(false),
                            Icon {
                                icon: IconType::Close,
                            }
                        }
                    }
                    DialogDescription {
                        "这里是一些关于项目的额外信息。"
                    }
                }
            }
        }
    }
}

#[component]
fn DialogVariantsDemo() -> Element {
    let mut open1 = use_signal(|| false);
    let mut open2 = use_signal(|| false);
    let mut open3 = use_signal(|| false);

    rsx! {
        div {
            class: "flex flex-wrap gap-4",
            div {
                class: "space-y-2",
                Button {
                    button_type: ButtonType::Primary,
                    onclick: move |_| open1.set(true),
                    "信息对话框"
                }
                DialogRoot {
                    open: open1(),
                    on_open_change: move |v| open1.set(v),
                    DialogContent {
                        div {
                            class: "flex justify-between items-start mb-2",
                        DialogTitle {
                            "📋 信息提示"
                        }
                        Button {
                            button_type: ButtonType::Tertiary,
                            size: ButtonSize::Small,
                            shape: ButtonShape::Circle,
                            class: "text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 flex-shrink-0",
                            onclick: move |_| open1.set(false),
                            Icon {
                                icon: IconType::Close,
                            }
                        }
                        }
                        DialogDescription {
                            "这是一条信息提示，用于告知用户当前状态或提供帮助信息。"
                        }
                    }
                }
            }
            div {
                class: "space-y-2",
                Button {
                    button_type: ButtonType::Warning,
                    onclick: move |_| open2.set(true),
                    "警告对话框"
                }
                DialogRoot {
                    open: open2(),
                    on_open_change: move |v| open2.set(v),
                    DialogContent {
                        div {
                            class: "flex justify-between items-start mb-2",
                        DialogTitle {
                            "⚠️ 警告提示"
                        }
                        Button {
                            button_type: ButtonType::Tertiary,
                            size: ButtonSize::Small,
                            shape: ButtonShape::Circle,
                            class: "text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 flex-shrink-0",
                            onclick: move |_| open2.set(false),
                            Icon {
                                icon: IconType::Close,
                            }
                        }
                        }
                        DialogDescription {
                            "这是一条警告提示，提醒用户注意潜在的风险或问题。"
                        }
                    }
                }
            }
            div {
                class: "space-y-2",
                Button {
                    button_type: ButtonType::Error,
                    onclick: move |_| open3.set(true),
                    "错误对话框"
                }
                DialogRoot {
                    open: open3(),
                    on_open_change: move |v| open3.set(v),
                    DialogContent {
                        div {
                            class: "flex justify-between items-start mb-2",
                        DialogTitle {
                            "❌ 错误提示"
                        }
                        Button {
                            button_type: ButtonType::Tertiary,
                            size: ButtonSize::Small,
                            shape: ButtonShape::Circle,
                            class: "text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 flex-shrink-0",
                            onclick: move |_| open3.set(false),
                            Icon {
                                icon: IconType::Close,
                            }
                        }
                        }
                        DialogDescription {
                            "操作失败，请检查输入信息后重试。"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DialogCustomContentDemo() -> Element {
    let mut open1 = use_signal(|| false);
    let mut open2 = use_signal(|| false);

    rsx! {
        div {
            class: "flex flex-wrap gap-4",
            div {
                class: "space-y-2",
                Button {
                    button_type: ButtonType::Primary,
                    onclick: move |_| open1.set(true),
                    "复杂内容"
                }
                DialogRoot {
                    open: open1(),
                    on_open_change: move |v| open1.set(v),
                    DialogContent {
                        div {
                            class: "flex justify-between items-start mb-2",
                        DialogTitle {
                            "🎯 系统更新"
                        }
                        Button {
                            button_type: ButtonType::Tertiary,
                            size: ButtonSize::Small,
                            shape: ButtonShape::Circle,
                            class: "text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 flex-shrink-0",
                            onclick: move |_| open1.set(false),
                            Icon {
                                icon: IconType::Close,
                            }
                        }
                        }
                        DialogDescription {
                            div {
                                class: "space-y-3",
                                p {
                                    "系统将进行重要更新，更新内容包括："
                                }
                                ul {
                                    class: "list-disc list-inside space-y-1 text-sm",
                                    li { "修复安全漏洞" }
                                    li { "优化性能表现" }
                                    li { "新增功能特性" }
                                    li { "改进用户体验" }
                                }
                                p {
                                    class: "text-sm text-gray-600 dark:text-gray-400",
                                    "更新过程大约需要 5-10 分钟，期间系统将暂时不可用。"
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "space-y-2",
                Button {
                    button_type: ButtonType::Info,
                    onclick: move |_| open2.set(true),
                    "表单内容"
                }
                DialogRoot {
                    open: open2(),
                    on_open_change: move |v| open2.set(v),
                    DialogContent {
                        div {
                            class: "flex justify-between items-start mb-2",
                        DialogTitle {
                            "📝 用户反馈"
                        }
                        Button {
                            button_type: ButtonType::Tertiary,
                            size: ButtonSize::Small,
                            shape: ButtonShape::Circle,
                            class: "text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 flex-shrink-0",
                            onclick: move |_| open2.set(false),
                            Icon {
                                icon: IconType::Close,
                            }
                        }
                        }
                        DialogDescription {
                            div {
                                class: "space-y-3",
                                p {
                                    "请告诉我们您遇到的问题："
                                }
                                textarea {
                                    class: "w-full p-2 border border-gray-300 dark:border-gray-600 rounded-md resize-none",
                                    placeholder: "请描述您遇到的问题...",
                                    rows: 3
                                }
                                div {
                                    class: "flex items-center gap-2",
                                    input {
                                        r#type: "checkbox",
                                        id: "urgent"
                                    }
                                    label {
                                        r#for: "urgent",
                                        class: "text-sm",
                                        "紧急问题"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DialogModalDemo() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        div {
            class: "space-y-4",
            Button {
                button_type: ButtonType::Primary,
                onclick: move |_| open.set(true),
                "打开模态对话框"
            }
            DialogRoot {
                open: open(),
                on_open_change: move |v| open.set(v),
                DialogContent {
                    div {
                        class: "flex justify-between items-start mb-2",
                        DialogTitle {
                            "模态对话框"
                        }
                        Button {
                            button_type: ButtonType::Tertiary,
                            size: ButtonSize::Small,
                            shape: ButtonShape::Circle,
                            class: "text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 flex-shrink-0",
                            onclick: move |_| open.set(false),
                            Icon {
                                icon: IconType::Close,
                            }
                        }
                    }
                    DialogDescription {
                        "这是一个模态对话框，点击背景或按 ESC 键可以关闭。"
                    }
                }
            }
        }
    }
}

#[component]
fn DialogAdvancedDemo() -> Element {
    let mut open1 = use_signal(|| false);
    let mut open2 = use_signal(|| false);

    rsx! {
        div {
            class: "flex flex-wrap gap-4",
            div {
                class: "space-y-2",
                Button {
                    button_type: ButtonType::Primary,
                    onclick: move |_| open1.set(true),
                    "多步骤对话框"
                }
                DialogRoot {
                    open: open1(),
                    on_open_change: move |v| open1.set(v),
                    DialogContent {
                        div {
                            class: "flex justify-between items-start mb-2",
                        DialogTitle {
                            "🚀 发布新版本"
                        }
                        Button {
                            button_type: ButtonType::Tertiary,
                            size: ButtonSize::Small,
                            shape: ButtonShape::Circle,
                            class: "text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 flex-shrink-0",
                            onclick: move |_| open1.set(false),
                            Icon {
                                icon: IconType::Close,
                            }
                        }
                        }
                        DialogDescription {
                            div {
                                class: "space-y-4",
                                div {
                                    class: "flex items-center gap-2",
                                    div {
                                        class: "w-8 h-8 rounded-full bg-blue-100 dark:bg-blue-900 flex items-center justify-center text-sm font-medium",
                                        "1"
                                    }
                                    span { "检查代码质量" }
                                }
                                div {
                                    class: "flex items-center gap-2",
                                    div {
                                        class: "w-8 h-8 rounded-full bg-blue-100 dark:bg-blue-900 flex items-center justify-center text-sm font-medium",
                                        "2"
                                    }
                                    span { "运行测试套件" }
                                }
                                div {
                                    class: "flex items-center gap-2",
                                    div {
                                        class: "w-8 h-8 rounded-full bg-blue-100 dark:bg-blue-900 flex items-center justify-center text-sm font-medium",
                                        "3"
                                    }
                                    span { "构建生产版本" }
                                }
                                div {
                                    class: "flex items-center gap-2",
                                    div {
                                        class: "w-8 h-8 rounded-full bg-blue-100 dark:bg-blue-900 flex items-center justify-center text-sm font-medium",
                                        "4"
                                    }
                                    span { "部署到服务器" }
                                }
                                p {
                                    class: "text-sm text-gray-600 dark:text-gray-400",
                                    "请确认您已完成所有必要的准备工作。"
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "space-y-2",
                Button {
                    button_type: ButtonType::Info,
                    onclick: move |_| open2.set(true),
                    "动态内容"
                }
                DialogRoot {
                    open: open2(),
                    on_open_change: move |v| open2.set(v),
                    DialogContent {
                        div {
                            class: "flex justify-between items-start mb-2",
                        DialogTitle {
                            "📊 数据分析"
                        }
                        Button {
                            button_type: ButtonType::Tertiary,
                            size: ButtonSize::Small,
                            shape: ButtonShape::Circle,
                            class: "text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 flex-shrink-0",
                            onclick: move |_| open2.set(false),
                            Icon {
                                icon: IconType::Close,
                            }
                        }
                        }
                        DialogDescription {
                            div {
                                class: "space-y-3",
                                p {
                                    "正在分析您的数据，请稍候..."
                                }
                                div {
                                    class: "w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2",
                                    div {
                                        class: "bg-blue-600 h-2 rounded-full transition-all duration-300",
                                        style: "width: 75%"
                                    }
                                }
                                div {
                                    class: "flex justify-between text-sm text-gray-600 dark:text-gray-400",
                                    span { "进度: 75%" }
                                    span { "预计剩余时间: 30秒" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
