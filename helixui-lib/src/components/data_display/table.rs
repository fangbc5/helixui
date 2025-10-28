use dioxus::prelude::*;

/// 表格组件的属性
#[derive(Props, Clone, PartialEq)]
pub struct TableProps {
    /// 表格数据
    pub data: Vec<Vec<String>>,

    /// 表格标题
    #[props(default)]
    pub headers: Option<Vec<String>>,

    /// 是否显示边框
    #[props(default = true)]
    pub bordered: bool,

    /// 是否显示斑马纹
    #[props(default = false)]
    pub striped: bool,

    /// 是否紧凑模式
    #[props(default = false)]
    pub compact: bool,

    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,
}

/// 表格组件
#[component]
pub fn Table(props: TableProps) -> Element {
    let base_class = "min-w-full divide-y divide-gray-200 dark:divide-gray-700";
    let bordered_class = if props.bordered {
        "border border-gray-300 dark:border-gray-600"
    } else {
        ""
    };
    let striped_class = if props.striped {
        "even:bg-gray-50 dark:even:bg-gray-800/50"
    } else {
        ""
    };
    let compact_class = if props.compact { "text-sm" } else { "" };

    let final_class = if let Some(custom_class) = props.class {
        format!(
            "{} {} {} {} {}",
            base_class, bordered_class, striped_class, compact_class, custom_class
        )
    } else {
        format!(
            "{} {} {} {}",
            base_class, bordered_class, striped_class, compact_class
        )
    };

    rsx! {
        div {
            class: "overflow-x-auto",
            table {
                class: final_class,
                if let Some(headers) = &props.headers {
                    thead {
                        class: "bg-gray-50 dark:bg-gray-800",
                        tr {
                            for header in headers {
                                th {
                                    class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider",
                                    {header.clone()}
                                }
                            }
                        }
                    }
                }
                tbody {
                    class: "bg-white dark:bg-gray-900 divide-y divide-gray-200 dark:divide-gray-700",
                    for (row_index, row) in props.data.iter().enumerate() {
                        tr {
                            class: if props.striped && row_index % 2 == 1 { "bg-gray-50 dark:bg-gray-800/50" } else { "" },
                            for cell in row {
                                td {
                                    class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100",
                                    {cell.clone()}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
