use crate::{Button, ButtonShape, ButtonSize, ButtonType, ButtonVariant, Icon, IconType};
use dioxus::prelude::*;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

/// Dialog 尺寸
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DialogSize {
    /// 小型对话框
    Small,
    /// 中型对话框（默认）
    Medium,
    /// 大型对话框
    Large,
    /// 全屏对话框
    Fullscreen,
}

/// Dialog 位置
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DialogPosition {
    /// 顶部
    Top,
    /// 居中（默认）
    Center,
    /// 底部
    Bottom,
    /// 左侧
    Left,
    /// 右侧
    Right,
}

/// Dialog 类型（基于新的 SimpleDialogType）
#[derive(Clone, Debug, PartialEq)]
pub enum DialogType {
    Info,
    Success,
    Warning,
    Error,
    Confirm,
}

/// Dialog 属性
#[derive(Props, Clone, PartialEq)]
pub struct DialogProps {
    /// 是否显示对话框
    #[props(default = false)]
    pub visible: bool,

    /// 对话框标题
    #[props(default)]
    pub title: Option<String>,

    /// 对话框内容
    pub children: Element,

    /// 对话框尺寸
    #[props(default = DialogSize::Medium)]
    pub size: DialogSize,

    /// 对话框位置
    #[props(default = DialogPosition::Center)]
    pub position: DialogPosition,

    /// 对话框类型
    #[props(default = DialogType::Info)]
    pub dialog_type: DialogType,

    /// 是否显示遮罩层
    #[props(default = true)]
    pub show_mask: bool,

    /// 遮罩层点击是否关闭
    #[props(default = true)]
    pub mask_closable: bool,

    /// 是否显示关闭按钮
    #[props(default = true)]
    pub closable: bool,

    /// 是否可拖拽
    #[props(default = false)]
    pub draggable: bool,

    /// 关闭事件
    pub on_close: Option<EventHandler<()>>,

    /// 确认事件（仅 Confirm 类型）
    pub on_confirm: Option<EventHandler<()>>,

    /// 取消事件（仅 Confirm 类型）
    pub on_cancel: Option<EventHandler<()>>,

    /// 自定义样式类
    pub class: Option<String>,
}

#[component]
pub fn Dialog(props: DialogProps) -> Element {
    // 直接渲染受控对话框（仅作为受控组件使用）
    if !props.visible {
        return rsx! { div {} };
    }
    let type_class = match props.dialog_type {
        DialogType::Info => "border-blue-200 dark:border-blue-700",
        DialogType::Success => "border-green-200 dark:border-green-700",
        DialogType::Warning => "border-yellow-200 dark:border-yellow-700",
        DialogType::Error => "border-red-200 dark:border-red-700",
        DialogType::Confirm => "",
    };

    rsx! {
        // 遮罩层
        if props.show_mask {
            div { class: "fixed inset-0 bg-black bg-opacity-50 z-40",
                onclick: move |_| {
                    if props.mask_closable {
                        if let Some(on_close) = &props.on_close { on_close.call(()); }
                    }
                }
            }
        }

        // 对话框内容
        div { class: "fixed inset-0 flex items-center justify-center z-50",
            div { class: "bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full mx-4 border-2 {type_class}",
                onclick: move |e: MouseEvent| { e.stop_propagation(); },

                if props.title.is_some() || props.closable {
                    div { class: "flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700",
                        if let Some(title) = &props.title {
                            h3 { class: "text-lg font-semibold text-gray-900 dark:text-white", "{title}" }
                        }
                        if props.closable {
                            Button { button_type: ButtonType::PureText, size: ButtonSize::Small, variant: ButtonVariant::Text, shape: ButtonShape::Circle,
                                class: Some("text-gray-400 hover:text-gray-600 dark:hover:text-gray-300".to_string()),
                                onclick: move |_| { if let Some(on_close) = &props.on_close { on_close.call(()); } },
                                Icon { icon: IconType::Close, class: "w-4 h-4".to_string() }
                            }
                        }
                    }
                }

                div { class: "p-4", {props.children} }

                div { class: "flex justify-end space-x-2 p-4 border-t border-gray-200 dark:border-gray-700",
                    if props.dialog_type == DialogType::Confirm {
                            Button { button_type: ButtonType::Default, size: ButtonSize::Small, variant: ButtonVariant::Text, class: Some("text-sm".to_string()),
                            onclick: move |_| { if let Some(on_cancel) = &props.on_cancel { on_cancel.call(()); } },
                                Icon { icon: IconType::Close, class: "w-4 h-4".to_string() }
                        }
                        Button { button_type: ButtonType::Primary, size: ButtonSize::Small, variant: ButtonVariant::Text,
                            onclick: move |_| { if let Some(on_confirm) = &props.on_confirm { on_confirm.call(()); } },
                            "确认"
                        }
                    } else {
                        Button { button_type: ButtonType::Primary, size: ButtonSize::Small, variant: ButtonVariant::Text,
                            onclick: move |_| { if let Some(on_close) = &props.on_close { on_close.call(()); } },
                            Icon { icon: IconType::Close, class: "w-4 h-4".to_string() }
                        }
                    }
                }
            }
        }
    }
}

/// 内置命令式简化 Dialog 系统（保持 API 与原始 simple_overlay 一致）
#[derive(Clone, Debug, PartialEq)]
pub struct SimpleDialogData {
    pub id: String,
    pub title: Option<String>,
    pub content: String,
    pub dialog_type: DialogType,
    pub visible: bool,
    pub on_confirm: Option<fn()>,
    pub on_cancel: Option<fn()>,
}

pub struct SimpleDialogManager {
    dialogs: HashMap<String, SimpleDialogData>,
}

impl SimpleDialogManager {
    pub fn new() -> Self {
        Self {
            dialogs: HashMap::new(),
        }
    }
    pub fn show_dialog(&mut self, mut data: SimpleDialogData) -> String {
        let id = generate_id("dialog");
        data.id = id.clone();
        data.visible = true;
        self.dialogs.insert(id.clone(), data);
        id
    }
    pub fn hide_dialog(&mut self, id: &str) {
        if let Some(d) = self.dialogs.get_mut(id) {
            d.visible = false;
        }
    }
    pub fn remove_dialog(&mut self, id: &str) {
        self.dialogs.remove(id);
    }
    pub fn get_dialogs(&self) -> Vec<SimpleDialogData> {
        self.dialogs.values().cloned().collect()
    }
    pub fn clear_all(&mut self) {
        self.dialogs.clear();
    }
}

static DIALOG_MANAGER: LazyLock<Mutex<SimpleDialogManager>> =
    LazyLock::new(|| Mutex::new(SimpleDialogManager::new()));

pub fn show_info_dialog(title: String, content: String) {
    let data = SimpleDialogData {
        id: String::new(),
        title: Some(title),
        content,
        dialog_type: DialogType::Info,
        visible: false,
        on_confirm: None,
        on_cancel: None,
    };
    if let Ok(mut m) = DIALOG_MANAGER.lock() {
        m.show_dialog(data);
    }
}
pub fn show_success_dialog(title: String, content: String) {
    let data = SimpleDialogData {
        id: String::new(),
        title: Some(title),
        content,
        dialog_type: DialogType::Success,
        visible: false,
        on_confirm: None,
        on_cancel: None,
    };
    if let Ok(mut m) = DIALOG_MANAGER.lock() {
        m.show_dialog(data);
    }
}
pub fn show_warning_dialog(title: String, content: String) {
    let data = SimpleDialogData {
        id: String::new(),
        title: Some(title),
        content,
        dialog_type: DialogType::Warning,
        visible: false,
        on_confirm: None,
        on_cancel: None,
    };
    if let Ok(mut m) = DIALOG_MANAGER.lock() {
        m.show_dialog(data);
    }
}
pub fn show_error_dialog(title: String, content: String) {
    let data = SimpleDialogData {
        id: String::new(),
        title: Some(title),
        content,
        dialog_type: DialogType::Error,
        visible: false,
        on_confirm: None,
        on_cancel: None,
    };
    if let Ok(mut m) = DIALOG_MANAGER.lock() {
        m.show_dialog(data);
    }
}
pub fn show_confirm_dialog(title: String, content: String, on_confirm: fn(), on_cancel: fn()) {
    let data = SimpleDialogData {
        id: String::new(),
        title: Some(title),
        content,
        dialog_type: DialogType::Confirm,
        visible: false,
        on_confirm: Some(on_confirm),
        on_cancel: Some(on_cancel),
    };
    if let Ok(mut m) = DIALOG_MANAGER.lock() {
        m.show_dialog(data);
    }
}
pub fn close_dialog(id: &str) {
    if let Ok(mut m) = DIALOG_MANAGER.lock() {
        m.hide_dialog(id);
    }
}
pub fn close_all_dialogs() {
    if let Ok(mut m) = DIALOG_MANAGER.lock() {
        m.clear_all();
    }
}

#[component]
pub fn SimpleDialog(data: SimpleDialogData) -> Element {
    let id = data.id.clone();
    let title = data.title.clone();
    let content = data.content.clone();
    let dialog_type = data.dialog_type.clone();
    let on_confirm = data.on_confirm;
    let on_cancel = data.on_cancel;

    let type_class = match dialog_type {
        DialogType::Info => "border-blue-200 dark:border-blue-700",
        DialogType::Success => "border-green-200 dark:border-green-700",
        DialogType::Warning => "border-yellow-200 dark:border-yellow-700",
        DialogType::Error => "border-red-200 dark:border-red-700",
        DialogType::Confirm => "",
    };

    rsx! {
        if data.visible {
            div { class: "fixed inset-0 bg-black bg-opacity-50 z-40",
                onclick: { let id = id.clone(); move |_| { close_dialog(&id); } },
            }
            div { class: "fixed inset-0 flex items-center justify-center z-50",
                div { class: "bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full mx-4 border-2 {type_class}",
                    onclick: move |e: MouseEvent| { e.stop_propagation(); },
                    if title.is_some() {
                        div { class: "flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700",
                            h3 { class: "text-lg font-semibold text-gray-900 dark:text-white", "{title.as_ref().unwrap_or(&String::new())}" }
                            Button { button_type: ButtonType::PureText, size: ButtonSize::Small, variant: ButtonVariant::Text, shape: ButtonShape::Circle,
                                class: Some("text-gray-400 hover:text-gray-600 dark:hover:text-gray-300".to_string()),
                                onclick: { let id = id.clone(); move |_| { close_dialog(&id); } },
                                "×"
                            }
                        }
                    }
                    div { class: "p-4", "{content}" }
                    div { class: "flex justify-end space-x-2 p-4 border-t border-gray-200 dark:border-gray-700",
                        if dialog_type == DialogType::Confirm {
                            Button { button_type: ButtonType::Default, size: ButtonSize::Small, variant: ButtonVariant::Text, class: Some("text-sm".to_string()),
                                onclick: { let id = id.clone(); let on_cancel = on_cancel.clone(); move |_| { if let Some(on_cancel) = on_cancel { on_cancel(); } close_dialog(&id); } },
                                "取消"
                            }
                            Button { button_type: ButtonType::Primary, size: ButtonSize::Small, variant: ButtonVariant::Text,
                                onclick: { let id = id.clone(); let on_confirm = on_confirm.clone(); move |_| { if let Some(on_confirm) = on_confirm { on_confirm(); } close_dialog(&id); } },
                                "确认"
                            }
                        } else {
                            Button { button_type: ButtonType::Primary, size: ButtonSize::Small, variant: ButtonVariant::Text,
                                onclick: { let id = id.clone(); move |_| { close_dialog(&id); } },
                                "确定"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn GlobalDialogContainer() -> Element {
    let mut dialogs = use_signal(|| {
        if let Ok(manager) = DIALOG_MANAGER.lock() {
            manager.get_dialogs()
        } else {
            Vec::new()
        }
    });
    use_effect(move || {
        let interval = gloo_timers::callback::Interval::new(150, move || {
            if let Ok(manager) = DIALOG_MANAGER.lock() {
                dialogs.set(manager.get_dialogs());
            }
        });
        interval.forget();
    });
    rsx! {
        div { class: "fixed inset-0 pointer-events-none z-40",
            for dialog in dialogs.read().iter() { SimpleDialog { data: dialog.clone() } }
        }
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

// 便捷 API 保持不变（上面已实现）
