use crate::views::layout::ComponentsSidebar;
use crate::views::layout::TocItem;
use crate::views::DocPage;
use dioxus::prelude::*;
use helixui::components::{
    Button, ButtonType, ContentAlign, ContentSide, Tooltip, TooltipContent, TooltipTrigger,
};
use helixui::components::{DemoBox, Table};

/// Tooltip 演示页面
#[component]
pub fn TooltipPage() -> Element {
    // 定义目录项
    let toc_items = vec![
        TocItem {
            id: "basic".to_string(),
            title: "基础用法".to_string(),
            level: 1,
        },
        TocItem {
            id: "positions".to_string(),
            title: "不同位置".to_string(),
            level: 1,
        },
        TocItem {
            id: "alignment".to_string(),
            title: "对齐方式".to_string(),
            level: 1,
        },
        TocItem {
            id: "rich-content".to_string(),
            title: "富文本内容".to_string(),
            level: 1,
        },
        TocItem {
            id: "interactive".to_string(),
            title: "交互式内容".to_string(),
            level: 1,
        },
        TocItem {
            id: "advanced".to_string(),
            title: "高级用法".to_string(),
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
                        "Tooltip 提示框"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300",
                        "Tooltip 组件用于在鼠标悬停或聚焦时显示额外的信息，支持多种位置、对齐方式和丰富的交互功能。"
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
                        description: "最简单的 Tooltip 用法，鼠标悬停时显示提示信息。".to_string(),
                        code: r#"use helixui::components::{Tooltip, TooltipContent, TooltipTrigger, Button, ButtonType};

#[component]
fn TooltipDemo() -> Element {
    rsx! {
        Tooltip {
            TooltipTrigger {
                Button {
                    button_type: ButtonType::Primary,
                    "悬停显示提示"
                }
            }
            TooltipContent {
                "这是一个提示信息"
            }
        }
    }
}"#.to_string(),

                        children: rsx! {
                            TooltipDemo {}
                        }
                    }
                }

                // 不同位置
                section {
                    id: "positions",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "不同位置"
                    }

                    DemoBox {
                        title: "不同位置".to_string(),
                        description: "Tooltip 可以在触发元素的上下左右四个方向显示，自动调整位置避免超出屏幕边界。".to_string(),
                        code: r#"use helixui::components::{Tooltip, TooltipContent, TooltipTrigger, ContentSide, Button, ButtonType};

#[component]
fn TooltipPositionsDemo() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-6 items-center",
            div {
                class: "flex gap-4",
                Tooltip {
                    TooltipTrigger {
                        Button {
                            button_type: ButtonType::Primary,
                            "上方"
                        }
                    }
                    TooltipContent {
                        side: ContentSide::Top,
                        "在上方显示"
                    }
                }
            }
            div {
                class: "flex gap-4",
                Tooltip {
                    TooltipTrigger {
                        Button {
                            button_type: ButtonType::Primary,
                            "左侧"
                        }
                    }
                    TooltipContent {
                        side: ContentSide::Left,
                        "在左侧显示"
                    }
                }
                Tooltip {
                    TooltipTrigger {
                        Button {
                            button_type: ButtonType::Primary,
                            "右侧"
                        }
                    }
                    TooltipContent {
                        side: ContentSide::Right,
                        "在右侧显示"
                    }
                }
            }
            div {
                class: "flex gap-4",
                Tooltip {
                    TooltipTrigger {
                        Button {
                            button_type: ButtonType::Primary,
                            "下方"
                        }
                    }
                    TooltipContent {
                        side: ContentSide::Bottom,
                        "在下方显示"
                    }
                }
            }
        }
    }
}"#.to_string(),

                        children: rsx! {
                            TooltipPositionsDemo {}
                        }
                    }
                }

                // 对齐方式
                section {
                    id: "alignment",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "对齐方式"
                    }

                    DemoBox {
                        title: "对齐方式".to_string(),
                        description: "可以设置 Tooltip 相对于触发元素的对齐方式，提供更精确的定位控制。".to_string(),
                        code: r#"use helixui::components::{Tooltip, TooltipContent, TooltipTrigger, ContentSide, ContentAlign, Button, ButtonType};

#[component]
fn TooltipAlignmentDemo() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-6 items-center",
            div {
                class: "flex gap-4",
                Tooltip {
                    TooltipTrigger {
                        Button {
                            button_type: ButtonType::Primary,
                            "开始对齐"
                        }
                    }
                    TooltipContent {
                        side: ContentSide::Top,
                        align: ContentAlign::Start,
                        "开始对齐"
                    }
                }
                Tooltip {
                    TooltipTrigger {
                        Button {
                            button_type: ButtonType::Primary,
                            "居中对齐"
                        }
                    }
                    TooltipContent {
                        side: ContentSide::Top,
                        align: ContentAlign::Center,
                        "居中对齐"
                    }
                }
                Tooltip {
                    TooltipTrigger {
                        Button {
                            button_type: ButtonType::Primary,
                            "结束对齐"
                        }
                    }
                    TooltipContent {
                        side: ContentSide::Top,
                        align: ContentAlign::End,
                        "结束对齐"
                    }
                }
            }
        }
    }
}"#.to_string(),

                        children: rsx! {
                            TooltipAlignmentDemo {}
                        }
                    }
                }

                // 富文本内容
                section {
                    id: "rich-content",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "富文本内容"
                    }

                    DemoBox {
                        title: "富文本内容".to_string(),
                        description: "Tooltip 可以包含丰富的 HTML 内容，支持标题、段落、列表等多种元素。".to_string(),
                        code: r#"use helixui::components::{Tooltip, TooltipContent, TooltipTrigger, Button, ButtonType};

#[component]
fn TooltipRichContentDemo() -> Element {
    rsx! {
        div {
            class: "flex flex-wrap gap-4",
            Tooltip {
                TooltipTrigger {
                    Button {
                        button_type: ButtonType::Primary,
                        "悬停查看详细信息"
                    }
                }
                TooltipContent {
                    style: "width: 250px;",
                    h4 { 
                        style: "margin-top: 0; margin-bottom: 8px; font-weight: 600;", 
                        "📋 详细信息" 
                    }
                    p { 
                        style: "margin: 0 0 8px 0;", 
                        "这是一个包含丰富内容的 Tooltip，可以显示标题、段落等多种元素。" 
                    }
                    ul {
                        style: "margin: 0; padding-left: 16px;",
                        li { "支持多种 HTML 元素" }
                        li { "可以自定义样式" }
                        li { "响应式布局" }
                    }
                }
            }
            Tooltip {
                TooltipTrigger {
                    Button {
                        button_type: ButtonType::Success,
                        "查看功能说明"
                    }
                }
                TooltipContent {
                    style: "width: 200px;",
                    h4 { 
                        style: "margin-top: 0; margin-bottom: 8px; color: #059669;", 
                        "✨ 新功能" 
                    }
                    p { 
                        style: "margin: 0;", 
                        "这个功能可以帮助您更好地管理任务和提升工作效率。" 
                    }
                }
            }
        }
    }
}"#.to_string(),

                        children: rsx! {
                            TooltipRichContentDemo {}
                        }
                    }
                }

                // 交互式内容
                section {
                    id: "interactive",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "交互式内容"
                    }

                    DemoBox {
                        title: "交互式内容".to_string(),
                        description: "Tooltip 可以包含交互式元素，如按钮、链接等，提供更丰富的用户体验。".to_string(),
                        code: r#"use helixui::components::{Tooltip, TooltipContent, TooltipTrigger, Button, ButtonType};

#[component]
fn TooltipInteractiveDemo() -> Element {
    rsx! {
        div {
            class: "flex flex-wrap gap-4",
            Tooltip {
                TooltipTrigger {
                    Button {
                        button_type: ButtonType::Primary,
                        "悬停查看操作"
                    }
                }
                TooltipContent {
                    style: "width: 200px;",
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        p { 
                            style: "margin: 0; font-weight: 500;", 
                            "快速操作" 
                        }
                        div {
                            style: "display: flex; gap: 4px;",
                            Button {
                                button_type: ButtonType::Success,
                                "编辑"
                            }
                            Button {
                                button_type: ButtonType::Error,
                                "删除"
                            }
                        }
                    }
                }
            }
            Tooltip {
                TooltipTrigger {
                    Button {
                        button_type: ButtonType::Info,
                        "查看帮助链接"
                    }
                }
                TooltipContent {
                    style: "width: 180px;",
                    div {
                        p { 
                            style: "margin: 0 0 8px 0;", 
                            "需要更多帮助？" 
                        }
                        span {
                            style: "color: #3b82f6; text-decoration: underline;",
                            "查看详细文档"
                        }
                    }
                }
            }
        }
    }
}"#.to_string(),

                        children: rsx! {
                            TooltipInteractiveDemo {}
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
                        description: "展示 Tooltip 的高级用法，包括复杂布局、多级提示和特殊场景应用。".to_string(),
                        code: r#"use helixui::components::{Tooltip, TooltipContent, TooltipTrigger, ContentSide, Button, ButtonType};

#[component]
fn TooltipAdvancedDemo() -> Element {
    rsx! {
        div {
            class: "flex flex-wrap gap-4",
            Tooltip {
                TooltipTrigger {
                    Button {
                        button_type: ButtonType::Primary,
                        "🎯 智能提示"
                    }
                }
                TooltipContent {
                    style: "width: 280px;",
                    div {
                        style: "display: flex; align-items: center; gap: 8px; margin-bottom: 8px;",
                        span { 
                            style: "font-size: 16px;", 
                            "🎯" 
                        }
                        h4 { 
                            style: "margin: 0; font-weight: 600;", 
                            "智能建议" 
                        }
                    }
                    p { 
                        style: "margin: 0 0 8px 0;", 
                        "基于您的使用习惯，我们为您推荐以下操作：" 
                    }
                    div {
                        style: "display: flex; flex-wrap: wrap; gap: 4px;",
                        span {
                            style: "background: #f3f4f6; padding: 2px 6px; border-radius: 4px; font-size: 12px;",
                            "快捷键"
                        }
                        span {
                            style: "background: #f3f4f6; padding: 2px 6px; border-radius: 4px; font-size: 12px;",
                            "自动保存"
                        }
                        span {
                            style: "background: #f3f4f6; padding: 2px 6px; border-radius: 4px; font-size: 12px;",
                            "智能补全"
                        }
                    }
                }
            }
            Tooltip {
                TooltipTrigger {
                    Button {
                        button_type: ButtonType::Warning,
                        "⚠️ 状态提示"
                    }
                }
                TooltipContent {
                    side: ContentSide::Left,
                    style: "width: 220px;",
                    div {
                        style: "display: flex; align-items: center; gap: 8px; margin-bottom: 8px;",
                        span { 
                            style: "font-size: 16px;", 
                            "⚠️" 
                        }
                        h4 { 
                            style: "margin: 0; font-weight: 600; color: #d97706;", 
                            "注意事项" 
                        }
                    }
                    p { 
                        style: "margin: 0;", 
                        "此操作将影响多个文件，请确认您已备份重要数据。" 
                    }
                }
            }
            Tooltip {
                TooltipTrigger {
                    Button {
                        button_type: ButtonType::Success,
                        "📊 数据概览"
                    }
                }
                TooltipContent {
                    side: ContentSide::Right,
                    style: "width: 200px;",
                    div {
                        style: "display: flex; align-items: center; gap: 8px; margin-bottom: 8px;",
                        span { 
                            style: "font-size: 16px;", 
                            "📊" 
                        }
                        h4 { 
                            style: "margin: 0; font-weight: 600; color: #059669;", 
                            "数据统计" 
                        }
                    }
                    div {
                        style: "display: flex; justify-content: space-between; margin-bottom: 4px;",
                        span { "总文件数:" }
                        span { "1,234" }
                    }
                    div {
                        style: "display: flex; justify-content: space-between; margin-bottom: 4px;",
                        span { "已使用:" }
                        span { "85%" }
                    }
                    div {
                        style: "display: flex; justify-content: space-between;",
                        span { "最后更新:" }
                        span { "2分钟前" }
                    }
                }
            }
        }
    }
}"#.to_string(),

                        children: rsx! {
                            TooltipAdvancedDemo {}
                        }
                    }
                }

                // API 文档
                section {
                    id: "api",
                    class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4", "API" }

                    h3 { class: "text-xl font-semibold text-gray-900 dark:text-white mb-3", "Tooltip Props" }
                    Table {
                        headers: Some(vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()]),
                        data: vec![
                            vec!["children".to_string(), "Element".to_string(), "-".to_string(), "触发器与内容".to_string()],
                        ],
                        bordered: true,
                        striped: true,
                    }

                    h3 { class: "text-xl font-semibold text-gray-900 dark:text-white mb-3 mt-8", "TooltipTrigger Props" }
                    Table {
                        headers: Some(vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()]),
                        data: vec![
                            vec!["children".to_string(), "Element".to_string(), "-".to_string(), "触发元素".to_string()],
                        ],
                        bordered: true,
                        striped: true,
                    }

                    h3 { class: "text-xl font-semibold text-gray-900 dark:text-white mb-3 mt-8", "TooltipContent Props" }
                    Table {
                        headers: Some(vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()]),
                        data: vec![
                            vec!["side".to_string(), "ContentSide".to_string(), "Top".to_string(), "显示位置（Top/Right/Bottom/Left）".to_string()],
                            vec!["align".to_string(), "ContentAlign".to_string(), "Center".to_string(), "对齐方式（Start/Center/End）".to_string()],
                            vec!["class".to_string(), "Option<String>".to_string(), "None".to_string(), "自定义样式类".to_string()],
                            vec!["style".to_string(), "Option<String>".to_string(), "None".to_string(), "内联样式".to_string()],
                            vec!["children".to_string(), "Element".to_string(), "-".to_string(), "提示内容".to_string()],
                        ],
                        bordered: true,
                        striped: true,
                    }
                }
            }
        }
    }
}

// 演示组件
#[component]
fn TooltipDemo() -> Element {
    rsx! {
        Tooltip {
            TooltipTrigger {
                Button {
                    button_type: ButtonType::Primary,
                    "悬停显示提示"
                }
            }
            TooltipContent {
                "这是一个提示信息"
            }
        }
    }
}

#[component]
fn TooltipPositionsDemo() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-6 items-center",
            div {
                class: "flex gap-4",
                Tooltip {
                    TooltipTrigger {
                        Button {
                            button_type: ButtonType::Primary,
                            "上方"
                        }
                    }
                    TooltipContent {
                        side: ContentSide::Top,
                        "在上方显示"
                    }
                }
            }
            div {
                class: "flex gap-4",
                Tooltip {
                    TooltipTrigger {
                        Button {
                            button_type: ButtonType::Primary,
                            "左侧"
                        }
                    }
                    TooltipContent {
                        side: ContentSide::Left,
                        "在左侧显示"
                    }
                }
                Tooltip {
                    TooltipTrigger {
                        Button {
                            button_type: ButtonType::Primary,
                            "右侧"
                        }
                    }
                    TooltipContent {
                        side: ContentSide::Right,
                        "在右侧显示"
                    }
                }
            }
            div {
                class: "flex gap-4",
                Tooltip {
                    TooltipTrigger {
                        Button {
                            button_type: ButtonType::Primary,
                            "下方"
                        }
                    }
                    TooltipContent {
                        side: ContentSide::Bottom,
                        "在下方显示"
                    }
                }
            }
        }
    }
}

#[component]
fn TooltipAlignmentDemo() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-6 items-center",
            div {
                class: "flex gap-4",
                Tooltip {
                    TooltipTrigger {
                        Button {
                            button_type: ButtonType::Primary,
                            "开始对齐"
                        }
                    }
                    TooltipContent {
                        side: ContentSide::Top,
                        align: ContentAlign::Start,
                        "开始对齐"
                    }
                }
                Tooltip {
                    TooltipTrigger {
                        Button {
                            button_type: ButtonType::Primary,
                            "居中对齐"
                        }
                    }
                    TooltipContent {
                        side: ContentSide::Top,
                        align: ContentAlign::Center,
                        "居中对齐"
                    }
                }
                Tooltip {
                    TooltipTrigger {
                        Button {
                            button_type: ButtonType::Primary,
                            "结束对齐"
                        }
                    }
                    TooltipContent {
                        side: ContentSide::Top,
                        align: ContentAlign::End,
                        "结束对齐"
                    }
                }
            }
        }
    }
}

#[component]
fn TooltipRichContentDemo() -> Element {
    rsx! {
        div {
            class: "flex flex-wrap gap-4",
            Tooltip {
                TooltipTrigger {
                    Button {
                        button_type: ButtonType::Primary,
                        "悬停查看详细信息"
                    }
                }
                TooltipContent {
                    style: "width: 250px;",
                    h4 {
                        style: "margin-top: 0; margin-bottom: 8px; font-weight: 600;",
                        "📋 详细信息"
                    }
                    p {
                        style: "margin: 0 0 8px 0;",
                        "这是一个包含丰富内容的 Tooltip，可以显示标题、段落等多种元素。"
                    }
                    ul {
                        style: "margin: 0; padding-left: 16px;",
                        li { "支持多种 HTML 元素" }
                        li { "可以自定义样式" }
                        li { "响应式布局" }
                    }
                }
            }
            Tooltip {
                TooltipTrigger {
                    Button {
                        button_type: ButtonType::Success,
                        "查看功能说明"
                    }
                }
                TooltipContent {
                    style: "width: 200px;",
                    h4 {
                        style: "margin-top: 0; margin-bottom: 8px; color: #059669;",
                        "✨ 新功能"
                    }
                    p {
                        style: "margin: 0;",
                        "这个功能可以帮助您更好地管理任务和提升工作效率。"
                    }
                }
            }
        }
    }
}

#[component]
fn TooltipInteractiveDemo() -> Element {
    rsx! {
        div {
            class: "flex flex-wrap gap-4",
            Tooltip {
                TooltipTrigger {
                    Button {
                        button_type: ButtonType::Primary,
                        "悬停查看操作"
                    }
                }
                TooltipContent {
                    style: "width: 200px;",
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        p {
                            style: "margin: 0; font-weight: 500;",
                            "快速操作"
                        }
                        div {
                            style: "display: flex; gap: 4px;",
                            Button {
                                button_type: ButtonType::Success,
                                "编辑"
                            }
                            Button {
                                button_type: ButtonType::Error,
                                "删除"
                            }
                        }
                    }
                }
            }
            Tooltip {
                TooltipTrigger {
                    Button {
                        button_type: ButtonType::Info,
                        "查看帮助链接"
                    }
                }
                TooltipContent {
                    style: "width: 180px;",
                    div {
                        p {
                            style: "margin: 0 0 8px 0;",
                            "需要更多帮助？"
                        }
                        span {
                            style: "color: #3b82f6; text-decoration: underline;",
                            "查看详细文档"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TooltipAdvancedDemo() -> Element {
    rsx! {
        div {
            class: "flex flex-wrap gap-4",
            Tooltip {
                TooltipTrigger {
                    Button {
                        button_type: ButtonType::Primary,
                        "🎯 智能提示"
                    }
                }
                TooltipContent {
                    style: "width: 280px;",
                    div {
                        style: "display: flex; align-items: center; gap: 8px; margin-bottom: 8px;",
                        span {
                            style: "font-size: 16px;",
                            "🎯"
                        }
                        h4 {
                            style: "margin: 0; font-weight: 600;",
                            "智能建议"
                        }
                    }
                    p {
                        style: "margin: 0 0 8px 0;",
                        "基于您的使用习惯，我们为您推荐以下操作："
                    }
                    div {
                        style: "display: flex; flex-wrap: wrap; gap: 4px;",
                        span {
                            style: "background: #f3f4f6; padding: 2px 6px; border-radius: 4px; font-size: 12px;",
                            "快捷键"
                        }
                        span {
                            style: "background: #f3f4f6; padding: 2px 6px; border-radius: 4px; font-size: 12px;",
                            "自动保存"
                        }
                        span {
                            style: "background: #f3f4f6; padding: 2px 6px; border-radius: 4px; font-size: 12px;",
                            "智能补全"
                        }
                    }
                }
            }
            Tooltip {
                TooltipTrigger {
                    Button {
                        button_type: ButtonType::Warning,
                        "⚠️ 状态提示"
                    }
                }
                TooltipContent {
                    side: ContentSide::Left,
                    style: "width: 220px;",
                    div {
                        style: "display: flex; align-items: center; gap: 8px; margin-bottom: 8px;",
                        span {
                            style: "font-size: 16px;",
                            "⚠️"
                        }
                        h4 {
                            style: "margin: 0; font-weight: 600; color: #d97706;",
                            "注意事项"
                        }
                    }
                    p {
                        style: "margin: 0;",
                        "此操作将影响多个文件，请确认您已备份重要数据。"
                    }
                }
            }
            Tooltip {
                TooltipTrigger {
                    Button {
                        button_type: ButtonType::Success,
                        "📊 数据概览"
                    }
                }
                TooltipContent {
                    side: ContentSide::Right,
                    style: "width: 200px;",
                    div {
                        style: "display: flex; align-items: center; gap: 8px; margin-bottom: 8px;",
                        span {
                            style: "font-size: 16px;",
                            "📊"
                        }
                        h4 {
                            style: "margin: 0; font-weight: 600; color: #059669;",
                            "数据统计"
                        }
                    }
                    div {
                        style: "display: flex; justify-content: space-between; margin-bottom: 4px;",
                        span { "总文件数:" }
                        span { "1,234" }
                    }
                    div {
                        style: "display: flex; justify-content: space-between; margin-bottom: 4px;",
                        span { "已使用:" }
                        span { "85%" }
                    }
                    div {
                        style: "display: flex; justify-content: space-between;",
                        span { "最后更新:" }
                        span { "2分钟前" }
                    }
                }
            }
        }
    }
}
