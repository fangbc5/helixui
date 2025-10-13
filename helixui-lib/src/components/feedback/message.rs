use crate::overlay::config::{SimpleMessagePosition, SimpleMessageType};
use crate::{Button, ButtonShape, ButtonSize, ButtonType, ButtonVariant, Icon, IconSize, IconType};
use dioxus::prelude::*;
use std::collections::HashMap;
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
    }
    pub fn remove_message(&mut self, id: &str) {
        self.messages.remove(id);
    }
    pub fn get_messages(&self) -> Vec<SimpleMessageData> {
        self.messages.values().cloned().collect()
    }
    pub fn clear_all(&mut self) {
        self.messages.clear();
    }
}

static GLOBAL_MESSAGE_MANAGER: LazyLock<Mutex<SimpleMessageManager>> =
    LazyLock::new(|| Mutex::new(SimpleMessageManager::new()));

fn get_global_message_manager() -> &'static Mutex<SimpleMessageManager> {
    &GLOBAL_MESSAGE_MANAGER
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
                gloo_timers::future::TimeoutFuture::new(duration).await;
                if let Ok(mut manager) = get_global_message_manager().lock() {
                    manager.remove_message(&id);
                }
            });
        }
    });

    let base_class = "fixed p-4 rounded-lg shadow-lg border max-w-sm";
    let position_class = get_position_class(&data.position);
    let type_class = get_type_class(&data.message_type);
    let merged_class = format!("{} {} {}", base_class, position_class, type_class);

    rsx! {
        div { class: merged_class, role: "alert", "aria-live": "polite",
            div { class: "flex items-center",
                if data.show_icon {
                    div { class: "flex-shrink-0 mr-2", {get_icon(&data.message_type)} }
                }
                div { class: "flex-1", {data.content} }
                if data.closable {
                    Button {
                        button_type: ButtonType::PureIcon,
                        size: ButtonSize::Small,
                        variant: ButtonVariant::Text,
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

#[component]
pub fn GlobalMessageContainer() -> Element {
    let mut messages = use_signal(|| Vec::<SimpleMessageData>::new());
    use_effect(move || {
        let interval = gloo_timers::callback::Interval::new(150, move || {
            if let Ok(manager) = get_global_message_manager().lock() {
                let new_messages = manager.get_messages();
                messages.set(new_messages);
            }
        });
        interval.forget();
    });
    rsx! {
        div { class: "fixed inset-0 pointer-events-none z-50",
            for message in messages.read().iter() { SimpleMessage { data: message.clone() } }
        }
    }
}

pub fn show_message(content: &str, message_type: MessageType) {
    #[cfg(target_arch = "wasm32")]
    {
        if web_render_message(
            content,
            &message_type,
            &SimpleMessagePosition::TopRight,
            2000,
        )
        .is_ok()
        {
            return;
        }
    }
    let data = SimpleMessageData {
        id: generate_id("message"),
        content: content.to_string(),
        message_type,
        position: SimpleMessagePosition::TopRight,
        duration: 2000,
        closable: true,
        show_icon: true,
    };
    if let Ok(mut manager) = get_global_message_manager().lock() {
        manager.add_message(data);
    }
}

pub fn show_message_with_duration(content: &str, message_type: MessageType, duration: u32) {
    #[cfg(target_arch = "wasm32")]
    {
        if web_render_message(
            content,
            &message_type,
            &SimpleMessagePosition::TopRight,
            duration,
        )
        .is_ok()
        {
            return;
        }
    }
    let data = SimpleMessageData {
        id: generate_id("message"),
        content: content.to_string(),
        message_type,
        position: SimpleMessagePosition::TopRight,
        duration,
        closable: true,
        show_icon: true,
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
    #[cfg(target_arch = "wasm32")]
    {
        if web_render_message(content, &message_type, &position, 2000).is_ok() {
            return;
        }
    }
    let data = SimpleMessageData {
        id: generate_id("message"),
        content: content.to_string(),
        message_type,
        position,
        duration: 2000,
        closable: true,
        show_icon: true,
    };
    if let Ok(mut manager) = get_global_message_manager().lock() {
        manager.add_message(data);
    }
}

pub fn close_message(id: u32) {
    if let Ok(mut manager) = get_global_message_manager().lock() {
        manager.remove_message(&id.to_string());
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
    format!("{}_{}", prefix, timestamp)
}

#[cfg(target_arch = "wasm32")]
fn web_render_message(
    content: &str,
    message_type: &SimpleMessageType,
    position: &SimpleMessagePosition,
    duration: u32,
) -> Result<(), ()> {
    use wasm_bindgen::JsCast;
    use web_sys::{window, Document, Element};
    let win = window().ok_or(())?;
    let doc: Document = win.document().ok_or(())?;
    let root_id = "helixui-message-root";
    let root_el: Element = if let Some(el) = doc.get_element_by_id(root_id) {
        el
    } else {
        let el = doc.create_element("div").map_err(|_| ())?;
        el.set_id(root_id);
        el.set_attribute(
            "style",
            "position:fixed;inset:0;pointer-events:none;z-index:9999;",
        )
        .ok();
        doc.body().ok_or(())?.append_child(&el).map_err(|_| ())?;
        el
    };
    let msg = doc.create_element("div").map_err(|_| ())?;
    let base = "pointer-events-auto p-4 rounded-lg shadow-lg border max-w-sm text-sm";
    let type_class = match message_type {
        SimpleMessageType::Success => "bg-green-50 border-green-200 text-green-800",
        SimpleMessageType::Warning => "bg-yellow-50 border-yellow-200 text-yellow-800",
        SimpleMessageType::Error => "bg-red-50 border-red-200 text-red-800",
        SimpleMessageType::Info | SimpleMessageType::Loading => {
            "bg-blue-50 border-blue-200 text-blue-800"
        }
    };
    let pos_class = match position {
        SimpleMessagePosition::TopLeft => "top:1rem;left:1rem;",
        SimpleMessagePosition::TopCenter => "top:1rem;left:50%;transform:translateX(-50%);",
        SimpleMessagePosition::TopRight => "top:1rem;right:1rem;",
        SimpleMessagePosition::BottomLeft => "bottom:1rem;left:1rem;",
        SimpleMessagePosition::BottomCenter => "bottom:1rem;left:50%;transform:translateX(-50%);",
        SimpleMessagePosition::BottomRight => "bottom:1rem;right:1rem;",
        SimpleMessagePosition::Center => "top:50%;left:50%;transform:translate(-50%,-50%);",
    };
    msg.set_attribute("style", &format!("position:fixed;{}", pos_class))
        .ok();
    msg.set_attribute("class", &format!("{} {}", base, type_class))
        .ok();
    msg.set_text_content(Some(content));

    let close_btn = doc.create_element("button").map_err(|_| ())?;
    close_btn
        .set_attribute(
            "class",
            "ml-2 text-gray-400 hover:text-gray-600 float-right",
        )
        .ok();
    close_btn.set_text_content(Some("×"));
    {
        let msg_clone = msg.clone();
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
            let _ = msg_clone.remove();
        }) as Box<dyn FnMut()>);
        close_btn
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .ok();
        closure.forget();
    }
    let _ = msg.append_child(&close_btn);
    let _ = root_el.append_child(&msg);
    if duration > 0 {
        let msg_clone = msg.clone();
        let cb = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
            let _ = msg_clone.remove();
        }) as Box<dyn FnMut()>);
        let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(
            cb.as_ref().unchecked_ref(),
            duration as i32,
        );
        cb.forget();
    }
    Ok(())
}
