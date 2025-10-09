use crate::components::{Button, ButtonSize, ButtonType, DemoBox};
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
                    class: "text-4xl font-bold text-gray-900 dark:text-white mb-2",
                    "按钮 Button"
                }
                p {
                    class: "text-gray-600 dark:text-gray-300",
                    "按钮用来触发一些操作。"
                }
            }

            // 基础演示
            section {
                id: "basic",
                class: "mb-12",
                h2 {
                    class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                    "演示"
                }

                DemoBox {
                    title: "基础".to_string(),
                    description: "按钮的 type 分别为 default、tertiary、primary、info、success、warning 和 error。".to_string(),
                    code: r#"use helixui::components::{Button, ButtonType};

rsx! {
    Button { "Default" }
    Button { button_type: ButtonType::Tertiary, "Tertiary" }
    Button { button_type: ButtonType::Primary, "Primary" }
    Button { button_type: ButtonType::Info, "Info" }
    Button { button_type: ButtonType::Success, "Success" }
    Button { button_type: ButtonType::Warning, "Warning" }
    Button { button_type: ButtonType::Error, "Error" }
}"#.to_string(),

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
                h2 {
                    class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                    "次要按钮"
                }

                DemoBox {
                    title: "次要按钮".to_string(),
                    description: "次要按钮使用浅色背景。".to_string(),
                    code: r#"use helixui::components::{Button, ButtonType};

rsx! {
    Button { button_type: ButtonType::Default, "Default" }
    Button { button_type: ButtonType::Tertiary, "Tertiary" }
    Button { button_type: ButtonType::Primary, secondary: true, "Primary" }
    Button { button_type: ButtonType::Info, secondary: true, "Info" }
    Button { button_type: ButtonType::Success, secondary: true, "Success" }
    Button { button_type: ButtonType::Warning, secondary: true, "Warning" }
    Button { button_type: ButtonType::Error, secondary: true, "Error" }
}"#.to_string(),

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
                h2 {
                    class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                    "尺寸"
                }

                DemoBox {
                    title: "按钮尺寸".to_string(),
                    description: "按钮有超小、小、中、大四种尺寸。".to_string(),
                    code: r#"use helixui::components::{Button, ButtonSize, ButtonType};

rsx! {
    Button { 
        button_type: ButtonType::Primary, 
        size: ButtonSize::Tiny, 
        "Tiny" 
    }
    Button { 
        button_type: ButtonType::Primary, 
        size: ButtonSize::Small, 
        "Small" 
    }
    Button { 
        button_type: ButtonType::Primary, 
        size: ButtonSize::Medium, 
        "Medium" 
    }
    Button { 
        button_type: ButtonType::Primary, 
        size: ButtonSize::Large, 
        "Large" 
    }
}"#.to_string(),

                    div {
                        class: "flex flex-wrap items-center gap-3",
                        Button { button_type: ButtonType::Primary, size: ButtonSize::Tiny, "Tiny" }
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
                h2 {
                    class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                    "禁用状态"
                }

                DemoBox {
                    title: "禁用按钮".to_string(),
                    description: "按钮可以被禁用。".to_string(),
                    code: r#"use helixui::components::{Button, ButtonType};

rsx! {
    Button { button_type: ButtonType::Default, disabled: true, "Default" }
    Button { button_type: ButtonType::Primary, disabled: true, "Primary" }
    Button { button_type: ButtonType::Info, disabled: true, "Info" }
    Button { button_type: ButtonType::Success, disabled: true, "Success" }
}"#.to_string(),

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
                    class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                    "API"
                }

                h3 {
                    class: "text-xl font-semibold text-gray-900 dark:text-white mb-3",
                    "Button Props"
                }

                div {
                    class: "overflow-x-auto",
                    table {
                        class: "w-full text-left border-collapse",
                        thead {
                            tr {
                                class: "border-b border-gray-200 dark:border-gray-700",
                                th { class: "p-3 text-gray-900 dark:text-white font-semibold", "名称" }
                                th { class: "p-3 text-gray-900 dark:text-white font-semibold", "类型" }
                                th { class: "p-3 text-gray-900 dark:text-white font-semibold", "默认值" }
                                th { class: "p-3 text-gray-900 dark:text-white font-semibold", "说明" }
                            }
                        }
                        tbody {
                            tr {
                                class: "border-b border-gray-100 dark:border-gray-800",
                                td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "button_type" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "ButtonType" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "Default" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "按钮的类型" }
                            }
                            tr {
                                class: "border-b border-gray-100 dark:border-gray-800",
                                td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "size" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "ButtonSize" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "Medium" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "按钮的尺寸" }
                            }
                            tr {
                                class: "border-b border-gray-100 dark:border-gray-800",
                                td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "disabled" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "bool" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "false" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "是否禁用" }
                            }
                            tr {
                                td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "secondary" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "bool" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "false" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "是否为次要按钮" }
                            }
                        }
                    }
                }
            }
            }
        }
    }
}
