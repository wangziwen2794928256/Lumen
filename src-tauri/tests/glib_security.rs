// Optimized Linux coverage for RUSTSEC-2024-0429, upstream PR 1343.
#![cfg(target_os = "linux")]
use glib::variant::ToVariant;

#[test]
fn variant_string_iterator_out_pointer_is_valid() {
    let value = vec!["alpha", "beta", "gamma", "delta"].to_variant();
    let mut it = value.array_iter_str().unwrap();
    assert_eq!(it.next(), Some("alpha"));
    assert_eq!(it.next_back(), Some("delta"));
    assert_eq!(it.collect::<Vec<_>>(), vec!["beta", "gamma"]);
    assert_eq!(value.array_iter_str().unwrap().nth(2), Some("gamma"));
    assert_eq!(value.array_iter_str().unwrap().nth_back(1), Some("gamma"));
    assert_eq!(value.array_iter_str().unwrap().last(), Some("delta"));
    let empty = Vec::<&str>::new().to_variant();
    assert_eq!(empty.array_iter_str().unwrap().next(), None);
}
