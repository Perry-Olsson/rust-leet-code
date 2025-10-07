mod remove_duplicates_from_sorted_array;
mod index_of_first_occurence_in_string;
mod remove_element;
mod sum_3;
mod container_with_most_water;
mod search_insert_position;

pub fn run() {
    let mut input: Vec<i32> = vec![1, 2, 2, 3];
    remove_duplicates_from_sorted_array::Solution::remove_duplicates(&mut input);
    index_of_first_occurence_in_string::Solution::str_str(String::from("leetcode"), String::from("leeto"));
    remove_element::Solution::remove_element(&mut vec![3, 2, 2, 3], 3);
    sum_3::Solution::three_sum(vec![-1, -2, -5, 2, 0, 5, 3]);
    container_with_most_water::Solution::max_area(vec![1,8,6,2,5,4,8,3,7]);
    search_insert_position::Solution::search_insert(vec![1, 2, 3], 2);
}
