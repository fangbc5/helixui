use crate::views::layout::{ComponentsSidebar, TocItem};
use crate::views::DocPage;
use dioxus::prelude::*;
use helixui::components::layout::{
    Content, Footer, FooterTheme, Header, HeaderTheme,
    Layout, LayoutDirection, Sider, SiderTheme,
    SiderTrigger,
};
use helixui::components::DemoBox;

#[component]
pub fn LayoutPage() -> Element {
    let toc_items = vec![
        TocItem {
            id: "basic-layout".to_string(),
            title: "基础布局".to_string(),
            level: 1,
        },
        TocItem {
            id: "with-sider".to_string(),
            title: "带侧边栏".to_string(),
            level: 1,
        },
        TocItem {
            id: "fixed-header-footer".to_string(),
            title: "固定头部和底部".to_string(),
            level: 1,
        },
        TocItem {
            id: "sider-collapse".to_string(),
            title: "侧边栏收起".to_string(),
            level: 1,
        },
        TocItem {
            id: "themes".to_string(),
            title: "主题样式".to_string(),
            level: 1,
        },
        TocItem {
            id: "api".to_string(),
            title: "API".to_string(),
            level: 1,
        },
    ];

    rsx! {
        DocPage {
            sidebar: rsx! { ComponentsSidebar {} },
            toc_items: toc_items,

            // 基础布局
            section { id: "basic-layout", class: "mb-12",
                h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors", "基础布局" }
                p { class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors", "最简单的布局结构，包含头部、内容和底部。" }
                DemoBox {
                    title: "基础布局".to_string(),
                    description: "Header + Content + Footer".to_string(),
                    code: r#"use helixui::components::layout::{Layout, Header, Content, Footer};

rsx! {
    Layout {
        Header { height: Some(64),
            "页面头部"
        }
        Content { padding: Some(24),
            "主内容区域"
        }
        Footer { height: Some(48),
            "页面底部"
        }
    }
}"#.to_string(),
                    div { class: "border border-gray-200 dark:border-gray-700 rounded overflow-hidden",
                        Layout {
                            Header { height: Some(64), theme: Some(HeaderTheme::Light),
                                div { class: "flex items-center justify-between w-full",
                                    h1 { class: "text-lg font-semibold text-gray-900", "页面标题" }
                                    div { class: "text-sm text-gray-500", "用户信息" }
                                }
                            }
                            Content { padding: Some(24), background: Some("#f5f5f5".to_string()),
                                div { class: "space-y-4",
                                    h2 { class: "text-xl font-semibold text-gray-900", "主内容区域" }
                                    p { class: "text-gray-600", "这里是页面的主要内容区域，可以放置各种组件和内容。" }
                                    div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                                        for i in 1..=6 {
                                            div { class: "p-4 bg-white rounded-lg shadow-sm border",
                                                h3 { class: "font-medium text-gray-900", "卡片 {i}" }
                                                p { class: "text-sm text-gray-500 mt-2", "卡片内容描述" }
                                            }
                                        }
                                    }
                                }
                            }
                            Footer { height: Some(48), theme: Some(FooterTheme::Light),
                                div { class: "text-center text-sm text-gray-500",
                                    "© 2024 HelixUI. All rights reserved."
                                }
                            }
                        }
                    }
                }
            }

            // 带侧边栏布局
            section { id: "with-sider", class: "mb-12",
                h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors", "带侧边栏" }
                p { class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors", "包含侧边栏的布局结构，适用于管理后台等场景。" }
                DemoBox {
                    title: "带侧边栏布局".to_string(),
                    description: "Header + (Sider + Content) + Footer".to_string(),
                    code: r#"use helixui::components::layout::{Layout, LayoutDirection, Header, Sider, Content, Footer};

rsx! {
    Layout {
        Header { height: Some(64),
            "页面头部"
        }
        Layout { direction: LayoutDirection::Row,
            Sider { width: Some(200), theme: Some(SiderTheme::Dark),
                "侧边栏"
            }
            Content { padding: Some(24),
                "主内容区域"
            }
        }
        Footer { height: Some(48),
            "页面底部"
        }
    }
}"#.to_string(),
                    div { class: "border border-gray-200 dark:border-gray-700 rounded overflow-hidden h-96",
                        Layout {
                            Header { height: Some(64), theme: Some(HeaderTheme::Light),
                                div { class: "flex items-center justify-between w-full",
                                    h1 { class: "text-lg font-semibold text-gray-900", "管理后台" }
                                    div { class: "text-sm text-gray-500", "管理员" }
                                }
                            }
                            Layout { direction: LayoutDirection::Row,
                                Sider { width: Some(200), theme: Some(SiderTheme::Dark),
                                    div { class: "p-4 space-y-2",
                                        div { class: "text-white font-medium mb-4", "导航菜单" }
                                        for item in ["首页", "用户管理", "系统设置", "数据统计"] {
                                            div { class: "text-gray-300 hover:text-white cursor-pointer py-2 px-3 rounded",
                                                "{item}"
                                            }
                                        }
                                    }
                                }
                                Content { padding: Some(24), background: Some("#f5f5f5".to_string()),
                                    div { class: "space-y-4",
                                        h2 { class: "text-xl font-semibold text-gray-900", "仪表板" }
                                        p { class: "text-gray-600", "欢迎使用管理后台系统。" }
                                        div { class: "grid grid-cols-2 gap-4",
                                            for i in 1..=4 {
                                                div { class: "p-4 bg-white rounded-lg shadow-sm",
                                                    h3 { class: "font-medium text-gray-900", "统计卡片 {i}" }
                                                    p { class: "text-2xl font-bold text-blue-600 mt-2", "1,234" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Footer { height: Some(48), theme: Some(FooterTheme::Light),
                                div { class: "text-center text-sm text-gray-500",
                                    "© 2024 HelixUI Admin. All rights reserved."
                                }
                            }
                        }
                    }
                }
            }

            // 固定头部和底部
            section { id: "fixed-header-footer", class: "mb-12",
                h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors", "固定头部和底部" }
                p { class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors", "头部和底部固定定位，内容区域可滚动。" }
                DemoBox {
                    title: "固定头部和底部".to_string(),
                    description: "fixed: true".to_string(),
                    code: r#"use helixui::components::layout::{Layout, Header, Content, Footer};

rsx! {
    Layout {
        Header { height: Some(64), fixed: Some(true),
            "固定头部"
        }
        Content { padding: Some(24),
            "可滚动内容区域"
        }
        Footer { height: Some(48), fixed: Some(true),
            "固定底部"
        }
    }
}"#.to_string(),
                    div { class: "border border-gray-200 dark:border-gray-700 rounded overflow-hidden h-96 relative",
                        Layout {
                            Header { height: Some(64), fixed: Some(true), theme: Some(HeaderTheme::Light),
                                div { class: "flex items-center justify-between w-full",
                                    h1 { class: "text-lg font-semibold text-gray-900", "固定头部" }
                                    div { class: "text-sm text-gray-500", "始终可见" }
                                }
                            }
                            Content { padding: Some(24), background: Some("#f5f5f5".to_string()),
                                div { class: "space-y-4",
                                    h2 { class: "text-xl font-semibold text-gray-900", "可滚动内容" }
                                    for i in 1..=20 {
                                        div { class: "p-4 bg-white rounded-lg shadow-sm",
                                            h3 { class: "font-medium text-gray-900", "内容块 {i}" }
                                            p { class: "text-gray-600 mt-2", "这是第 {i} 个内容块，用于演示滚动效果。" }
                                        }
                                    }
                                }
                            }
                            Footer { height: Some(48), fixed: Some(true), theme: Some(FooterTheme::Light),
                                div { class: "text-center text-sm text-gray-500",
                                    "固定底部 - 始终可见"
                                }
                            }
                        }
                    }
                }
            }

            // 侧边栏收起
            section { id: "sider-collapse", class: "mb-12",
                h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors", "侧边栏收起" }
                p { class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors", "侧边栏支持收起/展开功能，节省空间。" }
                DemoBox {
                    title: "侧边栏收起".to_string(),
                    description: "collapsible: true".to_string(),
                    code: r#"use helixui::components::layout::{Layout, LayoutDirection, Header, Sider, SiderTrigger, Content, Footer};

rsx! {
    Layout {
        Header { height: Some(64),
            "页面头部"
        }
        Layout { direction: LayoutDirection::Row,
            Sider { 
                width: Some(200), 
                collapsed_width: Some(80),
                collapsible: Some(true),
                collapsed: Some(false),
                theme: Some(SiderTheme::Dark),
                "侧边栏内容"
                SiderTrigger { 
                    collapsed: Some(false),
                    on_toggle: Some(|collapsed| {
                        // 处理收起/展开逻辑
                    }),
                }
            }
            Content { padding: Some(24),
                "主内容区域"
            }
        }
        Footer { height: Some(48),
            "页面底部"
        }
    }
}"#.to_string(),
                    div { class: "border border-gray-200 dark:border-gray-700 rounded overflow-hidden h-96",
                        Layout {
                            Header { height: Some(64), theme: Some(HeaderTheme::Light),
                                div { class: "flex items-center justify-between w-full",
                                    h1 { class: "text-lg font-semibold text-gray-900", "可收起侧边栏" }
                                    div { class: "text-sm text-gray-500", "点击侧边栏底部收起" }
                                }
                            }
                            Layout { direction: LayoutDirection::Row,
                                Sider {
                                    width: Some(200),
                                    collapsed_width: Some(80),
                                    collapsible: Some(true),
                                    collapsed: Some(false),
                                    theme: Some(SiderTheme::Dark),
                                    div { class: "p-4 space-y-2 h-full flex flex-col",
                                        div { class: "text-white font-medium mb-4", "导航菜单" }
                                        div { class: "flex-1 space-y-2",
                                            for item in ["首页", "用户管理", "系统设置", "数据统计"] {
                                                div { class: "text-gray-300 hover:text-white cursor-pointer py-2 px-3 rounded",
                                                    "{item}"
                                                }
                                            }
                                        }
                                        SiderTrigger {
                                            collapsed: Some(false),
                                        }
                                    }
                                }
                                Content { padding: Some(24), background: Some("#f5f5f5".to_string()),
                                    div { class: "space-y-4",
                                        h2 { class: "text-xl font-semibold text-gray-900", "主内容区域" }
                                        p { class: "text-gray-600", "侧边栏可以收起以节省空间。" }
                                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                            for i in 1..=6 {
                                                div { class: "p-4 bg-white rounded-lg shadow-sm",
                                                    h3 { class: "font-medium text-gray-900", "内容卡片 {i}" }
                                                    p { class: "text-gray-600 mt-2", "卡片内容描述" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Footer { height: Some(48), theme: Some(FooterTheme::Light),
                                div { class: "text-center text-sm text-gray-500",
                                    "© 2024 HelixUI. All rights reserved."
                                }
                            }
                        }
                    }
                }
            }

            // 主题样式
            section { id: "themes", class: "mb-12",
                h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors", "主题样式" }
                p { class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors", "支持明暗主题切换。" }
                DemoBox {
                    title: "暗色主题".to_string(),
                    description: "theme: Dark".to_string(),
                    code: r#"use helixui::components::layout::{Layout, Header, Sider, Content, Footer, HeaderTheme, SiderTheme, FooterTheme};

rsx! {
    Layout {
        Header { height: Some(64), theme: Some(HeaderTheme::Dark),
            "暗色头部"
        }
        Layout { direction: LayoutDirection::Row,
            Sider { width: Some(200), theme: Some(SiderTheme::Dark),
                "暗色侧边栏"
            }
            Content { padding: Some(24),
                "主内容区域"
            }
        }
        Footer { height: Some(48), theme: Some(FooterTheme::Dark),
            "暗色底部"
        }
    }
}"#.to_string(),
                    div { class: "border border-gray-200 dark:border-gray-700 rounded overflow-hidden h-96",
                        Layout {
                            Header { height: Some(64), theme: Some(HeaderTheme::Dark),
                                div { class: "flex items-center justify-between w-full",
                                    h1 { class: "text-lg font-semibold text-white", "暗色主题" }
                                    div { class: "text-sm text-gray-300", "深色模式" }
                                }
                            }
                            Layout { direction: LayoutDirection::Row,
                                Sider { width: Some(200), theme: Some(SiderTheme::Dark),
                                    div { class: "p-4 space-y-2",
                                        div { class: "text-white font-medium mb-4", "导航菜单" }
                                        for item in ["首页", "用户管理", "系统设置", "数据统计"] {
                                            div { class: "text-gray-300 hover:text-white cursor-pointer py-2 px-3 rounded",
                                                "{item}"
                                            }
                                        }
                                    }
                                }
                                Content { padding: Some(24), background: Some("#f5f5f5".to_string()),
                                    div { class: "space-y-4",
                                        h2 { class: "text-xl font-semibold text-gray-900", "主内容区域" }
                                        p { class: "text-gray-600", "暗色主题的头部和侧边栏。" }
                                        div { class: "grid grid-cols-2 gap-4",
                                            for i in 1..=4 {
                                                div { class: "p-4 bg-white rounded-lg shadow-sm",
                                                    h3 { class: "font-medium text-gray-900", "内容卡片 {i}" }
                                                    p { class: "text-gray-600 mt-2", "卡片内容" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Footer { height: Some(48), theme: Some(FooterTheme::Dark),
                                div { class: "text-center text-sm text-gray-300",
                                    "© 2024 HelixUI Dark Theme. All rights reserved."
                                }
                            }
                        }
                    }
                }
            }

            // API 文档
            section { id: "api", class: "mb-12",
                h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors", "API" }
                div { class: "space-y-6",
                    div {
                        h3 { class: "text-lg font-semibold text-gray-900 dark:text-white mb-2", "Layout Props" }
                        div { class: "overflow-x-auto",
                            table { class: "min-w-full divide-y divide-gray-200 dark:divide-gray-700",
                                thead { class: "bg-gray-50 dark:bg-gray-800",
                                    tr {
                                        th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider", "属性" }
                                        th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider", "类型" }
                                        th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider", "默认值" }
                                        th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider", "说明" }
                                    }
                                }
                                tbody { class: "bg-white dark:bg-gray-900 divide-y divide-gray-200 dark:divide-gray-700",
                                    tr {
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white", "direction" }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300", "LayoutDirection" }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300", "Column" }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300", "布局方向" }
                                    }
                                    tr {
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white", "responsive" }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300", "Option<bool>" }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300", "None" }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300", "是否启用响应式" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
