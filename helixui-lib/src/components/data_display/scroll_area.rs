//! Defines the [`ScrollArea`] component for creating scrollable areas with customizable scrollbars.

use dioxus::prelude::*;

/// The direction in which scrolling is allowed.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ScrollDirection {
    /// Allow vertical scrolling only.
    Vertical,
    /// Allow horizontal scrolling only.
    Horizontal,
    /// Allow scrolling in both directions.
    #[default]
    Both,
}

/// The type of scrolling behavior.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ScrollType {
    /// Browser default scrolling.
    #[default]
    Auto,
    /// Always show scrollbars.
    Always,
    /// Hide scrollbars but enable scrolling.
    Hidden,
}

/// The props for the [`ScrollArea`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ScrollAreaProps {
    /// The scroll direction.
    #[props(default = ReadSignal::new(Signal::new(ScrollDirection::Both)))]
    pub direction: ReadSignal<ScrollDirection>,

    /// Whether the scrollbars should be always visible.
    #[props(default = ReadSignal::new(Signal::new(false)))]
    pub always_show_scrollbars: ReadSignal<bool>,

    /// The scroll type.
    #[props(default = ReadSignal::new(Signal::new(ScrollType::Auto)))]
    pub scroll_type: ReadSignal<ScrollType>,

    /// Additional attributes to apply to the scroll area element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the scroll area component.
    pub children: Element,
}

/// # ScrollArea
///
/// The `ScrollArea` component creates a scrollable area. If you don't
/// have any focusable content within the scroll area, you should make the
/// scroll area focusable by adding a `tabindex` attribute.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use helixui::components::data_display::scroll_area::{ScrollArea, ScrollDirection};
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         ScrollArea {
///             direction: ReadSignal::new(Signal::new(ScrollDirection::Vertical)),
///             div {
///                 class: "p-4",
///                 for i in 1..=20 {
///                     p {
///                         class: "mb-2",
///                         "Scrollable content item {i}"
///                     }
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`ScrollArea`] component defines the following data attributes you can use to control styling:
/// - `data-scroll-direction`: Indicates the scroll direction. Values are `vertical`, `horizontal`, or `both`.
#[component]
pub fn ScrollArea(props: ScrollAreaProps) -> Element {
    let direction = props.direction;
    let scroll_type = props.scroll_type;

    let (classes, style_attr) = {
        let mut class = String::from("h-full w-full");
        let mut style = String::new();

        match scroll_type() {
            ScrollType::Auto => match direction() {
                ScrollDirection::Vertical => class.push_str(" overflow-x-hidden overflow-y-auto"),
                ScrollDirection::Horizontal => class.push_str(" overflow-x-auto overflow-y-hidden"),
                ScrollDirection::Both => class.push_str(" overflow-x-auto overflow-y-auto"),
            },
            ScrollType::Always => match direction() {
                ScrollDirection::Vertical => {
                    style = "overflow-x: hidden; overflow-y: scroll; scrollbar-gutter: stable;"
                        .to_string();
                }
                ScrollDirection::Horizontal => {
                    style = "overflow-x: scroll; overflow-y: hidden; scrollbar-gutter: stable;"
                        .to_string();
                }
                ScrollDirection::Both => {
                    style = "overflow-x: scroll; overflow-y: scroll; scrollbar-gutter: stable;"
                        .to_string();
                }
            },
            ScrollType::Hidden => {
                match direction() {
                    ScrollDirection::Vertical => {
                        style = "overflow-x: hidden; overflow-y: scroll; scrollbar-width: none; -ms-overflow-style: none;".to_string();
                    }
                    ScrollDirection::Horizontal => {
                        style = "overflow-x: scroll; overflow-y: hidden; scrollbar-width: none; -ms-overflow-style: none;".to_string();
                    }
                    ScrollDirection::Both => {
                        style = "overflow-x: scroll; overflow-y: scroll; scrollbar-width: none; -ms-overflow-style: none;".to_string();
                    }
                }
                class.push_str(" [&::-webkit-scrollbar]:hidden");
            }
        }

        (class, style)
    };

    rsx! {
        match style_attr.is_empty() {
            true => rsx! {
                div {
                    class: classes,
                    "data-scroll-direction": match direction() {
                        ScrollDirection::Vertical => "vertical",
                        ScrollDirection::Horizontal => "horizontal",
                        ScrollDirection::Both => "both",
                    },
                    ..props.attributes,

                    {props.children}
                }
            },
            false => rsx! {
                div {
                    class: classes,
                    style: style_attr,
                    "data-scroll-direction": match direction() {
                        ScrollDirection::Vertical => "vertical",
                        ScrollDirection::Horizontal => "horizontal",
                        ScrollDirection::Both => "both",
                    },
                    ..props.attributes,

                    {props.children}
                }
            }
        }
    }
}
