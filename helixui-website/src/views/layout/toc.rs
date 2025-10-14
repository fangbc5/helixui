use dioxus::events::Key;
use dioxus::prelude::*;

/// 目录项结构
#[derive(Clone, PartialEq)]
pub struct TocItem {
    pub id: String,
    pub title: String,
    pub level: u32,
}

/// 目录项属性
#[derive(Props, Clone, PartialEq)]
pub struct TocItemProps {
    pub item: TocItem,
    pub is_active: bool,
    pub onclick: EventHandler<()>,
}

/// 单个目录项组件
#[component]
fn TocItemComponent(props: TocItemProps) -> Element {
    let active_class = if props.is_active {
        "text-green-600 dark:text-green-400 bg-green-50 dark:bg-green-900/20 border-r-2 border-green-600 dark:border-green-400"
    } else {
        "text-gray-600 dark:text-gray-300 hover:text-green-600 dark:hover:text-green-400"
    };

    let padding_class = match props.item.level {
        1 => "pl-2",
        2 => "pl-4",
        3 => "pl-6",
        4 => "pl-8",
        _ => "pl-10",
    };

    rsx! {
        div {
            role: "link",
            tabindex: 0,
            aria_current: if props.is_active { Some("true") } else { None },
            class: format!("block px-2 py-1 text-sm transition-colors {} {}", active_class, padding_class),
            onclick: move |_| { props.onclick.call(()); },
            onkeydown: move |e| {
                if e.key() == Key::Enter { props.onclick.call(()); }
            },
            {props.item.title}
        }
    }
}

/// 目录属性
#[derive(Props, Clone, PartialEq)]
pub struct PageTocProps {
    /// 目录项列表
    pub items: Vec<TocItem>,
    /// 当前激活的目录项ID（可选，用于外部控制）
    #[props(default)]
    pub active_id: Option<String>,
    /// 是否启用滚动监听（默认启用）
    #[props(default = true)]
    pub enable_scroll_highlight: bool,
    /// 当用户点击目录项时回调，传递 item.id（用于父层同步滚动状态）
    #[props(default)]
    pub on_navigate: Option<EventHandler<String>>,
}

/// 右侧页面目录
#[component]
pub fn PageToc(props: PageTocProps) -> Element {
    // 基础版：不做自动滚动高亮，仅点击设置
    let mut internal_active_id = use_signal(|| String::new());
    let active_id = if let Some(external_id) = &props.active_id {
        external_id.clone()
    } else {
        internal_active_id.read().clone()
    };

    // 点击目录项时的滚动处理
    // 点击目录项时，仅更新内部激活项，其余平台交给锚点默认行为处理
    let mut handle_toc_click = move |item_id: String| {
        internal_active_id.set(item_id);
        if let Some(cb) = &props.on_navigate {
            cb.call(internal_active_id.read().clone());
        }
    };

    rsx! {
        div {
            class: "p-4",
            h3 {
                class: "px-2 mb-4 text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider",
                "目录"
            }
            nav {
                class: "space-y-1",
                for item in props.items.into_iter() {
                    TocItemComponent {
                        item: item.clone(),
                        is_active: active_id == item.id,
                        onclick: move |_| { handle_toc_click(item.id.clone()); },
                    }
                }
            }
        }
    }
}
