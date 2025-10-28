use crate::views::layout::{ComponentsSidebar, TocItem};
use crate::views::DocPage;
use dioxus::prelude::*;
use helixui::components::{
    Carousel, CarouselArrow, CarouselArrowDirection, CarouselContent, CarouselDirection,
    CarouselDots, CarouselItem, CarouselSlide, DemoBox,
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
            }
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
                    "API"
                }
                h3 {
                    "Carousel Props"
                }
                table {
                    class: "api-table",
                    thead {
                        tr {
                            th { "属性" }
                            th { "说明" }
                            th { "类型" }
                            th { "默认值" }
                        }
                    }
                    tbody {
                        tr {
                            td { "current_index" }
                            td { "当前索引（受控）" }
                            td { "ReadSignal<Option<usize>>" }
                            td { "-" }
                        }
                        tr {
                            td { "default_index" }
                            td { "默认索引（非受控）" }
                            td { "usize" }
                            td { "0" }
                        }
                        tr {
                            td { "on_index_change" }
                            td { "索引变化回调" }
                            td { "Callback<usize>" }
                            td { "-" }
                        }
                        tr {
                            td { "auto_play" }
                            td { "是否自动播放" }
                            td { "ReadSignal<bool>" }
                            td { "false" }
                        }
                        tr {
                            td { "duration" }
                            td { "自动播放间隔（毫秒）" }
                            td { "ReadSignal<u32>" }
                            td { "3000" }
                        }
                        tr {
                            td { "direction" }
                            td { "轮播方向" }
                            td { "ReadSignal<CarouselDirection>" }
                            td { "Horizontal" }
                        }
                        tr {
                            td { "show_arrows" }
                            td { "是否显示箭头" }
                            td { "ReadSignal<bool>" }
                            td { "true" }
                        }
                        tr {
                            td { "show_dots" }
                            td { "是否显示指示点" }
                            td { "ReadSignal<bool>" }
                            td { "true" }
                        }
                    }
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
    rsx! {
        Carousel {
            auto_play: ReadSignal::new(Signal::new(true)),
            duration: ReadSignal::new(Signal::new(3000)),
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
