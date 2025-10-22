use crate::overlay::components::base::{BaseOverlay, BaseOverlayProps, OverlayPosition};
use crate::overlay::core::animation_manager::FillMode;
use crate::overlay::core::{AnimationConfig, AnimationType, EasingType, ThemeMode};
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

/// 模态框 Overlay 组件（使用新的 BaseOverlay 架构）
#[component]
pub fn ModalOverlay(props: ModalOverlayProps) -> Element {
    // 创建 BaseOverlay 属性
    let base_props = BaseOverlayProps {
        visible: props.visible,
        z_index: 3000,
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
            // Modal 的具体渲染逻辑
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

                // 底部
                if let Some(footer) = &props.footer {
                    div {
                        class: "px-6 py-4 border-t border-gray-200",
                        {footer}
                    }
                }
            }
        },
    };

    rsx! {
        BaseOverlay { ..base_props }
    }
}
