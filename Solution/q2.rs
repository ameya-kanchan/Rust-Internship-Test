fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;
        if arr[mid] == target {
            let mut temp_left = mid;
            while temp_left > 0 && arr[temp_left - 1] == target {
                temp_left -= 1;
            }
            return Some(temp_left);
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    None
}