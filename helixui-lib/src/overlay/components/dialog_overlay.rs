use crate::overlay::components::InteractiveOverlay;
use crate::{Button, ButtonShape, ButtonSize, ButtonType, ButtonVariant};
use dioxus::prelude::*;

/// 对话框 Overlay 组件
#[derive(Props, Clone, PartialEq)]
pub struct DialogOverlayProps {
    pub visible: bool,
    pub on_close: Option<fn()>,
    pub title: Option<String>,
    pub confirm_text: Option<String>,
    pub cancel_text: Option<String>,
    pub on_confirm: Option<fn()>,
    pub on_cancel: Option<fn()>,
    pub children: Element,
}

/// 对话框 Overlay 组件
#[component]
pub fn DialogOverlay(props: DialogOverlayProps) -> Element {
    rsx! {
        InteractiveOverlay {
            visible: props.visible,
            on_close: props.on_close,
            mask_closable: true,
            closable: true,
            draggable: false,
            resizable: false,
            z_index: 4000,

            div {
                class: "bg-white rounded-lg shadow-xl max-w-md w-full mx-4",

                // 标题
                if let Some(title) = &props.title {
                    div {
                        class: "px-6 py-4 border-b border-gray-200",
                        h3 {
                            class: "text-lg font-medium text-gray-900",
                            {title}
                        }
                    }
                }

                // 内容
                div {
                    class: "px-6 py-4",
                    {props.children}
                }

                // 底部按钮
                div {
                    class: "px-6 py-4 border-t border-gray-200 flex justify-end space-x-2",

                    // 取消按钮
                    if let Some(cancel_text) = &props.cancel_text {
                        Button {
                            button_type: ButtonType::Default,
                            size: ButtonSize::Small,
                            variant: ButtonVariant::Text,
                            class: Some("text-sm".to_string()),
                            onclick: move |_| {
                                if let Some(callback) = props.on_cancel {
                                    callback();
                                }
                            },
                            {cancel_text}
                        }
                    }

                    // 确认按钮
                    if let Some(confirm_text) = &props.confirm_text {
                        Button {
                            button_type: ButtonType::Primary,
                            size: ButtonSize::Small,
                            variant: ButtonVariant::Text,
                            onclick: move |_| {
                                if let Some(callback) = props.on_confirm {
                                    callback();
                                }
                            },
                            {confirm_text}
                        }
                    }
                }
            }
        }
    }
}
