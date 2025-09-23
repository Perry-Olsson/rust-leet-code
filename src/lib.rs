mod remove_duplicates_from_sorted_array;

pub fn run() {
    let mut input: Vec<i32> = vec![1, 2, 2, 3];
    remove_duplicates_from_sorted_array::Solution::remove_duplicates(&mut input);
}
