use crate::{GhostIcon, IconWeight, PhosphorAssets, icons::*, load, register};
use gpui::AssetSource;

#[test]
fn icon_names_match_upstream() {
    assert_eq!(GhostIcon::NAME, "ghost");
    assert_eq!(GhostIcon::DATA.name, "ghost");
    assert_eq!(AddressBookTabsIcon::NAME, "address-book-tabs");
}

#[test]
fn every_weight_is_there_by_default() {
    for weight in IconWeight::ALL {
        let asset = GhostIcon::DATA.weights[weight.index()].expect("the weight is in the build");
        assert_eq!(asset.path, format!("phosphor/{weight}/ghost.svg"));
        assert!(
            asset.bytes.starts_with(b"<svg"),
            "{} is not an svg",
            asset.path
        );
    }
}

#[test]
fn duotone_holds_the_second_layer() {
    let duotone = GhostIcon::DATA.asset(IconWeight::Duotone).unwrap();
    let regular = GhostIcon::DATA.asset(IconWeight::Regular).unwrap();
    let text = std::str::from_utf8(duotone.bytes).unwrap();
    assert!(text.contains("opacity=\"0.2\""));
    assert!(duotone.bytes.len() > regular.bytes.len());
}

#[test]
fn a_missing_weight_falls_back() {
    let mut data = *GhostIcon::DATA;
    data.weights[IconWeight::Thin.index()] = None;
    let asset = data.asset(IconWeight::Thin).expect("regular stands in");
    assert_eq!(asset.path, "phosphor/regular/ghost.svg");
}

#[test]
fn the_asset_source_serves_registered_icons() {
    let asset = HeartIcon::DATA.asset(IconWeight::Fill).unwrap();
    register(asset.path, asset.bytes);

    assert_eq!(load(asset.path).unwrap().as_ref(), asset.bytes);

    let assets = PhosphorAssets::new();
    assert_eq!(
        assets.load(asset.path).unwrap().unwrap().as_ref(),
        asset.bytes
    );
    assert!(
        assets
            .load("phosphor/fill/not-an-icon.svg")
            .unwrap()
            .is_none()
    );
    assert!(
        assets
            .list("phosphor/")
            .unwrap()
            .contains(&asset.path.into())
    );
}

#[test]
fn old_names_still_work() {
    assert_eq!(CaduceusIcon::NAME, AsclepiusIcon::NAME);
}

#[cfg(feature = "catalog")]
#[test]
fn the_catalog_holds_every_icon_sorted() {
    use crate::catalog;

    assert_eq!(catalog::ALL.len(), 1512);
    assert!(
        catalog::ALL
            .windows(2)
            .all(|pair| pair[0].name < pair[1].name)
    );
    assert_eq!(catalog::by_name("ghost").unwrap().name, "ghost");
    assert!(catalog::by_name("not-an-icon").is_none());
}
