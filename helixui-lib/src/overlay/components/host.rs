use crate::overlay::components::base::{BaseOverlay, BaseOverlayProps, OverlayPosition};
use crate::overlay::core::animation_manager::FillMode;
use crate::overlay::core::{get_global_state_manager, OverlayType};
use crate::overlay::core::{AnimationConfig, AnimationType, EasingType, ThemeMode};
use dioxus::prelude::*;

/// 全局 Overlay 宿主：集中渲染所有覆盖层
#[component]
pub fn GlobalOverlayHost() -> Element {
    // 读取可见 overlays 的快照，避免长持有借用
    let overlays = {
        if let Ok(manager) = get_global_state_manager().lock() {
            manager.get_visible_overlays()
        } else {
            Vec::new()
        }
    };

    rsx! {
        // 占位容器
        div { class: "fixed inset-0 pointer-events-none z-[1000]",
            // 使用通用 BaseOverlay 渲染
            for state in overlays {
                div { key: "{state.id}", class: "pointer-events-auto",
                    match state.component_type {
                        OverlayType::Message => {
                            // 创建简单的 Message Overlay
                            let props = BaseOverlayProps {
                                visible: true,
                                z_index: state.z_index,
                                position: OverlayPosition::TopRight,
                                animation: Some(AnimationConfig {
                                    duration: 200,
                                    delay: 0,
                                    easing: EasingType::EaseOut,
                                    enter: AnimationType::FadeIn,
                                    exit: AnimationType::FadeOut,
                                    fill_mode: FillMode::Forwards,
                                    iteration_count: 1,
                                }),
                                theme_mode: Some(ThemeMode::Auto),
                                mask_closable: false,
                                closable: true,
                                draggable: false,
                                resizable: false,
                                on_close: None,
                                on_show: None,
                                on_hide: None,
                                children: rsx! {
                                    div { class: "p-4 rounded-lg shadow-lg border max-w-sm pointer-events-auto bg-white",
                                        "Message: {state.data}"
                                    }
                                },
                            };
                            BaseOverlay(props)
                        }
                        OverlayType::Dialog => {
                            // 创建简单的 Dialog Overlay
                            let props = BaseOverlayProps {
                                visible: true,
                                z_index: state.z_index,
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
                                on_close: None,
                                on_show: None,
                                on_hide: None,
                                children: rsx! {
                                    div { class: "bg-white rounded-lg shadow-xl max-w-md w-full mx-4",
                                        "Dialog: {state.data}"
                                    }
                                },
                            };
                            BaseOverlay(props)
                        }
                        _ => rsx!()
                    }
                }
            }
        }
    }
}
