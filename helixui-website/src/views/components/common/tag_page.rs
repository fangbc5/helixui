use crate::views::layout::{ComponentsSidebar, TocItem};
use crate::views::DocPage;
use dioxus::prelude::*;
use helixui::components::{DemoBox, Icon, IconType, Table, Tag, TagSize, TagType, TagVariant};

#[component]
pub fn TagPage() -> Element {
    let toc_items = vec![
        TocItem {
            id: "basic".to_string(),
            title: "基础用法".to_string(),
            level: 2,
        },
        TocItem {
            id: "types".to_string(),
            title: "类型".to_string(),
            level: 2,
        },
        TocItem {
            id: "sizes".to_string(),
            title: "尺寸".to_string(),
            level: 2,
        },
        TocItem {
            id: "variants".to_string(),
            title: "样式变体".to_string(),
            level: 2,
        },
        TocItem {
            id: "closable".to_string(),
            title: "可关闭".to_string(),
            level: 2,
        },
        TocItem {
            id: "api".to_string(),
            title: "API".to_string(),
            level: 2,
        },
    ];

    rsx! {
        DocPage {
            sidebar: rsx! { ComponentsSidebar {} },
            toc_items,

            // 基础用法
            DemoBox {
                title: "基础用法",
                description: "最基础的标签展示。",
                code: r#"
                    Tag {{ "默认标签" }}
                    Tag {{ tag_type: TagType::Primary, "主要" }}
                    Tag {{ tag_type: TagType::Success, "成功" }}
                "#,
                children: rsx! {
                    div { class: "space-x-2",
                        Tag { "默认标签" }
                        Tag { tag_type: TagType::Primary, "主要" }
                        Tag { tag_type: TagType::Success, "成功" }
                    }
                }
            }

            // 类型
            DemoBox {
                title: "类型",
                description: "不同语义颜色。",
                code: r#"
                    Tag {{ tag_type: TagType::Default, "默认" }}
                    Tag {{ tag_type: TagType::Primary, "主要" }}
                    Tag {{ tag_type: TagType::Success, "成功" }}
                    Tag {{ tag_type: TagType::Warning, "警告" }}
                    Tag {{ tag_type: TagType::Error, "错误" }}
                    Tag {{ tag_type: TagType::Info, "信息" }}
                "#,
                children: rsx! {
                    div { class: "space-x-2",
                        Tag { tag_type: TagType::Default, "默认" }
                        Tag { tag_type: TagType::Primary, "主要" }
                        Tag { tag_type: TagType::Success, "成功" }
                        Tag { tag_type: TagType::Warning, "警告" }
                        Tag { tag_type: TagType::Error, "错误" }
                        Tag { tag_type: TagType::Info, "信息" }
                    }
                }
            }

            // 尺寸
            DemoBox {
                title: "尺寸",
                description: "小/中/大三种尺寸。",
                code: r#"
                    Tag {{ size: TagSize::Small, "小" }}
                    Tag {{ size: TagSize::Medium, "中" }}
                    Tag {{ size: TagSize::Large, "大" }}
                "#,
                children: rsx! {
                    div { class: "space-x-2",
                        Tag { size: TagSize::Small, "小" }
                        Tag { size: TagSize::Medium, "中" }
                        Tag { size: TagSize::Large, "大" }
                    }
                }
            }

            // 变体
            DemoBox {
                title: "样式变体",
                description: "实心、柔和、描边三种。",
                code: r#"
                    Tag {{ variant: TagVariant::Solid, tag_type: TagType::Primary, "Solid" }}
                    Tag {{ variant: TagVariant::Soft, tag_type: TagType::Primary, "Soft" }}
                    Tag {{ variant: TagVariant::Outline, tag_type: TagType::Primary, "Outline" }}
                "#,
                children: rsx! {
                    div { class: "space-x-2",
                        Tag { variant: TagVariant::Solid, tag_type: TagType::Primary, "Solid" }
                        Tag { variant: TagVariant::Soft, tag_type: TagType::Primary, "Soft" }
                        Tag { variant: TagVariant::Outline, tag_type: TagType::Primary, "Outline" }
                    }
                }
            }

            // 可关闭
            DemoBox {
                title: "可关闭",
                description: "带关闭按钮的标签。",
                code: r#"
                    Tag {{ closable: true, on_close: |_| {{}}, "可关闭" }}
                    Tag {{ closable: true, round: true, tag_type: TagType::Info, variant: TagVariant::Soft, "圆角关闭" }}
                "#,
                children: rsx! {
                    div { class: "space-x-2",
                        Tag { closable: true, "可关闭" }
                        Tag { closable: true, round: true, tag_type: TagType::Info, variant: TagVariant::Soft, "圆角关闭" }
                        Tag { closable: true, tag_type: TagType::Warning, variant: TagVariant::Outline,
                            Icon { icon: IconType::Info, class: "mr-1".to_string() }
                            "带图标"
                        }
                    }
                }
            }

            // API
            section {
                id: "api",
                class: "mb-12",
                h2 {
                    class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                    "API"
                }
                div { class: "mt-2",
                    Table {
                        headers: Some(vec![
                            "属性".to_string(),
                            "类型".to_string(),
                            "默认值".to_string(),
                            "说明".to_string(),
                        ]),
                        data: vec![
                            vec!["tag_type".to_string(), "TagType".to_string(), "Default".to_string(), "标签语义类型：Default/Primary/Success/Warning/Error/Info".to_string()],
                            vec!["variant".to_string(), "TagVariant".to_string(), "Soft".to_string(), "样式变体：Solid/Soft/Outline".to_string()],
                            vec!["size".to_string(), "TagSize".to_string(), "Medium".to_string(), "尺寸：Small/Medium/Large".to_string()],
                            vec!["round".to_string(), "bool".to_string(), "false".to_string(), "是否为全圆角标签".to_string()],
                            vec!["closable".to_string(), "bool".to_string(), "false".to_string(), "是否显示关闭按钮".to_string()],
                            vec!["on_close".to_string(), "Option<EventHandler<()>>".to_string(), "None".to_string(), "点击关闭按钮时触发".to_string()],
                            vec!["disabled".to_string(), "bool".to_string(), "false".to_string(), "禁用状态，关闭按钮不可点击".to_string()],
                            vec!["class".to_string(), "Option<String>".to_string(), "None".to_string(), "自定义类名".to_string()],
                            vec!["children".to_string(), "Element".to_string(), "—".to_string(), "标签内容".to_string()],
                        ],
                    }
                }
            }
        }
    }
}
