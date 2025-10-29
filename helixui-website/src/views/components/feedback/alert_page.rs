use crate::views::layout::ComponentsSidebar;
use crate::views::layout::TocItem;
use crate::views::DocPage;
use dioxus::prelude::*;
use helixui::components::{
    AlertDialogAction, AlertDialogActions, AlertDialogCancel, AlertDialogContent,
    AlertDialogDescription, AlertDialogRoot, AlertDialogTitle, Button, ButtonType,
};
use helixui::components::{DemoBox, Table};

/// Alert 演示页面
#[component]
pub fn AlertPage() -> Element {
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
            id: "destructive".to_string(),
            title: "危险操作".to_string(),
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
                        "Alert 警告框"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300",
                        "Alert 组件用于显示重要的警告信息，支持多种变体、自定义内容和丰富的交互功能。"
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
                        description: "最简单的 Alert 用法，显示一条警告信息。".to_string(),
                        code: r#"use helixui::components::{AlertDialogRoot, AlertDialogContent, AlertDialogTitle, AlertDialogDescription, AlertDialogActions, AlertDialogAction, AlertDialogCancel, Button, Button, ButtonType};

#[component]
fn AlertDemo() -> Element {
    rsx! {
        AlertDialogRoot {
            Button {
                Button {
                    button_type: ButtonType::Primary,
                    "显示警告"
                }
            }
            AlertDialogContent {
                AlertDialogTitle {
                    "确认操作"
                }
                AlertDialogDescription {
                    "您确定要执行此操作吗？此操作无法撤销。"
                }
                AlertDialogActions {
                    AlertDialogCancel {
                        Button {
                            button_type: ButtonType::Tertiary,
                            "取消"
                        }
                    }
                    AlertDialogAction {
                        Button {
                            button_type: ButtonType::Primary,
                            "确认"
                        }
                    }
                }
            }
        }
    }
}"#.to_string(),

                        children: rsx! {
                            AlertDemo {}
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
                        description: "Alert 支持多种变体，适应不同的使用场景和重要性级别。".to_string(),
                        code: r#"use helixui::components::{AlertDialogRoot, AlertDialogContent, AlertDialogTitle, AlertDialogDescription, AlertDialogActions, AlertDialogAction, AlertDialogCancel, Button, Button, ButtonType};

#[component]
fn AlertVariantsDemo() -> Element {
    rsx! {
        div {
            class: "flex flex-wrap gap-4",
            AlertDialogRoot {
                Button {
                    Button {
                        button_type: ButtonType::Primary,
                        "信息提示"
                    }
                }
                AlertDialogContent {
                    AlertDialogTitle {
                        "📋 信息提示"
                    }
                    AlertDialogDescription {
                        "这是一条信息提示，用于告知用户当前状态或提供帮助信息。"
                    }
                    AlertDialogActions {
                        AlertDialogAction {
                            Button {
                                button_type: ButtonType::Primary,
                                "知道了"
                            }
                        }
                    }
                }
            }
            AlertDialogRoot {
                Button {
                    Button {
                        button_type: ButtonType::Warning,
                        "警告提示"
                    }
                }
                AlertDialogContent {
                    AlertDialogTitle {
                        "⚠️ 警告提示"
                    }
                    AlertDialogDescription {
                        "这是一条警告提示，提醒用户注意潜在的风险或问题。"
                    }
                    AlertDialogActions {
                        AlertDialogCancel {
                            Button {
                                button_type: ButtonType::Tertiary,
                                "取消"
                            }
                        }
                        AlertDialogAction {
                            Button {
                                button_type: ButtonType::Warning,
                                "继续"
                            }
                        }
                    }
                }
            }
            AlertDialogRoot {
                Button {
                    Button {
                        button_type: ButtonType::Error,
                        "错误提示"
                    }
                }
                AlertDialogContent {
                    AlertDialogTitle {
                        "❌ 错误提示"
                    }
                    AlertDialogDescription {
                        "操作失败，请检查输入信息后重试。"
                    }
                    AlertDialogActions {
                        AlertDialogAction {
                            Button {
                                button_type: ButtonType::Error,
                                "重试"
                            }
                        }
                    }
                }
            }
        }
    }
}"#.to_string(),

                        children: rsx! {
                            AlertVariantsDemo {}
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
                        description: "Alert 可以包含丰富的自定义内容，如图标、列表、表单等。".to_string(),
                        code: r#"use helixui::components::{AlertDialogRoot, AlertDialogContent, AlertDialogTitle, AlertDialogDescription, AlertDialogActions, AlertDialogAction, AlertDialogCancel, Button, Button, ButtonType};

#[component]
fn AlertCustomContentDemo() -> Element {
    rsx! {
        div {
            class: "flex flex-wrap gap-4",
            AlertDialogRoot {
                Button {
                    button_type: ButtonType::Primary,
                    "复杂内容"
                }
                AlertDialogContent {
                    AlertDialogTitle {
                        "🎯 系统更新"
                    }
                    AlertDialogDescription {
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
                    AlertDialogActions {
                        AlertDialogCancel {
                            Button {
                                button_type: ButtonType::Tertiary,
                                "稍后更新"
                            }
                        }
                        AlertDialogAction {
                            Button {
                                button_type: ButtonType::Primary,
                                "立即更新"
                            }
                        }
                    }
                }
            }
            AlertDialogRoot {
                Button {
                    button_type: ButtonType::Info,
                    "表单内容"
                }
                AlertDialogContent {
                    AlertDialogTitle {
                        "📝 用户反馈"
                    }
                    AlertDialogDescription {
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
                    AlertDialogActions {
                        AlertDialogCancel {
                            Button {
                                button_type: ButtonType::Tertiary,
                                "取消"
                            }
                        }
                        AlertDialogAction {
                            Button {
                                button_type: ButtonType::Primary,
                                "提交反馈"
                            }
                        }
                    }
                }
            }
        }
    }
}"#.to_string(),

                        children: rsx! {
                            AlertCustomContentDemo {}
                        }
                    }
                }

                // 危险操作
                section {
                    id: "destructive",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "危险操作"
                    }

                    DemoBox {
                        title: "危险操作".to_string(),
                        description: "对于危险操作，Alert 提供特殊的视觉样式和交互模式。".to_string(),
                        code: r#"use helixui::components::{AlertDialogRoot, AlertDialogContent, AlertDialogTitle, AlertDialogDescription, AlertDialogActions, AlertDialogAction, AlertDialogCancel, Button, ButtonType};

#[component]
fn AlertDestructiveDemo() -> Element {
    let mut open1 = use_signal(|| false);
    let mut open2 = use_signal(|| false);
    
    rsx! {
        div {
            class: "flex flex-wrap gap-4",
            div {
                class: "space-y-2",
                Button {
                    button_type: ButtonType::Error,
                    onclick: move |_| open1.set(true),
                    "删除账户"
                }
                AlertDialogRoot {
                    open: open1(),
                    on_open_change: move |v| open1.set(v),
                    AlertDialogContent {
                        AlertDialogTitle {
                            "🗑️ 删除账户"
                        }
                        AlertDialogDescription {
                            div {
                                class: "space-y-3",
                                p {
                                    "您确定要删除您的账户吗？"
                                }
                                div {
                                    class: "p-3 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-md",
                                    p {
                                        class: "text-sm text-red-800 dark:text-red-200 font-medium",
                                        "⚠️ 此操作将永久删除："
                                    }
                                    ul {
                                        class: "list-disc list-inside text-sm text-red-700 dark:text-red-300 mt-2 space-y-1",
                                        li { "您的所有数据" }
                                        li { "您的个人资料" }
                                        li { "您的历史记录" }
                                        li { "您的设置和偏好" }
                                    }
                                }
                                p {
                                    class: "text-sm text-gray-600 dark:text-gray-400",
                                    "此操作无法撤销，请谨慎考虑。"
                                }
                            }
                        }
                        AlertDialogActions {
                            AlertDialogCancel {
                                "取消"
                            }
                            AlertDialogAction {
                                "确认删除"
                            }
                        }
                    }
                }
            }
            div {
                class: "space-y-2",
                Button {
                    button_type: ButtonType::Warning,
                    onclick: move |_| open2.set(true),
                    "重置设置"
                }
                AlertDialogRoot {
                    open: open2(),
                    on_open_change: move |v| open2.set(v),
                    AlertDialogContent {
                        AlertDialogTitle {
                            "🔄 重置设置"
                        }
                        AlertDialogDescription {
                            div {
                                class: "space-y-3",
                                p {
                                    "您确定要重置所有设置到默认值吗？"
                                }
                                div {
                                    class: "p-3 bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-md",
                                    p {
                                        class: "text-sm text-yellow-800 dark:text-yellow-200 font-medium",
                                        "⚠️ 重置后将丢失："
                                    }
                                    ul {
                                        class: "list-disc list-inside text-sm text-yellow-700 dark:text-yellow-300 mt-2 space-y-1",
                                        li { "自定义主题设置" }
                                        li { "个人偏好配置" }
                                        li { "快捷键绑定" }
                                        li { "界面布局设置" }
                                    }
                                }
                            }
                        }
                        AlertDialogActions {
                            AlertDialogCancel {
                                "取消"
                            }
                            AlertDialogAction {
                                "确认重置"
                            }
                        }
                    }
                }
            }
        }
    }
}"#.to_string(),

                        children: rsx! {
                            AlertDestructiveDemo {}
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
                        description: "展示 Alert 的高级用法，包括动态内容、条件渲染和复杂交互。".to_string(),
                        code: r#"use helixui::components::{AlertDialogRoot, AlertDialogContent, AlertDialogTitle, AlertDialogDescription, AlertDialogActions, AlertDialogAction, AlertDialogCancel, Button, Button, ButtonType};

"#.to_string(),

                        children: rsx! {
                            AlertAdvancedDemo {}
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

                        // AlertDialogRootRoot Props
                        div {
                            h3 {
                                class: "text-xl font-semibold text-gray-900 dark:text-white mb-4",
                                "AlertDialogRootRoot Props"
                            }
                            Table {
                                headers: Some(vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()]),
                                data: vec![
                                    vec!["id".to_string(), "ReadSignal<Option<String>>".to_string(), "None".to_string(), "Alert 对话框的 ID".to_string()],
                                    vec!["default_open".to_string(), "bool".to_string(), "false".to_string(), "默认是否打开".to_string()],
                                    vec!["open".to_string(), "ReadSignal<Option<bool>>".to_string(), "None".to_string(), "控制 Alert 的显示状态".to_string()],
                                    vec!["on_open_change".to_string(), "Callback<bool>".to_string(), "-".to_string(), "打开状态改变时的回调".to_string()],
                                    vec!["children".to_string(), "Element".to_string(), "-".to_string(), "Alert 的子元素".to_string()],
                                ],
                                bordered: true,
                                striped: true,
                            }
                        }

                        // AlertDialogContent Props
                        div {
                            h3 {
                                class: "text-xl font-semibold text-gray-900 dark:text-white mb-4",
                                "AlertDialogContent Props"
                            }
                            Table {
                                headers: Some(vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()]),
                                data: vec![
                                    vec!["id".to_string(), "ReadSignal<Option<String>>".to_string(), "None".to_string(), "内容区域的 ID".to_string()],
                                    vec!["children".to_string(), "Element".to_string(), "-".to_string(), "内容区域的子元素".to_string()],
                                ],
                                bordered: true,
                                striped: true,
                            }
                        }

                        // AlertDialogTitle Props
                        div {
                            h3 {
                                class: "text-xl font-semibold text-gray-900 dark:text-white mb-4",
                                "AlertDialogTitle Props"
                            }
                            Table {
                                headers: Some(vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()]),
                                data: vec![
                                    vec!["id".to_string(), "ReadSignal<Option<String>>".to_string(), "None".to_string(), "标题的 ID".to_string()],
                                    vec!["children".to_string(), "Element".to_string(), "-".to_string(), "标题内容".to_string()],
                                ],
                                bordered: true,
                                striped: true,
                            }
                        }

                        // AlertDialogDescription Props
                        div {
                            h3 {
                                class: "text-xl font-semibold text-gray-900 dark:text-white mb-4",
                                "AlertDialogDescription Props"
                            }
                            Table {
                                headers: Some(vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()]),
                                data: vec![
                                    vec!["id".to_string(), "ReadSignal<Option<String>>".to_string(), "None".to_string(), "描述的 ID".to_string()],
                                    vec!["children".to_string(), "Element".to_string(), "-".to_string(), "描述内容".to_string()],
                                ],
                                bordered: true,
                                striped: true,
                            }
                        }

                        // AlertDialogActions Props
                        div {
                            h3 {
                                class: "text-xl font-semibold text-gray-900 dark:text-white mb-4",
                                "AlertDialogActions Props"
                            }
                            Table {
                                headers: Some(vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()]),
                                data: vec![
                                    vec!["children".to_string(), "Element".to_string(), "-".to_string(), "操作按钮区域的内容".to_string()],
                                ],
                                bordered: true,
                                striped: true,
                            }
                        }

                        // AlertDialogAction Props
                        div {
                            h3 {
                                class: "text-xl font-semibold text-gray-900 dark:text-white mb-4",
                                "AlertDialogAction Props"
                            }
                            Table {
                                headers: Some(vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()]),
                                data: vec![
                                    vec!["children".to_string(), "Element".to_string(), "-".to_string(), "确认按钮的内容".to_string()],
                                ],
                                bordered: true,
                                striped: true,
                            }
                        }

                        // AlertDialogCancel Props
                        div {
                            h3 {
                                class: "text-xl font-semibold text-gray-900 dark:text-white mb-4",
                                "AlertDialogCancel Props"
                            }
                            Table {
                                headers: Some(vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()]),
                                data: vec![
                                    vec!["children".to_string(), "Element".to_string(), "-".to_string(), "取消按钮的内容".to_string()],
                                ],
                                bordered: true,
                                striped: true,
                            }
                        }

                        // Button Props
                        div {
                            h3 {
                                class: "text-xl font-semibold text-gray-900 dark:text-white mb-4",
                                "Button Props"
                            }
                            Table {
                                headers: Some(vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()]),
                                data: vec![
                                    vec!["children".to_string(), "Element".to_string(), "-".to_string(), "触发元素的内容".to_string()],
                                ],
                                bordered: true,
                                striped: true,
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
fn AlertDemo() -> Element {
    let mut open: Signal<Option<bool>> = use_signal(|| Some(false));

    rsx! {
        div {
            class: "space-y-4",
            Button {
                button_type: ButtonType::Primary,
                onclick: move |_| open.set(Some(true)),
                "显示警告对话框"
            }
            AlertDialogRoot {
                open: open(),
                on_open_change: move |v| open.set(Some(v)),
                AlertDialogContent {
                    AlertDialogTitle {
                        "确认操作"
                    }
                    AlertDialogDescription {
                        "您确定要执行此操作吗？此操作无法撤销。"
                    }
                    AlertDialogActions {
                        AlertDialogCancel {
                            "取消"
                        }
                        AlertDialogAction {
                            "确认"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AlertVariantsDemo() -> Element {
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
                    "信息提示"
                }
                AlertDialogRoot {
                    open: open1(),
                    on_open_change: move |v| open1.set(v),
                    AlertDialogContent {
                        AlertDialogTitle {
                            "📋 信息提示"
                        }
                        AlertDialogDescription {
                            "这是一条信息提示，用于告知用户当前状态或提供帮助信息。"
                        }
                        AlertDialogActions {
                            AlertDialogAction {
                                "知道了"
                            }
                        }
                    }
                }
            }
            div {
                class: "space-y-2",
                Button {
                    button_type: ButtonType::Warning,
                    onclick: move |_| open2.set(true),
                    "警告提示"
                }
                AlertDialogRoot {
                    open: open2(),
                    on_open_change: move |v| open2.set(v),
                    AlertDialogContent {
                        AlertDialogTitle {
                            "⚠️ 警告提示"
                        }
                        AlertDialogDescription {
                            "这是一条警告提示，提醒用户注意潜在的风险或问题。"
                        }
                        AlertDialogActions {
                            AlertDialogCancel {
                                "取消"
                            }
                            AlertDialogAction {
                                "继续"
                            }
                        }
                    }
                }
            }
            div {
                class: "space-y-2",
                Button {
                    button_type: ButtonType::Error,
                    onclick: move |_| open3.set(true),
                    "错误提示"
                }
                AlertDialogRoot {
                    open: open3(),
                    on_open_change: move |v| open3.set(v),
                    AlertDialogContent {
                        AlertDialogTitle {
                            "❌ 错误提示"
                        }
                        AlertDialogDescription {
                            "操作失败，请检查输入信息后重试。"
                        }
                        AlertDialogActions {
                            AlertDialogAction {
                                "重试"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AlertCustomContentDemo() -> Element {
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
                AlertDialogRoot {
                    open: open1(),
                    on_open_change: move |v| open1.set(v),
                    AlertDialogContent {
                        AlertDialogTitle {
                            "🎯 系统更新"
                        }
                        AlertDialogDescription {
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
                        AlertDialogActions {
                            AlertDialogCancel {
                                "稍后更新"
                            }
                            AlertDialogAction {
                                "立即更新"
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
                AlertDialogRoot {
                    open: open2(),
                    on_open_change: move |v| open2.set(v),
                    AlertDialogContent {
                        AlertDialogTitle {
                            "📝 用户反馈"
                        }
                        AlertDialogDescription {
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
                        AlertDialogActions {
                            AlertDialogCancel {
                                "取消"
                            }
                            AlertDialogAction {
                                "提交反馈"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AlertAdvancedDemo() -> Element {
    let _step = use_signal(|| 1);
    let _data = use_signal(|| "".to_string());
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
                    "多步骤确认"
                }
                AlertDialogRoot {
                    open: open1(),
                    on_open_change: move |v| open1.set(v),
                    AlertDialogContent {
                        AlertDialogTitle {
                            "🚀 发布新版本"
                        }
                        AlertDialogDescription {
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
                        AlertDialogActions {
                            AlertDialogCancel {
                                "取消"
                            }
                            AlertDialogAction {
                                "开始发布"
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
                AlertDialogRoot {
                    open: open2(),
                    on_open_change: move |v| open2.set(v),
                    AlertDialogContent {
                        AlertDialogTitle {
                            "📊 数据分析"
                        }
                        AlertDialogDescription {
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
                        AlertDialogActions {
                            AlertDialogCancel {
                                "取消分析"
                            }
                            AlertDialogAction {
                                "继续等待"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AlertDestructiveDemo() -> Element {
    let mut open1 = use_signal(|| false);
    let mut open2 = use_signal(|| false);

    rsx! {
        div {
            class: "flex flex-wrap gap-4",
            div {
                class: "space-y-2",
                Button {
                    button_type: ButtonType::Error,
                    onclick: move |_| open1.set(true),
                    "删除账户"
                }
                AlertDialogRoot {
                    open: open1(),
                    on_open_change: move |v| open1.set(v),
                    AlertDialogContent {
                        AlertDialogTitle {
                            "🗑️ 删除账户"
                        }
                        AlertDialogDescription {
                            div {
                                class: "space-y-3",
                                p {
                                    "您确定要删除您的账户吗？"
                                }
                                div {
                                    class: "p-3 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-md",
                                    p {
                                        class: "text-sm text-red-800 dark:text-red-200 font-medium",
                                        "⚠️ 此操作将永久删除："
                                    }
                                    ul {
                                        class: "list-disc list-inside text-sm text-red-700 dark:text-red-300 mt-2 space-y-1",
                                        li { "您的所有数据" }
                                        li { "您的个人资料" }
                                        li { "您的历史记录" }
                                        li { "您的设置和偏好" }
                                    }
                                }
                                p {
                                    class: "text-sm text-gray-600 dark:text-gray-400",
                                    "此操作无法撤销，请谨慎考虑。"
                                }
                            }
                        }
                        AlertDialogActions {
                            AlertDialogCancel {
                                "取消"
                            }
                            AlertDialogAction {
                                "确认删除"
                            }
                        }
                    }
                }
            }
            div {
                class: "space-y-2",
                Button {
                    button_type: ButtonType::Warning,
                    onclick: move |_| open2.set(true),
                    "重置设置"
                }
                AlertDialogRoot {
                    open: open2(),
                    on_open_change: move |v| open2.set(v),
                    AlertDialogContent {
                        AlertDialogTitle {
                            "🔄 重置设置"
                        }
                        AlertDialogDescription {
                            div {
                                class: "space-y-3",
                                p {
                                    "您确定要重置所有设置到默认值吗？"
                                }
                                div {
                                    class: "p-3 bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-md",
                                    p {
                                        class: "text-sm text-yellow-800 dark:text-yellow-200 font-medium",
                                        "⚠️ 重置后将丢失："
                                    }
                                    ul {
                                        class: "list-disc list-inside text-sm text-yellow-700 dark:text-yellow-300 mt-2 space-y-1",
                                        li { "自定义主题设置" }
                                        li { "个人偏好配置" }
                                        li { "快捷键绑定" }
                                        li { "界面布局设置" }
                                    }
                                }
                            }
                        }
                        AlertDialogActions {
                            AlertDialogCancel {
                                "取消"
                            }
                            AlertDialogAction {
                                "确认重置"
                            }
                        }
                    }
                }
            }
        }
    }
}
