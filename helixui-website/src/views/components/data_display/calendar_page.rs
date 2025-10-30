use crate::views::layout::{ComponentsSidebar, TocItem};
use crate::views::DocPage;
use dioxus::prelude::*;
use helixui::components::{
    Calendar, CalendarGrid, CalendarHeader, CalendarMonthTitle, CalendarNavigation,
    CalendarNextMonthButton, CalendarPreviousMonthButton, DemoBox, Table,
};
use helixui::time::cross_platform_date::today_utc;
use time::Date;

#[component]
pub fn CalendarPage() -> Element {
    let toc_items = vec![
        TocItem {
            id: "basic".to_string(),
            title: "基础用法".to_string(),
            level: 2,
        },
        TocItem {
            id: "api".to_string(),
            title: "API".to_string(),
            level: 2,
        },
    ];

    rsx! {
        DocPage {
            sidebar: rsx! { ComponentsSidebar {} },
            toc_items,

            div { class: "component-doc",
                // 标题
                div { class: "mb-8",
                    h1 { class: "text-4xl font-bold text-gray-900 dark:text-white mb-2", "日历 Calendar" }
                    p { class: "text-gray-600 dark:text-gray-300", "一个可选择日期的日历组件，支持键盘导航与月份切换。" }
                }

                // 基础用法
                section { id: "basic", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4", "基础用法" }
                    DemoBox {
                        title: "基础用法",
                        description: "包含头部的月份切换与月份标题，以及日历内容。",
                        code: r#"
use dioxus::prelude::*;
use helixui::components::{
    Calendar, CalendarGrid, CalendarHeader, CalendarMonthTitle, CalendarNavigation,
    CalendarNextMonthButton, CalendarPreviousMonthButton,
};
use helixui::time::cross_platform_date::today_utc;
use time::Date;

#[component]
fn CalendarDemo() -> Element {
    let mut selected_date = use_signal(|| None::<Date>);
    let mut view_date = use_signal(|| today_utc());

    rsx! {
        Calendar {
            selected_date: selected_date(),
            on_date_change: move |d| selected_date.set(d),
            view_date: view_date(),
            on_view_change: move |d| view_date.set(d),

            CalendarHeader {
                CalendarNavigation {
                    CalendarPreviousMonthButton { "<" }
                    CalendarMonthTitle {}
                    CalendarNextMonthButton { ">" }
                }
            }

            CalendarGrid {}
        }
    }
}
"#.to_string(),
                        children: rsx! {
                            div {
                                class: "flex flex-col gap-4",
                                // selected date text
                                CalendarDemo {}
                            }
                        }
                    }
                }

                // API
                section { id: "api", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4", "API" }
                    div { class: "mt-2",
                        Table {
                            headers: Some(vec!["属性".to_string(), "类型".to_string(), "说明".to_string()]),
                            data: vec![
                                vec!["selected_date".to_string(), "ReadSignal<Option<Date>>".to_string(), "当前选中的日期".to_string()],
                                vec!["on_date_change".to_string(), "Callback<Option<Date>>".to_string(), "选中日期变化回调".to_string()],
                                vec!["view_date".to_string(), "ReadSignal<Date>".to_string(), "当前视图月份（任意当天）".to_string()],
                                vec!["on_view_change".to_string(), "Callback<Date>".to_string(), "视图月份变化回调".to_string()],
                            ],
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CalendarDemo() -> Element {
    let mut selected_date = use_signal(|| None::<Date>);
    let mut view_date = use_signal(|| today_utc());

    rsx! {
        Calendar {
            selected_date: selected_date(),
            on_date_change: move |d| selected_date.set(d),
            view_date: view_date(),
            on_view_change: move |d| view_date.set(d),

            CalendarHeader {
                CalendarNavigation {
                    CalendarPreviousMonthButton { "<" }
                    CalendarMonthTitle {}
                    CalendarNextMonthButton { ">" }
                }
            }

            CalendarGrid {}
        }
    }
}
