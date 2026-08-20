//! Internal macros that back the generated icon types.
//!
//! There is one `asset_*` macro per weight. When the feature of a weight is
//! off, the macro expands to `None`, so no file of that weight reaches the
//! binary.

#[cfg(feature = "thin")]
macro_rules! asset_thin {
    ($name:literal) => {
        Some($crate::IconAsset {
            path: concat!("phosphor/thin/", $name, ".svg"),
            bytes: include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/thin/",
                $name,
                ".svg"
            )),
        })
    };
}
#[cfg(not(feature = "thin"))]
macro_rules! asset_thin {
    ($name:literal) => {
        None
    };
}

#[cfg(feature = "light")]
macro_rules! asset_light {
    ($name:literal) => {
        Some($crate::IconAsset {
            path: concat!("phosphor/light/", $name, ".svg"),
            bytes: include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/light/",
                $name,
                ".svg"
            )),
        })
    };
}
#[cfg(not(feature = "light"))]
macro_rules! asset_light {
    ($name:literal) => {
        None
    };
}

#[cfg(feature = "regular")]
macro_rules! asset_regular {
    ($name:literal) => {
        Some($crate::IconAsset {
            path: concat!("phosphor/regular/", $name, ".svg"),
            bytes: include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/regular/",
                $name,
                ".svg"
            )),
        })
    };
}
#[cfg(not(feature = "regular"))]
macro_rules! asset_regular {
    ($name:literal) => {
        None
    };
}

#[cfg(feature = "bold")]
macro_rules! asset_bold {
    ($name:literal) => {
        Some($crate::IconAsset {
            path: concat!("phosphor/bold/", $name, ".svg"),
            bytes: include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/bold/",
                $name,
                ".svg"
            )),
        })
    };
}
#[cfg(not(feature = "bold"))]
macro_rules! asset_bold {
    ($name:literal) => {
        None
    };
}

#[cfg(feature = "fill")]
macro_rules! asset_fill {
    ($name:literal) => {
        Some($crate::IconAsset {
            path: concat!("phosphor/fill/", $name, ".svg"),
            bytes: include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/fill/",
                $name,
                ".svg"
            )),
        })
    };
}
#[cfg(not(feature = "fill"))]
macro_rules! asset_fill {
    ($name:literal) => {
        None
    };
}

#[cfg(feature = "duotone")]
macro_rules! asset_duotone {
    ($name:literal) => {
        Some($crate::IconAsset {
            path: concat!("phosphor/duotone/", $name, ".svg"),
            bytes: include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/duotone/",
                $name,
                ".svg"
            )),
        })
    };
}
#[cfg(not(feature = "duotone"))]
macro_rules! asset_duotone {
    ($name:literal) => {
        None
    };
}

/// Declare one icon type, for example `declare_icon!(GhostIcon, "ghost");`.
macro_rules! declare_icon {
    ($(#[$meta:meta])* $ident:ident, $name:literal) => {
        #[doc = concat!("The `", $name, "` icon.")]
        $(#[$meta])*
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $ident;

        impl $ident {
            #[doc = concat!("The Phosphor name of this icon: `", $name, "`.")]
            pub const NAME: &'static str = $name;

            #[doc = concat!("The static data of `", $name, "`, one slot per weight.")]
            pub const DATA: &'static $crate::IconData = &$crate::IconData {
                name: $name,
                weights: [
                    asset_thin!($name),
                    asset_light!($name),
                    asset_regular!($name),
                    asset_bold!($name),
                    asset_fill!($name),
                    asset_duotone!($name),
                ],
            };

            #[doc = concat!("Build the `", $name, "` icon at the regular weight.")]
            #[allow(clippy::new_ret_no_self)]
            pub fn new() -> $crate::Icon {
                $crate::Icon::new(Self::DATA)
            }
        }

        impl From<$ident> for $crate::Icon {
            fn from(_: $ident) -> Self {
                $ident::new()
            }
        }

        impl ::gpui::IntoElement for $ident {
            type Element = <$crate::Icon as ::gpui::IntoElement>::Element;

            fn into_element(self) -> Self::Element {
                ::gpui::IntoElement::into_element($ident::new())
            }
        }
    };
}
