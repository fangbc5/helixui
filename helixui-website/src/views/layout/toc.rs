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
        a {
            href: format!("#{}", props.item.id),
            class: format!("block px-2 py-1 text-sm transition-colors {} {}", active_class, padding_class),
            onclick: move |e| {
                e.prevent_default();
                props.onclick.call(());
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
}

/// 右侧页面目录
#[component]
pub fn PageToc(props: PageTocProps) -> Element {
    let active_id = use_signal(|| String::new());

    // 点击目录项时的滚动处理
    let handle_toc_click = move |item_id: String| {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(element) = document.get_element_by_id(&item_id) {
                    // 获取元素位置
                    let rect = element.get_bounding_client_rect();
                    let current_scroll = window.scroll_y().unwrap_or(0.0);

                    // 计算目标滚动位置，添加偏移量避免被顶部遮挡
                    // 考虑顶部导航栏高度 (64px) + 额外间距 (32px) = 96px
                    let target_scroll = current_scroll + rect.top() - 96.0;

                    // 使用平滑滚动
                    window.scroll_to_with_x_and_y(0.0, target_scroll);
                }
            }
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
                        is_active: *active_id.read() == item.id,
                        onclick: move |_| handle_toc_click(item.id.clone()),
                    }
                }
            }
        }
    }
}
