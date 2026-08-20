//! Every Phosphor icon in a scrolling grid.
//!
//! ```text
//! cargo run --example gallery --features catalog
//! ```

use gpui::{
    App, Application, Bounds, Context, Window, WindowBounds, WindowOptions, div, prelude::*, px,
    rgb, size, uniform_list,
};
use gpui_phosphor_icons::{Icon, IconWeight, PhosphorAssets, catalog};

const COLUMNS: usize = 8;

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
            .flex()
            .gap_2()
            .p_3()
            .children(IconWeight::ALL.map(|option| {
                div()
                    .id(option.as_str())
                    .px_3()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .bg(if option == weight {
                        rgb(0x3b5bdb)
                    } else {
                        rgb(0x2a2a30)
                    })
                    .child(option.as_str())
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.weight = option;
                        cx.notify();
                    }))
            }));

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
                                    .child(
                                        div().text_xs().text_color(rgb(0x9a9aa2)).child(data.name),
                                    )
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
            .bg(rgb(0x1b1b1f))
            .text_color(rgb(0xf5f5f5))
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
