use crate::i18n;
use crate::Route;
use dioxus::prelude::*;

/// 侧边栏项数据结构
#[derive(Clone, PartialEq)]
pub struct SidebarItem {
    pub label: String,
    pub route: Option<Route>,
    pub children: Vec<SidebarItem>,
    pub expanded: bool,
    pub count: Option<u32>, // 组件数量
}

impl SidebarItem {
    /// 创建一个分类项（没有路由）
    pub fn category(label: String, children: Vec<SidebarItem>) -> Self {
        let count = Some(children.len() as u32);
        Self {
            label,
            route: None,
            children,
            expanded: true, // 默认展开
            count,
        }
    }

    /// 创建一个链接项（有路由）
    pub fn link(label: String, route: crate::Route) -> Self {
        Self {
            label,
            route: Some(route),
            children: vec![],
            expanded: true, // 链接项不需要展开状态
            count: None,
        }
    }
}

/// 通用侧边栏组件
#[component]
pub fn Sidebar(
    items: Vec<SidebarItem>,
    #[props(optional)] debug_label: Option<&'static str>,
) -> Element {
    let mut sidebar_items = use_signal(|| items);

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
                for (index, item) in sidebar_items.read().iter().enumerate() {
                    SidebarNode {
                        item: item.clone(),
                        on_toggle: move |idx: usize| {
                            let mut items = sidebar_items.write();
                            if let Some(item) = items.get_mut(idx) {
                                item.expanded = !item.expanded;
                            }
                        },
                        index: index,
                    }
                }
            }
        }
    }
}

/// 侧边栏节点（分类或链接）
#[component]
fn SidebarNode(item: SidebarItem, on_toggle: EventHandler<usize>, index: usize) -> Element {
    if item.route.is_none() && !item.children.is_empty() {
        // 这是一个分类
        rsx! {
            div {
                class: "mb-4",

                // 分类标题和展开/收起按钮
                div {
                    class: "flex items-center justify-between px-2 py-1 mb-2 cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-800 rounded-md transition-colors group",
                    onclick: move |_| {
                        on_toggle.call(index);
                    },

                    h3 {
                        class: "text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider group-hover:text-gray-700 dark:group-hover:text-gray-300 transition-colors",
                        "{item.label}"
                        if let Some(count) = item.count {
                            span {
                                class: "ml-1 text-gray-400 dark:text-gray-500 group-hover:text-gray-600 dark:group-hover:text-gray-300",
                                "({count})"
                            }
                        }
                    }

                    // 展开/收起图标（展开时向下，收起时向右）
                    div {
                        class: "w-4 h-4 flex items-center justify-center transition-transform duration-200",
                        svg {
                            class: "w-3 h-3 text-gray-400 dark:text-gray-500 group-hover:text-gray-600 dark:group-hover:text-gray-300 transition-colors",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: if item.expanded {
                                    "M19 9l-7 7-7-7" // 向下箭头
                                } else {
                                    "M9 5l7 7-7 7" // 向右箭头
                                }
                            }
                        }
                    }
                }

                // 子项列表（带平滑过渡）
                div {
                    class: if item.expanded {
                        "overflow-hidden transition-all duration-300 ease-in-out max-h-96 opacity-100 transform translate-y-0"
                    } else {
                        "overflow-hidden transition-all duration-300 ease-in-out max-h-0 opacity-0 transform -translate-y-2"
                    },
                    ul {
                        class: "space-y-1 pl-2",
                        for child in item.children {
                            SidebarNode {
                                item: child,
                                on_toggle: move |_| {}, // 子项不需要切换功能
                                index: 0, // 子项不需要索引
                            }
                        }
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
    let items = vec![
        SidebarItem::category(
            i18n::t("sidebar.general"),
            vec![
                SidebarItem::link(i18n::t("component.avatar"), crate::Route::AvatarPage {}),
                SidebarItem::link(i18n::t("component.button"), crate::Route::ButtonPage {}),
                SidebarItem::link(i18n::t("component.card"), crate::Route::CardPage {}),
                SidebarItem::link(i18n::t("component.carousel"), crate::Route::CarouselPage {}),
                SidebarItem::link(i18n::t("component.divider"), crate::Route::DividerPage {}),
                SidebarItem::link(i18n::t("component.icon"), crate::Route::IconPage {}),
            ],
        ),
        SidebarItem::category(
            i18n::t("sidebar.layout"),
            vec![
                SidebarItem::link(i18n::t("component.flex"), crate::Route::FlexPage {}),
                SidebarItem::link(i18n::t("component.layout"), crate::Route::LayoutPage {}),
                SidebarItem::link(i18n::t("component.grid"), crate::Route::GridPage {}),
                SidebarItem::link(i18n::t("component.space"), crate::Route::SpacePage {}),
                SidebarItem::link(i18n::t("component.split"), crate::Route::SplitPage {}),
            ],
        ),
        SidebarItem::category(
            i18n::t("sidebar.data-input"),
            vec![
                SidebarItem::link(i18n::t("component.input"), crate::Route::InputPage {}),
                SidebarItem::link(i18n::t("component.textarea"), crate::Route::TextAreaPage {}),
                SidebarItem::link(i18n::t("component.checkbox"), crate::Route::CheckboxPage {}),
                SidebarItem::link(i18n::t("component.radio"), crate::Route::RadioPage {}),
                SidebarItem::link(i18n::t("component.switch"), crate::Route::SwitchPage {}),
                SidebarItem::link(i18n::t("component.select"), crate::Route::ComponentsPage {}),
            ],
        ),
        SidebarItem::category(
            i18n::t("sidebar.data-display"),
            vec![
                SidebarItem::link(i18n::t("component.table"), crate::Route::TablePage {}),
                SidebarItem::link(
                    i18n::t("component.scroll-area"),
                    crate::Route::ScrollAreaPage {},
                ),
                SidebarItem::link(i18n::t("component.list"), crate::Route::ComponentsPage {}),
                SidebarItem::link(i18n::t("component.tag"), crate::Route::ComponentsPage {}),
                SidebarItem::link(
                    i18n::t("component.progress"),
                    crate::Route::ComponentsPage {},
                ),
                SidebarItem::link(
                    i18n::t("component.skeleton"),
                    crate::Route::ComponentsPage {},
                ),
                SidebarItem::link(i18n::t("component.empty"), crate::Route::ComponentsPage {}),
            ],
        ),
        SidebarItem::category(
            i18n::t("sidebar.feedback"),
            vec![
                SidebarItem::link(i18n::t("component.alert"), crate::Route::AlertPage {}),
                SidebarItem::link(i18n::t("component.badge"), crate::Route::BadgePage {}),
                SidebarItem::link(i18n::t("component.dialog"), crate::Route::DialogPage {}),
                SidebarItem::link(i18n::t("component.toast"), crate::Route::ToastPage {}),
                SidebarItem::link(i18n::t("component.tooltip"), crate::Route::TooltipPage {}),
            ],
        ),
    ];

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
