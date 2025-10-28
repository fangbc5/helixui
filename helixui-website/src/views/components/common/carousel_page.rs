use crate::views::layout::{ComponentsSidebar, TocItem};
use crate::views::DocPage;
use dioxus::prelude::*;
use helixui::components::{
    Carousel, CarouselArrow, CarouselArrowDirection, CarouselContent, CarouselDirection,
    CarouselDots, CarouselItem, CarouselSlide, DemoBox, Switch, Table,
};

#[component]
pub fn CarouselPage() -> Element {
    let toc_items = vec![
        TocItem {
            id: "basic".to_string(),
            title: "基础用法".to_string(),
            level: 1,
        },
        TocItem {
            id: "controlled".to_string(),
            title: "受控模式".to_string(),
            level: 1,
        },
        TocItem {
            id: "autoplay".to_string(),
            title: "自动播放".to_string(),
            level: 1,
        },
        TocItem {
            id: "vertical".to_string(),
            title: "垂直轮播".to_string(),
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
            toc_items,
            div {
                "轮播图 Carousel",
                p {
                    class: "description",
                    "Carousel 是一个用于展示轮播内容的组件。"
                }

                h2 {
                    id: "basic",
                    "基础用法"
                }
                p {
                    class: "text-gray-600 dark:text-gray-300 mb-4",
                    "最简单的用法，创建一个包含多个幻灯片的轮播图。"
                }
                DemoBox {
                    title: "基础轮播图".to_string(),
                    description: "包含箭头和指示点的轮播图。".to_string(),
                    code: r#"use helixui::components::{Carousel, CarouselContent, CarouselSlide, CarouselItem, CarouselArrow, CarouselDots};

    rsx! {
        div {
            class: "h-64",
            Carousel {
                CarouselContent {
                    count: Some(3),
                    CarouselSlide {
                        CarouselItem {
                            div { class: "h-full flex items-center justify-center bg-gradient-to-r from-blue-500 to-blue-600 text-white text-2xl font-bold", "Slide 1" }
                        }
                    }
                    CarouselSlide {
                        CarouselItem {
                            div { class: "h-full flex items-center justify-center bg-gradient-to-r from-purple-500 to-purple-600 text-white text-2xl font-bold", "Slide 2" }
                        }
                    }
                    CarouselSlide {
                        CarouselItem {
                            div { class: "h-full flex items-center justify-center bg-gradient-to-r from-green-500 to-green-600 text-white text-2xl font-bold", "Slide 3" }
                        }
                    }
                }
                CarouselArrow { direction: CarouselArrowDirection::Prev }
                CarouselArrow { direction: CarouselArrowDirection::Next }
                CarouselDots {}
            },
        }
    }"#.to_string(),
                    children: rsx! {
                        BasicCarouselDemo {}
                    }
                }

                h2 {
                    id: "controlled",
                    "受控模式"
                }
                p {
                    class: "text-gray-600 dark:text-gray-300 mb-4",
                    "通过 `current_index` 和 `on_index_change` 控制轮播图的当前索引。"
                }
                DemoBox {
                    title: "受控轮播图".to_string(),
                    description: "可以完全控制轮播图的索引。".to_string(),
                    code: r#"let mut current = use_signal(|| 0);

    rsx! {
        div {
            class: "space-y-4",
            Carousel {
                current_index: current(),
                on_index_change: move |idx| current.set(idx),
                CarouselContent { count: Some(3),
                    // ... slides
                }
                CarouselArrow { direction: CarouselArrowDirection::Prev }
                CarouselArrow { direction: CarouselArrowDirection::Next }
                CarouselDots {}
            }
            div { "当前索引: {current()}" }
        }
    }"#.to_string(),
                    children: rsx! {
                        ControlledCarouselDemo {}
                    }
                }

                h2 {
                    id: "autoplay",
                    "自动播放"
                }
                p {
                    class: "text-gray-600 dark:text-gray-300 mb-4",
                    "通过 `auto_play` 属性启用自动播放功能。"
                }
                DemoBox {
                    title: "自动播放".to_string(),
                    description: "启用自动切换幻灯片功能。".to_string(),
                    code: r#"Carousel {
        auto_play: ReadSignal::new(Signal::new(true)),
        duration: ReadSignal::new(Signal::new(3000)),
        CarouselContent { count: Some(3),
            // ... slides
        }
        CarouselArrow { direction: CarouselArrowDirection::Prev }
        CarouselArrow { direction: CarouselArrowDirection::Next }
        CarouselDots {}
    }"#.to_string(),
                    children: rsx! {
                        AutoplayCarouselDemo {}
                    }
                }

                h2 {
                    id: "vertical",
                    "垂直轮播"
                }
                p {
                    class: "text-gray-600 dark:text-gray-300 mb-4",
                    "通过 `direction` 属性设置轮播方向为垂直。"
                }
                DemoBox {
                    title: "垂直轮播".to_string(),
                    description: "轮播图可以垂直滑动。".to_string(),
                    code: r#"Carousel {
        direction: ReadSignal::new(Signal::new(CarouselDirection::Vertical)),
        CarouselContent { count: Some(3),
            // ... slides
        }
        CarouselArrow { direction: CarouselArrowDirection::Prev }
        CarouselArrow { direction: CarouselArrowDirection::Next }
        CarouselDots {}
    }"#.to_string(),
                    children: rsx! {
                        VerticalCarouselDemo {}
                    }
                }

                h2 {
                    id: "api",
                    class: "text-2xl font-bold text-gray-900 dark:text-white mb-4",
                    "API"
                }
                h3 {
                    class: "text-xl font-semibold text-gray-900 dark:text-white mb-3",
                    "Carousel Props"
                }
                Table {
                    headers: Some(vec!["属性".to_string(), "说明".to_string(), "类型".to_string(), "默认值".to_string()]),
                    data: vec![
                        vec!["current_index".to_string(), "当前索引（受控）".to_string(), "ReadSignal<Option<usize>>".to_string(), "-".to_string()],
                        vec!["default_index".to_string(), "默认索引（非受控）".to_string(), "usize".to_string(), "0".to_string()],
                        vec!["on_index_change".to_string(), "索引变化回调".to_string(), "Callback<usize>".to_string(), "-".to_string()],
                        vec!["auto_play".to_string(), "是否自动播放".to_string(), "ReadSignal<bool>".to_string(), "false".to_string()],
                        vec!["duration".to_string(), "自动播放间隔（毫秒）".to_string(), "ReadSignal<u32>".to_string(), "3000".to_string()],
                        vec!["direction".to_string(), "轮播方向".to_string(), "ReadSignal<CarouselDirection>".to_string(), "Horizontal".to_string()],
                        vec!["show_arrows".to_string(), "是否显示箭头".to_string(), "ReadSignal<bool>".to_string(), "true".to_string()],
                        vec!["show_dots".to_string(), "是否显示指示点".to_string(), "ReadSignal<bool>".to_string(), "true".to_string()],
                    ],
                    bordered: true,
                    striped: true,
                }
            }
        }
    }
}

#[component]
fn BasicCarouselDemo() -> Element {
    rsx! {
        div {
            class: "h-64",
            Carousel {
                CarouselContent {
                    count: Some(3),
                    CarouselSlide {
                        CarouselItem {
                            div {
                                class: "h-full flex items-center justify-center bg-gradient-to-r from-blue-500 to-blue-600 text-white text-2xl font-bold",
                                "Slide 1"
                            }
                        }
                    }
                    CarouselSlide {
                        CarouselItem {
                            div {
                                class: "h-full flex items-center justify-center bg-gradient-to-r from-purple-500 to-purple-600 text-white text-2xl font-bold",
                                "Slide 2"
                            }
                        }
                    }
                    CarouselSlide {
                        CarouselItem {
                            div {
                                class: "h-full flex items-center justify-center bg-gradient-to-r from-green-500 to-green-600 text-white text-2xl font-bold",
                                "Slide 3"
                            }
                        }
                    }
                }
                CarouselArrow { direction: CarouselArrowDirection::Prev }
                CarouselArrow { direction: CarouselArrowDirection::Next }
                CarouselDots {}
            }
        }
    }
}

#[component]
fn ControlledCarouselDemo() -> Element {
    let mut current = use_signal(|| 0);

    rsx! {
        div {
            class: "space-y-4",
            div {
                Carousel {
                    current_index: current(),
                    on_index_change: move |idx| current.set(idx),
                    CarouselContent {
                        count: Some(3),
                        CarouselSlide {
                            CarouselItem {
                                div {
                                    class: "h-full flex items-center justify-center bg-gradient-to-r from-blue-500 to-blue-600 text-white text-2xl font-bold",
                                    "Slide 1"
                                }
                            }
                        }
                        CarouselSlide {
                            CarouselItem {
                                div {
                                    class: "h-full flex items-center justify-center bg-gradient-to-r from-purple-500 to-purple-600 text-white text-2xl font-bold",
                                    "Slide 2"
                                }
                            }
                        }
                        CarouselSlide {
                            CarouselItem {
                                div {
                                    class: "h-full flex items-center justify-center bg-gradient-to-r from-green-500 to-green-600 text-white text-2xl font-bold",
                                    "Slide 3"
                                }
                            }
                        }
                    }
                CarouselArrow { direction: CarouselArrowDirection::Prev }
                CarouselArrow { direction: CarouselArrowDirection::Next }
                    CarouselDots {}
                }
            }
            div {
                class: "text-sm text-gray-600 dark:text-gray-400",
                "当前索引: {current()}"
            }
        }
    }
}

#[component]
fn AutoplayCarouselDemo() -> Element {
    let mut auto_play = use_signal(|| true);
    let mut duration = use_signal(|| 3000u32);

    rsx! {
        div {
            class: "space-y-4",
            div {
                class: "flex items-center gap-4 mb-4",
                span { class: "text-sm text-gray-700 dark:text-gray-300", "自动播放" }
                Switch {
                    checked: auto_play(),
                    on_checked_change: move |v| auto_play.set(v),
                }
            }
            div {
                class: "flex items-center gap-4 mb-4",
                span { class: "text-sm text-gray-700 dark:text-gray-300", "间隔时间: " }
                input {
                    class: "px-2 py-1 border border-gray-300 dark:border-gray-600 rounded text-sm dark:bg-gray-800 dark:text-gray-200",
                    r#type: "number",
                    value: "{duration()}",
                    oninput: move |e| {
                        if let Ok(n) = e.value().parse::<u32>() {
                            duration.set(n);
                        }
                    },
                }
                span { class: "text-sm text-gray-500", "毫秒" }
            },
            Carousel {
                auto_play: ReadSignal::new(auto_play),
                duration: ReadSignal::new(duration),
                CarouselContent {
                    count: Some(3),
                    CarouselSlide {
                        CarouselItem {
                            div {
                                class: "h-full flex items-center justify-center bg-gradient-to-r from-blue-500 to-blue-600 text-white text-2xl font-bold",
                                "Slide 1"
                            }
                        }
                    }
                    CarouselSlide {
                        CarouselItem {
                            div {
                                class: "h-full flex items-center justify-center bg-gradient-to-r from-purple-500 to-purple-600 text-white text-2xl font-bold",
                                "Slide 2"
                            }
                        }
                    }
                    CarouselSlide {
                        CarouselItem {
                            div {
                                class: "h-full flex items-center justify-center bg-gradient-to-r from-green-500 to-green-600 text-white text-2xl font-bold",
                                "Slide 3"
                            }
                        }
                    }
                }
                CarouselArrow { direction: CarouselArrowDirection::Prev }
                CarouselArrow { direction: CarouselArrowDirection::Next }
            CarouselDots {}
            }
        }
    }
}

#[component]
fn VerticalCarouselDemo() -> Element {
    rsx! {
        Carousel {
            direction: ReadSignal::new(Signal::new(CarouselDirection::Vertical)),
            CarouselContent {
                count: Some(3),
                CarouselSlide {
                    CarouselItem {
                        div {
                            class: "h-full flex items-center justify-center bg-gradient-to-b from-blue-500 to-blue-600 text-white text-2xl font-bold",
                            "Slide 1"
                        }
                    }
                }
                CarouselSlide {
                    CarouselItem {
                        div {
                            class: "h-full flex items-center justify-center bg-gradient-to-b from-purple-500 to-purple-600 text-white text-2xl font-bold",
                            "Slide 2"
                        }
                    }
                }
                CarouselSlide {
                    CarouselItem {
                        div {
                            class: "h-full flex items-center justify-center bg-gradient-to-b from-green-500 to-green-600 text-white text-2xl font-bold",
                            "Slide 3"
                        }
                    }
                }
            }
                CarouselArrow { direction: CarouselArrowDirection::Prev }
                CarouselArrow { direction: CarouselArrowDirection::Next }
            CarouselDots {}
        }
    }
}
