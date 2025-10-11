use crate::components::{
    show_message, show_message_with_duration, show_message_with_position, Button, ButtonType,
    DemoBox, Message, MessagePosition, MessageType,
};
use crate::views::{ComponentsSidebar, DocPage};
use crate::views::layout::TocItem;
use dioxus::prelude::*;

/// 消息数据结构
#[derive(Clone, PartialEq)]
struct MessageData {
    id: u32,
    message_type: MessageType,
    content: String,
}

/// 消息列表组件
#[component]
fn MessageList(messages: Vec<MessageData>, on_remove: EventHandler<u32>) -> Element {
    rsx! {
        div {
            class: "space-y-2",
            for message in messages {
                Message {
                    key: "{message.id}",
                    message_type: message.message_type,
                    content: message.content,
                    closable: true,
                    on_close: move |_| on_remove.call(message.id),
                }
            }
        }
    }
}

/// Message 组件文档页面
#[component]
pub fn MessagePage() -> Element {
    // 消息状态管理
    let mut messages = use_signal(|| Vec::<MessageData>::new());
    let mut message_id_counter = use_signal(|| 0u32);

    // 移除消息的处理函数
    let remove_message = move |id: u32| {
        messages.write().retain(|msg| msg.id != id);
    };

    // 定义目录项
    let toc_items = vec![
        TocItem { id: "basic".to_string(), title: "演示".to_string(), level: 1 },
        TocItem { id: "interactive".to_string(), title: "交互演示".to_string(), level: 1 },
        TocItem { id: "loading".to_string(), title: "加载状态".to_string(), level: 1 },
        TocItem { id: "closable".to_string(), title: "可关闭".to_string(), level: 1 },
        TocItem { id: "global".to_string(), title: "全局消息".to_string(), level: 1 },
        TocItem { id: "position".to_string(), title: "位置控制".to_string(), level: 1 },
        TocItem { id: "duration".to_string(), title: "持续时间".to_string(), level: 1 },
        TocItem { id: "api".to_string(), title: "API".to_string(), level: 1 },
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
                        "消息 Message"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300",
                        "向用户反馈信息。"
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
                        title: "基础消息".to_string(),
                        description: "Message 有四种类型：info、success、warning、error。".to_string(),
                        code: r#"use helixui::components::{Message, MessageType};

rsx! {
    Message {
        message_type: MessageType::Info,
        content: "这是一条信息提示".to_string(),
    }
    Message {
        message_type: MessageType::Success,
        content: "操作成功！".to_string(),
    }
    Message {
        message_type: MessageType::Warning,
        content: "这是一条警告信息".to_string(),
    }
    Message {
        message_type: MessageType::Error,
        content: "操作失败，请重试".to_string(),
    }
}"#.to_string(),

                        div {
                            class: "flex flex-col gap-3",
                            Message {
                                message_type: MessageType::Info,
                                content: "这是一条信息提示".to_string(),
                            }
                            Message {
                                message_type: MessageType::Success,
                                content: "操作成功！".to_string(),
                            }
                            Message {
                                message_type: MessageType::Warning,
                                content: "这是一条警告信息".to_string(),
                            }
                            Message {
                                message_type: MessageType::Error,
                                content: "操作失败，请重试".to_string(),
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
                        title: "点击按钮弹出消息".to_string(),
                        description: "点击不同按钮可以弹出不同类型的消息，支持关闭功能。".to_string(),
                        code: r#"use helixui::components::{Button, ButtonType, Message, MessageType};
use dioxus::prelude::*;

#[component]
fn InteractiveDemo() -> Element {
    let mut messages = use_signal(|| Vec::<MessageData>::new());
    let mut message_id_counter = use_signal(|| 0u32);

    let add_message = move |message_type: MessageType, content: String| {
        let id = *message_id_counter.read();
        *message_id_counter.write() += 1;
        messages.write().push(MessageData { id, message_type, content });
    };

    let remove_message = move |id: u32| {
        messages.write().retain(|msg| msg.id != id);
    };

    rsx! {
        div {
            class: "space-y-4",
            
            // 按钮组
            div {
                class: "flex flex-wrap gap-3",
                Button {
                    button_type: ButtonType::Info,
                    onclick: move |_| add_message(MessageType::Info, "这是一条信息提示".to_string()),
                    "信息消息"
                }
                Button {
                    button_type: ButtonType::Success,
                    onclick: move |_| add_message(MessageType::Success, "操作成功！".to_string()),
                    "成功消息"
                }
                Button {
                    button_type: ButtonType::Warning,
                    onclick: move |_| add_message(MessageType::Warning, "这是一条警告信息".to_string()),
                    "警告消息"
                }
                Button {
                    button_type: ButtonType::Error,
                    onclick: move |_| add_message(MessageType::Error, "操作失败，请重试".to_string()),
                    "错误消息"
                }
                Button {
                    button_type: ButtonType::Primary,
                    onclick: move |_| add_message(MessageType::Loading, "正在加载中...".to_string()),
                    "加载消息"
                }
            }

            // 消息列表
            div {
                class: "space-y-2",
                for message in messages.read().iter() {
                    Message {
                        key: "{message.id}",
                        message_type: message.message_type,
                        content: message.content.clone(),
                        closable: true,
                        on_close: move |_| remove_message(message.id),
                    }
                }
            }
        }
    }
}"#.to_string(),

                        div {
                            class: "space-y-4",

                            // 按钮组
                            div {
                                class: "flex flex-wrap gap-3",
                                Button {
                                    button_type: ButtonType::Info,
                                    onclick: move |_| {
                                        let id = *message_id_counter.read();
                                        *message_id_counter.write() += 1;
                                        messages.write().push(MessageData {
                                            id,
                                            message_type: MessageType::Info,
                                            content: "这是一条信息提示".to_string(),
                                        });
                                    },
                                    "信息消息"
                                }
                                Button {
                                    button_type: ButtonType::Success,
                                    onclick: move |_| {
                                        let id = *message_id_counter.read();
                                        *message_id_counter.write() += 1;
                                        messages.write().push(MessageData {
                                            id,
                                            message_type: MessageType::Success,
                                            content: "操作成功！".to_string(),
                                        });
                                    },
                                    "成功消息"
                                }
                                Button {
                                    button_type: ButtonType::Warning,
                                    onclick: move |_| {
                                        let id = *message_id_counter.read();
                                        *message_id_counter.write() += 1;
                                        messages.write().push(MessageData {
                                            id,
                                            message_type: MessageType::Warning,
                                            content: "这是一条警告信息".to_string(),
                                        });
                                    },
                                    "警告消息"
                                }
                                Button {
                                    button_type: ButtonType::Error,
                                    onclick: move |_| {
                                        let id = *message_id_counter.read();
                                        *message_id_counter.write() += 1;
                                        messages.write().push(MessageData {
                                            id,
                                            message_type: MessageType::Error,
                                            content: "操作失败，请重试".to_string(),
                                        });
                                    },
                                    "错误消息"
                                }
                                Button {
                                    button_type: ButtonType::Primary,
                                    onclick: move |_| {
                                        let id = *message_id_counter.read();
                                        *message_id_counter.write() += 1;
                                        messages.write().push(MessageData {
                                            id,
                                            message_type: MessageType::Loading,
                                            content: "正在加载中...".to_string(),
                                        });
                                    },
                                    "加载消息"
                                }
                            }

                            // 消息列表
                            MessageList {
                                messages: messages.read().clone(),
                                on_remove: remove_message,
                            }
                        }
                    }
                }

                // 加载状态
                section {
                    id: "loading",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "加载状态"
                    }

                    DemoBox {
                        title: "加载消息".to_string(),
                        description: "显示加载中的消息，图标会旋转。".to_string(),
                        code: r#"use helixui::components::{Message, MessageType};

rsx! {
    Message {
        message_type: MessageType::Loading,
        content: "正在加载中...".to_string(),
        closable: false,
    }
}"#.to_string(),

                        Message {
                            message_type: MessageType::Loading,
                            content: "正在加载中...".to_string(),
                            closable: false,
                        }
                    }
                }

                // 可关闭
                section {
                    id: "closable",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "可关闭"
                    }

                    DemoBox {
                        title: "可关闭的消息".to_string(),
                        description: "通过 closable 属性控制消息是否可关闭。".to_string(),
                        code: r#"use helixui::components::{Message, MessageType};

rsx! {
    Message {
        message_type: MessageType::Info,
        content: "可以关闭的消息".to_string(),
        closable: true,
    }
    Message {
        message_type: MessageType::Success,
        content: "不可关闭的消息".to_string(),
        closable: false,
    }
}"#.to_string(),

                        div {
                            class: "flex flex-col gap-3",
                            Message {
                                message_type: MessageType::Info,
                                content: "可以关闭的消息（点击 X 按钮）".to_string(),
                                closable: true,
                            }
                            Message {
                                message_type: MessageType::Success,
                                content: "不可关闭的消息".to_string(),
                                closable: false,
                            }
                        }
                    }
                }

                // 全局消息演示
                section {
                    id: "global",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "全局消息"
                    }

                    DemoBox {
                        title: "全局消息弹出".to_string(),
                        description: "消息会在页面顶部弹出，支持位置控制和自动消失。".to_string(),
                        code: r#"use helixui::components::{Button, ButtonType, MessagePosition, MessageType, show_message, show_message_with_position, show_message_with_duration};

rsx! {
    div {
        class: "space-y-4",
        
        // 基础消息
        div {
            class: "flex flex-wrap gap-3",
            Button {
                button_type: ButtonType::Info,
                onclick: move |_| show_message(MessageType::Info, "这是一条全局信息消息".to_string()),
                "信息消息"
            }
            Button {
                button_type: ButtonType::Success,
                onclick: move |_| show_message(MessageType::Success, "操作成功！".to_string()),
                "成功消息"
            }
            Button {
                button_type: ButtonType::Warning,
                onclick: move |_| show_message(MessageType::Warning, "这是一条警告消息".to_string()),
                "警告消息"
            }
            Button {
                button_type: ButtonType::Error,
                onclick: move |_| show_message(MessageType::Error, "操作失败，请重试".to_string()),
                "错误消息"
            }
        }
    }
}"#.to_string(),

                        div {
                            class: "space-y-4",

                            // 基础消息按钮
                            div {
                                class: "flex flex-wrap gap-3",
                                Button {
                                    button_type: ButtonType::Info,
                                    onclick: move |_| { show_message(MessageType::Info, "这是一条全局信息消息".to_string()); },
                                    "信息消息"
                                }
                                Button {
                                    button_type: ButtonType::Success,
                                    onclick: move |_| { show_message(MessageType::Success, "操作成功！".to_string()); },
                                    "成功消息"
                                }
                                Button {
                                    button_type: ButtonType::Warning,
                                    onclick: move |_| { show_message(MessageType::Warning, "这是一条警告消息".to_string()); },
                                    "警告消息"
                                }
                                Button {
                                    button_type: ButtonType::Error,
                                    onclick: move |_| { show_message(MessageType::Error, "操作失败，请重试".to_string()); },
                                    "错误消息"
                                }
                            }
                        }
                    }
                }

                // 位置控制
                section {
                    id: "position",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "位置控制"
                    }

                    DemoBox {
                        title: "不同位置的消息".to_string(),
                        description: "可以控制消息在页面中的弹出位置。".to_string(),
                        code: r#"use helixui::components::{Button, ButtonType, MessagePosition, MessageType, show_message_with_position};

rsx! {
    div {
        class: "space-y-4",
        
        // 位置按钮
        div {
            class: "grid grid-cols-3 gap-3",
            Button {
                button_type: ButtonType::Info,
                onclick: move |_| show_message_with_position(MessageType::Info, "顶部消息".to_string(), MessagePosition::Top),
                "顶部"
            }
            Button {
                button_type: ButtonType::Info,
                onclick: move |_| show_message_with_position(MessageType::Info, "底部消息".to_string(), MessagePosition::Bottom),
                "底部"
            }
            Button {
                button_type: ButtonType::Info,
                onclick: move |_| show_message_with_position(MessageType::Info, "左上消息".to_string(), MessagePosition::TopLeft),
                "左上"
            }
            Button {
                button_type: ButtonType::Info,
                onclick: move |_| show_message_with_position(MessageType::Info, "右上消息".to_string(), MessagePosition::TopRight),
                "右上"
            }
            Button {
                button_type: ButtonType::Info,
                onclick: move |_| show_message_with_position(MessageType::Info, "左下消息".to_string(), MessagePosition::BottomLeft),
                "左下"
            }
            Button {
                button_type: ButtonType::Info,
                onclick: move |_| show_message_with_position(MessageType::Info, "右下消息".to_string(), MessagePosition::BottomRight),
                "右下"
            }
        }
    }
}"#.to_string(),

                        div {
                            class: "space-y-4",

                            // 位置按钮
                            div {
                                class: "grid grid-cols-3 gap-3",
                                Button {
                                    button_type: ButtonType::Info,
                                    onclick: move |_| { show_message_with_position(MessageType::Info, "顶部消息".to_string(), MessagePosition::Top); },
                                    "顶部"
                                }
                                Button {
                                    button_type: ButtonType::Info,
                                    onclick: move |_| { show_message_with_position(MessageType::Info, "底部消息".to_string(), MessagePosition::Bottom); },
                                    "底部"
                                }
                                Button {
                                    button_type: ButtonType::Info,
                                    onclick: move |_| { show_message_with_position(MessageType::Info, "左上消息".to_string(), MessagePosition::TopLeft); },
                                    "左上"
                                }
                                Button {
                                    button_type: ButtonType::Info,
                                    onclick: move |_| { show_message_with_position(MessageType::Info, "右上消息".to_string(), MessagePosition::TopRight); },
                                    "右上"
                                }
                                Button {
                                    button_type: ButtonType::Info,
                                    onclick: move |_| { show_message_with_position(MessageType::Info, "左下消息".to_string(), MessagePosition::BottomLeft); },
                                    "左下"
                                }
                                Button {
                                    button_type: ButtonType::Info,
                                    onclick: move |_| { show_message_with_position(MessageType::Info, "右下消息".to_string(), MessagePosition::BottomRight); },
                                    "右下"
                                }
                            }
                        }
                    }
                }

                // 持续时间控制
                section {
                    id: "duration",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "持续时间控制"
                    }

                    DemoBox {
                        title: "不同持续时间的消息".to_string(),
                        description: "可以控制消息的自动消失时间。".to_string(),
                        code: r#"use helixui::components::{Button, ButtonType, MessageType, show_message_with_duration};

rsx! {
    div {
        class: "space-y-4",
        
        // 持续时间按钮
        div {
            class: "flex flex-wrap gap-3",
            Button {
                button_type: ButtonType::Success,
                onclick: move |_| show_message_with_duration(MessageType::Success, "1秒后消失".to_string(), 1000),
                "1秒"
            }
            Button {
                button_type: ButtonType::Warning,
                onclick: move |_| show_message_with_duration(MessageType::Warning, "3秒后消失".to_string(), 3000),
                "3秒"
            }
            Button {
                button_type: ButtonType::Info,
                onclick: move |_| show_message_with_duration(MessageType::Info, "5秒后消失".to_string(), 5000),
                "5秒"
            }
            Button {
                button_type: ButtonType::Error,
                onclick: move |_| show_message_with_duration(MessageType::Error, "不自动消失".to_string(), 0),
                "不消失"
            }
        }
    }
}"#.to_string(),

                        div {
                            class: "space-y-4",

                            // 持续时间按钮
                            div {
                                class: "flex flex-wrap gap-3",
                                Button {
                                    button_type: ButtonType::Success,
                                    onclick: move |_| { show_message_with_duration(MessageType::Success, "1秒后消失".to_string(), 1000); },
                                    "1秒"
                                }
                                Button {
                                    button_type: ButtonType::Warning,
                                    onclick: move |_| { show_message_with_duration(MessageType::Warning, "3秒后消失".to_string(), 3000); },
                                    "3秒"
                                }
                                Button {
                                    button_type: ButtonType::Info,
                                    onclick: move |_| { show_message_with_duration(MessageType::Info, "5秒后消失".to_string(), 5000); },
                                    "5秒"
                                }
                                Button {
                                    button_type: ButtonType::Error,
                                    onclick: move |_| { show_message_with_duration(MessageType::Error, "不自动消失".to_string(), 0); },
                                    "不消失"
                                }
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
                        "Message Props"
                    }

                    div {
                        class: "overflow-x-auto",
                        table {
                            class: "w-full text-left border-collapse",
                            thead {
                                tr {
                                    class: "border-b border-gray-200 dark:border-gray-700",
                                    th { class: "p-3 text-gray-900 dark:text-white font-semibold", "属性" }
                                    th { class: "p-3 text-gray-900 dark:text-white font-semibold", "说明" }
                                    th { class: "p-3 text-gray-900 dark:text-white font-semibold", "类型" }
                                    th { class: "p-3 text-gray-900 dark:text-white font-semibold", "默认值" }
                                }
                            }
                            tbody {
                                tr {
                                    class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "message_type" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "消息类型" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "MessageType" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "Info" }
                                }
                                tr {
                                    class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "content" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "消息内容" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "String" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "-" }
                                }
                                tr {
                                    class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "closable" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "是否可关闭" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "bool" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "true" }
                                }
                                tr {
                                    class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "on_close" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "关闭回调函数" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "EventHandler<()>" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "None" }
                                }
                                tr {
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "duration" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "持续时间（毫秒）" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "u32" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "0" }
                                }
                            }
                        }
                    }

                    h3 {
                        class: "text-xl font-semibold text-gray-900 dark:text-white mb-3 mt-6",
                        "MessageType"
                    }

                    div {
                        class: "bg-gray-50 dark:bg-gray-800 rounded-lg p-4 mb-6",
                        ul {
                            class: "space-y-1 text-sm text-gray-700 dark:text-gray-300 font-mono",
                            li { "• Info - 信息提示" }
                            li { "• Success - 成功提示" }
                            li { "• Warning - 警告提示" }
                            li { "• Error - 错误提示" }
                            li { "• Loading - 加载提示" }
                        }
                    }

                    h3 {
                        class: "text-xl font-semibold text-gray-900 dark:text-white mb-3",
                        "MessagePosition"
                    }

                    div {
                        class: "bg-gray-50 dark:bg-gray-800 rounded-lg p-4 mb-6",
                        ul {
                            class: "space-y-1 text-sm text-gray-700 dark:text-gray-300 font-mono",
                            li { "• Top - 顶部居中" }
                            li { "• Bottom - 底部居中" }
                            li { "• TopLeft - 左上角" }
                            li { "• TopRight - 右上角" }
                            li { "• BottomLeft - 左下角" }
                            li { "• BottomRight - 右下角" }
                        }
                    }

                    h3 {
                        class: "text-xl font-semibold text-gray-900 dark:text-white mb-3",
                        "全局消息函数"
                    }

                    div {
                        class: "bg-gray-50 dark:bg-gray-800 rounded-lg p-4",
                        ul {
                            class: "space-y-1 text-sm text-gray-700 dark:text-gray-300 font-mono",
                            li { "• show_message(type, content) - 显示基础消息" }
                            li { "• show_message_with_position(type, content, position) - 指定位置" }
                            li { "• show_message_with_duration(type, content, duration) - 指定持续时间" }
                            li { "• show_message_full(type, content, position, duration, closable) - 完整控制" }
                            li { "• close_message(id) - 关闭指定消息" }
                            li { "• close_all_messages() - 关闭所有消息" }
                        }
                    }
                }
            }
        }
    }
}
