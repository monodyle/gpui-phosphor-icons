/// The stroke weight (Phosphor calls it "style") of an icon.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum IconWeight {
    /// Hairline strokes.
    Thin,
    /// Light strokes.
    Light,
    /// The default weight.
    #[default]
    Regular,
    /// Heavy strokes.
    Bold,
    /// Solid shapes.
    Fill,
    /// Solid shapes at 20% opacity behind regular strokes.
    Duotone,
}

impl IconWeight {
    /// Every weight, in Phosphor order.
    pub const ALL: [IconWeight; 6] = [
        IconWeight::Thin,
        IconWeight::Light,
        IconWeight::Regular,
        IconWeight::Bold,
        IconWeight::Fill,
        IconWeight::Duotone,
    ];

    /// The lowercase name of the weight, as used by the Phosphor asset folders.
    pub const fn as_str(self) -> &'static str {
        match self {
            IconWeight::Thin => "thin",
            IconWeight::Light => "light",
            IconWeight::Regular => "regular",
            IconWeight::Bold => "bold",
            IconWeight::Fill => "fill",
            IconWeight::Duotone => "duotone",
        }
    }

    /// The index of the weight in [`IconWeight::ALL`].
    pub const fn index(self) -> usize {
        self as usize
    }

    /// Read a weight from its lowercase name.
    pub fn from_name(name: &str) -> Option<Self> {
        IconWeight::ALL.into_iter().find(|w| w.as_str() == name)
    }
}

impl std::fmt::Display for IconWeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
