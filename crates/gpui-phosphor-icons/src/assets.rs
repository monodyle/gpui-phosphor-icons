use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};

use gpui::{AssetSource, Result, SharedString};

/// The folder that every icon path starts with, for example
/// `phosphor/duotone/ghost.svg`.
pub const ASSET_PREFIX: &str = "phosphor/";

fn registry() -> &'static RwLock<HashMap<&'static str, &'static [u8]>> {
    static REGISTRY: OnceLock<RwLock<HashMap<&'static str, &'static [u8]>>> = OnceLock::new();
    REGISTRY.get_or_init(|| RwLock::new(HashMap::new()))
}

/// Make the bytes of one icon readable through [`PhosphorAssets`].
///
/// Every icon does this for itself when you build it, so you rarely need to
/// call this. Only the icons you use end up in the binary, because the linker
/// drops the data of the icons that no code refers to.
pub fn register(path: &'static str, bytes: &'static [u8]) {
    let registry = registry();
    if registry.read().unwrap().contains_key(path) {
        return;
    }
    registry.write().unwrap().insert(path, bytes);
}

/// Read one icon by path, for example `phosphor/duotone/ghost.svg`.
///
/// Use this to serve Phosphor icons from your own [`AssetSource`]:
///
/// ```no_run
/// # use std::borrow::Cow;
/// # use gpui::{AssetSource, Result, SharedString};
/// struct Assets;
///
/// impl AssetSource for Assets {
///     fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
///         if let Some(bytes) = gpui_phosphor_icons::load(path) {
///             return Ok(Some(bytes));
///         }
///         // ...your own assets here
///         Ok(None)
///     }
///
///     fn list(&self, _path: &str) -> Result<Vec<SharedString>> {
///         Ok(vec![])
///     }
/// }
/// ```
pub fn load(path: &str) -> Option<Cow<'static, [u8]>> {
    registry()
        .read()
        .unwrap()
        .get(path)
        .map(|bytes| Cow::Borrowed(*bytes))
}

/// Every icon path that is ready to read.
pub fn paths() -> Vec<SharedString> {
    let mut paths: Vec<SharedString> = registry()
        .read()
        .unwrap()
        .keys()
        .map(|path| SharedString::new_static(path))
        .collect();
    paths.sort();
    paths
}

/// An [`AssetSource`] that serves the Phosphor icons.
///
/// Give it to your application at start up:
///
/// ```no_run
/// use gpui::Application;
/// use gpui_phosphor_icons::PhosphorAssets;
///
/// Application::new().with_assets(PhosphorAssets::new()).run(|_cx| {});
/// ```
///
/// If your application has its own assets, chain them with
/// [`PhosphorAssets::with_fallback`], or call [`load`] from your own source.
#[derive(Clone, Default)]
pub struct PhosphorAssets {
    fallback: Option<Arc<dyn AssetSource>>,
}

impl PhosphorAssets {
    /// An asset source that only serves Phosphor icons.
    pub fn new() -> Self {
        Self { fallback: None }
    }

    /// Serve Phosphor icons first, then everything else from `source`.
    pub fn with_fallback(source: impl AssetSource) -> Self {
        Self {
            fallback: Some(Arc::new(source)),
        }
    }
}

impl AssetSource for PhosphorAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if let Some(bytes) = load(path) {
            return Ok(Some(bytes));
        }
        match &self.fallback {
            Some(fallback) => fallback.load(path),
            None => Ok(None),
        }
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let mut result: Vec<SharedString> = paths()
            .into_iter()
            .filter(|asset| asset.starts_with(path))
            .collect();
        if let Some(fallback) = &self.fallback {
            result.extend(fallback.list(path)?);
        }
        Ok(result)
    }
}
