use std::hash::{DefaultHasher, Hash, Hasher};

#[test]
fn hash_equivalence() {
    let mut hasher = DefaultHasher::new();
    "test".hash(&mut hasher);
    assert_eq!(macro_hash::hash!("test"), hasher.finish());
}

#[test]
fn more_hash_equivalence() {
    let mut hasher = DefaultHasher::new();
    "test".hash(&mut hasher);
    "SALT".hash(&mut hasher);
    assert_eq!(macro_hash::hash!("test", "SALT"), hasher.finish());
}
