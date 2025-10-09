use crate::i18n;
use dioxus::prelude::*;

/// 侧边栏项数据结构
#[derive(Clone, PartialEq)]
pub struct SidebarItem {
    pub label: String,
    pub route: Option<crate::Route>,
    pub children: Vec<SidebarItem>,
}

impl SidebarItem {
    /// 创建一个分类项（没有路由）
    pub fn category(label: String, children: Vec<SidebarItem>) -> Self {
        Self {
            label,
            route: None,
            children,
        }
    }

    /// 创建一个链接项（有路由）
    pub fn link(label: String, route: crate::Route) -> Self {
        Self {
            label,
            route: Some(route),
            children: vec![],
        }
    }
}

/// 通用侧边栏组件
#[component]
pub fn Sidebar(
    items: Vec<SidebarItem>,
    #[props(optional)] debug_label: Option<&'static str>,
) -> Element {
    rsx! {
        div {
            class: "p-4",

            // 调试标识（可选）
            if let Some(label) = debug_label {
                div {
                    class: "mb-4 p-2 bg-green-100 text-green-800 text-xs font-bold rounded",
                    "✓ {label}"
                }
            }

            nav {
                class: "space-y-1",
                for item in items {
                    SidebarNode { item: item }
                }
            }
        }
    }
}

/// 侧边栏节点（分类或链接）
#[component]
fn SidebarNode(item: SidebarItem) -> Element {
    if item.route.is_none() && !item.children.is_empty() {
        // 这是一个分类
        rsx! {
            div {
                class: "mb-4",
                h3 {
                    class: "px-2 mb-2 text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider",
                    "{item.label}"
                }
                ul {
                    class: "space-y-1",
                    for child in item.children {
                        SidebarNode { item: child }
                    }
                }
            }
        }
    } else if let Some(route) = item.route {
        // 这是一个链接
        rsx! {
            li {
                Link {
                    to: route,
                    class: "block px-2 py-2 text-sm text-gray-700 dark:text-gray-300 hover:bg-green-50 dark:hover:bg-green-900/20 hover:text-green-600 dark:hover:text-green-400 rounded-md transition-colors",
                    active_class: "bg-green-50 dark:bg-green-900/20 text-green-600 dark:text-green-400",
                    "{item.label}"
                }
            }
        }
    } else {
        // 空节点
        rsx! { div {} }
    }
}

/// 左侧组件导航（只包含组件列表）
#[component]
pub fn ComponentsSidebar() -> Element {
    let items = vec![SidebarItem::category(
        i18n::t("sidebar.components"),
        vec![SidebarItem::link(
            i18n::t("component.button"),
            crate::Route::ButtonPage {},
        )],
    )];

    rsx! {
        Sidebar { items: items }
    }
}

/// 左侧文档导航（只有文档）
#[component]
pub fn DocsSidebar() -> Element {
    let items = vec![SidebarItem::category(
        i18n::t("sidebar.docs"),
        vec![
            SidebarItem::link(i18n::t("docs.introduction"), crate::Route::Introduction {}),
            SidebarItem::link(i18n::t("docs.quick-start"), crate::Route::QuickStart {}),
            SidebarItem::link(i18n::t("docs.guide"), crate::Route::Guide {}),
            SidebarItem::link(i18n::t("docs.version"), crate::Route::Version {}),
        ],
    )];

    rsx! {
        Sidebar { items: items }
    }
}
