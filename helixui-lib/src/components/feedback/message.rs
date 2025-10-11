use crate::components::{Icon, IconType};
use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;

/// Message 类型
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MessageType {
    Info,
    Success,
    Warning,
    Error,
    Loading,
}

/// Message 位置
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MessagePosition {
    Top,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl MessagePosition {
    pub fn to_class(&self) -> &'static str {
        match self {
            MessagePosition::Top => "top-4 left-1/2 transform -translate-x-1/2",
            MessagePosition::Bottom => "bottom-4 left-1/2 transform -translate-x-1/2",
            MessagePosition::TopLeft => "top-4 left-4",
            MessagePosition::TopRight => "top-4 right-4",
            MessagePosition::BottomLeft => "bottom-4 left-4",
            MessagePosition::BottomRight => "bottom-4 right-4",
        }
    }
}

impl MessageType {
    /// 获取对应的图标
    pub fn icon(&self) -> IconType {
        match self {
            MessageType::Info => IconType::Info,
            MessageType::Success => IconType::Success,
            MessageType::Warning => IconType::Warning,
            MessageType::Error => IconType::Error,
            MessageType::Loading => IconType::Loading,
        }
    }

    /// 获取对应的颜色类
    pub fn color_class(&self) -> &'static str {
        match self {
            MessageType::Info => "text-blue-500",
            MessageType::Success => "text-green-500",
            MessageType::Warning => "text-orange-500",
            MessageType::Error => "text-red-500",
            MessageType::Loading => "text-blue-500",
        }
    }

    /// 获取背景色类
    pub fn bg_class(&self) -> &'static str {
        match self {
            MessageType::Info => {
                "bg-blue-50 dark:bg-blue-900/20 border-blue-200 dark:border-blue-800"
            }
            MessageType::Success => {
                "bg-green-50 dark:bg-green-900/20 border-green-200 dark:border-green-800"
            }
            MessageType::Warning => {
                "bg-orange-50 dark:bg-orange-900/20 border-orange-200 dark:border-orange-800"
            }
            MessageType::Error => "bg-red-50 dark:bg-red-900/20 border-red-200 dark:border-red-800",
            MessageType::Loading => {
                "bg-blue-50 dark:bg-blue-900/20 border-blue-200 dark:border-blue-800"
            }
        }
    }
}

/// Message 组件
#[component]
pub fn Message(
    /// 消息类型
    #[props(default = MessageType::Info)]
    message_type: MessageType,
    /// 消息内容
    content: String,
    /// 是否可关闭
    #[props(default = true)]
    closable: bool,
    /// 关闭回调
    #[props(default)]
    on_close: Option<EventHandler<()>>,
    /// 持续时间（毫秒），0 表示不自动关闭
    #[props(default = 2000)]
    duration: u32,
) -> Element {
    let icon_type = message_type.icon();
    let color_class = message_type.color_class();
    let bg_class = message_type.bg_class();
    let is_loading = message_type == MessageType::Loading;

    // 自动关闭功能
    let _timer_started = use_memo(move || {
        if duration > 0 {
            let handler = on_close.clone();
            spawn(async move {
                TimeoutFuture::new(duration).await;
                if let Some(handler) = handler {
                    handler.call(());
                }
            });
            true
        } else {
            false
        }
    });

    rsx! {
        div {
            class: "flex items-center gap-3 px-4 py-3 {bg_class} border rounded-lg shadow-lg transition-all",

            // 图标
            Icon {
                icon: icon_type,
                spin: is_loading,
                class: color_class.to_string(),
            }

            // 内容
            div {
                class: "flex-1 text-sm text-gray-700 dark:text-gray-200",
                "{content}"
            }

            // 关闭按钮
            if closable {
                button {
                    class: "text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors",
                    onclick: move |_| {
                        if let Some(handler) = &on_close {
                            handler.call(());
                        }
                    },
                    Icon {
                        icon: IconType::Close,
                        size: crate::components::IconSize::Small,
                    }
                }
            }
        }
    }
}

/// 全局消息数据
#[derive(Clone, PartialEq)]
pub struct GlobalMessage {
    pub id: u32,
    pub message_type: MessageType,
    pub content: String,
    pub position: MessagePosition,
    pub duration: u32,
    pub closable: bool,
}

/// 全局消息管理器
pub struct MessageManager {
    messages: Signal<Vec<GlobalMessage>>,
    next_id: Signal<u32>,
}

impl MessageManager {
    pub fn new() -> Self {
        Self {
            messages: Signal::new(Vec::new()),
            next_id: Signal::new(0),
        }
    }

    pub fn show(
        &mut self,
        message_type: MessageType,
        content: String,
        position: MessagePosition,
        duration: u32,
        closable: bool,
    ) -> u32 {
        let id = *self.next_id.read();
        *self.next_id.write() += 1;

        let message = GlobalMessage {
            id,
            message_type,
            content,
            position,
            duration,
            closable,
        };

        self.messages.write().push(message);
        id
    }

    pub fn close(&mut self, id: u32) {
        self.messages.write().retain(|msg| msg.id != id);
    }

    pub fn close_all(&mut self) {
        self.messages.write().clear();
    }

    pub fn get_messages(&self) -> Signal<Vec<GlobalMessage>> {
        self.messages
    }
}

/// 全局消息管理器实例
pub static MESSAGE_MANAGER: GlobalSignal<MessageManager> = Signal::global(MessageManager::new);

/// 全局消息容器组件
#[component]
pub fn GlobalMessageContainer() -> Element {
    let manager = MESSAGE_MANAGER.signal();
    let messages = manager.read().get_messages();

    rsx! {
        div {
            class: "fixed inset-0 pointer-events-none z-50",

            // 顶部位置
            div {
                class: "absolute top-4 left-1/2 transform -translate-x-1/2 flex flex-col gap-2 min-w-[300px] max-w-[500px] pointer-events-auto",
                for message in messages.read().iter().filter(|m| m.position == MessagePosition::Top) {
                    GlobalMessageItem { key: "msg-{message.id}", message: message.clone() }
                }
            }

            // 底部位置
            div {
                class: "absolute bottom-4 left-1/2 transform -translate-x-1/2 flex flex-col gap-2 min-w-[300px] max-w-[500px] pointer-events-auto",
                for message in messages.read().iter().filter(|m| m.position == MessagePosition::Bottom) {
                    GlobalMessageItem { key: "msg-{message.id}", message: message.clone() }
                }
            }

            // 左上位置
            div {
                class: "absolute top-4 left-4 flex flex-col gap-2 min-w-[300px] max-w-[500px] pointer-events-auto",
                for message in messages.read().iter().filter(|m| m.position == MessagePosition::TopLeft) {
                    GlobalMessageItem { key: "msg-{message.id}", message: message.clone() }
                }
            }

            // 右上位置
            div {
                class: "absolute top-4 right-4 flex flex-col gap-2 min-w-[300px] max-w-[500px] pointer-events-auto",
                for message in messages.read().iter().filter(|m| m.position == MessagePosition::TopRight) {
                    GlobalMessageItem { key: "msg-{message.id}", message: message.clone() }
                }
            }

            // 左下位置
            div {
                class: "absolute bottom-4 left-4 flex flex-col gap-2 min-w-[300px] max-w-[500px] pointer-events-auto",
                for message in messages.read().iter().filter(|m| m.position == MessagePosition::BottomLeft) {
                    GlobalMessageItem { key: "msg-{message.id}", message: message.clone() }
                }
            }

            // 右下位置
            div {
                class: "absolute bottom-4 right-4 flex flex-col gap-2 min-w-[300px] max-w-[500px] pointer-events-auto",
                for message in messages.read().iter().filter(|m| m.position == MessagePosition::BottomRight) {
                    GlobalMessageItem { key: "msg-{message.id}", message: message.clone() }
                }
            }
        }
    }
}

/// 全局消息项组件
#[component]
fn GlobalMessageItem(message: GlobalMessage) -> Element {
    let message_id = message.id;

    rsx! {
        Message {
            key: "msg-{message_id}",
            message_type: message.message_type,
            content: message.content,
            closable: message.closable,
            duration: message.duration,
            on_close: move |_| {
                MESSAGE_MANAGER.write().close(message_id);
            },
        }
    }
}

/// 便捷的消息显示函数
pub fn show_message(message_type: MessageType, content: String) -> u32 {
    MESSAGE_MANAGER
        .write()
        .show(message_type, content, MessagePosition::Top, 3000, true)
}

pub fn show_message_with_position(
    message_type: MessageType,
    content: String,
    position: MessagePosition,
) -> u32 {
    MESSAGE_MANAGER
        .write()
        .show(message_type, content, position, 3000, true)
}

pub fn show_message_with_duration(
    message_type: MessageType,
    content: String,
    duration: u32,
) -> u32 {
    MESSAGE_MANAGER
        .write()
        .show(message_type, content, MessagePosition::Top, duration, true)
}

pub fn show_message_full(
    message_type: MessageType,
    content: String,
    position: MessagePosition,
    duration: u32,
    closable: bool,
) -> u32 {
    MESSAGE_MANAGER
        .write()
        .show(message_type, content, position, duration, closable)
}

pub fn close_message(id: u32) {
    MESSAGE_MANAGER.write().close(id);
}

pub fn close_all_messages() {
    MESSAGE_MANAGER.write().close_all();
}
