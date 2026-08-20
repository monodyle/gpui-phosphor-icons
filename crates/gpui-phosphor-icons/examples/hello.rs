//! The smallest example: one icon in every weight.
//!
//! ```text
//! cargo run --example hello
//! ```

use gpui::{
    App, Application, Bounds, Context, Window, WindowBounds, WindowOptions, div, prelude::*, px,
    rgb, size,
};
use gpui_phosphor_icons::{GhostIcon, HeartIcon, IconWeight, PhosphorAssets};

struct Hello;

impl Render for Hello {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_8()
            .items_center()
            .justify_center()
            .size_full()
            .bg(rgb(0x1b1b1f))
            .text_color(rgb(0xf5f5f5))
            .child(GhostIcon::new().duotone().size(px(96.)))
            .child(
                div().flex().gap_4().items_center().children(
                    IconWeight::ALL.map(|weight| HeartIcon::new().weight(weight).size_8()),
                ),
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .items_center()
                    .text_color(rgb(0xff5555))
                    .child(HeartIcon::new().fill().size_8())
                    .child("Phosphor icons for gpui"),
            )
    }
}

fn main() {
    Application::new()
        .with_assets(PhosphorAssets::new())
        .run(|cx: &mut App| {
            let bounds = Bounds::centered(None, size(px(520.), px(420.)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| Hello),
            )
            .unwrap();
            cx.activate(true);
        });
}
