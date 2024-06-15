pub fn parts_sums(arr: &[u64]) -> Vec<u64>{
    let mut parts: Vec<u64> = Vec::new();
    let mut total = arr.iter().sum();

    for c in arr.iter().rev() {
        parts.push(total);
        total -= c;
    };
    parts.push(total);

    parts
}