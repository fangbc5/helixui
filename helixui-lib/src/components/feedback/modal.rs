use dioxus::prelude::*;

/// Modal 尺寸
#[derive(Debug, Clone, PartialEq)]
pub enum ModalSize {
    Small,
    Medium,
    Large,
    Full,
}

/// Modal 位置
#[derive(Debug, Clone, PartialEq)]
pub enum ModalPosition {
    Top,
    Center,
    Bottom,
    Left,
    Right,
}

/// Modal 类型
#[derive(Debug, Clone, PartialEq)]
pub enum ModalType {
    Default,
    Confirm,
    Alert,
    Custom,
}

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

    /// 是否可拖拽
    #[props(default = false)]
    pub draggable: bool,

    /// 是否可调整大小
    #[props(default = false)]
    pub resizable: bool,

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
pub fn Modal(props: ModalProps) -> Element {
    let size_class = get_size_class(&props.size);
    let position_class = get_position_class(&props.position);

    let content_class = format!(
        "bg-white dark:bg-gray-800 rounded-lg shadow-xl {} {} {}",
        size_class,
        position_class,
        props.class.unwrap_or_default()
    );

    if !props.visible {
        return rsx! { div {} };
    }

    rsx! {
        // 遮罩层
        if props.show_mask {
            div {
                class: "fixed inset-0 bg-black bg-opacity-50 z-40",
                onclick: move |_| {
                    if props.mask_closable {
                        if let Some(on_close) = &props.on_close {
                            on_close.call(());
                        }
                    }
                },
            }
        }

        // 模态框内容
        div {
            class: "fixed inset-0 flex items-center justify-center z-50",

            div {
                class: content_class,
                onclick: move |e: MouseEvent| {
                    e.stop_propagation();
                },

                // 标题栏
                if props.title.is_some() || props.closable {
                    div {
                        class: "flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700",

                        if let Some(title) = &props.title {
                            h3 {
                                class: "text-lg font-semibold text-gray-900 dark:text-white",
                                "{title}"
                            }
                        }

                        if props.closable {
                            button {
                                class: "text-gray-400 hover:text-gray-600 dark:hover:text-gray-300",
                                onclick: move |_| {
                                    if let Some(on_close) = &props.on_close {
                                        on_close.call(());
                                    }
                                },
                                "×"
                            }
                        }
                    }
                }

                // 内容区域
                div {
                    class: "p-4",
                    {props.children}
                }

                // 底部按钮（根据类型）
                if props.modal_type == ModalType::Confirm {
                    div {
                        class: "flex justify-end space-x-2 p-4 border-t border-gray-200 dark:border-gray-700",

                        button {
                            class: "px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 dark:bg-gray-800 dark:text-gray-300 dark:border-gray-600 dark:hover:bg-gray-700",
                            onclick: move |_| {
                                if let Some(on_cancel) = &props.on_cancel {
                                    on_cancel.call(());
                                }
                            },
                            "取消"
                        }

                        button {
                            class: "px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700",
                            onclick: move |_| {
                                if let Some(on_confirm) = &props.on_confirm {
                                    on_confirm.call(());
                                }
                            },
                            "确认"
                        }
                    }
                }
            }
        }
    }
}

/// 获取尺寸样式类
fn get_size_class(size: &ModalSize) -> String {
    match size {
        ModalSize::Small => "max-w-sm".to_string(),
        ModalSize::Medium => "max-w-md".to_string(),
        ModalSize::Large => "max-w-lg".to_string(),
        ModalSize::Full => "max-w-full mx-4".to_string(),
    }
}

/// 获取位置样式类
fn get_position_class(position: &ModalPosition) -> String {
    match position {
        ModalPosition::Top => "mt-8".to_string(),
        ModalPosition::Center => "".to_string(),
        ModalPosition::Bottom => "mb-8".to_string(),
        ModalPosition::Left => "ml-8".to_string(),
        ModalPosition::Right => "mr-8".to_string(),
    }
}

/// 全局 Modal 管理器
#[derive(Clone)]
pub struct ModalManager {
    modals: Signal<Vec<ModalData>>,
    next_id: Signal<u32>,
}

/// Modal 数据
#[derive(Clone, Debug, PartialEq)]
pub struct ModalData {
    pub id: u32,
    pub title: Option<String>,
    pub content: String,
    pub size: ModalSize,
    pub position: ModalPosition,
    pub modal_type: ModalType,
    pub visible: bool,
}

impl ModalManager {
    pub fn new() -> Self {
        Self {
            modals: use_signal(|| Vec::new()),
            next_id: use_signal(|| 1),
        }
    }

    pub fn show_modal(&mut self, data: ModalData) -> u32 {
        let id = *self.next_id.read();
        let mut modal_data = data;
        modal_data.id = id;
        modal_data.visible = true;

        let mut modals = self.modals.write();
        modals.push(modal_data);

        let mut next_id = self.next_id.write();
        *next_id += 1;

        id
    }

    pub fn hide_modal(&mut self, id: u32) {
        let mut modals = self.modals.write();
        if let Some(modal) = modals.iter_mut().find(|m| m.id == id) {
            modal.visible = false;
        }
    }

    pub fn remove_modal(&mut self, id: u32) {
        let mut modals = self.modals.write();
        modals.retain(|m| m.id != id);
    }

    pub fn get_modals(&self) -> Vec<ModalData> {
        self.modals.read().clone()
    }
}

/// 全局 Modal 容器
#[component]
pub fn GlobalModalContainer() -> Element {
    let manager = use_signal(|| ModalManager::new());

    rsx! {
        div {
            class: "fixed inset-0 pointer-events-none z-40",

            for modal in manager.read().get_modals() {
                if modal.visible {
                    Modal {
                        visible: modal.visible,
                        title: modal.title.clone(),
                        size: modal.size.clone(),
                        position: modal.position.clone(),
                        modal_type: modal.modal_type.clone(),
                        on_close: None,

                        div {
                            class: "whitespace-pre-wrap",
                            "{modal.content}"
                        }
                    }
                }
            }
        }
    }
}

/// 命令式 Modal API
#[derive(Clone)]
pub struct ImperativeModal {
    manager: Signal<ModalManager>,
}

impl ImperativeModal {
    pub fn new() -> Self {
        Self {
            manager: use_signal(|| ModalManager::new()),
        }
    }

    pub fn show(&mut self, data: ModalData) -> u32 {
        self.manager.write().show_modal(data)
    }

    pub fn hide(&mut self, id: u32) {
        self.manager.write().hide_modal(id);
    }

    pub fn remove(&mut self, id: u32) {
        self.manager.write().remove_modal(id);
    }
}

/// 便捷函数
pub fn show_modal(
    title: Option<String>,
    content: String,
    size: ModalSize,
    position: ModalPosition,
    modal_type: ModalType,
) -> u32 {
    let data = ModalData {
        id: 0,
        title,
        content,
        size,
        position,
        modal_type,
        visible: true,
    };

    // 在实际应用中，这里会使用全局 Modal 管理器
    // 目前返回一个模拟的 ID
    1
}
