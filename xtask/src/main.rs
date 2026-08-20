//! Build the icon crate from the upstream Phosphor catalog.
//!
//! ```text
//! cargo xtask sync       # clone or update vendor/phosphor-core
//! cargo xtask generate   # copy the SVG files and write the Rust sources
//! ```

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const UPSTREAM: &str = "https://github.com/phosphor-icons/core";
const WEIGHTS: [&str; 6] = ["thin", "light", "regular", "bold", "fill", "duotone"];

struct Icon {
    name: String,
    pascal_name: String,
    aliases: Vec<(String, String)>,
}

fn main() {
    let task = std::env::args().nth(1).unwrap_or_default();
    match task.as_str() {
        "sync" => sync(),
        "generate" => {
            if !vendor_dir().join("assets").is_dir() {
                sync();
            }
            generate();
        }
        _ => {
            eprintln!("usage: cargo xtask <sync|generate>");
            std::process::exit(1);
        }
    }
}

fn workspace_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask has a parent folder")
        .to_path_buf()
}

fn vendor_dir() -> PathBuf {
    workspace_dir().join("vendor/phosphor-core")
}

fn crate_dir() -> PathBuf {
    workspace_dir().join("crates/gpui-phosphor-icons")
}

fn sync() {
    let vendor = vendor_dir();
    if vendor.join(".git").is_dir() {
        println!("updating {}", vendor.display());
        run(
            "git",
            &["-C", vendor.to_str().unwrap(), "pull", "--ff-only"],
        );
    } else {
        println!("cloning {UPSTREAM}");
        fs::create_dir_all(vendor.parent().unwrap()).expect("create vendor folder");
        run(
            "git",
            &["clone", "--depth", "1", UPSTREAM, vendor.to_str().unwrap()],
        );
    }
}

fn run(program: &str, args: &[&str]) {
    let status = Command::new(program)
        .args(args)
        .status()
        .unwrap_or_else(|error| panic!("failed to start {program}: {error}"));
    assert!(status.success(), "{program} failed");
}

fn generate() {
    let icons = read_catalog(&vendor_dir().join("src/icons.ts"));
    println!("read {} icons", icons.len());

    copy_assets(&icons);
    write_icons(&icons);
    write_catalog(&icons);
    write_version();
    run("cargo", &["fmt", "--all"]);

    println!("done");
}

/// Read `name`, `pascal_name` and the aliases out of the upstream `icons.ts`.
fn read_catalog(path: &Path) -> Vec<Icon> {
    let source =
        fs::read_to_string(path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()));

    let mut icons: Vec<Icon> = Vec::new();
    for line in source.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("alias:") {
            let alias_name = find_field(rest, "name").expect("alias has a name");
            let alias_pascal = find_field(rest, "pascal_name").expect("alias has a pascal_name");
            icons
                .last_mut()
                .expect("alias belongs to an icon")
                .aliases
                .push((alias_name, alias_pascal));
        } else if let Some(name) = field(line, "name") {
            icons.push(Icon {
                name,
                pascal_name: String::new(),
                aliases: Vec::new(),
            });
        } else if let Some(pascal_name) = field(line, "pascal_name") {
            icons
                .last_mut()
                .expect("pascal_name belongs to an icon")
                .pascal_name = pascal_name;
        }
    }

    assert!(!icons.is_empty(), "the catalog is empty");
    for icon in &icons {
        assert!(!icon.pascal_name.is_empty(), "{} has no name", icon.name);
    }
    icons.sort_by(|left, right| left.name.cmp(&right.name));
    icons
}

/// Read `key: "value"` from the start of one line of the catalog.
fn field(line: &str, key: &str) -> Option<String> {
    value_after(line.trim().strip_prefix(key)?)
}

/// Read `key: "value"` from anywhere in one line of the catalog.
fn find_field(line: &str, key: &str) -> Option<String> {
    let start = line.find(key)?;
    value_after(&line[start + key.len()..])
}

fn value_after(rest: &str) -> Option<String> {
    let rest = rest.trim_start().strip_prefix(':')?.trim_start();
    let rest = rest.strip_prefix('"')?;
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

/// Copy every SVG file, and drop the weight suffix from its name.
fn copy_assets(icons: &[Icon]) {
    let source_root = vendor_dir().join("assets");
    let target_root = crate_dir().join("assets");
    if target_root.exists() {
        fs::remove_dir_all(&target_root).expect("clear the assets folder");
    }

    for weight in WEIGHTS {
        let target = target_root.join(weight);
        fs::create_dir_all(&target).expect("create a weight folder");
        for icon in icons {
            let file = match weight {
                "regular" => format!("{}.svg", icon.name),
                _ => format!("{}-{weight}.svg", icon.name),
            };
            let source = source_root.join(weight).join(&file);
            fs::copy(&source, target.join(format!("{}.svg", icon.name)))
                .unwrap_or_else(|error| panic!("copy {}: {error}", source.display()));
        }
    }

    println!("copied {} files", icons.len() * WEIGHTS.len());
}

fn write_icons(icons: &[Icon]) {
    let mut out = String::new();
    out.push_str(&header());
    out.push_str("//! Every Phosphor icon, one type each.\n\n");

    for icon in icons {
        out.push_str(&format!(
            "declare_icon!({}Icon, \"{}\");\n",
            icon.pascal_name, icon.name
        ));
        for (alias_name, alias_pascal) in &icon.aliases {
            out.push_str(&format!(
                "/// The `{alias_name}` icon, the old name of [`{pascal}Icon`].\npub type {alias_pascal}Icon = {pascal}Icon;\n",
                pascal = icon.pascal_name,
            ));
        }
    }

    write(crate_dir().join("src/icons.rs"), &out);
}

fn write_catalog(icons: &[Icon]) {
    let mut out = String::new();
    out.push_str(&header());
    out.push_str(
        "//! The table of every icon, for lookup by name at run time.\n\
         //!\n\
         //! This module holds every icon file in the binary. It is behind the\n\
         //! `catalog` feature.\n\n\
         use crate::IconData;\n\
         use crate::icons::*;\n\n\
         /// Every icon, sorted by name.\n\
         pub static ALL: &[&IconData] = &[\n",
    );
    for icon in icons {
        out.push_str(&format!("    {}Icon::DATA,\n", icon.pascal_name));
    }
    out.push_str(
        "];\n\n\
         /// Look one icon up by its kebab-case name, for example `ghost`.\n\
         pub fn by_name(name: &str) -> Option<&'static IconData> {\n\
         \x20   ALL.binary_search_by(|icon| icon.name.cmp(name))\n\
         \x20       .ok()\n\
         \x20       .map(|index| ALL[index])\n\
         }\n\n\
         /// Build one icon by its kebab-case name, for example `ghost`.\n\
         pub fn icon(name: &str) -> Option<crate::Icon> {\n\
         \x20   by_name(name).map(crate::Icon::new)\n\
         }\n",
    );

    write(crate_dir().join("src/catalog.rs"), &out);
}

/// Record which upstream version the assets come from.
fn write_version() {
    let package = fs::read_to_string(vendor_dir().join("package.json")).expect("read package.json");
    let version = package
        .lines()
        .find_map(|line| field(line, "\"version\"").or_else(|| field(line, "version")))
        .unwrap_or_else(|| "unknown".to_string());
    write(
        crate_dir().join("assets/VERSION"),
        &format!("@phosphor-icons/core {version}\n"),
    );
    println!("phosphor core version {version}");
}

fn header() -> String {
    "// Generated by `cargo xtask generate`. Do not edit by hand.\n\n".to_string()
}

fn write(path: PathBuf, contents: &str) {
    fs::write(&path, contents).unwrap_or_else(|error| panic!("write {}: {error}", path.display()));
    println!("wrote {}", path.display());
}
