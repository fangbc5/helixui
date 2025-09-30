use crate::{components::ImageLogo, Route};
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");
const LOGO_IMAGE: Asset = asset!("/assets/images/logo.png");

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This layout component wraps the UI of [Route::Home] and [Route::Blog] in a common navbar. The contents of the Home and Blog
/// routes will be rendered under the outlet inside this component
#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }
        div {
            id: "navbar",
            div { id: "navbar-logo", ImageLogo { logo_image: LOGO_IMAGE, width: None, height: None } }
            div { id: "navbar-menu",
                Link {
                    to: Route::Home {},
                    "Home"
                }
                Link {
                    to: Route::Blog { id: 1 },
                    "Blog"
                }
            }
            div { id: "navbar-buttons",

            }
        }

        // The `Outlet` component is used to render the next component inside the layout. In this case, it will render either
        // the [`Home`] or [`Blog`] component depending on the current route.
        Outlet::<Route> {}
    }
}
