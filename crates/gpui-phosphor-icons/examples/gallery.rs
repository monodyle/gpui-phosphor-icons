//! Every Phosphor icon in a scrolling grid.
//!
//! ```text
//! cargo run --example gallery --features catalog
//! ```

use gpui::{
    App, Application, Bounds, Context, MouseButton, Window, WindowBounds, WindowControlArea,
    WindowDecorations, WindowOptions, div, prelude::*, px, rgb, size, uniform_list,
};
use gpui_phosphor_icons::{Icon, IconWeight, PhosphorAssets, XIcon, catalog};

const COLUMNS: usize = 8;

/// Page background.
const BACKGROUND: u32 = 0xedeae3;
/// Page text.
const FOREGROUND: u32 = 0x000000;
/// Top bar background.
const BAR_BACKGROUND: u32 = 0x343330;
/// Top bar text.
const BAR_FOREGROUND: u32 = 0xffffff;
/// Top bar background of the selected weight.
const BAR_ACTIVE_BACKGROUND: u32 = 0x000000;

struct Gallery {
    weight: IconWeight,
}

impl Gallery {
    fn rows() -> usize {
        catalog::ALL.len().div_ceil(COLUMNS)
    }
}

impl Render for Gallery {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let weight = self.weight;

        let tabs = div()
            .id("top-bar")
            .flex()
            .items_center()
            .gap_2()
            .pl_3()
            .pr_1()
            .py_2()
            .bg(rgb(BAR_BACKGROUND))
            .text_color(rgb(BAR_FOREGROUND))
            .children(IconWeight::ALL.map(|option| {
                div()
                    .id(option.as_str())
                    .px_3()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .when(option == weight, |this| this.bg(rgb(BAR_ACTIVE_BACKGROUND)))
                    .child(option.as_str())
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.weight = option;
                        cx.notify();
                    }))
            }))
            .child(
                // Empty space that drags the window.
                div()
                    .id("drag-area")
                    .flex_1()
                    .h(px(28.))
                    .window_control_area(WindowControlArea::Drag)
                    .on_mouse_down(MouseButton::Left, |_, window, _| {
                        window.start_window_move();
                    }),
            )
            .child(
                div()
                    .id("close")
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(38.))
                    .h(px(28.))
                    .cursor_pointer()
                    .window_control_area(WindowControlArea::Close)
                    .hover(|this| this.bg(rgb(BAR_ACTIVE_BACKGROUND)))
                    .child(XIcon::new().weight(IconWeight::Bold).size_4())
                    .on_click(|_, window, _| window.remove_window()),
            );

        let grid = uniform_list(
            "icons",
            Self::rows(),
            cx.processor(move |this, range: std::ops::Range<usize>, _window, _cx| {
                range
                    .map(|row| {
                        let start = row * COLUMNS;
                        let end = (start + COLUMNS).min(catalog::ALL.len());
                        div()
                            .flex()
                            .children(catalog::ALL[start..end].iter().map(|data| {
                                div()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .gap_2()
                                    .w(px(120.))
                                    .p_2()
                                    .child(Icon::new(data).weight(this.weight).size_8())
                                    .child(div().text_xs().child(data.name))
                            }))
                    })
                    .collect()
            }),
        )
        .h_full();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(BACKGROUND))
            .text_color(rgb(FOREGROUND))
            .text_sm()
            .child(tabs)
            .child(div().flex_1().overflow_hidden().px_3().child(grid))
    }
}

fn main() {
    Application::new()
        .with_assets(PhosphorAssets::new())
        .run(|cx: &mut App| {
            let bounds = Bounds::centered(None, size(px(1000.), px(700.)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: None,
                    window_decorations: Some(WindowDecorations::Client),
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|_| Gallery {
                        weight: IconWeight::Duotone,
                    })
                },
            )
            .unwrap();
            cx.activate(true);
        });
}
