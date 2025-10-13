use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};

/// 事件类型
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum EventType {
    OverlayShow,
    OverlayHide,
    OverlayClose,
    ZIndexChange,
    PlatformChange,
    AnimationStart,
    AnimationEnd,
    ThemeChange,
}

/// 事件数据
#[derive(Debug, Clone)]
pub struct Event {
    pub event_type: EventType,
    pub data: EventData,
    pub timestamp: u64,
}

/// 事件数据枚举
#[derive(Debug, Clone)]
pub enum EventData {
    OverlayShow {
        overlay_id: String,
        z_index: u32,
    },
    OverlayHide {
        overlay_id: String,
    },
    OverlayClose {
        overlay_id: String,
    },
    ZIndexChange {
        old_z_index: u32,
        new_z_index: u32,
    },
    PlatformChange {
        old_platform: String,
        new_platform: String,
    },
    AnimationStart {
        overlay_id: String,
        animation_type: String,
    },
    AnimationEnd {
        overlay_id: String,
        animation_type: String,
    },
    ThemeChange {
        old_theme: String,
        new_theme: String,
    },
}

/// 事件监听器 trait
pub trait EventListener: Send + Sync {
    fn handle(&self, event: &Event);
    fn get_event_types(&self) -> Vec<EventType>;
}

/// 事件总线
pub struct EventBus {
    listeners: HashMap<EventType, Vec<Arc<dyn EventListener>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    /// 注册事件监听器
    pub fn register(&mut self, listener: Arc<dyn EventListener>) {
        for event_type in listener.get_event_types() {
            self.listeners
                .entry(event_type)
                .or_insert_with(Vec::new)
                .push(listener.clone());
        }
    }

    /// 移除事件监听器
    pub fn unregister(&mut self, event_type: &EventType, listener: Arc<dyn EventListener>) {
        if let Some(listeners) = self.listeners.get_mut(event_type) {
            listeners.retain(|l| !Arc::ptr_eq(l, &listener));
        }
    }

    /// 发送事件
    pub fn emit(&self, event: Event) {
        if let Some(listeners) = self.listeners.get(&event.event_type) {
            for listener in listeners {
                listener.handle(&event);
            }
        }
    }

    /// 发送简单事件
    pub fn emit_simple(&self, event_type: EventType, data: EventData) {
        let event = Event {
            event_type,
            data,
            timestamp: get_current_timestamp(),
        };
        self.emit(event);
    }
}

/// 获取当前时间戳（简化实现）
fn get_current_timestamp() -> u64 {
    // 在实际应用中，这里应该返回真实的时间戳
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// 全局事件总线实例
static GLOBAL_EVENT_BUS: LazyLock<Mutex<EventBus>> = LazyLock::new(|| Mutex::new(EventBus::new()));

/// 获取全局事件总线
pub fn get_global_event_bus() -> &'static Mutex<EventBus> {
    &GLOBAL_EVENT_BUS
}

/// 默认的事件监听器实现
pub struct DefaultEventListener {
    event_types: Vec<EventType>,
    handler: Box<dyn Fn(&Event) + Send + Sync>,
}

impl DefaultEventListener {
    pub fn new<F>(event_types: Vec<EventType>, handler: F) -> Self
    where
        F: Fn(&Event) + Send + Sync + 'static,
    {
        Self {
            event_types,
            handler: Box::new(handler),
        }
    }
}

impl EventListener for DefaultEventListener {
    fn handle(&self, event: &Event) {
        (self.handler)(event);
    }

    fn get_event_types(&self) -> Vec<EventType> {
        self.event_types.clone()
    }
}
