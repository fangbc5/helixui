use crate::views::layout::TocItem;
use crate::views::DocPage;
use crate::views::layout::ComponentsSidebar;
use dioxus::prelude::*;
use helixui::components::{
    show_confirm_dialog, show_error_dialog, show_info_dialog, show_success_dialog, Button, ButtonType, DemoBox,
};

/// Dialog 组件文档页面
#[component]
pub fn DialogPage() -> Element {
    let toc_items = vec![
        TocItem {
            id: "basic".to_string(),
            title: "基础用法".to_string(),
            level: 1,
        },
        TocItem {
            id: "types".to_string(),
            title: "不同类型".to_string(),
            level: 1,
        },
        TocItem {
            id: "mask".to_string(),
            title: "遮罩层控制".to_string(),
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

            div { class: "component-doc",
                div { class: "mb-8",
                    h1 { class: "text-4xl font-bold text-gray-900 dark:text-white mb-2", "对话框 Dialog" }
                    p { class: "text-lg text-gray-600 dark:text-gray-300", "对话框组件用于显示重要的信息或收集用户输入。" }
                }

                // 基础用法
                section { id: "basic", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4", "基础用法" }

                    DemoBox {
                        title: "基础对话框".to_string(),
                        description: "最简单的对话框，包含标题和内容。".to_string(),
                        code: r#"use helixui::components::{Button, ButtonType, show_info_dialog};

rsx! {
    Button {
        button_type: ButtonType::Primary,
        onclick: move |_| {
            show_info_dialog("信息", "这是一条信息提示。");
        },
        "显示对话框"
    }
}"#.to_string(),

                        div {
                            class: "space-x-4",
                            Button {
                                button_type: ButtonType::Primary,
                                onclick: move |_| {
                                    show_info_dialog("信息".to_string(), "这是一条信息提示。".to_string());
                                },
                                "显示对话框"
                            }
                        }
                    }
                }

                // 不同类型
                section { id: "types", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4", "不同类型" }

                    DemoBox {
                        title: "不同类型的对话框".to_string(),
                        description: "支持确认、信息、成功、警告、错误等不同类型的对话框。".to_string(),
                        code: r#"use helixui::components::{Button, ButtonType, show_confirm_dialog, show_info_dialog, show_success_dialog, show_warning_dialog, show_error_dialog};

rsx! {
    div { class: "space-x-4",
        Button {
            button_type: ButtonType::Warning,
            onclick: move |_| { show_confirm_dialog("警告", "你确定?"); },
            "确认对话框"
        }
        Button {
            button_type: ButtonType::Success,
            onclick: move |_| { show_success_dialog("成功", "操作成功完成！"); },
            "成功对话框"
        }
        Button {
            button_type: ButtonType::Error,
            onclick: move |_| { show_error_dialog("错误", "操作失败，请重试。"); },
            "错误对话框"
        }
    }
}"#.to_string(),

                        div {
                            class: "space-x-4",
                            Button {
                                button_type: ButtonType::Warning,
                                onclick: move |_| {
                                    show_confirm_dialog(
                                        "警告".to_string(),
                                        "你确定?".to_string(),
                                        || {},
                                        || {},
                                    );
                                },
                                "确认对话框"
                            }

                            Button {
                                button_type: ButtonType::Success,
                                onclick: move |_| {
                                    show_success_dialog("成功".to_string(), "操作成功完成！".to_string());
                                },
                                "成功对话框"
                            }

                            Button {
                                button_type: ButtonType::Error,
                                onclick: move |_| {
                                    show_error_dialog("错误".to_string(), "操作失败，请重试。".to_string());
                                },
                                "错误对话框"
                            }
                        }
                    }
                }

                // 遮罩层控制
                section { id: "mask", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4", "遮罩层控制" }

                    DemoBox {
                        title: "遮罩层测试".to_string(),
                        description: "点击遮罩层可以关闭对话框。".to_string(),
                        code: r#"use helixui::components::{Button, ButtonType, show_info_dialog};

rsx! {
    Button {
        button_type: ButtonType::Info,
        onclick: move |_| {
            show_info_dialog("遮罩层测试", "点击遮罩层可以关闭此对话框。");
        },
        "测试遮罩层"
    }
}"#.to_string(),

                        div {
                            class: "space-x-4",
                            Button {
                                button_type: ButtonType::Info,
                                onclick: move |_| {
                                    show_info_dialog(
                                        "遮罩层测试".to_string(),
                                        "点击遮罩层可以关闭此对话框。".to_string(),
                                    );
                                },
                                "测试遮罩层"
                            }
                        }
                    }
                }

                // API
                section { id: "api", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4", "API" }

                    h3 { class: "text-xl font-semibold text-gray-900 dark:text-white mb-3", "Dialog Props" }

                    div { class: "overflow-x-auto",
                        table { class: "w-full text-left border-collapse",
                            thead {
                                tr { class: "border-b border-gray-100 dark:border-gray-800",
                                    th { class: "px-4 py-3 text-gray-900 dark:text-white font-medium", "属性" }
                                    th { class: "px-4 py-3 text-gray-900 dark:text-white font-medium", "类型" }
                                    th { class: "px-4 py-3 text-gray-900 dark:text-white font-medium", "默认值" }
                                    th { class: "px-4 py-3 text-gray-900 dark:text-white font-medium", "说明" }
                                }
                            }
                            tbody {
                                tr { class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "px-4 py-3 text-gray-600 dark:text-gray-300", "visible" }
                                    td { class: "px-4 py-3 text-gray-600 dark:text-gray-300", "bool" }
                                    td { class: "px-4 py-3 text-gray-600 dark:text-gray-300", "false" }
                                    td { class: "px-4 py-3 text-gray-600 dark:text-gray-300", "是否显示对话框" }
                                }
                                tr { class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "px-4 py-3 text-gray-600 dark:text-gray-300", "title" }
                                    td { class: "px-4 py-3 text-gray-600 dark:text-gray-300", "Option<String>" }
                                    td { class: "px-4 py-3 text-gray-600 dark:text-gray-300", "None" }
                                    td { class: "px-4 py-3 text-gray-600 dark:text-gray-300", "对话框标题" }
                                }
                                tr { class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "px-4 py-3 text-gray-600 dark:text-gray-300", "dialog_type" }
                                    td { class: "px-4 py-3 text-gray-600 dark:text-gray-300", "DialogType" }
                                    td { class: "px-4 py-3 text-gray-600 dark:text-gray-300", "Default" }
                                    td { class: "px-4 py-3 text-gray-600 dark:text-gray-300", "对话框类型" }
                                }
                                tr { class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "px-4 py-3 text-gray-600 dark:text-gray-300", "closable" }
                                    td { class: "px-4 py-3 text-gray-600 dark:text-gray-300", "bool" }
                                    td { class: "px-4 py-3 text-gray-600 dark:text-gray-300", "true" }
                                    td { class: "px-4 py-3 text-gray-600 dark:text-gray-300", "是否显示关闭按钮" }
                                }
                                tr { class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "px-4 py-3 text-gray-600 dark:text-gray-300", "mask_closable" }
                                    td { class: "px-4 py-3 text-gray-600 dark:text-gray-300", "bool" }
                                    td { class: "px-4 py-3 text-gray-600 dark:text-gray-300", "true" }
                                    td { class: "px-4 py-3 text-gray-600 dark:text-gray-300", "遮罩层点击是否关闭" }
                                }
                                tr { class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "px-4 py-3 text-gray-600 dark:text-gray-300", "on_close" }
                                    td { class: "px-4 py-3 text-gray-600 dark:text-gray-300", "Option<EventHandler<()>>" }
                                    td { class: "px-4 py-3 text-gray-600 dark:text-gray-300", "None" }
                                    td { class: "px-4 py-3 text-gray-600 dark:text-gray-300", "关闭事件" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
