use monstertruck_core::id::Id;
use std::collections::HashMap;

#[test]
fn id_new_creates_from_pointer() {
    let x: f64 = 42.0;
    let id = Id::new(&x);
    // Should not panic.
    let _ = format!("{id:?}");
}

#[test]
fn same_pointer_equal() {
    let x: f64 = 1.0;
    let id1 = Id::new(&x);
    let id2 = Id::new(&x);
    assert_eq!(id1, id2);
}

#[test]
fn different_pointers_not_equal() {
    let x: f64 = 1.0;
    let y: f64 = 1.0;
    let id1 = Id::new(&x);
    let id2 = Id::new(&y);
    // Stack variables at different addresses.
    assert_ne!(id1, id2);
}

#[test]
fn id_is_copy() {
    let x: f64 = 5.0;
    let id1 = Id::new(&x);
    let id2 = id1;
    // Both copies still work independently.
    assert_eq!(id1, id2);
}

#[test]
fn id_as_hashmap_key() {
    let x: f64 = 1.0;
    let y: f64 = 2.0;
    let id_x = Id::new(&x);
    let id_y = Id::new(&y);

    let mut map = HashMap::new();
    map.insert(id_x, "x");
    map.insert(id_y, "y");

    assert_eq!(map.get(&id_x), Some(&"x"));
    assert_eq!(map.get(&id_y), Some(&"y"));
    assert_eq!(map.len(), 2);
}

#[test]
fn debug_format_starts_with_0x() {
    let x: f64 = 7.0;
    let id = Id::new(&x);
    let debug = format!("{id:?}");
    assert!(debug.starts_with("0x"), "debug output was: {debug}");
}
