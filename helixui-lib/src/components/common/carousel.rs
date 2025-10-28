//! Defines the [`Carousel`] component for creating image carousels with navigation controls.

use crate::components::use_controlled;
use dioxus::prelude::*;
use dioxus_time::use_interval;

/// The direction in which the carousel slides.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum CarouselDirection {
    /// Slides move horizontally (left/right).
    #[default]
    Horizontal,
    /// Slides move vertically (up/down).
    Vertical,
}

/// Carousel context that manages the carousel state.
#[derive(Clone, Copy)]
struct CarouselCtx {
    current_index: Memo<usize>,
    set_current_index: Callback<usize>,
    total_slides: Signal<usize>,
    set_total_slides: Callback<usize>,
    direction: Memo<CarouselDirection>,
    show_arrows: Memo<bool>,
    show_dots: Memo<bool>,
}

/// The props for the [`Carousel`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CarouselProps {
    /// The controlled current slide index.
    pub current_index: ReadSignal<Option<usize>>,

    /// The default slide index when uncontrolled.
    #[props(default = 0)]
    pub default_index: usize,

    /// Callback fired when the current slide changes.
    #[props(default)]
    pub on_index_change: Callback<usize>,

    /// Whether to auto-play the carousel.
    #[props(default = ReadSignal::new(Signal::new(false)))]
    pub auto_play: ReadSignal<bool>,

    /// Auto-play duration in milliseconds.
    #[props(default = ReadSignal::new(Signal::new(3000)))]
    pub duration: ReadSignal<u32>,

    /// The direction in which slides move.
    #[props(default = ReadSignal::new(Signal::new(CarouselDirection::Horizontal)))]
    pub direction: ReadSignal<CarouselDirection>,

    /// Whether to show navigation arrows.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub show_arrows: ReadSignal<bool>,

    /// Whether to show navigation dots.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub show_dots: ReadSignal<bool>,

    /// Additional attributes to apply to the carousel element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the carousel component.
    pub children: Element,
}

/// # Carousel
///
/// A carousel component that displays a series of slides with navigation controls.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use helixui::components::common::carousel::{Carousel, CarouselSlide, CarouselItem, CarouselArrow};
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Carousel {
///             div {
///                 for i in 1..=3 {
///                     CarouselSlide {
///                         CarouselItem {
///                             div {
///                                 class: "h-full flex items-center justify-center bg-blue-100",
///                                 "Slide {i}"
///                             }
///                         }
///                     }
///                 }
///                 CarouselArrow { direction: "prev" }
///                 CarouselArrow { direction: "next" }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`Carousel`] component defines the following data attributes you can use to control styling:
/// - `data-direction`: Indicates the carousel direction. Values are `horizontal` or `vertical`.
#[component]
pub fn Carousel(props: CarouselProps) -> Element {
    let (current_index, set_current_index) = use_controlled(
        props.current_index,
        props.default_index,
        props.on_index_change,
    );

    let mut total_slides = use_signal(|| 0usize);
    let set_total_slides = use_callback(move |count: usize| {
        total_slides.set(count);
    });

    let set_current_index_cb = use_callback(move |idx: usize| {
        set_current_index.call(idx);
    });

    // Memoize props at top level to avoid calling hooks inside a hook closure
    let direction_memo = use_memo(move || (props.direction)());
    let auto_play_memo = use_memo(move || (props.auto_play)());
    let duration_memo = use_memo(move || (props.duration)());
    let show_arrows_memo = use_memo(move || (props.show_arrows)());
    let show_dots_memo = use_memo(move || (props.show_dots)());

    use_context_provider(|| CarouselCtx {
        current_index,
        set_current_index: set_current_index_cb,
        total_slides,
        set_total_slides,
        direction: direction_memo,
        show_arrows: show_arrows_memo,
        show_dots: show_dots_memo,
    });

    // Autoplay: 基于固定短周期轮询 + 上次触发时间，支持动态变更 duration
    let mut paused = use_signal(|| false);
    let ci = current_index;
    let set_idx = set_current_index_cb;
    let total = total_slides;
    let auto = auto_play_memo;
    let dur = duration_memo;

    // 使用累计毫秒计数，避免在 wasm 中使用不支持的系统时间
    let mut elapsed_ms = use_signal(|| 0u32);

    // 当 duration 变化时，重置计时起点，确保新间隔立即生效
    {
        let dur_dep = dur;
        let mut elapsed_set = elapsed_ms;
        use_effect(move || {
            let _ = dur_dep();
            elapsed_set.set(0);
        });
    }

    // 使用较小固定周期检查是否达到用户设定的间隔
    let tick_period_ms: u32 = 50;
    let _tick = use_interval(
        std::time::Duration::from_millis(tick_period_ms as u64),
        move |()| {
            if !auto() || paused() {
                return;
            }
            let n = total();
            if n <= 1 {
                return;
            }
            // 累计时间达到设定间隔则切换
            let next_elapsed = elapsed_ms().saturating_add(tick_period_ms);
            if next_elapsed >= dur() {
                let next = (ci() + 1) % n;
                set_idx.call(next);
                elapsed_ms.set(0);
            } else {
                elapsed_ms.set(next_elapsed);
            }
        },
    );

    rsx! {
        div {
            // 默认高度，用户无需额外 class 即可看到控件
            class: "relative h-64 w-full overflow-hidden rounded-lg group",
            onmouseenter: move |_| paused.set(true),
            onmouseleave: move |_| paused.set(false),
            onfocusin: move |_| paused.set(true),
            onfocusout: move |_| paused.set(false),
            "data-direction": match (props.direction)() {
                CarouselDirection::Horizontal => "horizontal",
                CarouselDirection::Vertical => "vertical",
            },
            ..props.attributes,

            {props.children}
        }
    }
}

/// The props for the [`CarouselContent`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CarouselContentProps {
    /// Number of slides (optional, will auto-count if not provided).
    #[props(default)]
    pub count: Option<usize>,

    /// Additional attributes to apply to the carousel content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the carousel content component.
    pub children: Element,
}

/// # CarouselContent
///
/// The container for carousel slides. This should wrap all [`CarouselSlide`] components.
///
/// This must be used inside a [`Carousel`] component.
#[component]
pub fn CarouselContent(props: CarouselContentProps) -> Element {
    let ctx: CarouselCtx = use_context();
    let current_index = ctx.current_index;
    let direction = ctx.direction;

    // 使用 use_effect 来设置 count（如果提供了的话）
    let count = props.count;
    let set_total = ctx.set_total_slides;
    let current_total = ctx.total_slides;

    use_effect(move || {
        if let Some(count) = count {
            if current_total() != count {
                set_total.call(count);
            }
        }
    });

    rsx! {
        div {
            class: format!(
                "flex h-full {} transition-transform duration-500 ease-in-out",
                match direction() {
                    CarouselDirection::Horizontal => "flex-row",
                    CarouselDirection::Vertical => "flex-col",
                }
            ),
            style: match direction() {
                CarouselDirection::Horizontal => {
                    format!("transform: translateX(-{}%);", current_index() * 100)
                }
                CarouselDirection::Vertical => {
                    format!("transform: translateY(-{}%);", current_index() * 100)
                }
            },
            ..props.attributes,

            {props.children}
        }
    }
}

/// The props for the [`CarouselSlide`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CarouselSlideProps {
    /// Additional attributes to apply to the carousel slide element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the carousel slide component.
    pub children: Element,
}

/// # CarouselSlide
///
/// A single slide in the carousel. This must be used inside a [`Carousel`] component.
#[component]
pub fn CarouselSlide(props: CarouselSlideProps) -> Element {
    let ctx: CarouselCtx = use_context();
    let direction = ctx.direction;

    rsx! {
        div {
            class: match direction() {
                CarouselDirection::Horizontal => "min-w-full h-full shrink-0",
                CarouselDirection::Vertical => "w-full h-full shrink-0",
            },
            ..props.attributes,

            {props.children}
        }
    }
}

/// The props for the [`CarouselItem`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CarouselItemProps {
    /// Additional attributes to apply to the carousel item element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the carousel item component.
    pub children: Element,
}

/// # CarouselItem
///
/// The content within a slide. This must be used inside a [`CarouselSlide`] component.
#[component]
pub fn CarouselItem(props: CarouselItemProps) -> Element {
    rsx! {
        div {
            class: "w-full h-full",
            ..props.attributes,

            {props.children}
        }
    }
}

/// The props for the [`CarouselArrow`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CarouselArrowProps {
    /// The arrow direction.
    pub direction: CarouselArrowDirection,

    /// Additional attributes to apply to the carousel arrow element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the carousel arrow component.
    pub children: Element,
}

/// The direction of the carousel arrow.
#[derive(Clone, Copy, PartialEq)]
pub enum CarouselArrowDirection {
    Prev,
    Next,
}

/// # CarouselArrow
///
/// Navigation arrows for the carousel. This must be used inside a [`Carousel`] component.
#[component]
pub fn CarouselArrow(props: CarouselArrowProps) -> Element {
    let ctx: CarouselCtx = use_context();

    let handle_click = move |_| {
        let total = (ctx.total_slides)();
        // 边界检查：确保有可用的 slides
        if total == 0 || total == 1 {
            return;
        }

        let current = (ctx.current_index)();
        let new_index = match props.direction {
            CarouselArrowDirection::Prev => {
                if current == 0 {
                    total - 1
                } else {
                    current - 1
                }
            }
            CarouselArrowDirection::Next => (current + 1) % total,
        };
        ctx.set_current_index.call(new_index);
    };

    let arrow_icon = match props.direction {
        CarouselArrowDirection::Prev => "←",
        CarouselArrowDirection::Next => "→",
    };

    let position_class = match props.direction {
        CarouselArrowDirection::Prev => "left-4",
        CarouselArrowDirection::Next => "right-4",
    };

    let ctx_total = (ctx.total_slides)();
    let should_hide = ctx_total == 0 || ctx_total == 1;
    let always_show = (ctx.show_arrows)();

    rsx! {
        if !should_hide {
            button {
                class: format!(
                    "absolute top-1/2 z-10 {} h-8 w-8 -translate-y-1/2 select-none items-center justify-center rounded-full bg-black/40 text-white shadow transition-all duration-200 hover:bg-black/60 {}",
                    if always_show { "flex" } else { "hidden group-hover:flex" },
                    position_class
                ),
                onclick: handle_click,
                ..props.attributes,

                span { class: "text-sm font-bold", {arrow_icon} }

                {props.children}
            }
        }
    }
}

/// The props for the [`CarouselDots`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CarouselDotsProps {
    /// Additional attributes to apply to the carousel dots element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the carousel dots component.
    pub children: Element,
}

/// # CarouselDots
///
/// Navigation dots for the carousel. This must be used inside a [`Carousel`] component.
#[component]
pub fn CarouselDots(props: CarouselDotsProps) -> Element {
    let ctx: CarouselCtx = use_context();

    let ctx_total = (ctx.total_slides)();
    let should_hide = ctx_total <= 1;
    let always_show = (ctx.show_dots)();

    rsx! {
        if !should_hide {
            div {
                class: format!(
                    "absolute bottom-4 left-1/2 z-10 {} -translate-x-1/2 gap-2",
                    if always_show { "flex" } else { "hidden group-hover:flex" }
                ),
                ..props.attributes,

                for i in 0..ctx_total {
                    button {
                        class: if (ctx.current_index)() == i {
                            // 在浅色背景下也能看见
                            "h-3 w-6 rounded-full bg-black/80 transition-all"
                        } else {
                            "h-3 w-3 rounded-full bg-black/40 transition-all hover:bg-black/60"
                        },
                        onclick: move |_| ctx.set_current_index.call(i),
                        aria_label: format!("Go to slide {}", i + 1),
                    }
                }

                {props.children}
            }
        }
    }
}
