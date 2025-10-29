use crate::views::layout::{ComponentsSidebar, TocItem};
use crate::views::DocPage;
use dioxus::prelude::*;
use helixui::components::{DemoBox, Table};

/// Table 演示页面
#[component]
pub fn TablePage() -> Element {
    // 定义目录项
    let toc_items = vec![
        TocItem {
            id: "basic-usage".to_string(),
            title: "基础用法".to_string(),
            level: 2,
        },
        TocItem {
            id: "variants".to_string(),
            title: "不同样式".to_string(),
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
            div {
                // 基础用法
                DemoBox {
                    title: "基础用法",
                    description: "最简单的用法，包含表头和数据。",
                    code: r#"
                        Table {{
                            headers: Some(vec!["姓名".to_string(), "年龄".to_string(), "城市".to_string()]),
                            data: vec![
                                vec!["张三".to_string(), "25".to_string(), "北京".to_string()],
                                vec!["李四".to_string(), "30".to_string(), "上海".to_string()],
                                vec!["王五".to_string(), "28".to_string(), "广州".to_string()],
                            ],
                        }}
                    "#,
                    children: rsx! {
                        BasicTableDemo {}
                    }
                }

                // 不同样式
                DemoBox {
                    title: "不同样式",
                    description: "支持边框、斑马纹、紧凑模式等样式。",
                    code: r#"
                        Table {{
                            headers: Some(vec!["产品".to_string(), "价格".to_string(), "库存".to_string()]),
                            data: table_data,
                            bordered: true,
                            striped: true,
                            compact: false,
                        }}
                    "#,
                    children: rsx! {
                        TableVariantsDemo {}
                    }
                }

                // API 文档
                div {
                    class: "mt-8",
                    h2 {
                        id: "api",
                        class: "text-2xl font-bold text-gray-900 dark:text-white mb-4",
                        "API"
                    }

                    Table {
                        headers: Some(vec![
                            "参数".to_string(),
                            "说明".to_string(),
                            "类型".to_string(),
                            "默认值".to_string(),
                        ]),
                        data: vec![
                            vec!["data".to_string(), "表格数据（二维数组）".to_string(), "Vec<Vec<String>>".to_string(), "-".to_string()],
                            vec!["headers".to_string(), "表头数据".to_string(), "Option<Vec<String>>".to_string(), "None".to_string()],
                            vec!["bordered".to_string(), "是否显示边框".to_string(), "bool".to_string(), "true".to_string()],
                            vec!["striped".to_string(), "是否显示斑马纹".to_string(), "bool".to_string(), "false".to_string()],
                            vec!["compact".to_string(), "是否紧凑模式".to_string(), "bool".to_string(), "false".to_string()],
                            vec!["class".to_string(), "自定义类名".to_string(), "Option<String>".to_string(), "None".to_string()],
                        ],
                        bordered: true,
                        striped: true,
                    }
                }
            }
        }
    }
}

/// 基础用法演示
#[component]
fn BasicTableDemo() -> Element {
    rsx! {
        Table {
            headers: Some(vec!["姓名".to_string(), "年龄".to_string(), "城市".to_string()]),
            data: vec![
                vec!["张三".to_string(), "25".to_string(), "北京".to_string()],
                vec!["李四".to_string(), "30".to_string(), "上海".to_string()],
                vec!["王五".to_string(), "28".to_string(), "广州".to_string()],
            ],
        }
    }
}

/// 不同样式演示
#[component]
fn TableVariantsDemo() -> Element {
    rsx! {
        div {
            class: "space-y-6",

            // 带边框和斑马纹
            div {
                h3 {
                    class: "text-lg font-semibold text-gray-900 dark:text-white mb-3",
                    "带边框和斑马纹"
                }
                Table {
                    headers: Some(vec!["产品".to_string(), "价格".to_string(), "库存".to_string()]),
                    data: vec![
                        vec!["苹果".to_string(), "¥8".to_string(), "100".to_string()],
                        vec!["香蕉".to_string(), "¥6".to_string(), "80".to_string()],
                        vec!["橙子".to_string(), "¥10".to_string(), "120".to_string()],
                    ],
                    bordered: true,
                    striped: true,
                }
            }

            // 紧凑模式
            div {
                h3 {
                    class: "text-lg font-semibold text-gray-900 dark:text-white mb-3",
                    "紧凑模式"
                }
                Table {
                    headers: Some(vec!["序号".to_string(), "名称".to_string(), "状态".to_string()]),
                    data: vec![
                        vec!["1".to_string(), "任务A".to_string(), "进行中".to_string()],
                        vec!["2".to_string(), "任务B".to_string(), "已完成".to_string()],
                        vec!["3".to_string(), "任务C".to_string(), "待开始".to_string()],
                    ],
                    bordered: true,
                    compact: true,
                }
            }
        }
    }
}
