use dioxus::prelude::*;
use super::dialog::{Dialog, DialogSize, DialogPosition, DialogType};

/// Modal 尺寸（基于 DialogSize）
pub type ModalSize = DialogSize;

/// Modal 位置（基于 DialogPosition）
pub type ModalPosition = DialogPosition;

/// Modal 类型（基于 DialogType）
pub type ModalType = DialogType;

/// Modal 属性
#[derive(Props, Clone, PartialEq)]
pub struct ModalProps {
    /// 是否显示模态框
    #[props(default = false)]
    pub visible: bool,
    
    /// 模态框标题
    #[props(default)]
    pub title: Option<String>,
    
    /// 模态框内容
    pub children: Element,
    
    /// 模态框尺寸
    #[props(default = ModalSize::Medium)]
    pub size: ModalSize,
    
    /// 模态框位置
    #[props(default = ModalPosition::Center)]
    pub position: ModalPosition,
    
    /// 模态框类型
    #[props(default = ModalType::Default)]
    pub modal_type: ModalType,
    
    /// 是否显示遮罩层
    #[props(default = true)]
    pub show_mask: bool,
    
    /// 遮罩层点击是否关闭
    #[props(default = true)]
    pub mask_closable: bool,
    
    /// 是否显示关闭按钮
    #[props(default = true)]
    pub closable: bool,
    
    /// 是否显示确认按钮
    #[props(default = false)]
    pub show_confirm: bool,
    
    /// 是否显示取消按钮
    #[props(default = false)]
    pub show_cancel: bool,
    
    /// 确认按钮文本
    #[props(default = "确认".to_string())]
    pub confirm_text: String,
    
    /// 取消按钮文本
    #[props(default = "取消".to_string())]
    pub cancel_text: String,
    
    /// 确认事件
    pub on_confirm: Option<EventHandler<()>>,
    
    /// 取消事件
    pub on_cancel: Option<EventHandler<()>>,
    
    /// 关闭事件
    pub on_close: Option<EventHandler<()>>,
    
    /// 是否可拖拽
    #[props(default = false)]
    pub draggable: bool,
    
    /// 变换原点
    #[props(default = "center".to_string())]
    pub transform_origin: String,
    
    /// 自定义样式类
    pub class: Option<String>,
}

#[component]
pub fn Modal(props: ModalProps) -> Element {
    let handle_close = move || {
        if let Some(on_close) = &props.on_close {
            on_close.call(());
        }
    };

    let handle_confirm = move || {
        if let Some(on_confirm) = &props.on_confirm {
            on_confirm.call(());
        }
        handle_close();
    };

    let handle_cancel = move || {
        if let Some(on_cancel) = &props.on_cancel {
            on_cancel.call(());
        }
        handle_close();
    };

    rsx! {
        Dialog {
            visible: props.visible,
            title: props.title.clone(),
            size: props.size,
            position: props.position,
            dialog_type: props.modal_type,
            show_mask: props.show_mask,
            mask_closable: props.mask_closable,
            closable: props.closable,
            draggable: props.draggable,
            transform_origin: props.transform_origin.clone(),
            on_close: props.on_close.clone(),
            class: props.class.clone(),
            
            div { class: "space-y-4",
                // 内容
                {props.children}
                
                // 按钮区域
                if props.show_confirm || props.show_cancel {
                    div { class: "flex justify-end space-x-3 pt-4 border-t border-gray-200 dark:border-gray-700",
                        if props.show_cancel {
                            button {
                                class: "px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 dark:bg-gray-800 dark:text-gray-300 dark:border-gray-600 dark:hover:bg-gray-700",
                                onclick: move |_| handle_cancel(),
                                "{props.cancel_text}"
                            }
                        }
                        
                        if props.show_confirm {
                            button {
                                class: "px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700 dark:bg-blue-500 dark:hover:bg-blue-600",
                                onclick: move |_| handle_confirm(),
                                "{props.confirm_text}"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Modal 管理器（简化版本）
#[derive(Clone)]
pub struct ModalManager {
    visible: Signal<bool>,
}

impl ModalManager {
    pub fn new() -> Self {
        Self {
            visible: use_signal(|| false),
        }
    }

    pub fn show(&mut self) {
        self.visible.set(true);
    }

    pub fn hide(&mut self) {
        self.visible.set(false);
    }

    pub fn is_visible(&self) -> bool {
        *self.visible.read()
    }
}

/// 命令式 Modal（简化版本）
pub struct ImperativeModal {
    _id: u32,
}

impl ImperativeModal {
    pub fn new(_id: u32) -> Self {
        Self { _id }
    }

    pub fn show(_title: &str, _content: &str, _id: u32) {
        // 在实际应用中，这里会显示模态框
    }

    pub fn hide(_id: u32) {
        // 在实际应用中，这里会隐藏模态框
    }
}

/// 全局 Modal 容器（简化版本）
#[component]
pub fn GlobalModalContainer() -> Element {
    rsx! {
        div { class: "modal-container" }
    }
}