//! A video player skin: a placeholder frame, a timeline and a row of controls.
//!
//! ```text
//! cargo run --example video_player
//! ```

use std::cell::Cell;
use std::rc::Rc;

use gpui::{
    App, Application, Bounds, Context, Div, MouseButton, MouseDownEvent, MouseMoveEvent,
    MouseUpEvent, Pixels, Point, Stateful, Window, WindowBounds, WindowControlArea,
    WindowDecorations, WindowOptions, canvas, div, prelude::*, px, relative, rgb, rgba, size,
};
use gpui_phosphor_icons::{
    CornersOutIcon, GearIcon, Icon, PauseIcon, PhosphorAssets, PictureInPictureIcon, PlayIcon,
    ScreencastIcon, SkipForwardIcon, SpeakerHighIcon, SpeakerSlashIcon, XIcon,
};

const BACKGROUND: u32 = 0x343330;
const FOREGROUND: u32 = 0xffffff;
const DISABLED: u32 = 0x5d5c59;

const TIMELINE: u32 = 0xffffff33;
const TIMELINE_TRACK: u32 = 0xffffff4d;

/// Where the video starts, in seconds.
const ELAPSED: u32 = 24;
/// How long the video runs, in seconds.
const DURATION: u32 = 123;

struct VideoPlayer {
    playing: bool,
    muted: bool,
    /// How much of the video has played, in seconds.
    elapsed: u32,
    /// True while the viewer holds the timeline.
    scrubbing: bool,
    /// True while the mouse sits on the timeline.
    hovering: bool,
    /// Where the timeline sits, so a mouse position becomes a time.
    timeline: Rc<Cell<Bounds<Pixels>>>,
}

impl VideoPlayer {
    /// How far the video has played, from 0 to 1.
    fn progress(&self) -> f32 {
        self.elapsed as f32 / DURATION as f32
    }

    /// Move the play head to the mouse.
    fn seek(&mut self, position: Point<Pixels>, cx: &mut Context<Self>) {
        let bounds = self.timeline.get();
        if bounds.size.width <= px(0.) {
            return;
        }
        let fraction = ((position.x - bounds.origin.x) / bounds.size.width).clamp(0., 1.);
        self.elapsed = (fraction * DURATION as f32).round() as u32;
        cx.notify();
    }
}

/// `m:ss`, the way a player writes a time.
fn timestamp(seconds: u32) -> String {
    format!("{}:{:02}", seconds / 60, seconds % 60)
}

/// A control the viewer can press.
fn button(id: &'static str, icon: Icon) -> Stateful<Div> {
    div()
        .id(id)
        .flex()
        .cursor_pointer()
        .text_color(rgb(FOREGROUND))
        .child(icon.size(px(20.)))
}

/// A control the viewer cannot press.
fn disabled_button(icon: Icon) -> Div {
    div()
        .flex()
        .text_color(rgb(DISABLED))
        .child(icon.size(px(20.)))
}

impl Render for VideoPlayer {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let play = if self.playing {
            PauseIcon::new().fill()
        } else {
            PlayIcon::new().fill()
        };
        let speaker = if self.muted {
            SpeakerSlashIcon::new().fill()
        } else {
            SpeakerHighIcon::new().fill()
        };

        // `occlude` keeps the drag area below from taking the click.
        let close = div()
            .id("close")
            .absolute()
            .top(px(16.))
            .right(px(16.))
            .occlude()
            .cursor_pointer()
            .text_color(rgb(FOREGROUND))
            .child(XIcon::new().bold().size(px(20.)))
            .on_click(|_, window, _| window.remove_window());

        // Dragging the frame moves the window, because the window has no title bar.
        let video = div()
            .flex_1()
            .w_full()
            .window_control_area(WindowControlArea::Drag);

        let measure = self.timeline.clone();
        let timeline = div()
            .id("timeline")
            .relative()
            .w_full()
            // The height comes from state, because gpui lays an element out
            // before it knows about hover.
            .h(px(if self.hovering || self.scrubbing {
                8.
            } else {
                4.
            }))
            .bg(rgba(TIMELINE))
            .cursor_pointer()
            .on_hover(cx.listener(|this, hovering: &bool, _, cx| {
                this.hovering = *hovering;
                cx.notify();
            }))
            .child(
                canvas(move |bounds, _, _| measure.set(bounds), |_, _, _, _| {})
                    .absolute()
                    .size_full(),
            )
            .child(
                div()
                    .h_full()
                    .w(relative(self.progress()))
                    .bg(rgba(TIMELINE_TRACK)),
            )
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, event: &MouseDownEvent, _, cx| {
                    this.scrubbing = true;
                    this.seek(event.position, cx);
                }),
            );

        let left = div()
            .flex()
            .items_center()
            .gap(px(16.))
            .child(button("play", play).on_click(cx.listener(|this, _, _, cx| {
                this.playing = !this.playing;
                cx.notify();
            })))
            .child(button("skip-forward", SkipForwardIcon::new().fill()))
            .child(
                button("speaker", speaker).on_click(cx.listener(|this, _, _, cx| {
                    this.muted = !this.muted;
                    cx.notify();
                })),
            )
            .child(
                div()
                    .text_size(px(13.))
                    .text_color(rgb(FOREGROUND))
                    .child(format!(
                        "{} / {}",
                        timestamp(self.elapsed),
                        timestamp(DURATION)
                    )),
            );

        let right = div()
            .flex()
            .items_center()
            .gap(px(16.))
            .child(disabled_button(GearIcon::new().fill()))
            .child(disabled_button(PictureInPictureIcon::new().fill()))
            .child(disabled_button(ScreencastIcon::new().fill()))
            .child(disabled_button(CornersOutIcon::new().fill()));

        let controls = div()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .px(px(16.))
            .py(px(12.))
            .child(left)
            .child(right);

        div()
            .relative()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(BACKGROUND))
            .text_color(rgb(FOREGROUND))
            .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _, cx| {
                if this.scrubbing {
                    this.seek(event.position, cx);
                }
            }))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, _: &MouseUpEvent, _, cx| {
                    if this.scrubbing {
                        this.scrubbing = false;
                        cx.notify();
                    }
                }),
            )
            .child(video)
            .child(div().w_full().px(px(16.)).child(timeline))
            .child(controls)
            // The close button paints last, so it takes the mouse before the
            // area that drags the window.
            .child(close)
    }
}

fn main() {
    Application::new()
        .with_assets(PhosphorAssets::new())
        .run(|cx: &mut App| {
            let bounds = Bounds::centered(None, size(px(640.), px(400.)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: None,
                    window_decorations: Some(WindowDecorations::Client),
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|_| VideoPlayer {
                        playing: false,
                        muted: false,
                        elapsed: ELAPSED,
                        scrubbing: false,
                        hovering: false,
                        timeline: Rc::new(Cell::new(Bounds::default())),
                    })
                },
            )
            .unwrap();
            cx.activate(true);
        });
}
