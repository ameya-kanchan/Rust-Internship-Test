fn median(arr: &[i32]) -> f64 {
    let n = arr.len();
    if n % 2 == 0 {
        let mid1 = arr[n / 2 - 1];
        let mid2 = arr[n / 2];
        (mid1 + mid2) as f64 / 2.0
    } else {
        arr[n / 2] as f64
    }
}