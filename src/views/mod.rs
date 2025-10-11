//! The views module contains the components for all Layouts and Routes for our app. Each layout and route in our [`Route`]
//! enum will render one of these components.
//!
//!
//! The [`Home`] and [`Blog`] components will be rendered when the current route is [`Route::Home`] or [`Route::Blog`] respectively.
//!
//!
//! The [`Navbar`] component will be rendered on all pages of our app since every page is under the layout. The layout defines
//! a common wrapper around all child routes.

mod components;
mod components_index;
mod docs;
mod home;
mod layout;

pub use components::{BadgePage, ButtonPage, DialogPage, IconPage, MessagePage};
pub use components_index::ComponentsIndex;
pub use docs::{Guide, Introduction, QuickStart, Version};
pub use home::Home;
pub use layout::{ComponentsSidebar, DocPage, DocsSidebar, TopNavbar};
