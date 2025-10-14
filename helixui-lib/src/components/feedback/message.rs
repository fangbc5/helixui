use crate::overlay::config::{SimpleMessagePosition, SimpleMessageType};
use crate::{Button, ButtonShape, ButtonSize, ButtonType, Icon, IconSize, IconType};
use async_broadcast::broadcast;
use dioxus::prelude::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{LazyLock, Mutex};

/// Message 类型（基于新的 SimpleMessageType）
pub type MessageType = SimpleMessageType;

/// Message 位置（基于新的 SimpleMessagePosition）
pub type MessagePosition = SimpleMessagePosition;

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

#[component]
pub fn Message(props: MessageProps) -> Element {
    let data = SimpleMessageData {
        id: "0".to_string(), // 在全局容器中会生成真实 ID
        content: props.content.clone(),
        message_type: props.message_type.clone(),
        position: props.position.clone(),
        duration: props.duration,
        closable: props.closable,
        show_icon: true,
        seq: NEXT_SEQ.fetch_add(1, Ordering::Relaxed),
    };

    rsx! {
        if props.visible {
            SimpleMessage { data: data }
        }
    }
}

/// Message 数据
#[derive(Clone, Debug, PartialEq)]
pub struct MessageData {
    pub id: u32,
    pub content: String,
    pub message_type: MessageType,
    pub position: MessagePosition,
    pub duration: u32,
}

/// 简化的消息数据
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleMessageData {
    pub id: String,
    pub content: String,
    pub message_type: SimpleMessageType,
    pub position: SimpleMessagePosition,
    pub duration: u32,
    pub closable: bool,
    pub show_icon: bool,
    pub seq: u64,
}

/// 简化的消息管理器（全局）
pub struct SimpleMessageManager {
    messages: HashMap<String, SimpleMessageData>,
}

impl SimpleMessageManager {
    pub fn new() -> Self {
        Self {
            messages: HashMap::new(),
        }
    }
    pub fn add_message(&mut self, data: SimpleMessageData) {
        self.messages.insert(data.id.clone(), data);
        notify_message_change();
    }
    pub fn remove_message(&mut self, id: &str) {
        self.messages.remove(id);
        notify_message_change();
    }
    pub fn get_messages(&self) -> Vec<SimpleMessageData> {
        self.messages.values().cloned().collect()
    }
    pub fn clear_all(&mut self) {
        self.messages.clear();
        notify_message_change();
    }
}

static GLOBAL_MESSAGE_MANAGER: LazyLock<Mutex<SimpleMessageManager>> =
    LazyLock::new(|| Mutex::new(SimpleMessageManager::new()));

// 全局自增序列与计数器，用于稳定排序与避免 ID 碰撞
static NEXT_SEQ: AtomicU64 = AtomicU64::new(1);
static NEXT_COUNTER: AtomicU64 = AtomicU64::new(1);

// 跨平台广播：用于通知容器更新列表
static MESSAGE_BUS: LazyLock<(async_broadcast::Sender<()>, async_broadcast::Receiver<()>)> =
    LazyLock::new(|| broadcast(64));

fn get_global_message_manager() -> &'static Mutex<SimpleMessageManager> {
    &GLOBAL_MESSAGE_MANAGER
}

// 广播一条空事件，跨平台
fn notify_message_change() {
    let _ = MESSAGE_BUS.0.try_broadcast(());
}

#[component]
pub fn SimpleMessage(data: SimpleMessageData) -> Element {
    let data_id = data.id.clone();
    let data_duration = data.duration;

    use_effect(move || {
        if data_duration > 0 {
            let duration = data_duration;
            let id = data_id.clone();
            spawn(async move {
                use std::time::Duration;
                futures_timer::Delay::new(Duration::from_millis(duration as u64)).await;
                if let Ok(mut manager) = get_global_message_manager().lock() {
                    manager.remove_message(&id);
                }
            });
        }
    });

    let base_class = "p-4 rounded-lg shadow-lg border max-w-sm pointer-events-auto";
    let type_class = get_type_class(&data.message_type);
    let merged_class = format!("{} {}", base_class, type_class);

    let (role, aria_live) = match data.message_type {
        SimpleMessageType::Error => ("alert", "assertive"),
        SimpleMessageType::Warning => ("alert", "polite"),
        _ => ("status", "polite"),
    };

    rsx! {
        div { class: merged_class, role: role, "aria-live": aria_live,
            div { class: "flex items-center",
                if data.show_icon {
                    div { class: "flex-shrink-0 mr-2", {get_icon(&data.message_type)} }
                }
                div { class: "flex-1", {data.content} }
                if data.closable {
                    Button {

                        size: ButtonSize::Small,
                        shape: ButtonShape::Circle,
                        class: Some("flex-shrink-0 ml-2 text-gray-400 hover:text-gray-600".to_string()),
                        onclick: move |_| {
                            if let Ok(mut manager) = get_global_message_manager().lock() {
                                manager.remove_message(&data.id);
                            }
                        },
                        Icon { icon: IconType::Close, class: "w-4 h-4".to_string() }
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
fn get_position_class(position: &SimpleMessagePosition) -> String {
    match position {
        SimpleMessagePosition::TopLeft => "top-4 left-4".to_string(),
        SimpleMessagePosition::TopCenter => "top-4 left-1/2 transform -translate-x-1/2".to_string(),
        SimpleMessagePosition::TopRight => "top-4 right-4".to_string(),
        SimpleMessagePosition::BottomLeft => "bottom-4 left-4".to_string(),
        SimpleMessagePosition::BottomCenter => {
            "bottom-4 left-1/2 transform -translate-x-1/2".to_string()
        }
        SimpleMessagePosition::BottomRight => "bottom-4 right-4".to_string(),
        SimpleMessagePosition::Center => {
            "top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2".to_string()
        }
    }
}

fn get_container_class(position: &SimpleMessagePosition) -> String {
    match position {
        SimpleMessagePosition::TopLeft =>
            "fixed z-50 top-4 left-4 flex flex-col items-start space-y-2 pointer-events-none".to_string(),
        SimpleMessagePosition::TopCenter =>
            "fixed z-50 top-4 left-1/2 transform -translate-x-1/2 flex flex-col items-center space-y-2 pointer-events-none".to_string(),
        SimpleMessagePosition::TopRight =>
            "fixed z-50 top-4 right-4 flex flex-col items-end space-y-2 pointer-events-none".to_string(),
        SimpleMessagePosition::BottomLeft =>
            "fixed z-50 bottom-4 left-4 flex flex-col items-start space-y-2 pointer-events-none".to_string(),
        SimpleMessagePosition::BottomCenter =>
            "fixed z-50 bottom-4 left-1/2 transform -translate-x-1/2 flex flex-col items-center space-y-2 pointer-events-none".to_string(),
        SimpleMessagePosition::BottomRight =>
            "fixed z-50 bottom-4 right-4 flex flex-col items-end space-y-2 pointer-events-none".to_string(),
        SimpleMessagePosition::Center =>
            "fixed z-50 top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 flex flex-col items-center space-y-2 pointer-events-none".to_string(),
    }
}

fn get_type_class(message_type: &SimpleMessageType) -> String {
    match message_type {
        SimpleMessageType::Success => "bg-green-50 border-green-200 text-green-800".to_string(),
        SimpleMessageType::Warning => "bg-yellow-50 border-yellow-200 text-yellow-800".to_string(),
        SimpleMessageType::Error => "bg-red-50 border-red-200 text-red-800".to_string(),
        SimpleMessageType::Info => "bg-blue-50 border-blue-200 text-blue-800".to_string(),
        SimpleMessageType::Loading => "bg-blue-50 border-blue-200 text-blue-800".to_string(),
    }
}

fn get_icon(message_type: &SimpleMessageType) -> Element {
    let (icon, class, spin) = match message_type {
        SimpleMessageType::Success => (IconType::Success, "text-green-500", false),
        SimpleMessageType::Warning => (IconType::Warning, "text-yellow-500", false),
        SimpleMessageType::Error => (IconType::Error, "text-red-500", false),
        SimpleMessageType::Info => (IconType::Info, "text-blue-500", false),
        SimpleMessageType::Loading => (IconType::Loading, "text-blue-500", true),
    };

    rsx! {
        Icon { icon: icon, size: IconSize::Medium, class: class.to_string(), spin: spin }
    }
}

// 原直插分支的图标路径已不再需要

#[component]
pub fn GlobalMessageContainer() -> Element {
    let mut messages = use_signal(|| Vec::<SimpleMessageData>::new());
    // 首次同步一次
    if let Ok(manager) = get_global_message_manager().lock() {
        messages.set(manager.get_messages());
    }
    // 事件驱动更新（跨平台广播）
    use_effect(move || {
        let mut rx = MESSAGE_BUS.0.new_receiver();
        let mut messages_signal = messages.clone();
        spawn(async move {
            loop {
                let _ = rx.recv().await;
                if let Ok(manager) = get_global_message_manager().lock() {
                    messages_signal.set(manager.get_messages());
                }
            }
        });
    });
    // 按位置分组并按 seq 排序，分别渲染到对应容器
    let all = messages.read();
    let mut top_left: Vec<SimpleMessageData> = all
        .iter()
        .filter(|m| m.position == SimpleMessagePosition::TopLeft)
        .cloned()
        .collect();
    let mut top_center: Vec<SimpleMessageData> = all
        .iter()
        .filter(|m| m.position == SimpleMessagePosition::TopCenter)
        .cloned()
        .collect();
    let mut top_right: Vec<SimpleMessageData> = all
        .iter()
        .filter(|m| m.position == SimpleMessagePosition::TopRight)
        .cloned()
        .collect();
    let mut bottom_left: Vec<SimpleMessageData> = all
        .iter()
        .filter(|m| m.position == SimpleMessagePosition::BottomLeft)
        .cloned()
        .collect();
    let mut bottom_center: Vec<SimpleMessageData> = all
        .iter()
        .filter(|m| m.position == SimpleMessagePosition::BottomCenter)
        .cloned()
        .collect();
    let mut bottom_right: Vec<SimpleMessageData> = all
        .iter()
        .filter(|m| m.position == SimpleMessagePosition::BottomRight)
        .cloned()
        .collect();
    let mut center: Vec<SimpleMessageData> = all
        .iter()
        .filter(|m| m.position == SimpleMessagePosition::Center)
        .cloned()
        .collect();

    let sort_by_seq = |v: &mut Vec<SimpleMessageData>| v.sort_by_key(|m| m.seq);
    sort_by_seq(&mut top_left);
    sort_by_seq(&mut top_center);
    sort_by_seq(&mut top_right);
    sort_by_seq(&mut bottom_left);
    sort_by_seq(&mut bottom_center);
    sort_by_seq(&mut bottom_right);
    sort_by_seq(&mut center);

    rsx! {
        // 独立位置容器（避免重叠），每个容器自身 pointer-events-none，子项启用 pointer-events-auto
        if !top_left.is_empty() {
            div { class: get_container_class(&SimpleMessagePosition::TopLeft),
                for message in top_left { SimpleMessage { data: message } }
            }
        }
        if !top_center.is_empty() {
            div { class: get_container_class(&SimpleMessagePosition::TopCenter),
                for message in top_center { SimpleMessage { data: message } }
            }
        }
        if !top_right.is_empty() {
            div { class: get_container_class(&SimpleMessagePosition::TopRight),
                for message in top_right { SimpleMessage { data: message } }
            }
        }
        if !bottom_left.is_empty() {
            div { class: get_container_class(&SimpleMessagePosition::BottomLeft),
                for message in bottom_left { SimpleMessage { data: message } }
            }
        }
        if !bottom_center.is_empty() {
            div { class: get_container_class(&SimpleMessagePosition::BottomCenter),
                for message in bottom_center { SimpleMessage { data: message } }
            }
        }
        if !bottom_right.is_empty() {
            div { class: get_container_class(&SimpleMessagePosition::BottomRight),
                for message in bottom_right { SimpleMessage { data: message } }
            }
        }
        if !center.is_empty() {
            div { class: get_container_class(&SimpleMessagePosition::Center),
                for message in center { SimpleMessage { data: message } }
            }
        }
    }
}

pub fn show_message(content: &str, message_type: MessageType) {
    let data = SimpleMessageData {
        id: generate_id("message"),
        content: content.to_string(),
        message_type,
        position: SimpleMessagePosition::TopRight,
        duration: 2000,
        closable: true,
        show_icon: true,
        seq: NEXT_SEQ.fetch_add(1, Ordering::Relaxed),
    };
    if let Ok(mut manager) = get_global_message_manager().lock() {
        manager.add_message(data);
    }
}

pub fn show_message_with_duration(content: &str, message_type: MessageType, duration: u32) {
    let data = SimpleMessageData {
        id: generate_id("message"),
        content: content.to_string(),
        message_type,
        position: SimpleMessagePosition::TopRight,
        duration,
        closable: true,
        show_icon: true,
        seq: NEXT_SEQ.fetch_add(1, Ordering::Relaxed),
    };
    if let Ok(mut manager) = get_global_message_manager().lock() {
        manager.add_message(data);
    }
}

pub fn show_message_with_position(
    content: &str,
    message_type: MessageType,
    position: MessagePosition,
) {
    let data = SimpleMessageData {
        id: generate_id("message"),
        content: content.to_string(),
        message_type,
        position,
        duration: 2000,
        closable: true,
        show_icon: true,
        seq: NEXT_SEQ.fetch_add(1, Ordering::Relaxed),
    };
    if let Ok(mut manager) = get_global_message_manager().lock() {
        manager.add_message(data);
    }
}

pub fn close_message(id: &str) {
    if let Ok(mut manager) = get_global_message_manager().lock() {
        manager.remove_message(id);
    }
}

pub fn close_all_messages() {
    if let Ok(mut manager) = get_global_message_manager().lock() {
        manager.clear_all();
    }
}

fn generate_id(prefix: &str) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let counter = NEXT_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{}_{}_{}", prefix, timestamp, counter)
}

// 直插 DOM 渲染已移除，统一走全局容器组件渲染
