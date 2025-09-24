mod remove_duplicates_from_sorted_array;
mod index_of_first_occurence_in_string;

pub fn run() {
    let mut input: Vec<i32> = vec![1, 2, 2, 3];
    remove_duplicates_from_sorted_array::Solution::remove_duplicates(&mut input);
    index_of_first_occurence_in_string::Solution::str_str(String::from("leetcode"), String::from("leeto"));
}
