use iridium_map_utils::{fast_map, fast_map_any};

#[test]
fn fast_map_test() {
    let map = fast_map! {
        "a" => 1,
        "b" => 2,
        "c" => 3,
    };

    assert_eq!(map["a"], 1);
    assert_eq!(map["b"], 2);
    assert_eq!(map["c"], 3);
}

#[allow(clippy::float_cmp)]
#[test]
fn fast_map_any_test() {
    let map = fast_map_any! {
        "a" => 1_f32,
        "b" => 2_f32,
        "c" => 3_f32,
    };

    assert_eq!(
        *map["a"]
            .downcast_ref::<f32>()
            .expect("Wrong type was stored"),
        1_f32
    );
    assert_eq!(
        *map["b"]
            .downcast_ref::<f32>()
            .expect("Wrong type was stored"),
        2_f32
    );
    assert_eq!(
        *map["c"]
            .downcast_ref::<f32>()
            .expect("Wrong type was stored"),
        3_f32
    );
}
