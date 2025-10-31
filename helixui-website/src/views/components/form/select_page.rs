use crate::views::layout::ComponentsSidebar;
use crate::views::DocPage;
use dioxus::prelude::*;
use helixui::components::form::select::{
    Select, SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectOption,
    SelectTrigger, SelectValue,
};

#[component]
pub fn SelectPage() -> Element {
    rsx! {
        DocPage {
            sidebar: rsx! { ComponentsSidebar {} },
            toc_items: vec![],

            div { class: "component-doc",
                // 标题
                div { class: "mb-8",
                    h1 { class: "text-4xl font-bold text-gray-900 dark:text-white mb-2", "选择器 Select" }
                    p { class: "text-gray-600 dark:text-gray-300", "用于从多个选项中进行选择，支持键盘导航与快速搜索。" }
                }

                // 基础用法
                section { id: "basic", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4", "基础用法" }
                    helixui::components::DemoBox {
                        title: "基础用法".to_string(),
                        description: "包含触发器、下拉列表、分组、选项与已选标记。".to_string(),
                        code: {r##"
use dioxus::prelude::*;
use helixui::components::form::select::{
    Select, SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectOption,
    SelectTrigger, SelectValue,
};

#[component]
fn Demo() -> Element {
    rsx! {
        Select::<String> {
            placeholder: "选择一个水果...",
            SelectTrigger { aria_label: "选择器触发", width: "12rem", SelectValue {} }
            SelectList { aria_label: "选择器演示",
                SelectGroup {
                    SelectGroupLabel { "水果" }
                    SelectOption::<String> { index: 0usize, value: "apple", span { class: "flex-1 truncate", "Apple" } SelectItemIndicator { "✔" } }
                    SelectOption::<String> { index: 1usize, value: "banana", span { class: "flex-1 truncate", "Banana" } SelectItemIndicator { "✔" } }
                    SelectOption::<String> { index: 2usize, value: "orange", span { class: "flex-1 truncate", "Orange" } SelectItemIndicator { "✔" } }
                }
            }
        }
    }
}
"##.to_string()},
                        children: rsx! {
                            div { class: "flex flex-col gap-4",
                                Select::<String> {
                                    placeholder: "选择一个水果...",
                                    SelectTrigger { aria_label: "选择器触发", width: "12rem", SelectValue {} }
                                    SelectList { aria_label: "选择器演示",
                                        SelectGroup {
                                            SelectGroupLabel { "水果" }
                                            SelectOption::<String> { index: 0usize, value: "apple", span { class: "flex-1 truncate", "Apple" } SelectItemIndicator { "✔" } }
                                            SelectOption::<String> { index: 1usize, value: "banana", span { class: "flex-1 truncate", "Banana" } SelectItemIndicator { "✔" } }
                                            SelectOption::<String> { index: 2usize, value: "orange", span { class: "flex-1 truncate", "Orange" } SelectItemIndicator { "✔" } }
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
}
