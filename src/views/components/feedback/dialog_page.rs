use crate::components::{
    show_confirm_dialog, show_error_dialog, show_info_dialog, show_success_dialog,
    show_warning_dialog, Button, ButtonType, DemoBox,
};
use crate::views::layout::TocItem;
use crate::views::{ComponentsSidebar, DocPage};
use dioxus::prelude::*;

/// Dialog 组件文档页面
#[component]
pub fn DialogPage() -> Element {
    // 定义目录项
    let toc_items = vec![
        TocItem {
            id: "basic".to_string(),
            title: "演示".to_string(),
            level: 1,
        },
        TocItem {
            id: "type".to_string(),
            title: "类型".to_string(),
            level: 1,
        },
        TocItem {
            id: "mask".to_string(),
            title: "遮罩".to_string(),
            level: 1,
        },
        TocItem {
            id: "interactive".to_string(),
            title: "交互".to_string(),
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
                        "对话框 Dialog"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300",
                        "模态对话框，在保留当前页面状态的情况下，告知用户并承载相关操作。"
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
                        title: "基础用法".to_string(),
                        description: "注入 dialog 来创建一个弹框。".to_string(),
                        code: r#"use helixui::components::{show_confirm_dialog, show_info_dialog, show_success_dialog, show_error_dialog, Button, ButtonType};

        rsx! {
            div {
                class: "space-x-4",
                Button {
                    button_type: ButtonType::Warning,
                    onclick: move |_| {
                        show_confirm_dialog(
                            Some("警告".to_string()),
                            "你确定?".to_string(),
                            Some(EventHandler::new(|_| {
                                println!("用户确认了操作");
                            })),
                            Some(EventHandler::new(|_| {
                                println!("用户取消了操作");
                            })),
                        );
                    },
                    "警告"
                }
                
                Button {
                    button_type: ButtonType::Success,
                    onclick: move |_| {
                        show_success_dialog(
                            Some("成功".to_string()),
                            "操作成功完成！".to_string(),
                            Some(EventHandler::new(|_| {
                                println!("用户确认了成功信息");
                            })),
                        );
                    },
                    "成功"
                }
                
                Button {
                    button_type: ButtonType::Error,
                    onclick: move |_| {
                        show_error_dialog(
                            Some("错误".to_string()),
                            "操作失败，请重试。".to_string(),
                            Some(EventHandler::new(|_| {
                                println!("用户确认了错误信息");
                            })),
                        );
                    },
                    "错误"
                }
            }
        }"#.to_string(),

                        div {
                            class: "space-x-4",
                            Button {
                                button_type: ButtonType::Warning,
                                onclick: move |_| {
                                    show_confirm_dialog(
                                        Some("警告".to_string()),
                                        "你确定?".to_string(),
                                        Some(EventHandler::new(|_| {
                                            println!("用户确认了操作");
                                        })),
                                        Some(EventHandler::new(|_| {
                                            println!("用户取消了操作");
                                        })),
                                    );
                                },
                                "警告"
                            }

                            Button {
                                button_type: ButtonType::Success,
                                onclick: move |_| {
                                    show_success_dialog(
                                        Some("成功".to_string()),
                                        "操作成功完成！".to_string(),
                                        Some(EventHandler::new(|_| {
                                            println!("用户确认了成功信息");
                                        })),
                                    );
                                },
                                "成功"
                            }

                            Button {
                                button_type: ButtonType::Error,
                                onclick: move |_| {
                                    show_error_dialog(
                                        Some("错误".to_string()),
                                        "操作失败，请重试。".to_string(),
                                        Some(EventHandler::new(|_| {
                                            println!("用户确认了错误信息");
                                        })),
                                    );
                                },
                                "错误"
                            }
                        }
                    }
                }

                // 类型演示
                section {
                    id: "type",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "类型"
                    }

                    DemoBox {
                        title: "不同类型".to_string(),
                        description: "对话框支持不同类型，每种类型都有对应的图标和颜色。".to_string(),
                        code: r#"use helixui::components::{Dialog, DialogType, DialogSize};

        rsx! {
            // 信息对话框
            Dialog {
                visible: true,
                title: Some("信息".to_string()),
                content: "这是一条信息提示。".to_string(),
                dialog_type: DialogType::Info,
                size: DialogSize::Medium,
            }
            
            // 成功对话框
            Dialog {
                visible: true,
                title: Some("成功".to_string()),
                content: "操作成功完成！".to_string(),
                dialog_type: DialogType::Success,
                size: DialogSize::Medium,
            }
            
            // 警告对话框
            Dialog {
                visible: true,
                title: Some("警告".to_string()),
                content: "请注意相关风险。".to_string(),
                dialog_type: DialogType::Warning,
                size: DialogSize::Medium,
            }
            
            // 错误对话框
            Dialog {
                visible: true,
                title: Some("错误".to_string()),
                content: "操作失败，请重试。".to_string(),
                dialog_type: DialogType::Error,
                size: DialogSize::Medium,
            }
        }"#.to_string(),

                        div {
                            class: "space-x-4",
                            Button {
                                button_type: ButtonType::Info,
                                onclick: move |_| {
                                    show_info_dialog(
                                        Some("信息".to_string()),
                                        "这是一条信息提示。".to_string(),
                                        Some(EventHandler::new(|_| {
                                            println!("用户查看了信息");
                                        })),
                                    );
                                },
                                "信息"
                            }

                            Button {
                                button_type: ButtonType::Success,
                                onclick: move |_| {
                                    show_success_dialog(
                                        Some("成功".to_string()),
                                        "操作成功完成！".to_string(),
                                        Some(EventHandler::new(|_| {
                                            println!("用户确认了成功信息");
                                        })),
                                    );
                                },
                                "成功"
                            }

                            Button {
                                button_type: ButtonType::Warning,
                                onclick: move |_| {
                                    show_warning_dialog(
                                        Some("警告".to_string()),
                                        "请注意相关风险。".to_string(),
                                        Some(EventHandler::new(|_| {
                                            println!("用户确认了警告");
                                        })),
                                        Some(EventHandler::new(|_| {
                                            println!("用户取消了操作");
                                        })),
                                    );
                                },
                                "警告"
                            }

                            Button {
                                button_type: ButtonType::Error,
                                onclick: move |_| {
                                    show_error_dialog(
                                        Some("错误".to_string()),
                                        "操作失败，请重试。".to_string(),
                                        Some(EventHandler::new(|_| {
                                            println!("用户确认了错误信息");
                                        })),
                                    );
                                },
                                "错误"
                            }
                        }
                    }
                }

                // 遮罩层行为演示
                section {
                    id: "mask",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "遮罩层行为"
                    }

                    DemoBox {
                        title: "点击遮罩层关闭".to_string(),
                        description: "设置 mask_closable 为 true 时，点击遮罩层可以关闭对话框。".to_string(),
                        code: r#"use helixui::components::{Dialog, DialogType, Button, ButtonType};

        rsx! {
            Dialog {
                visible: true,
                title: Some("遮罩层测试".to_string()),
                content: "点击遮罩层可以关闭此对话框。".to_string(),
                dialog_type: DialogType::Info,
                mask_closable: true,
                closable: true,
                show_confirm: true,
                show_cancel: false,
                confirm_text: "知道了".to_string(),
            }
        }"#.to_string(),

                        div {
                            class: "space-x-4",
                            Button {
                                button_type: ButtonType::Info,
                                onclick: move |_| {
                                    // 这里应该使用一个自定义的对话框，但为了演示，我们使用便捷函数
                                    show_info_dialog(
                                        Some("遮罩层测试".to_string()),
                                        "点击遮罩层可以关闭此对话框。".to_string(),
                                        Some(EventHandler::new(|_| {
                                            println!("用户确认了信息");
                                        })),
                                    );
                                },
                                "测试遮罩层"
                            }
                        }
                    }
                }

                // 交互演示
                section {
                    id: "interactive",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "交互演示"
                    }

                    DemoBox {
                        title: "全局对话框".to_string(),
                        description: "使用全局对话框管理器显示不同类型的对话框。".to_string(),
                        code: r#"use helixui::components::{show_confirm_dialog, show_info_dialog, show_success_dialog, show_warning_dialog, show_error_dialog, Button, ButtonType};

        rsx! {
            div {
                class: "space-x-4",
                Button {
                    button_type: ButtonType::Default,
                    onclick: move |_| {
                        show_confirm_dialog(
                            Some("确认操作".to_string()),
                            "您确定要执行此操作吗？".to_string(),
                            Some(EventHandler::new(|_| {
                                println!("用户确认了操作");
                            })),
                            Some(EventHandler::new(|_| {
                                println!("用户取消了操作");
                            })),
                        );
                    },
                    "确认对话框"
                }
                
                Button {
                    button_type: ButtonType::Info,
                    onclick: move |_| {
                        show_info_dialog(
                            Some("信息提示".to_string()),
                            "这是一条信息提示。".to_string(),
                            Some(EventHandler::new(|_| {
                                println!("用户查看了信息");
                            })),
                        );
                    },
                    "信息对话框"
                }
                
                Button {
                    button_type: ButtonType::Success,
                    onclick: move |_| {
                        show_success_dialog(
                            Some("操作成功".to_string()),
                            "您的操作已成功完成！".to_string(),
                            Some(EventHandler::new(|_| {
                                println!("用户确认了成功信息");
                            })),
                        );
                    },
                    "成功对话框"
                }
                
                Button {
                    button_type: ButtonType::Warning,
                    onclick: move |_| {
                        show_warning_dialog(
                            Some("警告信息".to_string()),
                            "请注意相关风险。".to_string(),
                            Some(EventHandler::new(|_| {
                                println!("用户确认了警告");
                            })),
                            Some(EventHandler::new(|_| {
                                println!("用户取消了操作");
                            })),
                        );
                    },
                    "警告对话框"
                }
                
                Button {
                    button_type: ButtonType::Error,
                    onclick: move |_| {
                        show_error_dialog(
                            Some("错误信息".to_string()),
                            "操作失败，请重试。".to_string(),
                            Some(EventHandler::new(|_| {
                                println!("用户确认了错误信息");
                            })),
                        );
                    },
                    "错误对话框"
                }
            }
        }"#.to_string(),

                        div {
                            class: "space-x-4",
                            Button {
                                button_type: ButtonType::Default,
                                onclick: move |_| {
                                    show_confirm_dialog(
                                        Some("确认操作".to_string()),
                                        "您确定要执行此操作吗？".to_string(),
                                        Some(EventHandler::new(|_| {
                                            println!("用户确认了操作");
                                        })),
                                        Some(EventHandler::new(|_| {
                                            println!("用户取消了操作");
                                        })),
                                    );
                                },
                                "确认对话框"
                            }

                            Button {
                                button_type: ButtonType::Info,
                                onclick: move |_| {
                                    show_info_dialog(
                                        Some("信息提示".to_string()),
                                        "这是一条信息提示。".to_string(),
                                        Some(EventHandler::new(|_| {
                                            println!("用户查看了信息");
                                        })),
                                    );
                                },
                                "信息对话框"
                            }

                            Button {
                                button_type: ButtonType::Success,
                                onclick: move |_| {
                                    show_success_dialog(
                                        Some("操作成功".to_string()),
                                        "您的操作已成功完成！".to_string(),
                                        Some(EventHandler::new(|_| {
                                            println!("用户确认了成功信息");
                                        })),
                                    );
                                },
                                "成功对话框"
                            }

                            Button {
                                button_type: ButtonType::Warning,
                                onclick: move |_| {
                                    show_warning_dialog(
                                        Some("警告信息".to_string()),
                                        "请注意相关风险。".to_string(),
                                        Some(EventHandler::new(|_| {
                                            println!("用户确认了警告");
                                        })),
                                        Some(EventHandler::new(|_| {
                                            println!("用户取消了操作");
                                        })),
                                    );
                                },
                                "警告对话框"
                            }

                            Button {
                                button_type: ButtonType::Error,
                                onclick: move |_| {
                                    show_error_dialog(
                                        Some("错误信息".to_string()),
                                        "操作失败，请重试。".to_string(),
                                        Some(EventHandler::new(|_| {
                                            println!("用户确认了错误信息");
                                        })),
                                    );
                                },
                                "错误对话框"
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

                    h3 {
                        class: "text-xl font-semibold text-gray-900 dark:text-white mb-3",
                        "Dialog Props"
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
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "visible" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "bool" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "false" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "是否显示对话框" }
                                }
                                tr {
                                    class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "title" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "Option<String>" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "None" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "对话框标题" }
                                }
                                tr {
                                    class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "content" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "String" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "-" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "对话框内容" }
                                }
                                tr {
                                    class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "dialog_type" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "DialogType" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "Default" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "对话框类型" }
                                }
                                tr {
                                    class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "mask_closable" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "bool" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "false" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "是否可以通过点击遮罩层关闭" }
                                }
                                tr {
                                    class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "closable" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "bool" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "true" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "是否显示关闭按钮" }
                                }
                                tr {
                                    class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "show_confirm" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "bool" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "true" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "是否显示确认按钮" }
                                }
                                tr {
                                    class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "show_cancel" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "bool" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "true" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "是否显示取消按钮" }
                                }
                                tr {
                                    class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "confirm_text" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "String" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "\"确认\"" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "确认按钮文本" }
                                }
                                tr {
                                    class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "cancel_text" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "String" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "\"取消\"" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "取消按钮文本" }
                                }
                                tr {
                                    class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "confirm_type" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "ButtonType" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "Primary" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "确认按钮类型" }
                                }
                                tr {
                                    class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "cancel_type" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "ButtonType" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "Default" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "取消按钮类型" }
                                }
                                tr {
                                    class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "on_confirm" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "Option<EventHandler<()>>" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "None" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "确认回调" }
                                }
                                tr {
                                    class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "on_cancel" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "Option<EventHandler<()>>" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "None" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "取消回调" }
                                }
                                tr {
                                    class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "on_close" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "Option<EventHandler<()>>" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "None" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "关闭回调" }
                                }
                                tr {
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "class" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "Option<String>" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "None" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "自定义 CSS 类名" }
                                }
                            }
                        }
                    }

                    h3 {
                        class: "text-xl font-semibold text-gray-900 dark:text-white mb-3 mt-8",
                        "DialogType"
                    }

                    div {
                        class: "overflow-x-auto",
                        table {
                            class: "w-full text-left border-collapse",
                            thead {
                                tr {
                                    class: "border-b border-gray-200 dark:border-gray-700",
                                    th { class: "p-3 text-gray-900 dark:text-white font-semibold", "名称" }
                                    th { class: "p-3 text-gray-900 dark:text-white font-semibold", "说明" }
                                }
                            }
                            tbody {
                                tr {
                                    class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "Default" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "默认类型" }
                                }
                                tr {
                                    class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "Info" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "信息类型" }
                                }
                                tr {
                                    class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "Success" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "成功类型" }
                                }
                                tr {
                                    class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "Warning" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "警告类型" }
                                }
                                tr {
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "Error" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "错误类型" }
                                }
                            }
                        }
                    }

                }
            }
        }
    }
}
