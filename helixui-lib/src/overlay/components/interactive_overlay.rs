use crate::overlay::core::{AnimationManager, PlatformAdapter, ThemeManager};
use crate::overlay::utils::{ErrorHandler, OverlayError, Result};
use crate::{Button, ButtonShape, ButtonSize, ButtonType};
use dioxus::prelude::*;

/// 交互式 Overlay 基类
#[derive(Props, Clone, PartialEq)]
pub struct InteractiveOverlayProps {
    pub visible: bool,
    pub on_close: Option<fn()>,
    pub mask_closable: bool,
    pub closable: bool,
    pub draggable: bool,
    pub resizable: bool,
    pub z_index: u32,
    pub children: Element,
}

/// 交互式 Overlay 组件
#[component]
pub fn InteractiveOverlay(props: InteractiveOverlayProps) -> Element {
    let platform_adapter = use_signal(|| PlatformAdapter::new());
    let animation_manager = use_signal(|| AnimationManager::new(platform_adapter.read().clone()));
    let theme_manager = use_signal(|| ThemeManager::new());
    let error_handler = use_signal(|| ErrorHandler::new());

    let is_dragging = use_signal(|| false);
    let drag_offset = use_signal(|| (0.0, 0.0));
    let position = use_signal(|| (0.0, 0.0));

    // 获取样式类
    let base_class = theme_manager.read().get_base_class();
    let shadow_class = theme_manager.read().get_shadow_class();
    let mask_class = theme_manager.read().get_mask_class();
    let animation_class = if props.visible {
        animation_manager.read().create_enter_animation()
    } else {
        animation_manager.read().create_exit_animation()
    };

    let merged_class = merge_classes(&[&base_class, &shadow_class, &animation_class]);

    rsx! {
        if props.visible {
            // 遮罩层
            div {
                class: format!("fixed inset-0 z-{} {}", props.z_index - 1, mask_class),
                onclick: move |_| {
                    if props.mask_closable {
                        if let Some(callback) = props.on_close {
                            callback();
                        }
                    }
                },
            }

            // 内容层
            div {
                class: format!("fixed z-{} {}", props.z_index, merged_class),
                style: format!("left: {}px; top: {}px;", position.read().0, position.read().1),

                // 拖拽处理
                if props.draggable {
                    div {
                        class: "cursor-move select-none",
                        onmousedown: move |e| {
                            is_dragging.set(true);
                            // 计算拖拽偏移
                            let rect = e.target().get_bounding_client_rect();
                            let offset_x = e.client_x() as f64 - rect.left();
                            let offset_y = e.client_y() as f64 - rect.top();
                            drag_offset.set((offset_x, offset_y));
                        },
                        "拖拽区域"
                    }
                }

                // 关闭按钮
                if props.closable {
                    Button {
                        button_type: ButtonType::PureText,
                        size: ButtonSize::Small,
                        shape: ButtonShape::Circle,
                        class: Some("absolute top-2 right-2 text-gray-400 hover:text-gray-600".to_string()),
                        onclick: move |_| {
                            if let Some(callback) = props.on_close {
                                callback();
                            }
                        },
                        "×"
                    }
                }

                // 内容
                div {
                    class: "p-4",
                    {props.children}
                }
            }
        }
    }
}

/// 合并类名
fn merge_classes(classes: &[&str]) -> String {
    classes
        .iter()
        .filter(|&&class| !class.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}
