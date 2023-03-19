#![allow(clippy::unwrap_used)]

use iridium_assets::{Asset, Assets};
use iridium_ecs_macros::HasStableTypeId;

#[derive(HasStableTypeId)]
struct Test1(pub i32);

impl Asset for Test1 {}

#[derive(HasStableTypeId)]
struct Test2(pub i32);

impl Asset for Test2 {}

fn test_assets() -> Assets {
    let mut assets = Assets::new();

    assets.add("a", Test1(1));
    assets.add("b", Test1(2));

    assets
}

#[test]
fn get_all() {
    let assets = test_assets();

    let all_assets = assets.get_all();

    assert_eq!(all_assets.len(), 2);
}

#[test]
fn get() {
    let assets = test_assets();

    let a = assets.get::<Test1>("a").expect("Asset not found");

    assert_eq!(a.id(), "a");
    assert_eq!(a.get().unwrap().0, 1);
}

#[test]
fn get_mut() {
    let assets = test_assets();

    let a = assets.get::<Test1>("a").expect("Asset not found");

    assert_eq!(a.id(), "a");
    assert_eq!(a.get_mut().unwrap().0, 1);
}

#[test]
fn not_found() {
    let assets = test_assets();

    let c = assets.get::<Test1>("c");

    assert!(c.is_err());
}

#[test]
#[should_panic]
fn wrong_type() {
    let assets = test_assets();

    let a = assets.get::<Test2>("a").expect("Asset not found");

    let _t = a.get();
}

#[test]
fn change_id() {
    let assets = test_assets();

    let mut a = assets.get::<Test1>("a").expect("Asset not found");

    a.change_id("b".to_owned());

    assert_eq!(a.0, 1);

    let updated = a.update_asset(&assets).expect("New asset not found");

    assert!(updated);

    assert_eq!(a.0, 2);
}
