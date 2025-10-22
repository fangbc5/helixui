use crate::overlay::components::{BaseOverlay, BaseOverlayProps, OverlayPosition};
use crate::overlay::core::animation_manager::FillMode;
use crate::overlay::core::{AnimationConfig, AnimationType, EasingType, ThemeMode};
use crate::overlay::{
    MessagePosition as OverlayMessagePosition, MessageType as OverlayMessageType,
};
use crate::{Button, ButtonShape, ButtonSize, Icon, IconSize, IconType};
use dioxus::prelude::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

/// Message 类型
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MessageType {
    Success,
    Warning,
    Error,
    Info,
    Loading,
}

/// Message 位置
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MessagePosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    Center,
}

// 类型转换函数
impl From<MessageType> for OverlayMessageType {
    fn from(msg_type: MessageType) -> Self {
        match msg_type {
            MessageType::Success => OverlayMessageType::Success,
            MessageType::Warning => OverlayMessageType::Warning,
            MessageType::Error => OverlayMessageType::Error,
            MessageType::Info => OverlayMessageType::Info,
            MessageType::Loading => OverlayMessageType::Loading,
        }
    }
}

impl From<MessagePosition> for OverlayMessagePosition {
    fn from(pos: MessagePosition) -> Self {
        match pos {
            MessagePosition::TopLeft => OverlayMessagePosition::TopLeft,
            MessagePosition::TopCenter => OverlayMessagePosition::TopCenter,
            MessagePosition::TopRight => OverlayMessagePosition::TopRight,
            MessagePosition::BottomLeft => OverlayMessagePosition::BottomLeft,
            MessagePosition::BottomCenter => OverlayMessagePosition::BottomCenter,
            MessagePosition::BottomRight => OverlayMessagePosition::BottomRight,
            MessagePosition::Center => OverlayMessagePosition::Center,
        }
    }
}

impl From<MessagePosition> for OverlayPosition {
    fn from(pos: MessagePosition) -> Self {
        match pos {
            MessagePosition::TopLeft => OverlayPosition::TopLeft,
            MessagePosition::TopCenter => OverlayPosition::TopCenter,
            MessagePosition::TopRight => OverlayPosition::TopRight,
            MessagePosition::BottomLeft => OverlayPosition::BottomLeft,
            MessagePosition::BottomCenter => OverlayPosition::BottomCenter,
            MessagePosition::BottomRight => OverlayPosition::BottomRight,
            MessagePosition::Center => OverlayPosition::Center,
        }
    }
}

/// Message 属性
#[derive(Props, Clone, PartialEq)]
pub struct MessageProps {
    /// 是否显示消息
    #[props(default = false)]
    pub visible: bool,

    /// 消息内容
    pub content: String,

    /// 消息类型
    #[props(default = MessageType::Info)]
    pub message_type: MessageType,

    /// 消息位置
    #[props(default = MessagePosition::TopRight)]
    pub position: MessagePosition,

    /// 持续时间（毫秒），0 表示不自动关闭
    #[props(default = 2000)]
    pub duration: u32,

    /// 是否可关闭
    #[props(default = true)]
    pub closable: bool,

    /// 关闭事件
    pub on_close: Option<EventHandler<()>>,

    /// 自定义样式类
    pub class: Option<String>,
}

/// Message 数据
#[derive(Clone, Debug, PartialEq)]
pub struct MessageData {
    pub id: String,
    pub content: String,
    pub message_type: MessageType,
    pub position: MessagePosition,
    pub duration: u32,
    pub closable: bool,
    pub show_icon: bool,
    pub seq: u64,
}

/// 简化的消息管理器
#[derive(Clone)]
pub struct MessageManager {
    messages: Arc<Mutex<HashMap<String, MessageData>>>,
}

impl MessageManager {
    pub fn new() -> Self {
        Self {
            messages: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_message(&self, message: MessageData) {
        if let Ok(mut messages) = self.messages.lock() {
            messages.insert(message.id.clone(), message);
        }
    }

    pub fn remove_message(&self, id: &str) {
        if let Ok(mut messages) = self.messages.lock() {
            messages.remove(id);
        }
    }

    pub fn get_messages(&self) -> Vec<MessageData> {
        if let Ok(messages) = self.messages.lock() {
            messages.values().cloned().collect()
        } else {
            Vec::new()
        }
    }

    pub fn clear_all(&self) {
        if let Ok(mut messages) = self.messages.lock() {
            messages.clear();
        }
    }
}

// 全局消息管理器
static GLOBAL_MESSAGE_MANAGER: std::sync::OnceLock<MessageManager> = std::sync::OnceLock::new();

fn get_global_message_manager() -> &'static MessageManager {
    GLOBAL_MESSAGE_MANAGER.get_or_init(|| MessageManager::new())
}

// 全局更新触发器
static GLOBAL_UPDATE_TRIGGER: std::sync::OnceLock<std::sync::atomic::AtomicU32> =
    std::sync::OnceLock::new();

fn get_global_update_trigger() -> &'static std::sync::atomic::AtomicU32 {
    GLOBAL_UPDATE_TRIGGER.get_or_init(|| std::sync::atomic::AtomicU32::new(0))
}

// 序列号生成器
static NEXT_SEQ: AtomicU64 = AtomicU64::new(1);

fn generate_id(prefix: &str) -> String {
    let seq = NEXT_SEQ.fetch_add(1, Ordering::Relaxed);
    format!("{}_{}", prefix, seq)
}

/// 简化的 Message 组件（使用 BaseOverlay）
#[component]
pub fn Message(props: MessageProps) -> Element {
    let mut visible = use_signal(|| props.visible);

    // 自动关闭逻辑
    use_effect(move || {
        if props.duration > 0 && *visible.read() {
            let duration = props.duration;
            let mut visible = visible.clone();
            use_future(move || async move {
                async_std::task::sleep(std::time::Duration::from_millis(duration as u64)).await;
                visible.set(false);
            });
        }
    });

    // 监听 visible 变化
    use_effect(move || {
        if !*visible.read() {
            if let Some(on_close) = &props.on_close {
                on_close.call(());
            }
        }
    });

    // 创建 BaseOverlay 属性
    let base_props = BaseOverlayProps {
        visible: *visible.read(),
        z_index: 1000,
        position: props.position.into(),
        animation: Some(AnimationConfig {
            duration: 200,
            delay: 0,
            easing: EasingType::EaseOut,
            enter: AnimationType::FadeIn,
            exit: AnimationType::FadeOut,
            fill_mode: FillMode::Forwards,
            iteration_count: 1,
        }),
        theme_mode: Some(ThemeMode::Auto),
        mask_closable: false,
        closable: true,
        draggable: false,
        resizable: false,
        on_close: None,
        on_show: None,
        on_hide: None,
        children: rsx! {
                // Message 的具体渲染逻辑
                div {
                    class: "p-4 rounded-lg shadow-lg border max-w-sm pointer-events-auto bg-white",
                    role: match props.message_type {
                        MessageType::Error => "alert",
                        MessageType::Warning => "alert",
                        _ => "status",
                    },
                    "aria-live": match props.message_type {
                        MessageType::Error => "assertive",
                        MessageType::Warning => "polite",
                        _ => "polite",
                    },

                    // 图标和内容
                    if props.message_type != MessageType::Loading {
                div { class: "flex items-center",
                            div { class: "flex-shrink-0 mr-3",
                                {get_icon(&props.message_type)}
                            }
                            div { class: "flex-1 text-sm font-medium",
                                {props.content}
                            }
                            if props.closable {
                                div { class: "ml-3 flex-shrink-0",
                        Button {
                            size: ButtonSize::Small,
                            shape: ButtonShape::Circle,
                            onclick: move |_| {
                                            visible.set(false);
                            },
                                        Icon {
                                            icon: IconType::Close,
                                            size: IconSize::Small
                        }
                    }
                }
            }
        }
                    } else {
                        div { class: "flex items-center",
                            div { class: "flex-shrink-0 mr-3",
                                {get_icon(&props.message_type)}
                            }
                            div { class: "flex-1 text-sm font-medium",
                                {props.content}
                            }
                        }
                    }
                }
            },
    };

    rsx! {
        BaseOverlay { ..base_props }
    }
}

/// 全局消息容器组件（使用 overlay 系统）
#[component]
pub fn MessageContainer() -> Element {
    let mut messages = use_signal(|| Vec::<MessageData>::new());
    let update_counter = use_signal(|| 0u32);

    // 立即检查一次
    use_effect(move || {
        if let Ok(manager) = get_global_message_manager().messages.lock() {
            let new_messages: Vec<MessageData> = manager.values().cloned().collect();
            println!(
                "[DEBUG] MessageContainer: Found {} messages",
                new_messages.len()
            );
            messages.set(new_messages);
        }
    });

    // 使用 Dioxus 的 use_future 进行跨平台定时更新
    use_future(move || {
        let mut messages_signal = messages.clone();
        let mut update_counter_signal = update_counter.clone();
        async move {
            loop {
                async_std::task::sleep(std::time::Duration::from_millis(500)).await;
                if let Ok(manager) = get_global_message_manager().messages.lock() {
                    let new_messages: Vec<MessageData> = manager.values().cloned().collect();
                    let current_messages = messages_signal.read().clone();
                    if new_messages != current_messages {
                        messages_signal.set(new_messages);
                        update_counter_signal.set(update_counter_signal() + 1);
                    }
                }
            }
        }
    });

    // 强制触发重新渲染
    let _ = update_counter.read();

    // 按位置分组并按 seq 排序
    let all: Vec<MessageData> = { messages.read().clone() };
    let mut top_left: Vec<MessageData> = all
        .iter()
        .filter(|m| m.position == MessagePosition::TopLeft)
        .cloned()
        .collect();
    let mut top_center: Vec<MessageData> = all
        .iter()
        .filter(|m| m.position == MessagePosition::TopCenter)
        .cloned()
        .collect();
    let mut top_right: Vec<MessageData> = all
        .iter()
        .filter(|m| m.position == MessagePosition::TopRight)
        .cloned()
        .collect();
    let mut bottom_left: Vec<MessageData> = all
        .iter()
        .filter(|m| m.position == MessagePosition::BottomLeft)
        .cloned()
        .collect();
    let mut bottom_center: Vec<MessageData> = all
        .iter()
        .filter(|m| m.position == MessagePosition::BottomCenter)
        .cloned()
        .collect();
    let mut bottom_right: Vec<MessageData> = all
        .iter()
        .filter(|m| m.position == MessagePosition::BottomRight)
        .cloned()
        .collect();
    let mut center: Vec<MessageData> = all
        .iter()
        .filter(|m| m.position == MessagePosition::Center)
        .cloned()
        .collect();

    let sort_by_seq = |v: &mut Vec<MessageData>| v.sort_by_key(|m| m.seq);
    sort_by_seq(&mut top_left);
    sort_by_seq(&mut top_center);
    sort_by_seq(&mut top_right);
    sort_by_seq(&mut bottom_left);
    sort_by_seq(&mut bottom_center);
    sort_by_seq(&mut bottom_right);
    sort_by_seq(&mut center);

    rsx! {
        // 简化版本：直接渲染消息，不使用 overlay 系统
        if !top_left.is_empty() {
            div { class: get_container_class(&MessagePosition::TopLeft),
                for message in top_left {
                    MessageItem { data: message }
                }
            }
        }
        if !top_center.is_empty() {
            div { class: get_container_class(&MessagePosition::TopCenter),
                for message in top_center {
                    MessageItem { data: message }
                }
            }
        }
        if !top_right.is_empty() {
            div { class: get_container_class(&MessagePosition::TopRight),
                for message in top_right {
                    MessageItem { data: message }
                }
            }
        }
        if !bottom_left.is_empty() {
            div { class: get_container_class(&MessagePosition::BottomLeft),
                for message in bottom_left {
                    MessageItem { data: message }
                }
            }
        }
        if !bottom_center.is_empty() {
            div { class: get_container_class(&MessagePosition::BottomCenter),
                for message in bottom_center {
                    MessageItem { data: message }
                }
            }
        }
        if !bottom_right.is_empty() {
            div { class: get_container_class(&MessagePosition::BottomRight),
                for message in bottom_right {
                    MessageItem { data: message }
                }
            }
        }
        if !center.is_empty() {
            div { class: get_container_class(&MessagePosition::Center),
                for message in center {
                    MessageItem { data: message }
                }
            }
        }
    }
}

/// 单个消息项组件（带定时关闭功能）
#[component]
pub fn MessageItem(data: MessageData) -> Element {
    let mut visible = use_signal(|| true);
    let data_id = data.id.clone();
    let data_duration = data.duration;

    // 定时关闭逻辑
    use_effect(move || {
        if data_duration > 0 && *visible.read() {
            let duration = data_duration;
            let mut visible = visible.clone();
            let data_id = data_id.clone();

            use_future(move || {
                let data_id = data_id.clone();
                async move {
                    async_std::task::sleep(std::time::Duration::from_millis(duration as u64)).await;
                    visible.set(false);
                    // 从全局管理器中移除消息
                    if let Ok(mut manager) = get_global_message_manager().messages.lock() {
                        manager.remove(&data_id);
                    }
                }
            });
        }
    });

    // 手动关闭处理
    let data_id_for_close = data.id.clone();
    let on_close = move |_| {
        visible.set(false);
        if let Ok(mut manager) = get_global_message_manager().messages.lock() {
            manager.remove(&data_id_for_close);
        }
    };

    if !*visible.read() {
        return rsx! { div {} };
    }

    let type_class = match data.message_type {
        MessageType::Success => "bg-green-50 border-green-200 text-green-800",
        MessageType::Warning => "bg-yellow-50 border-yellow-200 text-yellow-800",
        MessageType::Error => "bg-red-50 border-red-200 text-red-800",
        MessageType::Info => "bg-blue-50 border-blue-200 text-blue-800",
        MessageType::Loading => "bg-blue-50 border-blue-200 text-blue-800",
    };

    rsx! {
        div {
            class: format!("p-4 rounded-lg shadow-lg border max-w-sm pointer-events-auto mb-2 {}", type_class),
            role: match data.message_type {
                MessageType::Error => "alert",
                MessageType::Warning => "alert",
                _ => "status",
            },
            "aria-live": match data.message_type {
                MessageType::Error => "assertive",
                MessageType::Warning => "polite",
                _ => "polite",
            },

            div {
                class: "flex items-center justify-between",

                // 图标和内容
                div {
                    class: "flex items-center",
                    if data.show_icon {
                        Icon {
                            icon: match data.message_type {
                                MessageType::Success => IconType::Check,
                                MessageType::Warning => IconType::Info,
                                MessageType::Error => IconType::Error,
                                MessageType::Info => IconType::Info,
                                MessageType::Loading => IconType::Loading,
                            },
                            size: IconSize::Small,
                            class: "mr-2",
                        }
                    }

                    span {
                        class: "text-sm font-medium",
                        "{data.content}"
                    }
                }

                // 关闭按钮
                if data.closable {
                    button {
                        class: "ml-2 text-gray-400 hover:text-gray-600 focus:outline-none",
                        onclick: on_close,
                        "aria-label": "关闭消息",
                        "×"
                    }
                }
            }
        }
    }
}

/// 使用 overlay 系统的消息组件
#[component]
pub fn OverlayMessage(data: MessageData) -> Element {
    let mut visible = use_signal(|| true);

    // 自动关闭逻辑
    use_effect(move || {
        if data.duration > 0 && *visible.read() {
            let duration = data.duration;
            let mut visible = visible.clone();
            use_future(move || async move {
                async_std::task::sleep(std::time::Duration::from_millis(duration as u64)).await;
                visible.set(false);
            });
        }
    });

    // 创建 BaseOverlay 属性
    let base_props = BaseOverlayProps {
        visible: *visible.read(),
        z_index: 1000,
        position: data.position.into(),
        animation: Some(AnimationConfig {
            duration: 200,
            delay: 0,
            easing: EasingType::EaseOut,
            enter: AnimationType::FadeIn,
            exit: AnimationType::FadeOut,
            fill_mode: FillMode::Forwards,
            iteration_count: 1,
        }),
        theme_mode: Some(ThemeMode::Auto),
        mask_closable: false,
        closable: true,
        draggable: false,
        resizable: false,
        on_close: None,
        on_show: None,
        on_hide: None,
        children: rsx! {
            // Message 的具体渲染逻辑
            div {
                class: "p-4 rounded-lg shadow-lg border max-w-sm pointer-events-auto bg-white",
                role: match data.message_type {
                    MessageType::Error => "alert",
                    MessageType::Warning => "alert",
                    _ => "status",
                },
                "aria-live": match data.message_type {
                    MessageType::Error => "assertive",
                    MessageType::Warning => "polite",
                    _ => "polite",
                },

                // 图标和内容
                if data.message_type != MessageType::Loading {
                    div { class: "flex items-center",
                        div { class: "flex-shrink-0 mr-3",
                            {get_icon(&data.message_type)}
                        }
                        div { class: "flex-1 text-sm font-medium",
                            {data.content}
                        }
                        if data.closable {
                            div { class: "ml-3 flex-shrink-0",
                                Button {
                                    size: ButtonSize::Small,
                                    shape: ButtonShape::Circle,
                                    onclick: move |_| {
                                        visible.set(false);
                                        // 从全局管理器中移除
                                        if let Ok(mut manager) = get_global_message_manager().messages.lock() {
                                            manager.remove(&data.id);
                                        }
                                    },
                                    Icon {
                                        icon: IconType::Close,
                                        size: IconSize::Small
                                    }
                                }
                            }
                        }
                    }
                } else {
                    div { class: "flex items-center",
                        div { class: "flex-shrink-0 mr-3",
                            {get_icon(&data.message_type)}
                        }
                        div { class: "flex-1 text-sm font-medium",
                            {data.content}
                        }
                    }
                }
            }
        },
    };

    rsx! {
        BaseOverlay { ..base_props }
    }
}

/// 简化的消息组件（用于全局容器）
#[component]
pub fn SimpleMessage(data: MessageData) -> Element {
    let mut visible = use_signal(|| true);
    let auto_close = use_signal(|| data.duration > 0);

    // 自动关闭逻辑
    let data_id = data.id.clone();
    use_effect(move || {
        if *auto_close.read() && *visible.read() {
            let duration = data.duration;
            let id = data_id.clone();
            let mut visible = visible.clone();
            use_future(move || {
                let id = id.clone();
                async move {
                    async_std::task::sleep(std::time::Duration::from_millis(duration as u64)).await;
                    visible.set(false);
                    if let Ok(mut manager) = get_global_message_manager().messages.lock() {
                        manager.remove(&id);
                    }
                }
            });
        }
    });

    let base_class = "p-4 rounded-lg shadow-lg border max-w-sm pointer-events-auto";
    let type_class = get_type_class(&data.message_type);
    let merged_class = format!("{} {}", base_class, type_class);

    let (role, aria_live) = match data.message_type {
        MessageType::Error => ("alert", "assertive"),
        MessageType::Warning => ("alert", "polite"),
        _ => ("status", "polite"),
    };

    rsx! {
        if *visible.read() {
            div {
                class: merged_class,
                role: role,
                "aria-live": aria_live,

                // 图标
                if data.message_type != MessageType::Loading {
                    div { class: "flex items-center",
                        div { class: "flex-shrink-0 mr-3",
                            {get_icon(&data.message_type)}
                        }
                        div { class: "flex-1 text-sm font-medium",
                            {data.content}
                        }
                        if data.closable {
                            div { class: "ml-3 flex-shrink-0",
                                Button {
                                    size: ButtonSize::Small,
                                    shape: ButtonShape::Circle,
                                    onclick: move |_| {
                                        visible.set(false);
                                        if let Ok(mut manager) = get_global_message_manager().messages.lock() {
                                            manager.remove(&data.id);
                                        }
                                    },
                                    Icon {
                                        icon: IconType::Close,
                                        size: IconSize::Small
                                    }
                                }
                            }
                        }
                    }
                } else {
                    div { class: "flex items-center",
                        div { class: "flex-shrink-0 mr-3",
                            {get_icon(&data.message_type)}
                        }
                        div { class: "flex-1 text-sm font-medium",
                            {data.content}
                        }
                    }
                }
            }
        }
    }
}

// 工具函数
fn get_position_class(position: &MessagePosition) -> String {
    match position {
        MessagePosition::TopLeft => "fixed z-50 top-4 left-4".to_string(),
        MessagePosition::TopCenter => {
            "fixed z-50 top-4 left-1/2 transform -translate-x-1/2".to_string()
        }
        MessagePosition::TopRight => "fixed z-50 top-4 right-4".to_string(),
        MessagePosition::BottomLeft => "fixed z-50 bottom-4 left-4".to_string(),
        MessagePosition::BottomCenter => {
            "fixed z-50 bottom-4 left-1/2 transform -translate-x-1/2".to_string()
        }
        MessagePosition::BottomRight => "fixed z-50 bottom-4 right-4".to_string(),
        MessagePosition::Center => {
            "fixed z-50 top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2".to_string()
        }
    }
}

fn get_container_class(position: &MessagePosition) -> String {
    match position {
        MessagePosition::TopLeft => "fixed z-50 top-4 left-4 flex flex-col items-start space-y-2 pointer-events-none".to_string(),
        MessagePosition::TopCenter => "fixed z-50 top-4 left-1/2 transform -translate-x-1/2 flex flex-col items-center space-y-2 pointer-events-none".to_string(),
        MessagePosition::TopRight => "fixed z-50 top-4 right-4 flex flex-col items-end space-y-2 pointer-events-none".to_string(),
        MessagePosition::BottomLeft => "fixed z-50 bottom-4 left-4 flex flex-col items-start space-y-2 pointer-events-none".to_string(),
        MessagePosition::BottomCenter => "fixed z-50 bottom-4 left-1/2 transform -translate-x-1/2 flex flex-col items-center space-y-2 pointer-events-none".to_string(),
        MessagePosition::BottomRight => "fixed z-50 bottom-4 right-4 flex flex-col items-end space-y-2 pointer-events-none".to_string(),
        MessagePosition::Center => "fixed z-50 top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 flex flex-col items-center space-y-2 pointer-events-none".to_string(),
    }
}

fn get_type_class(message_type: &MessageType) -> String {
    match message_type {
        MessageType::Success => "bg-green-50 border-green-200 text-green-800".to_string(),
        MessageType::Warning => "bg-yellow-50 border-yellow-200 text-yellow-800".to_string(),
        MessageType::Error => "bg-red-50 border-red-200 text-red-800".to_string(),
        MessageType::Info => "bg-blue-50 border-blue-200 text-blue-800".to_string(),
        MessageType::Loading => "bg-blue-50 border-blue-200 text-blue-800".to_string(),
    }
}

fn get_icon(message_type: &MessageType) -> Element {
    let (icon, class, spin) = match message_type {
        MessageType::Success => (IconType::Success, "text-green-500", false),
        MessageType::Warning => (IconType::Warning, "text-yellow-500", false),
        MessageType::Error => (IconType::Error, "text-red-500", false),
        MessageType::Info => (IconType::Info, "text-blue-500", false),
        MessageType::Loading => (IconType::Loading, "text-blue-500", true),
    };

    rsx! {
        Icon {
            icon: icon,
            size: IconSize::Small,
            class: if spin { format!("{} animate-spin", class) } else { class.to_string() }
        }
    }
}

// 全局 API 函数
pub fn show_message(content: &str, message_type: MessageType) {
    let data = MessageData {
        id: generate_id("message"),
        content: content.to_string(),
        message_type,
        position: MessagePosition::TopRight,
        duration: 2000,
        closable: true,
        show_icon: true,
        seq: NEXT_SEQ.fetch_add(1, Ordering::Relaxed),
    };
    println!("[DEBUG] show_message called with content: {}", content);
    get_global_message_manager().add_message(data);
    // 触发更新
    get_global_update_trigger().fetch_add(1, Ordering::Relaxed);
    println!("[DEBUG] Message added to global manager");
}

pub fn show_message_with_duration(content: &str, message_type: MessageType, duration: u32) {
    let data = MessageData {
        id: generate_id("message"),
        content: content.to_string(),
        message_type,
        position: MessagePosition::TopRight,
        duration,
        closable: true,
        show_icon: true,
        seq: NEXT_SEQ.fetch_add(1, Ordering::Relaxed),
    };
    get_global_message_manager().add_message(data);
    // 触发更新
    get_global_update_trigger().fetch_add(1, Ordering::Relaxed);
}

pub fn show_message_with_position(
    content: &str,
    message_type: MessageType,
    position: MessagePosition,
) {
    let data = MessageData {
        id: generate_id("message"),
        content: content.to_string(),
        message_type,
        position,
        duration: 2000,
        closable: true,
        show_icon: true,
        seq: NEXT_SEQ.fetch_add(1, Ordering::Relaxed),
    };
    get_global_message_manager().add_message(data);
    // 触发更新
    get_global_update_trigger().fetch_add(1, Ordering::Relaxed);
}

pub fn close_message(id: &str) {
    get_global_message_manager().remove_message(id);
}

pub fn close_all_messages() {
    get_global_message_manager().clear_all();
}
