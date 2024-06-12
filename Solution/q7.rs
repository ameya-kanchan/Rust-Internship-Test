fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > arr.len() {
        return None;
    }

    arr.iter().cloned().min_by_key(|&x| x).unwrap_or(-1)
}
