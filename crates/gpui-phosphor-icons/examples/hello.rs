//! A small borderless window with a few icons.
//!
//! ```text
//! cargo run --example hello
//! ```

use gpui::{
    App, Application, Bounds, Context, Window, WindowBounds, WindowDecorations, WindowOptions, div,
    prelude::*, px, rgb, size,
};
use gpui_phosphor_icons::{LightningIcon, PhosphorAssets, XIcon};

const REPO: &str = "https://github.com/monodyle/gpui-phosphor-icons";

struct Hello;

impl Render for Hello {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(8.))
            .size_full()
            .p(px(24.))
            .bg(rgb(0xf1c970))
            .text_color(rgb(0x343330))
            .child(
                div().flex().justify_end().child(
                    div()
                        .id("close")
                        .cursor_pointer()
                        .child(XIcon::new().bold().size(px(20.)))
                        .on_click(|_, window, _| window.remove_window()),
                ),
            )
            .child(
                div()
                    .flex()
                    .flex_1()
                    .items_center()
                    .justify_center()
                    .child(LightningIcon::new().duotone().size(px(128.))),
            )
            .child(
                div()
                    .id("repo")
                    .w_full()
                    .text_center()
                    .text_size(px(20.))
                    .cursor_pointer()
                    .hover(|style| style.underline())
                    .child("gpui-phosphor-icons")
                    .on_click(|_, _, cx| cx.open_url(REPO)),
            )
    }
}

fn main() {
    Application::new()
        .with_assets(PhosphorAssets::new())
        .run(|cx: &mut App| {
            let bounds = Bounds::centered(None, size(px(416.), px(416.)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: None,
                    is_resizable: false,
                    window_decorations: Some(WindowDecorations::Client),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| Hello),
            )
            .unwrap();
            cx.activate(true);
        });
}
