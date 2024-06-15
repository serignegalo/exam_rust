use std::collections::BTreeSet;

pub fn flatten_tree<T: Clone>(tree: &BTreeSet<T>) -> Vec<T> {
    tree.into_iter().cloned().collect()
}