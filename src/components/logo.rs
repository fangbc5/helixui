use dioxus::prelude::*;

#[component]
pub fn ImageLogo(
    logo_image: Asset,
    mut width: Option<String>,
    mut height: Option<String>,
) -> Element {
    // Set default values before rsx!
    if width.is_none() {
        width = Some("300px".to_string());
    }
    if height.is_none() {
        height = Some("100px".to_string());
    }

    rsx! {
        div {
            img { src: logo_image, id: "logo", width: width, height: height }
        }
    }
}

#[component]
pub fn TextLogo(logo_text: String, size: String) -> Element {
    rsx! {
        div {
            span { class: "text-2xl font-bold", "{size}", "{logo_text}"}
        }
    }
}

#[component]
pub fn IconTextLogo(logo_text: String, size: String) -> Element {
    rsx! {
        div {
            span { class: "text-2xl font-bold", "{size}", "{logo_text}"}
        }
    }
}
