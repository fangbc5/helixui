// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;

use views::{
    AlertPage, AvatarPage, BadgePage, ButtonPage, CalendarPage, CardPage, CarouselPage,
    CheckboxPage, CollapsiblePage, ComponentsPage, DialogPage, DividerPage, DropdownPage,
    EllipsisPage, FlexPage, FloatButtonPage, GradientTextPage, GridPage, Guide, Home, IconPage,
    InputPage, Introduction, LayoutPage, QuickStart, RadioPage, ScrollAreaPage, SpacePage,
    SplitPage, SwitchPage, TablePage, TagPage, TextAreaPage, ToastPage, TooltipPage, TopNavbar,
    Version, WatermarkPage,
};

/// i18n internationalization support
mod i18n;
/// Theme management
mod theme;
/// Define a views module that contains the UI for all Layouts and Routes for our app.
mod views;

/// The Route enum is used to define the structure of internal routes in our app. All route enums need to derive
/// the [`Routable`] trait, which provides the necessary methods for the router to work.
/// 
/// Each variant represents a different URL pattern that can be matched by the router. If that pattern is matched,
/// the components for that route will be rendered.
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(TopNavbar)]
        #[route("/")]
        Home {},
        #[route("/components")]
        ComponentsPage {},
    
        #[route("/docs/introduction")]
        Introduction {},
        #[route("/docs/quick-start")]
        QuickStart {},
        #[route("/docs/guide")]
        Guide {},
        #[route("/docs/version")]
        Version {},
        
        #[route("/component/avatar")]
        AvatarPage {},
        #[route("/component/badge")]
        BadgePage {},
        #[route("/component/button")]
        ButtonPage {},
        #[route("/component/card")]
        CardPage {},
        #[route("/component/carousel")]
        CarouselPage {},
        #[route("/component/collapsible")]
        CollapsiblePage {},
        #[route("/component/divider")]
        DividerPage {},
        #[route("/component/dropdown")]
        DropdownPage {},
        #[route("/component/icon")]
        IconPage {},
        #[route("/component/flex")]
        FlexPage {},
        #[route("/component/layout")]
        LayoutPage {},
        #[route("/component/grid")]
        GridPage {},
        #[route("/component/space")]
        SpacePage {},
        #[route("/component/split")]
        SplitPage {},
        #[route("/component/alert")]
        AlertPage {},
        #[route("/component/dialog")]
        DialogPage {},
        #[route("/component/toast")]
        ToastPage {},
        #[route("/component/tooltip")]
        TooltipPage {},
        #[route("/component/input")]
        InputPage {},
        #[route("/component/textarea")]
        TextAreaPage {},
        #[route("/component/checkbox")]
        CheckboxPage {},
        #[route("/component/radio")]
        RadioPage {},
        #[route("/component/switch")]
        SwitchPage {},
        #[route("/component/table")]
        TablePage {},
        #[route("/component/calendar")]
        CalendarPage {},
        #[route("/component/scroll-area")]
        ScrollAreaPage {},
        #[route("/component/tag")]
        TagPage {},
        #[route("/component/watermark")]
        WatermarkPage {},
        #[route("/component/float-button")]
        FloatButtonPage {},
        #[route("/component/ellipsis")]
        EllipsisPage {},
        #[route("/component/gradient-text")]
        GradientTextPage {},
}

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
const FAVICON: Asset = asset!("/assets/favicon.ico");
// The asset macro also minifies some assets like CSS and JS to make bundled smaller
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it with the platform feature
    // you have enabled
    dioxus::launch(App);
}

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
    // The `rsx!` macro lets us define HTML inside of rust. It expands to an Element with all of our HTML inside.
    rsx! {
        // In addition to element and text (which we will see later), rsx can contain other components. In this case,
        // we are using the `document::Link` component to add a link to our favicon and main CSS file into the head of our app.
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: MAIN_CSS }
        document::Stylesheet { href: TAILWIND_CSS }

        // the layouts and components for the active route.
        Router::<Route> {}
    }
}
