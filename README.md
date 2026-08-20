# gpui-phosphor-icons

[Phosphor icons](https://phosphoricons.com) for [gpui](https://www.gpui.rs).

All 1512 icons, all six weights, one Rust type per icon.

```rust
use gpui::*;
use gpui_phosphor_icons::*;

div().child(GhostIcon::new().duotone())
```

## Install

The crate is not on crates.io yet, so take it from git:

```toml
[dependencies]
gpui = "0.2"
gpui-phosphor-icons = { git = "https://github.com/monodyle/gpui-phosphor-icons" }
```

## Set up

gpui reads SVG files through an `AssetSource`, so hand `PhosphorAssets` to your application:

```rust
use gpui::Application;
use gpui_phosphor_icons::PhosphorAssets;

Application::new()
    .with_assets(PhosphorAssets::new())
    .run(|cx| { /* ... */ });
```

If your application already has assets, put yours behind the icons:

```rust
PhosphorAssets::with_fallback(MyAssets)
```

Or serve the icons from your own source:

```rust
impl AssetSource for MyAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if let Some(bytes) = gpui_phosphor_icons::load(path) {
            return Ok(Some(bytes));
        }
        // ...your own assets here
        Ok(None)
    }

    fn list(&self, _path: &str) -> Result<Vec<SharedString>> {
        Ok(vec![])
    }
}
```

## Use

Every icon has a type named after it: `ghost` becomes `GhostIcon`, `arrow-right` becomes `ArrowRightIcon`.
`Icon::new` gives you the regular weight; the weight methods change it.

```rust
GhostIcon::new()            // regular
GhostIcon::new().thin()
GhostIcon::new().light()
GhostIcon::new().bold()
GhostIcon::new().fill()
GhostIcon::new().duotone()
GhostIcon::new().weight(IconWeight::Duotone)
```

`Icon` implements `Styled` and `InteractiveElement`, so size, color, margin and mouse handlers work as
they do on any other element:

```rust
HeartIcon::new().fill().size_8().text_color(red())
```

An icon is `1rem` square and takes the text color of the window until you set one, so it lines up with
the text beside it.

### Icons by name

Turn on the `catalog` feature to look an icon up at run time:

```rust
let icon = gpui_phosphor_icons::catalog::icon("ghost").unwrap().bold();
```

## Binary size

The SVG files sit behind the type of each icon, so the linker keeps only the icons your code names.
A program that uses ten icons carries ten icons.

Turn off the weights you never use to make the crate smaller still:

```toml
gpui-phosphor-icons = { git = "https://github.com/monodyle/gpui-phosphor-icons", default-features = false, features = ["regular", "fill"] }
```

When a weight is missing, an icon falls back to the regular weight.

The `catalog` feature keeps every icon in the binary, because the table names them all.

## Examples

```text
cargo run --example hello
cargo run --example video_player
cargo run --example gallery --features catalog
```

## Update the icons

The Rust sources and the SVG files come from the upstream catalog:

```text
cargo xtask sync       # clone or update vendor/phosphor-core
cargo xtask generate   # copy the SVG files and write src/icons.rs and src/catalog.rs
```

## License

- MIT - Copyright (c) 2025 Monody Le
- MIT - Copyright (c) 2023 [Phosphor Icons](https://github.com/phosphor-icons/core)
