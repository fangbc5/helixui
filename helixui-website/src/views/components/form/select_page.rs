use crate::views::layout::ComponentsSidebar;
use crate::views::layout::TocItem;
use crate::views::DocPage;
use dioxus::prelude::*;
use helixui::components::form::select::{
    Select, SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectOption,
    SelectSize, SelectTrigger, SelectValue,
};

#[component]
pub fn SelectPage() -> Element {
    let toc_items = vec![
        TocItem {
            id: "basic".to_string(),
            title: "基础用法".to_string(),
            level: 1,
        },
        TocItem {
            id: "size".to_string(),
            title: "尺寸".to_string(),
            level: 1,
        },
        TocItem {
            id: "multiple".to_string(),
            title: "多选".to_string(),
            level: 1,
        },
        TocItem {
            id: "filter".to_string(),
            title: "可过滤".to_string(),
            level: 1,
        },
    ];
    rsx! {
        DocPage {
            sidebar: rsx! { ComponentsSidebar {} },
            toc_items: toc_items,

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
                                            SelectOption::<String> { index: 0usize, value: "apple", span { class: "flex-1 truncate", "Apple" } SelectItemIndicator {} }
                                            SelectOption::<String> { index: 1usize, value: "banana", span { class: "flex-1 truncate", "Banana" } SelectItemIndicator {} }
                                            SelectOption::<String> { index: 2usize, value: "orange", span { class: "flex-1 truncate", "Orange" } SelectItemIndicator {} }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // 尺寸演示
                section { id: "size", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4", "尺寸" }
                    helixui::components::DemoBox {
                        title: "尺寸".to_string(),
                        description: "Small / Medium / Large 三种尺寸".to_string(),
                        code: {r##"
use dioxus::prelude::*;
use helixui::components::form::select::{
    Select, SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectOption,
    SelectTrigger, SelectValue,
};
use helixui::components::form::select::context::SelectSize;

#[component]
fn Demo() -> Element {
    rsx! {
        div { class: "flex flex-col gap-4",
            // Small
            Select::<String> {
                size: ReadSignal::new(Signal::new(SelectSize::Small)),
                placeholder: "Small...",
                SelectTrigger { aria_label: "Small", width: "12rem", SelectValue {} }
                SelectList { aria_label: "Small List",
                    SelectGroup {
                        SelectGroupLabel { "Fruits" }
                        SelectOption::<String> { index: 0usize, value: "apple", span { class: "flex-1 truncate", "Apple" } SelectItemIndicator { "✔" } }
                        SelectOption::<String> { index: 1usize, value: "banana", span { class: "flex-1 truncate", "Banana" } SelectItemIndicator { "✔" } }
                        SelectOption::<String> { index: 2usize, value: "orange", span { class: "flex-1 truncate", "Orange" } SelectItemIndicator { "✔" } }
                    }
                }
            }
            // Medium
            Select::<String> {
                size: ReadSignal::new(Signal::new(SelectSize::Medium)),
                placeholder: "Medium...",
                SelectTrigger { aria_label: "Medium", width: "14rem", SelectValue {} }
                SelectList { aria_label: "Medium List",
                    SelectGroup {
                        SelectGroupLabel { "Fruits" }
                        SelectOption::<String> { index: 0usize, value: "apple", span { class: "flex-1 truncate", "Apple" } SelectItemIndicator { "✔" } }
                        SelectOption::<String> { index: 1usize, value: "banana", span { class: "flex-1 truncate", "Banana" } SelectItemIndicator { "✔" } }
                        SelectOption::<String> { index: 2usize, value: "orange", span { class: "flex-1 truncate", "Orange" } SelectItemIndicator { "✔" } }
                    }
                }
            }
            // Large
            Select::<String> {
                size: ReadSignal::new(Signal::new(SelectSize::Large)),
                placeholder: "Large...",
                SelectTrigger { aria_label: "Large", width: "16rem", SelectValue {} }
                SelectList { aria_label: "Large List",
                    SelectGroup {
                        SelectGroupLabel { "Fruits" }
                        SelectOption::<String> { index: 0usize, value: "apple", span { class: "flex-1 truncate", "Apple" } SelectItemIndicator { "✔" } }
                        SelectOption::<String> { index: 1usize, value: "banana", span { class: "flex-1 truncate", "Banana" } SelectItemIndicator { "✔" } }
                        SelectOption::<String> { index: 2usize, value: "orange", span { class: "flex-1 truncate", "Orange" } SelectItemIndicator { "✔" } }
                    }
                }
            }
        }
    }
}
"##.to_string()},
                        children: rsx! {
                            div { class: "flex flex-col gap-6",
                                // Small
                                Select::<String> {
                                    size: ReadSignal::new(Signal::new(SelectSize::Small)),
                                    placeholder: "Small...",
                                    SelectTrigger { aria_label: "Small", width: "12rem", SelectValue {} }
                                    SelectList { aria_label: "Small List",
                                        SelectGroup {
                                            SelectGroupLabel { "Fruits" }
                                            SelectOption::<String> { index: 0usize, value: "apple", span { class: "flex-1 truncate", "Apple" } SelectItemIndicator { "✔" } }
                                            SelectOption::<String> { index: 1usize, value: "banana", span { class: "flex-1 truncate", "Banana" } SelectItemIndicator { "✔" } }
                                            SelectOption::<String> { index: 2usize, value: "orange", span { class: "flex-1 truncate", "Orange" } SelectItemIndicator { "✔" } }
                                        }
                                    }
                                }
                                // Medium
                                Select::<String> {
                                    size: ReadSignal::new(Signal::new(SelectSize::Medium)),
                                    placeholder: "Medium...",
                                    SelectTrigger { aria_label: "Medium", width: "14rem", SelectValue {} }
                                    SelectList { aria_label: "Medium List",
                                        SelectGroup {
                                            SelectGroupLabel { "Fruits" }
                                            SelectOption::<String> { index: 0usize, value: "apple", span { class: "flex-1 truncate", "Apple" } SelectItemIndicator { "✔" } }
                                            SelectOption::<String> { index: 1usize, value: "banana", span { class: "flex-1 truncate", "Banana" } SelectItemIndicator { "✔" } }
                                            SelectOption::<String> { index: 2usize, value: "orange", span { class: "flex-1 truncate", "Orange" } SelectItemIndicator { "✔" } }
                                        }
                                    }
                                }
                                // Large
                                Select::<String> {
                                    size: ReadSignal::new(Signal::new(SelectSize::Large)),
                                    placeholder: "Large...",
                                    SelectTrigger { aria_label: "Large", width: "16rem", SelectValue {} }
                                    SelectList { aria_label: "Large List",
                                        SelectGroup {
                                            SelectGroupLabel { "Fruits" }
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

                // 多选演示
                section { id: "multiple", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4", "多选" }
                    helixui::components::DemoBox {
                        title: "多选".to_string(),
                        description: "点击选项进行勾选/取消勾选，面板不自动关闭，触发器展示逗号分隔的文本".to_string(),
                        code: {r##"
use dioxus::prelude::*;
use helixui::components::form::select::{Select, SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectOption, SelectTrigger, SelectValue, SelectSize};

#[component]
fn Demo() -> Element {
    rsx! {
        Select::<String> {
            multiple: ReadSignal::new(Signal::new(true)),
            size: ReadSignal::new(Signal::new(SelectSize::Medium)),
            placeholder: "选择多个水果...",
            SelectTrigger { aria_label: "Multiple", width: "18rem", SelectValue {} }
            SelectList { aria_label: "Multiple List",
                SelectGroup {
                    SelectGroupLabel { "Fruits" }
                    SelectOption::<String> { index: 0usize, value: "apple", span { class: "flex-1 truncate", "Apple" } SelectItemIndicator { "✔" } }
                    SelectOption::<String> { index: 1usize, value: "banana", span { class: "flex-1 truncate", "Banana" } SelectItemIndicator { "✔" } }
                    SelectOption::<String> { index: 2usize, value: "orange", span { class: "flex-1 truncate", "Orange" } SelectItemIndicator { "✔" } }
                    SelectOption::<String> { index: 3usize, value: "strawberry", span { class: "flex-1 truncate", "Strawberry" } SelectItemIndicator { "✔" } }
                }
            }
        }
    }
}
"##.to_string()},
                        children: rsx! {
                            Select::<String> {
                                multiple: ReadSignal::new(Signal::new(true)),
                                size: ReadSignal::new(Signal::new(SelectSize::Medium)),
                                placeholder: "选择多个水果...",
                                SelectTrigger { aria_label: "Multiple", width: "18rem", SelectValue {} }
                                SelectList { aria_label: "Multiple List",
                                    SelectGroup {
                                        SelectGroupLabel { "Fruits" }
                                        SelectOption::<String> { index: 0usize, value: "apple", span { class: "flex-1 truncate", "Apple" } SelectItemIndicator { "✔" } }
                                        SelectOption::<String> { index: 1usize, value: "banana", span { class: "flex-1 truncate", "Banana" } SelectItemIndicator { "✔" } }
                                        SelectOption::<String> { index: 2usize, value: "orange", span { class: "flex-1 truncate", "Orange" } SelectItemIndicator { "✔" } }
                                        SelectOption::<String> { index: 3usize, value: "strawberry", span { class: "flex-1 truncate", "Strawberry" } SelectItemIndicator { "✔" } }
                                    }
                                }
                            }
                        }
                    }
                }

                // 可过滤演示
                section { id: "filter", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4", "可过滤" }
                    helixui::components::DemoBox {
                        title: "可过滤".to_string(),
                        description: "在面板顶部输入框实时过滤选项（大小写不敏感）".to_string(),
                        code: {r##"
use dioxus::prelude::*;
use helixui::components::form::select::{Select, SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectOption, SelectTrigger, SelectValue, SelectSize};

#[component]
fn Demo() -> Element {
    rsx! {
        Select::<String> {
            filterable: ReadSignal::new(Signal::new(true)),
            size: ReadSignal::new(Signal::new(SelectSize::Medium)),
            placeholder: "搜索水果...",
            SelectTrigger { aria_label: "Filter", width: "18rem", SelectValue {} }
            SelectList { aria_label: "Filter List",
                SelectGroup {
                    SelectGroupLabel { "Fruits" }
                    SelectOption::<String> { index: 0usize, value: "Apple", span { class: "flex-1 truncate", "Apple" } SelectItemIndicator { "✔" } }
                    SelectOption::<String> { index: 1usize, value: "Banana", span { class: "flex-1 truncate", "Banana" } SelectItemIndicator { "✔" } }
                    SelectOption::<String> { index: 2usize, value: "Orange", span { class: "flex-1 truncate", "Orange" } SelectItemIndicator { "✔" } }
                    SelectOption::<String> { index: 3usize, value: "Watermelon", span { class: "flex-1 truncate", "Watermelon" } SelectItemIndicator { "✔" } }
                    SelectOption::<String> { index: 4usize, value: "Strawberry", span { class: "flex-1 truncate", "Strawberry" } SelectItemIndicator { "✔" } }
                }
            }
        }
    }
}
"##.to_string()},
                        children: rsx! {
                            Select::<String> {
                                filterable: ReadSignal::new(Signal::new(true)),
                                size: ReadSignal::new(Signal::new(SelectSize::Medium)),
                                placeholder: "搜索水果...",
                                SelectTrigger { aria_label: "Filter", width: "18rem", SelectValue {} }
                                SelectList { aria_label: "Filter List",
                                    SelectGroup {
                                        SelectGroupLabel { "Fruits" }
                                        SelectOption::<String> { index: 0usize, value: "Apple", span { class: "flex-1 truncate", "Apple" } SelectItemIndicator { "✔" } }
                                        SelectOption::<String> { index: 1usize, value: "Banana", span { class: "flex-1 truncate", "Banana" } SelectItemIndicator { "✔" } }
                                        SelectOption::<String> { index: 2usize, value: "Orange", span { class: "flex-1 truncate", "Orange" } SelectItemIndicator { "✔" } }
                                        SelectOption::<String> { index: 3usize, value: "Watermelon", span { class: "flex-1 truncate", "Watermelon" } SelectItemIndicator { "✔" } }
                                        SelectOption::<String> { index: 4usize, value: "Strawberry", span { class: "flex-1 truncate", "Strawberry" } SelectItemIndicator { "✔" } }
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
