fn max_subarray_sum(arr: Vec<i128>) -> i128 {
    let mut max_sum = arr[0];
    let mut current_sum = arr[0];

    for i in 1..arr.len() {
        current_sum = current_sum.max(arr[i]).wrapping_add(arr[i]);
        max_sum = max_sum.max(current_sum);
    }

    max_sum
}
fn main() {
    let arr = vec![1, -2, 3, -4, 5, -6];
    let max_sum = max_subarray_sum(arr);
    println!("The maximum subarray sum is: {}", max_sum);
}