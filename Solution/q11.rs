fn merge_sorted_arrays(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut merged = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            merged.push(a[i]);
            i += 1;
        } else {
            merged.push(b[j]);
            j += 1;
        }
    }

    merged.extend_from_slice(&a[i..]);
    merged.extend_from_slice(&b[j..]);

    merged
}