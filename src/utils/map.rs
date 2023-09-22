use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

pub fn sort_map<K: Ord + Clone, V: Ord + Clone>(
    map: HashMap<K, V>
) -> BTreeMap<K, V> {
    map.into_iter().collect()
}

pub fn filter<
    K: Ord + Clone + Hash,
    V: Ord + Clone,
    P: FnMut(&(K, V)) -> bool
>(
    map: HashMap<K, V>,
    predicate: P,
) -> HashMap<K, V> {
    map.into_iter().filter(predicate).collect()
}
