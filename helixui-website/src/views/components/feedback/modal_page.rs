use dioxus::prelude::*;
use helixui::components::{Button, ButtonType, DemoBox, Modal, ModalSize, ModalType};
use crate::views::layout::{ComponentsSidebar, DocPage, TocItem};

#[component]
pub fn ModalPage() -> Element {
    let mut basic_modal = use_signal(|| false);
    let mut size_small = use_signal(|| false);
    let mut size_medium = use_signal(|| false);
    let mut size_large = use_signal(|| false);
    let mut confirm_modal = use_signal(|| false);
    let mut controlled_modal = use_signal(|| false);
    let mut transform_origin_modal = use_signal(|| false);
    let mut no_mask_modal = use_signal(|| false);
    let mut draggable_modal = use_signal(|| false);

    let toc_items = vec![
        TocItem { id: "basic".to_string(), title: "基础用法".to_string(), level: 1 },
        TocItem { id: "controlled".to_string(), title: "受控显示".to_string(), level: 1 },
        TocItem { id: "size".to_string(), title: "不同尺寸".to_string(), level: 1 },
        TocItem { id: "confirm".to_string(), title: "确认对话框".to_string(), level: 1 },
        TocItem { id: "transform".to_string(), title: "变换原点".to_string(), level: 1 },
        TocItem { id: "no-mask".to_string(), title: "不显示遮罩层".to_string(), level: 1 },
        TocItem { id: "draggable".to_string(), title: "可拖拽".to_string(), level: 1 },
        TocItem { id: "api".to_string(), title: "API".to_string(), level: 1 },
    ];

    rsx! {
        DocPage {
            sidebar: rsx! { ComponentsSidebar {} },
            toc_items,

            div { class: "component-doc",
                div { class: "mb-8",
                    h1 { class: "text-4xl font-bold text-gray-900 dark:text-white mb-2", "模态框 Modal" }
                    p { class: "text-lg text-gray-600 dark:text-gray-300", "模态框组件用于显示重要的信息或收集用户输入。" }
                }

                section { id: "basic", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4", "基础用法" }
                    
                    DemoBox {
                        title: "基础模态框".to_string(),
                        description: "最简单的模态框，包含标题和内容。".to_string(),
                        code: r#"use helixui::components::{Button, ButtonType, Modal};

rsx! {
    Button {
        button_type: ButtonType::Primary,
        onclick: move |_| modal_visible.set(true),
        "打开模态框"
    }
    Modal {
        visible: modal_visible(),
        title: Some("基础模态框".to_string()),
        on_close: move |_| modal_visible.set(false),
        "这是一个基础的模态框内容。"
    }
}"#.to_string(),
                        children: rsx! {
                            Button {
                                button_type: ButtonType::Primary,
                                onclick: move |_| basic_modal.set(true),
                                "打开基础模态框"
                            }
                            Modal {
                                visible: basic_modal(),
                                title: Some("基础模态框".to_string()),
                                on_close: move |_| basic_modal.set(false),
                                "这是一个基础的模态框内容。你可以在这里放置任何内容。"
                            }
                        }
                    }
                }

                section { id: "controlled", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4", "受控显示" }
                    
                    DemoBox {
                        title: "受控显示".to_string(),
                        description: "模态框的显示可以是受控的，通过外部状态控制。".to_string(),
                        code: r#"use helixui::components::{Button, Modal};

rsx! {
    Button {
        button_type: ButtonType::Primary,
        onclick: move |_| controlled_modal.set(true),
        "打开受控模态框"
    }
    Modal {
        visible: controlled_modal(),
        title: Some("受控模态框".to_string()),
        on_close: move |_| controlled_modal.set(false),
        "这是一个受控的模态框，显示状态完全由外部控制。"
    }
}"#.to_string(),
                        children: rsx! {
                            Button {
                                button_type: ButtonType::Primary,
                                onclick: move |_| controlled_modal.set(true),
                                "打开受控模态框"
                            }
                            Modal {
                                visible: controlled_modal(),
                                title: Some("受控模态框".to_string()),
                                on_close: move |_| controlled_modal.set(false),
                                "这是一个受控的模态框，显示状态完全由外部控制。"
                            }
                        }
                    }
                }

                section { id: "size", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4", "不同尺寸" }
                    
                    DemoBox {
                        title: "尺寸对比".to_string(),
                        description: "模态框支持小型、中型、大型三种尺寸。".to_string(),
                        code: r#"use helixui::components::{Button, Modal, ModalSize};

rsx! {
    Button { onclick: move |_| small.set(true), "小型" }
    Button { onclick: move |_| medium.set(true), "中型" }
    Button { onclick: move |_| large.set(true), "大型" }
}"#.to_string(),
                        children: rsx! {
                            div { class: "flex gap-4",
                                Button { button_type: ButtonType::Primary, onclick: move |_| size_small.set(true), "小型" }
                                Button { button_type: ButtonType::Primary, onclick: move |_| size_medium.set(true), "中型" }
                                Button { button_type: ButtonType::Primary, onclick: move |_| size_large.set(true), "大型" }
                            }
                            Modal {
                                visible: size_small(),
                                title: Some("小型模态框".to_string()),
                                size: ModalSize::Small,
                                on_close: move |_| size_small.set(false),
                                "这是小型模态框的内容。"
                            }
                            Modal {
                                visible: size_medium(),
                                title: Some("中型模态框".to_string()),
                                size: ModalSize::Medium,
                                on_close: move |_| size_medium.set(false),
                                "这是中型模态框的内容，适合大多数使用场景。"
                            }
                            Modal {
                                visible: size_large(),
                                title: Some("大型模态框".to_string()),
                                size: ModalSize::Large,
                                on_close: move |_| size_large.set(false),
                                "这是大型模态框的内容，适合显示更多信息。"
                            }
                        }
                    }
                }

                section { id: "confirm", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4", "确认对话框" }
                    
                    DemoBox {
                        title: "确认对话框".to_string(),
                        description: "带有确认和取消按钮的模态框。".to_string(),
                        code: r#"use helixui::components::{Button, Modal, ModalType};

rsx! {
    Modal {
        visible: confirm(),
        title: Some("确认删除".to_string()),
        modal_type: ModalType::Confirm,
        show_confirm: true,
        show_cancel: true,
        on_confirm: move |_| confirm.set(false),
        "你确定要删除这个项目吗？"
    }
}"#.to_string(),
                        children: rsx! {
                            Button {
                                button_type: ButtonType::Error,
                                onclick: move |_| confirm_modal.set(true),
                                "删除项目"
                            }
                            Modal {
                                visible: confirm_modal(),
                                title: Some("确认删除".to_string()),
                                modal_type: ModalType::Confirm,
                                show_confirm: true,
                                show_cancel: true,
                                confirm_text: "删除".to_string(),
                                on_confirm: move |_| confirm_modal.set(false),
                                on_cancel: move |_| confirm_modal.set(false),
                                "你确定要删除这个项目吗？此操作不可撤销。"
                            }
                        }
                    }
                }

                section { id: "transform", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4", "变换原点" }
                    
                    DemoBox {
                        title: "变换原点".to_string(),
                        description: "控制模态框动画的展开原点，可以设置为 center 或其他位置。".to_string(),
                        code: r#"use helixui::components::{Button, Modal};

rsx! {
    Button {
        button_type: ButtonType::Primary,
        onclick: move |_| transform_modal.set(true),
        "打开变换原点模态框"
    }
    Modal {
        visible: transform_modal(),
        title: Some("变换原点模态框".to_string()),
        transform_origin: "center".to_string(),
        on_close: move |_| transform_modal.set(false),
        "这个模态框的动画从中心点展开。"
    }
}"#.to_string(),
                        children: rsx! {
                            Button {
                                button_type: ButtonType::Primary,
                                onclick: move |_| transform_origin_modal.set(true),
                                "打开变换原点模态框"
                            }
                            Modal {
                                visible: transform_origin_modal(),
                                title: Some("变换原点模态框".to_string()),
                                transform_origin: "center".to_string(),
                                on_close: move |_| transform_origin_modal.set(false),
                                "这个模态框的动画从中心点展开。"
                            }
                        }
                    }
                }

                section { id: "no-mask", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4", "不显示遮罩层" }
                    
                    DemoBox {
                        title: "不显示遮罩层".to_string(),
                        description: "可以设置不显示遮罩层，适合做悬浮窗。".to_string(),
                        code: r#"use helixui::components::{Button, Modal};

rsx! {
    Button {
        button_type: ButtonType::Primary,
        onclick: move |_| no_mask_modal.set(true),
        "打开无遮罩模态框"
    }
    Modal {
        visible: no_mask_modal(),
        title: Some("无遮罩模态框".to_string()),
        show_mask: false,
        on_close: move |_| no_mask_modal.set(false),
        "这个模态框没有遮罩层，可以当作悬浮窗使用。"
    }
}"#.to_string(),
                        children: rsx! {
                            Button {
                                button_type: ButtonType::Primary,
                                onclick: move |_| no_mask_modal.set(true),
                                "打开无遮罩模态框"
                            }
                            Modal {
                                visible: no_mask_modal(),
                                title: Some("无遮罩模态框".to_string()),
                                show_mask: false,
                                on_close: move |_| no_mask_modal.set(false),
                                "这个模态框没有遮罩层，可以当作悬浮窗使用。"
                            }
                        }
                    }
                }

                section { id: "draggable", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4", "可拖拽" }
                    
                    DemoBox {
                        title: "可拖拽模态框".to_string(),
                        description: "设置 draggable 属性为 true，模态框即可拖拽。".to_string(),
                        code: r#"use helixui::components::{Button, Modal};

rsx! {
    Button {
        button_type: ButtonType::Primary,
        onclick: move |_| draggable_modal.set(true),
        "打开可拖拽模态框"
    }
    Modal {
        visible: draggable_modal(),
        title: Some("可拖拽模态框".to_string()),
        draggable: true,
        on_close: move |_| draggable_modal.set(false),
        "这个模态框可以被拖拽移动位置。"
    }
}"#.to_string(),
                        children: rsx! {
                            Button {
                                button_type: ButtonType::Primary,
                                onclick: move |_| draggable_modal.set(true),
                                "打开可拖拽模态框"
                            }
                            Modal {
                                visible: draggable_modal(),
                                title: Some("可拖拽模态框".to_string()),
                                draggable: true,
                                on_close: move |_| draggable_modal.set(false),
                                "这个模态框可以被拖拽移动位置。"
                            }
                        }
                    }
                }

                section { id: "api", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4", "API" }
                    h3 { class: "text-xl font-semibold text-gray-900 dark:text-white mb-3", "Modal Props" }
                    
                    div { class: "overflow-x-auto",
                        table { class: "w-full border-collapse border border-gray-200 dark:border-gray-700",
                            thead {
                                tr { class: "bg-gray-50 dark:bg-gray-800",
                                    th { class: "p-3 text-left text-gray-900 dark:text-white font-semibold", "属性" }
                                    th { class: "p-3 text-left text-gray-900 dark:text-white font-semibold", "类型" }
                                    th { class: "p-3 text-left text-gray-900 dark:text-white font-semibold", "默认值" }
                                    th { class: "p-3 text-left text-gray-900 dark:text-white font-semibold", "描述" }
                                }
                            }
                            tbody {
                                tr { class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "visible" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "bool" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "false" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "是否显示模态框" }
                                }
                                tr { class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "title" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "Option<String>" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "None" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "模态框标题" }
                                }
                                tr { class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "size" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "ModalSize" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "Medium" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "模态框尺寸" }
                                }
                                tr { class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "position" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "ModalPosition" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "Center" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "模态框位置" }
                                }
                                tr { class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "modal_type" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "ModalType" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "Default" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "模态框类型" }
                                }
                                tr { class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "show_confirm" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "bool" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "false" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "是否显示确认按钮" }
                                }
                                tr { class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "show_cancel" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "bool" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "false" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "是否显示取消按钮" }
                                }
                                tr { class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "on_close" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "Option<EventHandler<()>>" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "None" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "关闭事件" }
                                }
                                tr { class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "transform_origin" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "String" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "\"center\"" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "变换原点，控制动画展开位置" }
                                }
                                tr { class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "show_mask" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "bool" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "true" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "是否显示遮罩层" }
                                }
                                tr { class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "draggable" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "bool" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "false" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "是否可拖拽" }
                                }
                                tr {
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "class" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "Option<String>" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "None" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "自定义 CSS 类名" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

