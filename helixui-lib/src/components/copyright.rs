use dioxus::prelude::*;

#[component]
pub fn Copyright() -> Element {
    rsx! {
        div {
            id: "copyright",
            class: "flex justify-center items-center",
            "Copyright@2025 HelixUI"
        }
    }
}