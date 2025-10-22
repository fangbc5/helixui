use crate::overlay::components::base::{BaseOverlay, BaseOverlayProps, OverlayPosition};
use crate::overlay::core::animation_manager::FillMode;
use crate::overlay::core::{AnimationConfig, AnimationType, EasingType, ThemeMode};
use crate::{Button, ButtonSize, ButtonType};
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

/// 对话框 Overlay 组件（使用新的 BaseOverlay 架构）
#[component]
pub fn DialogOverlay(props: DialogOverlayProps) -> Element {
    // 创建 BaseOverlay 属性
    let base_props = BaseOverlayProps {
        visible: props.visible,
        z_index: 4000,
        position: OverlayPosition::Center,
        animation: Some(AnimationConfig {
            duration: 300,
            delay: 0,
            easing: EasingType::EaseOut,
            enter: AnimationType::ScaleIn,
            exit: AnimationType::ScaleOut,
            fill_mode: FillMode::Forwards,
            iteration_count: 1,
        }),
        theme_mode: Some(ThemeMode::Auto),
        mask_closable: true,
        closable: true,
        draggable: false,
        resizable: false,
        on_close: props.on_close,
        on_show: None,
        on_hide: None,
        children: rsx! {
            // Dialog 的具体渲染逻辑
            div {
                class: "bg-white rounded-lg shadow-xl max-w-md w-full mx-4",
                onclick: move |e| e.stop_propagation(),

                // 标题
                if let Some(title) = &props.title {
                    div {
                        class: "px-6 py-4 border-b border-gray-200",
                        h3 {
                            class: "text-lg font-medium text-gray-900",
                            {title.clone()}
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
                            class: Some("text-sm".to_string()),
                            onclick: move |_| {
                                if let Some(callback) = props.on_cancel {
                                    callback();
                                }
                            },
                            {cancel_text.clone()}
                        }
                    }

                    // 确认按钮
                    if let Some(confirm_text) = &props.confirm_text {
                        Button {
                            button_type: ButtonType::Primary,
                            size: ButtonSize::Small,
                            onclick: move |_| {
                                if let Some(callback) = props.on_confirm {
                                    callback();
                                }
                            },
                            {confirm_text.clone()}
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
