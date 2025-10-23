use crate::views::layout::ComponentsSidebar;
use crate::views::layout::TocItem;
use crate::views::DocPage;
use dioxus::prelude::*;
use helixui::components::{use_toast, Button, ButtonType, DemoBox, ToastOptions, ToastProvider};
use std::time::Duration;

/// Toast 演示页面
#[component]
pub fn ToastPage() -> Element {
    // 定义目录项
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
            id: "with-description".to_string(),
            title: "带描述信息".to_string(),
            level: 1,
        },
        TocItem {
            id: "duration".to_string(),
            title: "自定义持续时间".to_string(),
            level: 1,
        },
        TocItem {
            id: "permanent".to_string(),
            title: "永久显示".to_string(),
            level: 1,
        },
        TocItem {
            id: "batch".to_string(),
            title: "批量操作".to_string(),
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
                        "Toast 提示框"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300",
                        "Toast 组件用于显示临时的通知消息，支持多种类型、自定义选项和丰富的交互功能。"
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
                        description: "最简单的 Toast 用法，显示一条消息。".to_string(),
                        code: r#"use helixui::components::{ToastProvider, use_toast, ToastOptions, Button, ButtonType};

#[component]
fn ToastDemo() -> Element {
    let toast_api = use_toast();
    
    rsx! {
        ToastProvider {
            Button {
                button_type: ButtonType::Primary,
                onclick: move |_| {
                    toast_api.info(
                        "欢迎使用 HelixUI！".to_string(),
                        ToastOptions::new()
                    );
                },
                "显示 Toast"
            }
        }
    }
}"#.to_string(),

                        children: rsx! {
                            ToastProvider {
                                ToastDemo {}
                            }
                        }
                    }
                }

                // 不同类型
                section {
                    id: "types",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "不同类型"
                    }

                    DemoBox {
                        title: "不同类型".to_string(),
                        description: "Toast 支持成功、错误、警告、信息四种类型，每种类型都有对应的视觉样式。".to_string(),
                        code: r#"use helixui::components::{ToastProvider, use_toast, ToastOptions, Button, ButtonType};

#[component]
fn ToastTypesDemo() -> Element {
    let toast_api = use_toast();
    
    rsx! {
        ToastProvider {
            div {
                class: "flex flex-wrap gap-3",
                Button {
                    button_type: ButtonType::Success,
                    onclick: move |_| {
                        toast_api.success("操作成功！".to_string(), ToastOptions::new());
                    },
                    "成功提示"
                }
                Button {
                    button_type: ButtonType::Error,
                    onclick: move |_| {
                        toast_api.error("操作失败，请重试".to_string(), ToastOptions::new());
                    },
                    "错误提示"
                }
                Button {
                    button_type: ButtonType::Warning,
                    onclick: move |_| {
                        toast_api.warning("请注意相关风险".to_string(), ToastOptions::new());
                    },
                    "警告提示"
                }
                Button {
                    button_type: ButtonType::Info,
                    onclick: move |_| {
                        toast_api.info("这是一条信息提示".to_string(), ToastOptions::new());
                    },
                    "信息提示"
                }
            }
        }
    }
}"#.to_string(),

                        children: rsx! {
                            ToastProvider {
                                ToastTypesDemo {}
                            }
                        }
                    }
                }

                // 带描述信息
                section {
                    id: "with-description",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "带描述信息"
                    }

                    DemoBox {
                        title: "带描述信息".to_string(),
                        description: "Toast 可以包含标题和详细描述信息，提供更丰富的上下文。".to_string(),
                        code: r#"use helixui::components::{ToastProvider, use_toast, ToastOptions, Button, ButtonType};

#[component]
fn ToastWithDescriptionDemo() -> Element {
    let toast_api = use_toast();
    
    rsx! {
        ToastProvider {
            div {
                class: "flex flex-wrap gap-3",
                Button {
                    button_type: ButtonType::Success,
                    onclick: move |_| {
                        toast_api.success(
                            "文件上传成功".to_string(),
                            ToastOptions::new()
                                .description("您的文件已成功上传到云端，可以开始编辑了。")
                        );
                    },
                    "上传成功"
                }
                Button {
                    button_type: ButtonType::Error,
                    onclick: move |_| {
                        toast_api.error(
                            "网络连接失败".to_string(),
                            ToastOptions::new()
                                .description("请检查您的网络连接，然后重试操作。")
                        );
                    },
                    "连接失败"
                }
                Button {
                    button_type: ButtonType::Warning,
                    onclick: move |_| {
                        toast_api.warning(
                            "存储空间不足".to_string(),
                            ToastOptions::new()
                                .description("您的存储空间即将用完，建议清理一些文件。")
                        );
                    },
                    "存储警告"
                }
            }
        }
    }
}"#.to_string(),

                        children: rsx! {
                            ToastProvider {
                                ToastWithDescriptionDemo {}
                            }
                        }
                    }
                }

                // 自定义持续时间
                section {
                    id: "duration",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "自定义持续时间"
                    }

                    DemoBox {
                        title: "自定义持续时间".to_string(),
                        description: "可以设置 Toast 的显示时间，适应不同的使用场景。".to_string(),
                        code: r#"use helixui::components::{ToastProvider, use_toast, ToastOptions, Button, ButtonType};
use std::time::Duration;

#[component]
fn ToastDurationDemo() -> Element {
    let toast_api = use_toast();
    
    rsx! {
        ToastProvider {
            div {
                class: "flex flex-wrap gap-3",
                Button {
                    button_type: ButtonType::Primary,
                    onclick: move |_| {
                        toast_api.info(
                            "快速提示（2秒）".to_string(),
                            ToastOptions::new().duration(Duration::from_secs(2))
                        );
                    },
                    "2秒"
                }
                Button {
                    button_type: ButtonType::Primary,
                    onclick: move |_| {
                        toast_api.info(
                            "标准提示（5秒）".to_string(),
                            ToastOptions::new().duration(Duration::from_secs(5))
                        );
                    },
                    "5秒"
                }
                Button {
                    button_type: ButtonType::Primary,
                    onclick: move |_| {
                        toast_api.info(
                            "长时间提示（10秒）".to_string(),
                            ToastOptions::new().duration(Duration::from_secs(10))
                        );
                    },
                    "10秒"
                }
            }
        }
    }
}"#.to_string(),

                        children: rsx! {
                            ToastProvider {
                                ToastDurationDemo {}
                            }
                        }
                    }
                }

                // 永久显示
                section {
                    id: "permanent",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "永久显示"
                    }

                    DemoBox {
                        title: "永久显示".to_string(),
                        description: "设置 permanent 为 true 可以让 Toast 永久显示，需要手动关闭。适用于重要通知。".to_string(),
                        code: r#"use helixui::components::{ToastProvider, use_toast, ToastOptions, Button, ButtonType};

#[component]
fn ToastPermanentDemo() -> Element {
    let toast_api = use_toast();
    
    rsx! {
        ToastProvider {
            div {
                class: "flex flex-wrap gap-3",
                Button {
                    button_type: ButtonType::Warning,
                    onclick: move |_| {
                        toast_api.warning(
                            "重要通知".to_string(),
                            ToastOptions::new()
                                .description("这是一条重要通知，需要您手动关闭。")
                                .permanent(true)
                        );
                    },
                    "重要通知"
                }
                Button {
                    button_type: ButtonType::Error,
                    onclick: move |_| {
                        toast_api.error(
                            "系统维护通知".to_string(),
                            ToastOptions::new()
                                .description("系统将于今晚 22:00-24:00 进行维护，请提前保存工作。")
                                .permanent(true)
                        );
                    },
                    "维护通知"
                }
            }
        }
    }
}"#.to_string(),

                        children: rsx! {
                            ToastProvider {
                                ToastPermanentDemo {}
                            }
                        }
                    }
                }

                // 批量操作
                section {
                    id: "batch",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "批量操作"
                    }

                    DemoBox {
                        title: "批量操作".to_string(),
                        description: "演示如何快速显示多个 Toast 消息，模拟批量操作场景。".to_string(),
                        code: r#"use helixui::components::{ToastProvider, use_toast, ToastOptions, Button, ButtonType};
use std::time::Duration;

#[component]
fn ToastBatchDemo() -> Element {
    let toast_api = use_toast();
    
    rsx! {
        ToastProvider {
            div {
                class: "flex flex-wrap gap-3",
                Button {
                    button_type: ButtonType::Primary,
                    onclick: move |_| {
                        // 模拟批量删除操作
                        toast_api.info("开始批量删除...".to_string(), ToastOptions::new());
                        
                        // 模拟异步操作
                        spawn(async move {
                            tokio::time::sleep(Duration::from_millis(500)).await;
                            toast_api.success("已删除 5 个文件".to_string(), ToastOptions::new());
                        });
                        
                        spawn(async move {
                            tokio::time::sleep(Duration::from_millis(1000)).await;
                            toast_api.success("已删除 3 个文件夹".to_string(), ToastOptions::new());
                        });
                        
                        spawn(async move {
                            tokio::time::sleep(Duration::from_millis(1500)).await;
                            toast_api.success("批量删除完成！".to_string(), ToastOptions::new());
                        });
                    },
                    "批量删除"
                }
                Button {
                    button_type: ButtonType::Success,
                    onclick: move |_| {
                        // 模拟批量上传操作
                        toast_api.info("开始批量上传...".to_string(), ToastOptions::new());
                        
                        spawn(async move {
                            tokio::time::sleep(Duration::from_millis(800)).await;
                            toast_api.success("文件 1 上传成功".to_string(), ToastOptions::new());
                        });
                        
                        spawn(async move {
                            tokio::time::sleep(Duration::from_millis(1200)).await;
                            toast_api.success("文件 2 上传成功".to_string(), ToastOptions::new());
                        });
                        
                        spawn(async move {
                            tokio::time::sleep(Duration::from_millis(1800)).await;
                            toast_api.success("所有文件上传完成！".to_string(), ToastOptions::new());
                        });
                    },
                    "批量上传"
                }
            }
        }
    }
}"#.to_string(),

                        children: rsx! {
                            ToastProvider {
                                ToastBatchDemo {}
                            }
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
                        description: "展示 Toast 的高级用法，包括自定义样式和复杂交互。".to_string(),
                        code: r#"use helixui::components::{ToastProvider, use_toast, ToastOptions, Button, ButtonType};
use std::time::Duration;

#[component]
fn ToastAdvancedDemo() -> Element {
    let toast_api = use_toast();
    
    rsx! {
        ToastProvider {
            div {
                class: "flex flex-wrap gap-3",
                Button {
                    button_type: ButtonType::Primary,
                    onclick: move |_| {
                        toast_api.success(
                            "🎉 恭喜！".to_string(),
                            ToastOptions::new()
                                .description("您已成功完成所有任务，获得 100 积分奖励！")
                                .duration(Duration::from_secs(8))
                        );
                    },
                    "成就解锁"
                }
                Button {
                    button_type: ButtonType::Info,
                    onclick: move |_| {
                        toast_api.info(
                            "📊 数据同步中...".to_string(),
                            ToastOptions::new()
                                .description("正在同步您的数据到云端，预计需要 30 秒。")
                                .duration(Duration::from_secs(6))
                        );
                    },
                    "数据同步"
                }
                Button {
                    button_type: ButtonType::Warning,
                    onclick: move |_| {
                        toast_api.warning(
                            "⚠️ 安全提醒".to_string(),
                            ToastOptions::new()
                                .description("检测到异常登录活动，建议立即修改密码。")
                                .permanent(true)
                        );
                    },
                    "安全提醒"
                }
            }
        }
    }
}"#.to_string(),

                        children: rsx! {
                            ToastProvider {
                                ToastAdvancedDemo {}
                            }
                        }
                    }
                }
            }
        }
    }
}

// 演示组件
#[component]
fn ToastDemo() -> Element {
    let toast_api = use_toast();

    rsx! {
        Button {
            button_type: ButtonType::Primary,
            onclick: move |_| {
                toast_api.info(
                    "欢迎使用 HelixUI！".to_string(),
                    ToastOptions::new()
                );
            },
            "显示 Toast"
        }
    }
}

#[component]
fn ToastTypesDemo() -> Element {
    let toast_api = use_toast();

    rsx! {
        div {
            class: "flex flex-wrap gap-3",
            Button {
                button_type: ButtonType::Success,
                onclick: move |_| {
                    toast_api.success("操作成功！".to_string(), ToastOptions::new());
                },
                "成功提示"
            }
            Button {
                button_type: ButtonType::Error,
                onclick: move |_| {
                    toast_api.error("操作失败，请重试".to_string(), ToastOptions::new());
                },
                "错误提示"
            }
            Button {
                button_type: ButtonType::Warning,
                onclick: move |_| {
                    toast_api.warning("请注意相关风险".to_string(), ToastOptions::new());
                },
                "警告提示"
            }
            Button {
                button_type: ButtonType::Info,
                onclick: move |_| {
                    toast_api.info("这是一条信息提示".to_string(), ToastOptions::new());
                },
                "信息提示"
            }
        }
    }
}

#[component]
fn ToastWithDescriptionDemo() -> Element {
    let toast_api = use_toast();

    rsx! {
        div {
            class: "flex flex-wrap gap-3",
            Button {
                button_type: ButtonType::Success,
                onclick: move |_| {
                    toast_api.success(
                        "文件上传成功".to_string(),
                        ToastOptions::new()
                            .description("您的文件已成功上传到云端，可以开始编辑了。")
                    );
                },
                "上传成功"
            }
            Button {
                button_type: ButtonType::Error,
                onclick: move |_| {
                    toast_api.error(
                        "网络连接失败".to_string(),
                        ToastOptions::new()
                            .description("请检查您的网络连接，然后重试操作。")
                    );
                },
                "连接失败"
            }
            Button {
                button_type: ButtonType::Warning,
                onclick: move |_| {
                    toast_api.warning(
                        "存储空间不足".to_string(),
                        ToastOptions::new()
                            .description("您的存储空间即将用完，建议清理一些文件。")
                    );
                },
                "存储警告"
            }
        }
    }
}

#[component]
fn ToastDurationDemo() -> Element {
    let toast_api = use_toast();

    rsx! {
        div {
            class: "flex flex-wrap gap-3",
            Button {
                button_type: ButtonType::Primary,
                onclick: move |_| {
                    toast_api.info(
                        "快速提示（2秒）".to_string(),
                        ToastOptions::new().duration(Duration::from_secs(2))
                    );
                },
                "2秒"
            }
            Button {
                button_type: ButtonType::Primary,
                onclick: move |_| {
                    toast_api.info(
                        "标准提示（5秒）".to_string(),
                        ToastOptions::new().duration(Duration::from_secs(5))
                    );
                },
                "5秒"
            }
            Button {
                button_type: ButtonType::Primary,
                onclick: move |_| {
                    toast_api.info(
                        "长时间提示（10秒）".to_string(),
                        ToastOptions::new().duration(Duration::from_secs(10))
                    );
                },
                "10秒"
            }
        }
    }
}

#[component]
fn ToastPermanentDemo() -> Element {
    let toast_api = use_toast();

    rsx! {
        div {
            class: "flex flex-wrap gap-3",
            Button {
                button_type: ButtonType::Warning,
                onclick: move |_| {
                    toast_api.warning(
                        "重要通知".to_string(),
                        ToastOptions::new()
                            .description("这是一条重要通知，需要您手动关闭。")
                            .permanent(true)
                    );
                },
                "重要通知"
            }
            Button {
                button_type: ButtonType::Error,
                onclick: move |_| {
                    toast_api.error(
                        "系统维护通知".to_string(),
                        ToastOptions::new()
                            .description("系统将于今晚 22:00-24:00 进行维护，请提前保存工作。")
                            .permanent(true)
                    );
                },
                "维护通知"
            }
        }
    }
}

#[component]
fn ToastBatchDemo() -> Element {
    let toast_api = use_toast();

    rsx! {
        div {
            class: "flex flex-wrap gap-3",
            Button {
                button_type: ButtonType::Primary,
                onclick: move |_| {
                    // 模拟批量删除操作
                    toast_api.info("开始批量删除...".to_string(), ToastOptions::new());
                    toast_api.success("已删除 5 个文件".to_string(), ToastOptions::new());
                    toast_api.success("已删除 3 个文件夹".to_string(), ToastOptions::new());
                    toast_api.success("批量删除完成！".to_string(), ToastOptions::new());
                },
                "批量删除"
            }
            Button {
                button_type: ButtonType::Success,
                onclick: move |_| {
                    // 模拟批量上传操作
                    toast_api.info("开始批量上传...".to_string(), ToastOptions::new());
                    toast_api.success("文件 1 上传成功".to_string(), ToastOptions::new());
                    toast_api.success("文件 2 上传成功".to_string(), ToastOptions::new());
                    toast_api.success("所有文件上传完成！".to_string(), ToastOptions::new());
                },
                "批量上传"
            }
        }
    }
}

#[component]
fn ToastAdvancedDemo() -> Element {
    let toast_api = use_toast();

    rsx! {
        div {
            class: "flex flex-wrap gap-3",
            Button {
                button_type: ButtonType::Primary,
                onclick: move |_| {
                    toast_api.success(
                        "🎉 恭喜！".to_string(),
                        ToastOptions::new()
                            .description("您已成功完成所有任务，获得 100 积分奖励！")
                            .duration(Duration::from_secs(8))
                    );
                },
                "成就解锁"
            }
            Button {
                button_type: ButtonType::Info,
                onclick: move |_| {
                    toast_api.info(
                        "📊 数据同步中...".to_string(),
                        ToastOptions::new()
                            .description("正在同步您的数据到云端，预计需要 30 秒。")
                            .duration(Duration::from_secs(6))
                    );
                },
                "数据同步"
            }
            Button {
                button_type: ButtonType::Warning,
                onclick: move |_| {
                    toast_api.warning(
                        "⚠️ 安全提醒".to_string(),
                        ToastOptions::new()
                            .description("检测到异常登录活动，建议立即修改密码。")
                            .permanent(true)
                    );
                },
                "安全提醒"
            }
        }
    }
}
