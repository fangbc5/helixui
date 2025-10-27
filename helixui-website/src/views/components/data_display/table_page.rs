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

                    div {
                        class: "overflow-x-auto mb-6",
                        table {
                            class: "min-w-full divide-y divide-gray-200 dark:divide-gray-700",
                            thead {
                                class: "bg-gray-50 dark:bg-gray-800",
                                tr {
                                    th {
                                        class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider",
                                        "参数"
                                    }
                                    th {
                                        class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider",
                                        "说明"
                                    }
                                    th {
                                        class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider",
                                        "类型"
                                    }
                                    th {
                                        class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider",
                                        "默认值"
                                    }
                                }
                            }
                            tbody {
                                class: "bg-white dark:bg-gray-900 divide-y divide-gray-200 dark:divide-gray-700",
                                tr {
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100", "data" }
                                    td { class: "px-6 py-4 text-sm text-gray-600 dark:text-gray-400", "表格数据（二维数组）" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900 dark:text-gray-100", "Vec<Vec<String>>" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "-" }
                                }
                                tr {
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100", "headers" }
                                    td { class: "px-6 py-4 text-sm text-gray-600 dark:text-gray-400", "表头数据" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900 dark:text-gray-100", "Option<Vec<String>>" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "None" }
                                }
                                tr {
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100", "bordered" }
                                    td { class: "px-6 py-4 text-sm text-gray-600 dark:text-gray-400", "是否显示边框" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900 dark:text-gray-100", "bool" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "true" }
                                }
                                tr {
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100", "striped" }
                                    td { class: "px-6 py-4 text-sm text-gray-600 dark:text-gray-400", "是否显示斑马纹" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900 dark:text-gray-100", "bool" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "false" }
                                }
                                tr {
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100", "compact" }
                                    td { class: "px-6 py-4 text-sm text-gray-600 dark:text-gray-400", "是否紧凑模式" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900 dark:text-gray-100", "bool" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "false" }
                                }
                                tr {
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100", "class" }
                                    td { class: "px-6 py-4 text-sm text-gray-600 dark:text-gray-400", "自定义类名" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900 dark:text-gray-100", "Option<String>" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "None" }
                                }
                            }
                        }
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
