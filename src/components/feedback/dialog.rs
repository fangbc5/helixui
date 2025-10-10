use crate::components::{Button, ButtonType, Icon, IconType};
use dioxus::prelude::*;

/// 对话框类型
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DialogType {
    Default,
    Info,
    Success,
    Warning,
    Error,
}

impl DialogType {
    pub fn icon(&self) -> IconType {
        match self {
            DialogType::Default => IconType::Info,
            DialogType::Info => IconType::Info,
            DialogType::Success => IconType::Success,
            DialogType::Warning => IconType::Warning,
            DialogType::Error => IconType::Error,
        }
    }

    pub fn color_class(&self) -> &'static str {
        match self {
            DialogType::Default => "text-gray-600 dark:text-gray-300",
            DialogType::Info => "text-blue-600 dark:text-blue-400",
            DialogType::Success => "text-green-600 dark:text-green-400",
            DialogType::Warning => "text-orange-600 dark:text-orange-400",
            DialogType::Error => "text-red-600 dark:text-red-400",
        }
    }
}

/// 对话框属性
#[derive(Props, Clone, PartialEq)]
pub struct DialogProps {
    /// 是否显示对话框
    #[props(default = false)]
    pub visible: bool,
    /// 对话框标题
    #[props(default)]
    pub title: Option<String>,
    /// 对话框内容
    pub content: String,
    /// 对话框类型
    #[props(default = DialogType::Default)]
    pub dialog_type: DialogType,
    /// 是否可以通过点击遮罩层关闭
    #[props(default = false)]
    pub mask_closable: bool,
    /// 是否显示关闭按钮
    #[props(default = true)]
    pub closable: bool,
    /// 是否显示确认按钮
    #[props(default = true)]
    pub show_confirm: bool,
    /// 是否显示取消按钮
    #[props(default = true)]
    pub show_cancel: bool,
    /// 确认按钮文本
    #[props(default = "确认".to_string())]
    pub confirm_text: String,
    /// 取消按钮文本
    #[props(default = "取消".to_string())]
    pub cancel_text: String,
    /// 确认按钮类型
    #[props(default = ButtonType::Primary)]
    pub confirm_type: ButtonType,
    /// 取消按钮类型
    #[props(default = ButtonType::Default)]
    pub cancel_type: ButtonType,
    /// 确认回调
    #[props(default)]
    pub on_confirm: Option<EventHandler<()>>,
    /// 取消回调
    #[props(default)]
    pub on_cancel: Option<EventHandler<()>>,
    /// 关闭回调
    #[props(default)]
    pub on_close: Option<EventHandler<()>>,
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,
}

/// 对话框组件
#[component]
pub fn Dialog(props: DialogProps) -> Element {
    let DialogProps {
        visible,
        title,
        content,
        dialog_type,
        mask_closable,
        closable,
        show_confirm,
        show_cancel,
        confirm_text,
        cancel_text,
        confirm_type,
        cancel_type,
        on_confirm,
        on_cancel,
        on_close,
        class,
    } = props;

    // 处理遮罩层点击
    let handle_mask_click = move |_| {
        if mask_closable {
            if let Some(handler) = &on_close {
                handler.call(());
            }
        }
    };

    // 处理确认按钮点击
    let handle_confirm = move |_| {
        if let Some(handler) = &on_confirm {
            handler.call(());
        }
        // 点击确认后总是关闭对话框
        if let Some(handler) = &on_close {
            handler.call(());
        }
    };

    // 处理取消按钮点击
    let handle_cancel = move |_| {
        if let Some(handler) = &on_cancel {
            handler.call(());
        }
        // 点击取消后总是关闭对话框
        if let Some(handler) = &on_close {
            handler.call(());
        }
    };

    // 处理关闭按钮点击
    let handle_close = move |_| {
        if let Some(handler) = &on_close {
            handler.call(());
        }
    };

    if !visible {
        return rsx! { div {} };
    }

    let dialog_class = if let Some(custom_class) = class {
        format!(
            "fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-auto {}",
            custom_class
        )
    } else {
        "fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-auto".to_string()
    };

    rsx! {
        div {
            class: dialog_class,

            // 遮罩层
            div {
                class: "absolute inset-0 bg-black bg-opacity-30 transition-opacity pointer-events-auto",
                onclick: handle_mask_click,
            }

            // 对话框内容
            div {
                class: "relative bg-white dark:bg-gray-800 rounded-lg shadow-xl w-full max-w-md max-h-[90vh] overflow-hidden pointer-events-auto",

                // 内容区域
                div {
                    class: "p-6",

                    // 标题和图标
                    if title.is_some() || dialog_type != DialogType::Default {
                        div {
                            class: "flex items-center gap-3 mb-4",
                            // 图标
                            if dialog_type != DialogType::Default {
                                Icon {
                                    icon: dialog_type.icon(),
                                    class: format!("w-5 h-5 {}", dialog_type.color_class()),
                                }
                            }
                            // 标题
                            if let Some(title_text) = title {
                                h3 {
                                    class: "text-lg font-semibold text-gray-900 dark:text-white",
                                    "{title_text}"
                                }
                            }
                        }
                    }

                    // 内容文本
                    div {
                        class: "text-sm text-gray-700 dark:text-gray-300 leading-relaxed mb-6",
                        "{content}"
                    }

                    // 按钮区域
                    if show_confirm || show_cancel {
                        div {
                            class: "flex justify-end gap-3",

                            // 取消按钮
                            if show_cancel {
                                Button {
                                    button_type: cancel_type,
                                    onclick: handle_cancel,
                                    "{cancel_text}"
                                }
                            }

                            // 确认按钮
                            if show_confirm {
                                Button {
                                    button_type: confirm_type,
                                    onclick: handle_confirm,
                                    "{confirm_text}"
                                }
                            }
                        }
                    }

                    // 关闭按钮
                    if closable {
                        button {
                            class: "absolute top-4 right-4 p-1 rounded-full hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors",
                            onclick: handle_close,
                            Icon {
                                icon: IconType::Close,
                                class: "w-5 h-5".to_string(),
                            }
                        }
                    }
                }
            }
        }
    }
}

/// 对话框管理器
#[derive(Clone)]
pub struct DialogManager {
    dialogs: Signal<Vec<DialogData>>,
    next_id: Signal<u32>,
}

#[derive(Clone, PartialEq)]
pub struct DialogData {
    pub id: u32,
    pub visible: bool,
    pub title: Option<String>,
    pub content: String,
    pub dialog_type: DialogType,
    pub mask_closable: bool,
    pub closable: bool,
    pub show_confirm: bool,
    pub show_cancel: bool,
    pub confirm_text: String,
    pub cancel_text: String,
    pub confirm_type: ButtonType,
    pub cancel_type: ButtonType,
    pub on_confirm: Option<EventHandler<()>>,
    pub on_cancel: Option<EventHandler<()>>,
    pub on_close: Option<EventHandler<()>>,
}

impl DialogManager {
    pub fn new() -> Self {
        Self {
            dialogs: Signal::new(vec![]),
            next_id: Signal::new(1),
        }
    }

    pub fn show(&mut self, dialog: DialogData) -> u32 {
        let id = *self.next_id.read();
        *self.next_id.write() = id + 1;

        let mut dialogs = self.dialogs.write();
        dialogs.push(dialog);
        id
    }

    pub fn close(&mut self, id: u32) {
        let mut dialogs = self.dialogs.write();
        dialogs.retain(|d| d.id != id);
    }

    pub fn close_all(&mut self) {
        let mut dialogs = self.dialogs.write();
        for dialog in dialogs.iter_mut() {
            dialog.visible = false;
        }
    }

    pub fn remove(&mut self, id: u32) {
        let mut dialogs = self.dialogs.write();
        dialogs.retain(|d| d.id != id);
    }

    pub fn get_dialogs(&self) -> Vec<DialogData> {
        self.dialogs.read().clone()
    }
}

/// 全局对话框管理器
pub static DIALOG_MANAGER: GlobalSignal<DialogManager> = Signal::global(DialogManager::new);

/// 全局对话框容器组件
#[component]
pub fn GlobalDialogContainer() -> Element {
    let manager = DIALOG_MANAGER.signal();
    let dialogs = manager.read().get_dialogs();

    rsx! {
        div {
            class: "fixed inset-0 z-50 pointer-events-none",

            for dialog in dialogs {
                if dialog.visible {
                    Dialog {
                        visible: dialog.visible,
                        title: dialog.title.clone(),
                        content: dialog.content.clone(),
                        dialog_type: dialog.dialog_type,
                        mask_closable: dialog.mask_closable,
                        closable: dialog.closable,
                        show_confirm: dialog.show_confirm,
                        show_cancel: dialog.show_cancel,
                        confirm_text: dialog.confirm_text.clone(),
                        cancel_text: dialog.cancel_text.clone(),
                        confirm_type: dialog.confirm_type,
                        cancel_type: dialog.cancel_type,
                        on_confirm: dialog.on_confirm.clone(),
                        on_cancel: dialog.on_cancel.clone(),
                        on_close: {
                            let dialog_id = dialog.id;
                            move |_| {
                                DIALOG_MANAGER.write().close(dialog_id);
                            }
                        },
                    }
                }
            }
        }
    }
}

/// 便捷函数：显示确认对话框
pub fn show_confirm_dialog(
    title: Option<String>,
    content: String,
    on_confirm: Option<EventHandler<()>>,
    on_cancel: Option<EventHandler<()>>,
) -> u32 {
    let dialog = DialogData {
        id: 0, // 会被管理器重新分配
        visible: true,
        title,
        content,
        dialog_type: DialogType::Default,
        mask_closable: false,
        closable: true,
        show_confirm: true,
        show_cancel: true,
        confirm_text: "确认".to_string(),
        cancel_text: "取消".to_string(),
        confirm_type: ButtonType::Primary,
        cancel_type: ButtonType::Default,
        on_confirm,
        on_cancel,
        on_close: None, // 将在 GlobalDialogContainer 中处理
    };

    DIALOG_MANAGER.write().show(dialog)
}

/// 便捷函数：显示信息对话框
pub fn show_info_dialog(
    title: Option<String>,
    content: String,
    on_confirm: Option<EventHandler<()>>,
) -> u32 {
    let dialog = DialogData {
        id: 0,
        visible: true,
        title,
        content,
        dialog_type: DialogType::Info,
        mask_closable: false,
        closable: true,
        show_confirm: true,
        show_cancel: false,
        confirm_text: "知道了".to_string(),
        cancel_text: "取消".to_string(),
        confirm_type: ButtonType::Primary,
        cancel_type: ButtonType::Default,
        on_confirm,
        on_cancel: None,
        on_close: None,
    };

    DIALOG_MANAGER.write().show(dialog)
}

/// 便捷函数：显示成功对话框
pub fn show_success_dialog(
    title: Option<String>,
    content: String,
    on_confirm: Option<EventHandler<()>>,
) -> u32 {
    let dialog = DialogData {
        id: 0,
        visible: true,
        title,
        content,
        dialog_type: DialogType::Success,
        mask_closable: false,
        closable: true,
        show_confirm: true,
        show_cancel: false,
        confirm_text: "知道了".to_string(),
        cancel_text: "取消".to_string(),
        confirm_type: ButtonType::Success,
        cancel_type: ButtonType::Default,
        on_confirm,
        on_cancel: None,
        on_close: None,
    };

    DIALOG_MANAGER.write().show(dialog)
}

/// 便捷函数：显示警告对话框
pub fn show_warning_dialog(
    title: Option<String>,
    content: String,
    on_confirm: Option<EventHandler<()>>,
    on_cancel: Option<EventHandler<()>>,
) -> u32 {
    let dialog = DialogData {
        id: 0,
        visible: true,
        title,
        content,
        dialog_type: DialogType::Warning,
        mask_closable: false,
        closable: true,
        show_confirm: true,
        show_cancel: true,
        confirm_text: "确认".to_string(),
        cancel_text: "取消".to_string(),
        confirm_type: ButtonType::Warning,
        cancel_type: ButtonType::Default,
        on_confirm,
        on_cancel,
        on_close: None,
    };

    DIALOG_MANAGER.write().show(dialog)
}

/// 便捷函数：显示错误对话框
pub fn show_error_dialog(
    title: Option<String>,
    content: String,
    on_confirm: Option<EventHandler<()>>,
) -> u32 {
    let dialog = DialogData {
        id: 0,
        visible: true,
        title,
        content,
        dialog_type: DialogType::Error,
        mask_closable: false,
        closable: true,
        show_confirm: true,
        show_cancel: false,
        confirm_text: "知道了".to_string(),
        cancel_text: "取消".to_string(),
        confirm_type: ButtonType::Error,
        cancel_type: ButtonType::Default,
        on_confirm,
        on_cancel: None,
        on_close: None,
    };

    DIALOG_MANAGER.write().show(dialog)
}

/// 便捷函数：关闭对话框
pub fn close_dialog(id: u32) {
    DIALOG_MANAGER.write().close(id);
}

/// 便捷函数：关闭所有对话框
pub fn close_all_dialogs() {
    DIALOG_MANAGER.write().close_all();
}
