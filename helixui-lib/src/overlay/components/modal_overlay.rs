use crate::overlay::components::InteractiveOverlay;
use dioxus::prelude::*;

/// 模态框 Overlay 组件
#[derive(Props, Clone, PartialEq)]
pub struct ModalOverlayProps {
    pub visible: bool,
    pub on_close: Option<fn()>,
    pub title: Option<String>,
    pub footer: Option<Element>,
    pub children: Element,
}

/// 模态框 Overlay 组件
#[component]
pub fn ModalOverlay(props: ModalOverlayProps) -> Element {
    rsx! {
        InteractiveOverlay {
            visible: props.visible,
            on_close: props.on_close,
            mask_closable: true,
            closable: true,
            draggable: false,
            resizable: false,
            z_index: 3000,

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

                // 底部
                if let Some(footer) = &props.footer {
                    div {
                        class: "px-6 py-4 border-t border-gray-200",
                        {footer}
                    }
                }
            }
        }
    }
}
