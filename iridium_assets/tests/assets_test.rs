use iridium_assets::Assets;

fn test_assets() -> Assets {
    let mut assets = Assets::new();

    assets.add("a", 1_i32);
    assets.add("b", 2_i32);

    assets
}

#[test]
fn get_all() {
    let assets = test_assets();

    let all_assets = assets.get_all();

    assert_eq!(all_assets.len(), 2);

    all_assets.iter().all(|(key, value)| {
        match key.as_str() {
            "a" => assert_eq!(
                *value
                    .read()
                    .expect("Asset RwLock poisoned")
                    .downcast_ref::<i32>()
                    .expect("Asset is not an i32"),
                1
            ),
            "b" => assert_eq!(
                *value
                    .read()
                    .expect("Asset RwLock poisoned")
                    .downcast_ref::<i32>()
                    .expect("Asset is not an i32"),
                2
            ),
            _ => panic!("Unexpected key"),
        };
        true
    });
}

#[test]
fn get() {
    let assets = test_assets();

    let a = assets.get::<i32>("a").expect("Asset not found");

    assert_eq!(a.id(), "a");
    assert_eq!(*a.get(), 1);
}

#[test]
fn get_mut() {
    let assets = test_assets();

    let a = assets.get::<i32>("a").expect("Asset not found");

    assert_eq!(a.id(), "a");
    assert_eq!(*a.get_mut(), 1);
}

#[test]
fn deref() {
    let assets = test_assets();

    let a = assets.get::<i32>("a").expect("Asset not found");

    assert_eq!(a.id(), "a");
    assert_eq!(*a, 1);
}

#[test]
fn not_found() {
    let assets = test_assets();

    let c = assets.get::<i32>("c");

    assert!(c.is_err());
}

#[test]
#[should_panic]
fn wrong_type() {
    let assets = test_assets();

    let a = assets.get::<String>("a").expect("Asset not found");

    let _ = a.get();
}

#[test]
fn change_id() {
    let assets = test_assets();

    let mut a = assets.get::<i32>("a").expect("Asset not found");

    a.change_id("b".to_owned());

    assert_eq!(*a, 1);

    let updated = a.update_asset(&assets).expect("New asset not found");

    assert!(updated);

    assert_eq!(*a, 2);
}
