use gpui::{
    App, InteractiveElement, Interactivity, IntoElement, RenderOnce, SharedString, StyleRefinement,
    Styled, Svg, Transformation, Window, rems, svg,
};

use crate::assets::register;
use crate::weight::IconWeight;

/// One weight of one icon: where it lives and what it holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IconAsset {
    /// The asset path, for example `phosphor/duotone/ghost.svg`.
    pub path: &'static str,
    /// The SVG file itself.
    pub bytes: &'static [u8],
}

/// Every weight of one icon that this build holds.
///
/// The generated icon types own this data. A weight is `None` when its cargo
/// feature is off.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IconData {
    /// The kebab-case Phosphor name, for example `ghost`.
    pub name: &'static str,
    /// One slot per [`IconWeight`], in [`IconWeight::ALL`] order.
    pub weights: [Option<IconAsset>; 6],
}

impl IconData {
    /// The asset for `weight`.
    ///
    /// When that weight is not in the build, this falls back to the regular
    /// weight, then to the first weight that is in the build.
    pub fn asset(&self, weight: IconWeight) -> Option<IconAsset> {
        if let Some(asset) = self.weights[weight.index()] {
            return Some(asset);
        }
        if let Some(asset) = self.weights[IconWeight::Regular.index()] {
            return Some(asset);
        }
        self.weights.iter().find_map(|asset| *asset)
    }

    /// The weights that this build holds.
    pub fn available_weights(&self) -> impl Iterator<Item = IconWeight> + '_ {
        IconWeight::ALL
            .into_iter()
            .filter(|weight| self.weights[weight.index()].is_some())
    }
}

/// A Phosphor icon element.
///
/// Build one from a generated icon type, then set the weight and the style:
///
/// ```no_run
/// use gpui::*;
/// use gpui_phosphor_icons::*;
///
/// # fn render() -> impl IntoElement {
/// div().child(GhostIcon::new().duotone().size_6().text_color(gpui::red()))
/// # }
/// ```
///
/// The icon takes the text color of the window when you do not set one, so it
/// matches the text around it.
#[derive(IntoElement)]
pub struct Icon {
    base: Svg,
    data: &'static IconData,
    weight: IconWeight,
}

impl Icon {
    /// Build an icon from static icon data, at the regular weight.
    pub fn new(data: &'static IconData) -> Self {
        Self {
            base: svg().flex_none().size(rems(1.)),
            data,
            weight: IconWeight::Regular,
        }
    }

    /// The kebab-case Phosphor name of this icon, for example `ghost`.
    pub fn name(&self) -> &'static str {
        self.data.name
    }

    /// The static data behind this icon.
    pub fn data(&self) -> &'static IconData {
        self.data
    }

    /// The weight this icon draws with.
    pub fn current_weight(&self) -> IconWeight {
        self.weight
    }

    /// Draw the icon with the given weight.
    pub fn weight(mut self, weight: IconWeight) -> Self {
        self.weight = weight;
        self
    }

    /// Draw the icon with hairline strokes.
    pub fn thin(self) -> Self {
        self.weight(IconWeight::Thin)
    }

    /// Draw the icon with light strokes.
    pub fn light(self) -> Self {
        self.weight(IconWeight::Light)
    }

    /// Draw the icon with the default strokes.
    pub fn regular(self) -> Self {
        self.weight(IconWeight::Regular)
    }

    /// Draw the icon with heavy strokes.
    pub fn bold(self) -> Self {
        self.weight(IconWeight::Bold)
    }

    /// Draw the icon as a solid shape.
    pub fn fill(self) -> Self {
        self.weight(IconWeight::Fill)
    }

    /// Draw the icon with a solid shape behind regular strokes.
    pub fn duotone(self) -> Self {
        self.weight(IconWeight::Duotone)
    }

    /// Rotate, scale or move the icon while it draws.
    pub fn transform(mut self, transformation: Transformation) -> Self {
        self.base = self.base.with_transformation(transformation);
        self
    }
}

impl Styled for Icon {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for Icon {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl RenderOnce for Icon {
    fn render(mut self, window: &mut Window, _: &mut App) -> impl IntoElement {
        let Some(asset) = self.data.asset(self.weight) else {
            return self.base;
        };

        register(asset.path, asset.bytes);

        let has_color = self
            .base
            .style()
            .text
            .as_ref()
            .is_some_and(|text| text.color.is_some());
        if !has_color {
            let color = window.text_style().color;
            self.base = self.base.text_color(color);
        }

        self.base.path(SharedString::new_static(asset.path))
    }
}
