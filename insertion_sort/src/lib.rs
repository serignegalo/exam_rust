pub fn insertion_sort(slice: &mut [i32], steps: usize) {
    (slice.split_at_mut(steps + 1).0).sort()
    // slice[0..=step].sort
}


// EX: slice = &mut [8, 3, 7, 5, 2] and steps = 2

// steps + 1 = 3 passaka index starts à 0
// slice.split_at_mut(3) donne &mut [8, 3, 7] et &mut [5, 2]

// (slice.split_at_mut(3).0) index 0 puis on sort ça

// Après sorting, the combined slice donne &mut [3, 7, 8, 5, 2]