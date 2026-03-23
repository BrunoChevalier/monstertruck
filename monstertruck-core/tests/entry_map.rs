use monstertruck_core::entry_map::EntryMap;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;

/// Helper to create an `EntryMap` with explicit `RandomState` hasher.
fn new_entry_map<K, V, KF, VF, P>(
    k_closure: KF,
    v_closure: VF,
) -> EntryMap<K, V, KF, VF, P, RandomState>
where
    K: Eq + std::hash::Hash,
    P: Copy,
    KF: FnMut(P) -> K,
    VF: FnMut(P) -> V,
{
    EntryMap::new(k_closure, v_closure)
}

#[test]
fn basic_deduplication() {
    let mut counter = 0;
    let mut map = new_entry_map(
        |x: f64| x.floor() as i32,
        |_| {
            counter += 1;
            counter
        },
    );
    // Same key (floor of 3.5 and 3.6 is 3).
    assert_eq!(*map.entry_or_insert(3.5), 1);
    assert_eq!(*map.entry_or_insert(3.6), 1);
}

#[test]
fn different_keys_different_values() {
    let mut counter = 0;
    let mut map = new_entry_map(
        |x: f64| x.floor() as i32,
        |_| {
            counter += 1;
            counter
        },
    );
    assert_eq!(*map.entry_or_insert(3.5), 1);
    assert_eq!(*map.entry_or_insert(4.2), 2);
    assert_eq!(*map.entry_or_insert(5.9), 3);
}

#[test]
fn into_iter_yields_all_entries() {
    let mut counter = 0;
    let mut map = new_entry_map(
        |x: f64| x as i32,
        |_| {
            counter += 1;
            counter
        },
    );
    map.entry_or_insert(1.0);
    map.entry_or_insert(2.0);
    map.entry_or_insert(3.0);

    let collected: HashMap<i32, i32> = map.into_iter().collect();
    assert_eq!(collected.len(), 3);
    assert_eq!(collected[&1], 1);
    assert_eq!(collected[&2], 2);
    assert_eq!(collected[&3], 3);
}

#[test]
fn from_entry_map_to_hashmap() {
    let mut counter = 0;
    let mut map = new_entry_map(
        |x: f64| (x * 10.0) as i32,
        |_| {
            counter += 1;
            counter
        },
    );
    map.entry_or_insert(1.0);
    map.entry_or_insert(2.0);

    let hm: HashMap<i32, i32> = map.into();
    assert_eq!(hm.len(), 2);
    assert_eq!(hm[&10], 1);
    assert_eq!(hm[&20], 2);
}

#[test]
fn value_closure_not_called_for_existing_key() {
    let val_calls = std::cell::Cell::new(0);
    let mut map = new_entry_map(
        |x: f64| x as i32,
        |x: f64| {
            val_calls.set(val_calls.get() + 1);
            (x * 100.0) as i32
        },
    );

    assert_eq!(*map.entry_or_insert(5.0), 500);
    assert_eq!(val_calls.get(), 1);

    // Inserting same key again: value closure should NOT be called.
    assert_eq!(*map.entry_or_insert(5.0), 500);
    assert_eq!(val_calls.get(), 1);
}
