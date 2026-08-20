//! [Phosphor icons](https://phosphoricons.com) for [gpui](https://www.gpui.rs).
//!
//! Every one of the 1512 Phosphor icons has its own type, and every icon draws
//! in any of the six weights.
//!
//! ```no_run
//! use gpui::*;
//! use gpui_phosphor_icons::*;
//!
//! # fn render() -> impl IntoElement {
//! div().child(GhostIcon::new().duotone())
//! # }
//! ```
//!
//! # Set up
//!
//! gpui reads SVG files through an [`AssetSource`](gpui::AssetSource), so give
//! [`PhosphorAssets`] to your application before you run it:
//!
//! ```no_run
//! use gpui::Application;
//! use gpui_phosphor_icons::PhosphorAssets;
//!
//! Application::new().with_assets(PhosphorAssets::new()).run(|_cx| {});
//! ```
//!
//! If your application already has assets, wrap them with
//! [`PhosphorAssets::with_fallback`], or call [`load`] from your own source.
//!
//! # Style
//!
//! [`Icon`] implements [`Styled`](gpui::Styled), so size and color work like
//! they do on any other element. An icon is `1rem` wide and takes the text
//! color of the window until you say otherwise.
//!
//! ```no_run
//! # use gpui::*;
//! # use gpui_phosphor_icons::*;
//! # fn render() -> impl IntoElement {
//! HeartIcon::new().fill().size_8().text_color(red())
//! # }
//! ```
//!
//! # Binary size
//!
//! The icon files sit behind the type of each icon, so the linker keeps only
//! the icons that your code names. Turn off the weights you never use to drop
//! them as well:
//!
//! ```toml
//! gpui-phosphor-icons = { version = "0.1", default-features = false, features = ["regular", "fill"] }
//! ```
//!
//! The `catalog` feature adds [`catalog::ALL`], a table of every icon for
//! lookup by name. It holds every icon file in the binary.

#![deny(missing_docs)]

#[macro_use]
mod macros;

mod assets;
mod icon;
mod weight;

pub mod icons;

#[cfg(feature = "catalog")]
pub mod catalog;

#[cfg(test)]
mod tests;

pub use assets::{ASSET_PREFIX, PhosphorAssets, load, paths, register};
pub use icon::{Icon, IconAsset, IconData};
pub use icons::*;
pub use weight::IconWeight;
