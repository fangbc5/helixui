use crate::components::{Button, ButtonSize, ButtonType};
use crate::views::{ComponentsSidebar, DocPage};
use dioxus::prelude::*;

/// Button 组件文档页面
#[component]
pub fn ButtonPage() -> Element {
    rsx! {
        DocPage {
            sidebar: rsx! { ComponentsSidebar {} },

            div {
                class: "component-doc",

            // 标题
            div {
                class: "mb-8",
                h1 {
                    class: "text-4xl font-bold text-gray-900 mb-2",
                    "按钮 Button"
                }
                p {
                    class: "text-gray-600",
                    "按钮用来触发一些操作。"
                }
            }

            // 基础演示
            section {
                id: "basic",
                class: "mb-12",
                h2 {
                    class: "text-2xl font-semibold text-gray-900 mb-4",
                    "演示"
                }

                DemoSection {
                    title: "基础",
                    description: "按钮的 type 分别为 default、tertiary、primary、info、success、warning 和 error。",

                    div {
                        class: "flex flex-wrap gap-3",
                        Button { "Default" }
                        Button { button_type: ButtonType::Tertiary, "Tertiary" }
                        Button { button_type: ButtonType::Primary, "Primary" }
                        Button { button_type: ButtonType::Info, "Info" }
                        Button { button_type: ButtonType::Success, "Success" }
                        Button { button_type: ButtonType::Warning, "Warning" }
                        Button { button_type: ButtonType::Error, "Error" }
                    }
                }
            }

            // 次要按钮
            section {
                id: "secondary",
                class: "mb-12",
                DemoSection {
                    title: "次要按钮",
                    description: "次要按钮使用浅色背景。",

                    div {
                        class: "flex flex-wrap gap-3",
                        Button { button_type: ButtonType::Default, "Default" }
                        Button { button_type: ButtonType::Tertiary, "Tertiary" }
                        Button { button_type: ButtonType::Primary, secondary: true, "Primary" }
                        Button { button_type: ButtonType::Info, secondary: true, "Info" }
                        Button { button_type: ButtonType::Success, secondary: true, "Success" }
                        Button { button_type: ButtonType::Warning, secondary: true, "Warning" }
                        Button { button_type: ButtonType::Error, secondary: true, "Error" }
                    }
                }
            }

            // 尺寸
            section {
                id: "size",
                class: "mb-12",
                DemoSection {
                    title: "尺寸",
                    description: "按钮有小、中、大三种尺寸。",

                    div {
                        class: "flex flex-wrap items-center gap-3",
                        Button { button_type: ButtonType::Primary, size: ButtonSize::Small, "Small" }
                        Button { button_type: ButtonType::Primary, size: ButtonSize::Medium, "Medium" }
                        Button { button_type: ButtonType::Primary, size: ButtonSize::Large, "Large" }
                    }
                }
            }

            // 禁用
            section {
                id: "disabled",
                class: "mb-12",
                DemoSection {
                    title: "禁用",
                    description: "按钮可以被禁用。",

                    div {
                        class: "flex flex-wrap gap-3",
                        Button { button_type: ButtonType::Default, disabled: true, "Default" }
                        Button { button_type: ButtonType::Primary, disabled: true, "Primary" }
                        Button { button_type: ButtonType::Info, disabled: true, "Info" }
                        Button { button_type: ButtonType::Success, disabled: true, "Success" }
                    }
                }
            }

            // API
            section {
                id: "api",
                class: "mb-12",
                h2 {
                    class: "text-2xl font-semibold text-gray-900 mb-4",
                    "API"
                }

                h3 {
                    class: "text-xl font-semibold text-gray-900 mb-3",
                    "Button Props"
                }

                div {
                    class: "overflow-x-auto bg-white rounded-lg border border-gray-200",
                    table {
                        class: "min-w-full divide-y divide-gray-200",
                        thead {
                            class: "bg-gray-50",
                            tr {
                                th {
                                    class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                    "名称"
                                }
                                th {
                                    class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                    "类型"
                                }
                                th {
                                    class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                    "默认值"
                                }
                                th {
                                    class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                    "说明"
                                }
                            }
                        }
                        tbody {
                            class: "bg-white divide-y divide-gray-200",
                            tr {
                                td { class: "px-6 py-4 text-sm font-mono text-gray-900", "button_type" }
                                td { class: "px-6 py-4 text-sm text-gray-500", "ButtonType" }
                                td { class: "px-6 py-4 text-sm text-gray-500", "Default" }
                                td { class: "px-6 py-4 text-sm text-gray-500", "按钮的类型" }
                            }
                            tr {
                                td { class: "px-6 py-4 text-sm font-mono text-gray-900", "size" }
                                td { class: "px-6 py-4 text-sm text-gray-500", "ButtonSize" }
                                td { class: "px-6 py-4 text-sm text-gray-500", "Medium" }
                                td { class: "px-6 py-4 text-sm text-gray-500", "按钮的尺寸" }
                            }
                            tr {
                                td { class: "px-6 py-4 text-sm font-mono text-gray-900", "disabled" }
                                td { class: "px-6 py-4 text-sm text-gray-500", "bool" }
                                td { class: "px-6 py-4 text-sm text-gray-500", "false" }
                                td { class: "px-6 py-4 text-sm text-gray-500", "是否禁用" }
                            }
                            tr {
                                td { class: "px-6 py-4 text-sm font-mono text-gray-900", "secondary" }
                                td { class: "px-6 py-4 text-sm text-gray-500", "bool" }
                                td { class: "px-6 py-4 text-sm text-gray-500", "false" }
                                td { class: "px-6 py-4 text-sm text-gray-500", "是否为次要按钮" }
                            }
                        }
                    }
                }
            }
            }
        }
    }
}

/// 演示区域组件
#[component]
fn DemoSection(title: String, description: String, children: Element) -> Element {
    rsx! {
        div {
            class: "demo-section",
            h3 {
                class: "text-xl font-semibold text-gray-900 mb-2",
                "{title}"
            }
            p {
                class: "text-gray-600 mb-4",
                "{description}"
            }
            div {
                class: "bg-white rounded-lg border border-gray-200 p-6",
                {children}
            }
        }
    }
}
