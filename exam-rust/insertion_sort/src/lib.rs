pub fn insertion_sort(slice: &mut [i32], steps: usize) {
    slice.get_mut(..steps + 1).unwrap_or_default().sort();
}