pub fn parts_sums(arr: &[u64]) -> Vec<u64>{
    let mut my_vec = Vec::new();
    let mut sum = 0;

   for v in arr.iter() {
        sum += v;
        my_vec.push(sum)
    }
    my_vec.reverse();
    my_vec.push(0);
    my_vec
}
