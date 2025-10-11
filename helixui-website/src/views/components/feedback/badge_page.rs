use helixui::components::{
    feedback::{Badge, BadgeType},
    Button, ButtonSize, ButtonType, DemoBox,
};
use crate::views::layout::TocItem;
use crate::views::layout::{ComponentsSidebar, DocPage};
use dioxus::prelude::*;

/// Badge 页面组件
#[component]
pub fn BadgePage() -> Element {
    // 基础用法部分的状态
    let mut basic_count = use_signal(|| 5);
    let mut basic_count_str = use_signal(|| "5".to_string());

    // 受控显示部分的状态
    let controlled_count = use_signal(|| 5);
    let mut controlled_count_str = use_signal(|| "5".to_string());
    let mut show_badge = use_signal(|| true);

    // 处理中状态
    let mut processing = use_signal(|| false);

    // 同步基础用法的 count 和 count_str
    use_effect(move || {
        *basic_count_str.write() = basic_count.read().to_string();
    });

    // 同步受控显示的 count 和 count_str
    use_effect(move || {
        *controlled_count_str.write() = controlled_count.read().to_string();
    });

    // 定义目录项
    let toc_items = vec![
        TocItem {
            id: "basic".to_string(),
            title: "演示".to_string(),
            level: 1,
        },
        TocItem {
            id: "types".to_string(),
            title: "类型".to_string(),
            level: 1,
        },
        TocItem {
            id: "processing".to_string(),
            title: "处理中".to_string(),
            level: 1,
        },
        TocItem {
            id: "show-zero".to_string(),
            title: "显示 0".to_string(),
            level: 1,
        },
        TocItem {
            id: "overflow".to_string(),
            title: "溢出".to_string(),
            level: 1,
        },
        TocItem {
            id: "controlled".to_string(),
            title: "受控显示".to_string(),
            level: 1,
        },
        TocItem {
            id: "custom-content".to_string(),
            title: "自定义内容".to_string(),
            level: 1,
        },
        TocItem {
            id: "custom-color".to_string(),
            title: "自定义颜色".to_string(),
            level: 1,
        },
        TocItem {
            id: "custom-offset".to_string(),
            title: "自定义偏移".to_string(),
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
                        "标记 Badge"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300",
                        "用于显示状态标记或数量信息。"
                    }
                }

                // 基础用法
                section {
                    id: "basic",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "演示"
                    }

                    DemoBox {
                        title: "基础用法".to_string(),
                        description: "标记有数字和点两种形式。".to_string(),
                        code: r#"use helixui::components::{Badge, BadgeType};

rsx! {
    Badge {
        value: Some("5".to_string()),
        children: rsx! {
            div { class: "w-12 h-12 bg-gray-200 rounded flex items-center justify-center", "📦" }
        }
    }
    Badge {
        dot: true,
        children: rsx! {
            div { class: "w-12 h-12 bg-gray-200 rounded flex items-center justify-center", "📦" }
        }
    }
}"#.to_string(),
                        children: rsx! {
                            div {
                                class: "flex items-center gap-8",

                                Badge {
                                    value: Some(basic_count_str.read().clone()),
                                    children: rsx! {
                                        div {
                                            class: "w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded flex items-center justify-center",
                                            "📦"
                                        }
                                    }
                                }

                                Badge {
                                    dot: true,
                                    children: rsx! {
                                        div {
                                            class: "w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded flex items-center justify-center",
                                            "📦"
                                        }
                                    }
                                }
                            }

                            div {
                                class: "flex gap-2 mt-4",
                                Button {
                                    button_type: ButtonType::Primary,
                                    size: ButtonSize::Small,
                                    onclick: move |_| {
                                        *basic_count.write() += 1;
                                    },
                                    "+"
                                }
                                Button {
                                    button_type: ButtonType::Default,
                                    size: ButtonSize::Small,
                                    onclick: move |_| {
                                        if *basic_count.read() > 0 {
                                            *basic_count.write() -= 1;
                                        }
                                    },
                                    "-"
                                }
                            }
                        }
                    }
                }

                // 类型
                section {
                    id: "types",
                    class: "mb-12",
                    DemoBox {
                        title: "类型".to_string(),
                        description: "标记有 default、error、info、success、warning 类型。".to_string(),
                        code: r#"use helixui::components::{Badge, BadgeType};

rsx! {
    Badge { badge_type: BadgeType::Default, value: Some("1".to_string()), children: rsx! { div { "📦" } } }
    Badge { badge_type: BadgeType::Error, value: Some("1".to_string()), children: rsx! { div { "📦" } } }
    Badge { badge_type: BadgeType::Info, value: Some("1".to_string()), children: rsx! { div { "📦" } } }
    Badge { badge_type: BadgeType::Success, value: Some("1".to_string()), children: rsx! { div { "📦" } } }
    Badge { badge_type: BadgeType::Warning, value: Some("1".to_string()), children: rsx! { div { "📦" } } }
}"#.to_string(),
                        children: rsx! {
                            div {
                                class: "flex items-center gap-8",

                                Badge {
                                    badge_type: BadgeType::Default,
                                    value: Some("1".to_string()),
                                    children: rsx! {
                                        div {
                                            class: "w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded flex items-center justify-center",
                                            "📦"
                                        }
                                    }
                                }

                                Badge {
                                    badge_type: BadgeType::Error,
                                    value: Some("1".to_string()),
                                    children: rsx! {
                                        div {
                                            class: "w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded flex items-center justify-center",
                                            "📦"
                                        }
                                    }
                                }

                                Badge {
                                    badge_type: BadgeType::Info,
                                    value: Some("1".to_string()),
                                    children: rsx! {
                                        div {
                                            class: "w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded flex items-center justify-center",
                                            "📦"
                                        }
                                    }
                                }

                                Badge {
                                    badge_type: BadgeType::Success,
                                    value: Some("1".to_string()),
                                    children: rsx! {
                                        div {
                                            class: "w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded flex items-center justify-center",
                                            "📦"
                                        }
                                    }
                                }

                                Badge {
                                    badge_type: BadgeType::Warning,
                                    value: Some("1".to_string()),
                                    children: rsx! {
                                        div {
                                            class: "w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded flex items-center justify-center",
                                            "📦"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // 处理中
                section {
                    id: "processing",
                    class: "mb-12",
                    DemoBox {
                        title: "处理中".to_string(),
                        description: "设定 processing 来表明正在处理。".to_string(),
                        code: r#"use helixui::components::{Badge, BadgeType};

rsx! {
    Badge { badge_type: BadgeType::Error, dot: true, children: rsx! { div { "📦" } } }
    Badge { 
        badge_type: BadgeType::Error, 
        value: Some("20".to_string()),
        processing: true,
        children: rsx! { div { "📦" } } 
    }
    Badge { badge_type: BadgeType::Info, dot: true, children: rsx! { div { "📦" } } }
}"#.to_string(),
                        children: rsx! {
                            div {
                                class: "flex items-center gap-8",

                                Badge {
                                    badge_type: BadgeType::Error,
                                    dot: true,
                                    children: rsx! {
                                        div {
                                            class: "w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded flex items-center justify-center",
                                            "📦"
                                        }
                                    }
                                }

                                Badge {
                                    badge_type: BadgeType::Error,
                                    value: Some("20".to_string()),
                                    processing: *processing.read(),
                                    children: rsx! {
                                        div {
                                            class: "w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded flex items-center justify-center",
                                            "📦"
                                        }
                                    }
                                }

                                Badge {
                                    badge_type: BadgeType::Info,
                                    dot: true,
                                    children: rsx! {
                                        div {
                                            class: "w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded flex items-center justify-center",
                                            "📦"
                                        }
                                    }
                                }
                            }

                            Button {
                                button_type: ButtonType::Primary,
                                size: ButtonSize::Small,
                                onclick: move |_| {
                                    let current = *processing.read();
                                    *processing.write() = !current;
                                },
                                if *processing.read() { "停止处理" } else { "开始处理" }
                            }
                        }
                    }
                }

                // 显示 0
                section {
                    id: "show-zero",
                    class: "mb-12",
                    DemoBox {
                        title: "显示 0".to_string(),
                        description: "设定 show-zero 来显示 0。".to_string(),
                        code: r#"use helixui::components::{Badge, BadgeType};

rsx! {
    Badge { 
        value: Some("0".to_string()), 
        show_zero: true,
        children: rsx! { div { "📦" } } 
    }
    Badge { 
        value: Some("0".to_string()), 
        show_zero: false,
        children: rsx! { div { "📦" } } 
    }
}"#.to_string(),
                        children: rsx! {
                            div {
                                class: "flex items-center gap-8",

                                Badge {
                                    value: Some("0".to_string()),
                                    show_zero: true,
                                    children: rsx! {
                                        div {
                                            class: "w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded flex items-center justify-center",
                                            "📦"
                                        }
                                    }
                                }

                                Badge {
                                    value: Some("0".to_string()),
                                    show_zero: false,
                                    children: rsx! {
                                        div {
                                            class: "w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded flex items-center justify-center",
                                            "📦"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // 溢出
                section {
                    id: "overflow",
                    class: "mb-12",
                    DemoBox {
                        title: "溢出".to_string(),
                        description: "设定 max 来处理溢出情况。".to_string(),
                        code: r#"use helixui::components::{Badge, BadgeType};

rsx! {
    Badge { value: Some("101".to_string()), max: Some(99), children: rsx! { div { "📦" } } }
    Badge { value: Some("99".to_string()), max: Some(99), children: rsx! { div { "📦" } } }
    Badge { value: Some("100".to_string()), max: Some(99), children: rsx! { div { "📦" } } }
    Badge { value: Some("10".to_string()), max: Some(9), children: rsx! { div { "📦" } } }
}"#.to_string(),
                        children: rsx! {
                            div {
                                class: "flex items-center gap-8",

                                Badge {
                                    value: Some("101".to_string()),
                                    max: Some(99),
                                    children: rsx! {
                                        div {
                                            class: "w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded flex items-center justify-center",
                                            "📦"
                                        }
                                    }
                                }

                                Badge {
                                    value: Some("99".to_string()),
                                    max: Some(99),
                                    children: rsx! {
                                        div {
                                            class: "w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded flex items-center justify-center",
                                            "📦"
                                        }
                                    }
                                }

                                Badge {
                                    value: Some("100".to_string()),
                                    max: Some(99),
                                    children: rsx! {
                                        div {
                                            class: "w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded flex items-center justify-center",
                                            "📦"
                                        }
                                    }
                                }

                                Badge {
                                    value: Some("10".to_string()),
                                    max: Some(9),
                                    children: rsx! {
                                        div {
                                            class: "w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded flex items-center justify-center",
                                            "📦"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // 受控显示
                section {
                    id: "controlled",
                    class: "mb-12",
                    DemoBox {
                        title: "受控显示".to_string(),
                        description: "通过 show 属性控制徽章的显示。".to_string(),
                        code: r#"use helixui::components::{Badge, BadgeType};

rsx! {
    Badge { 
        value: Some("5".to_string()), 
        show: true,
        children: rsx! { div { "📦" } } 
    }
    Badge { 
        dot: true, 
        show: false,
        children: rsx! { div { "📦" } } 
    }
}"#.to_string(),
                        children: rsx! {
                            div {
                                class: "flex items-center gap-8",

                                Badge {
                                    value: Some(controlled_count_str.read().clone()),
                                    show: *show_badge.read(),
                                    children: rsx! {
                                        div {
                                            class: "w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded flex items-center justify-center",
                                            "📦"
                                        }
                                    }
                                }

                                Badge {
                                    dot: true,
                                    show: *show_badge.read(),
                                    children: rsx! {
                                        div {
                                            class: "w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded flex items-center justify-center",
                                            "📦"
                                        }
                                    }
                                }
                            }

                            Button {
                                button_type: ButtonType::Primary,
                                size: ButtonSize::Small,
                                onclick: move |_| {
                                    let current = *show_badge.read();
                                    *show_badge.write() = !current;
                                },
                                if *show_badge.read() { "隐藏徽章" } else { "显示徽章" }
                            }
                        }
                    }
                }

                // 自定义内容
                section {
                    id: "custom-content",
                    class: "mb-12",
                    DemoBox {
                        title: "自定义内容".to_string(),
                        description: "在里面插入一些自定义内容。".to_string(),
                        code: r#"use helixui::components::{Badge, BadgeType};

rsx! {
    Badge { value: Some("新".to_string()), children: rsx! { div { "📦" } } }
    Badge { value: Some("火".to_string()), children: rsx! { div { "📦" } } }
    Badge { value: Some("🔥".to_string()), children: rsx! { div { "📦" } } }
}"#.to_string(),
                        children: rsx! {
                            div {
                                class: "flex items-center gap-8",

                                Badge {
                                    value: Some("新".to_string()),
                                    children: rsx! {
                                        div {
                                            class: "w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded flex items-center justify-center",
                                            "📦"
                                        }
                                    }
                                }

                                Badge {
                                    value: Some("火".to_string()),
                                    children: rsx! {
                                        div {
                                            class: "w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded flex items-center justify-center",
                                            "📦"
                                        }
                                    }
                                }

                                Badge {
                                    value: Some("🔥".to_string()),
                                    children: rsx! {
                                        div {
                                            class: "w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded flex items-center justify-center",
                                            "📦"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // 自定义颜色
                section {
                    id: "custom-color",
                    class: "mb-12",
                    DemoBox {
                        title: "自定义颜色".to_string(),
                        description: "通过 color 属性自定义徽章颜色。".to_string(),
                        code: format!(r#"use helixui::components::{{Badge, BadgeType}};

rsx! {{
    Badge {{ 
        value: Some("15".to_string()), 
        color: Some("{}".to_string()),
        children: rsx! {{ div {{ "📦" }} }} 
    }}
    Badge {{ 
        value: Some("8".to_string()), 
        color: Some("{}".to_string()),
        children: rsx! {{ div {{ "📦" }} }} 
    }}
    Badge {{ 
        value: Some("NEW".to_string()), 
        color: Some("{}".to_string()),
        children: rsx! {{ div {{ "📦" }} }} 
    }}
}}"#, "#6b7280", "#8b5cf6", "#f59e0b"),
                        children: rsx! {
                            div {
                                class: "flex items-center gap-8",

                                Badge {
                                    value: Some("15".to_string()),
                                    color: Some("#6b7280".to_string()),
                                    children: rsx! {
                                        div {
                                            class: "w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded flex items-center justify-center",
                                            "📦"
                                        }
                                    }
                                }

                                Badge {
                                    value: Some("8".to_string()),
                                    color: Some("#8b5cf6".to_string()),
                                    children: rsx! {
                                        div {
                                            class: "w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded flex items-center justify-center",
                                            "📦"
                                        }
                                    }
                                }

                                Badge {
                                    value: Some("NEW".to_string()),
                                    color: Some("#f59e0b".to_string()),
                                    children: rsx! {
                                        div {
                                            class: "w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded flex items-center justify-center",
                                            "📦"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // 自定义位置偏移
                section {
                    id: "custom-offset",
                    class: "mb-12",
                    DemoBox {
                        title: "自定义位置偏移".to_string(),
                        description: "通过 offset 属性自定义徽章位置。".to_string(),
                        code: r#"use helixui::components::{Badge, BadgeType};

rsx! {
    Badge { 
        value: Some("5".to_string()), 
        offset: Some(("-4px".to_string(), "-4px".to_string())),
        children: rsx! { div { "📦" } } 
    }
    Badge { 
        value: Some("12".to_string()), 
        offset: Some(("4px".to_string(), "-4px".to_string())),
        children: rsx! { div { "📦" } } 
    }
    Badge { 
        value: Some("3".to_string()), 
        offset: Some(("-4px".to_string(), "4px".to_string())),
        children: rsx! { div { "📦" } } 
    }
}"#.to_string(),
                        children: rsx! {
                            div {
                                class: "flex items-center gap-8",

                                Badge {
                                    value: Some("5".to_string()),
                                    offset: Some(("-4px".to_string(), "-4px".to_string())),
                                    children: rsx! {
                                        div {
                                            class: "w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded flex items-center justify-center",
                                            "📦"
                                        }
                                    }
                                }

                                Badge {
                                    value: Some("12".to_string()),
                                    offset: Some(("4px".to_string(), "-4px".to_string())),
                                    children: rsx! {
                                        div {
                                            class: "w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded flex items-center justify-center",
                                            "📦"
                                        }
                                    }
                                }

                                Badge {
                                    value: Some("3".to_string()),
                                    offset: Some(("-4px".to_string(), "4px".to_string())),
                                    children: rsx! {
                                        div {
                                            class: "w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded flex items-center justify-center",
                                            "📦"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // API 说明
                section {
                    id: "api",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "API"
                    }

                    div {
                        class: "overflow-x-auto",
                        table {
                            class: "min-w-full divide-y divide-gray-200 dark:divide-gray-700",
                            thead {
                                class: "bg-gray-50 dark:bg-gray-800",
                                tr {
                                    th {
                                        class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider",
                                        "属性"
                                    }
                                    th {
                                        class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider",
                                        "类型"
                                    }
                                    th {
                                        class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider",
                                        "默认值"
                                    }
                                    th {
                                        class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider",
                                        "说明"
                                    }
                                }
                            }
                            tbody {
                                class: "bg-white dark:bg-gray-900 divide-y divide-gray-200 dark:divide-gray-700",

                                tr {
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white",
                                        "value"
                                    }
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400",
                                        "Option&lt;String&gt;"
                                    }
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400",
                                        "undefined"
                                    }
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400",
                                        "标记数量"
                                    }
                                }

                                tr {
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white",
                                        "badge_type"
                                    }
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400",
                                        "BadgeType"
                                    }
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400",
                                        "Default"
                                    }
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400",
                                        "标记显示类型"
                                    }
                                }

                                tr {
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white",
                                        "dot"
                                    }
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400",
                                        "bool"
                                    }
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400",
                                        "false"
                                    }
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400",
                                        "标记是否显示为点"
                                    }
                                }

                                tr {
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white",
                                        "max"
                                    }
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400",
                                        "Option&lt;u32&gt;"
                                    }
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400",
                                        "undefined"
                                    }
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400",
                                        "标记最大数来处理溢出情况"
                                    }
                                }

                                tr {
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white",
                                        "show_zero"
                                    }
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400",
                                        "bool"
                                    }
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400",
                                        "false"
                                    }
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400",
                                        "标记为0时是否显示"
                                    }
                                }

                                tr {
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white",
                                        "show"
                                    }
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400",
                                        "bool"
                                    }
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400",
                                        "true"
                                    }
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400",
                                        "标记受控显示"
                                    }
                                }

                                tr {
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white",
                                        "processing"
                                    }
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400",
                                        "bool"
                                    }
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400",
                                        "false"
                                    }
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400",
                                        "标记显示进度"
                                    }
                                }

                                tr {
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white",
                                        "color"
                                    }
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400",
                                        "Option&lt;String&gt;"
                                    }
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400",
                                        "undefined"
                                    }
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400",
                                        "标记的颜色"
                                    }
                                }

                                tr {
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white",
                                        "offset"
                                    }
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400",
                                        "Option&lt;(String, String)&gt;"
                                    }
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400",
                                        "undefined"
                                    }
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400",
                                        "距默认位置左侧、上方的偏移量"
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
