use crate::views::{
    AlertPage, AvatarPage, BadgePage, ButtonPage, CalendarPage, CardPage, CarouselPage,
    CheckboxPage, CollapsiblePage, ComponentsPage, DialogPage, DividerPage, DropdownPage,
    EllipsisPage, FlexPage, FloatButtonPage, GradientTextPage, GridPage, Guide, Home, IconPage,
    InputPage, Introduction, LayoutPage, QuickStart, RadioPage, ScrollAreaPage, SelectPage,
    SpacePage, SplitPage, SwitchPage, TablePage, TagPage, TextAreaPage, ToastPage, TooltipPage,
    TopNavbar, Version, WatermarkPage,
};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
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
        #[route("/component/select")]
        SelectPage {},
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
