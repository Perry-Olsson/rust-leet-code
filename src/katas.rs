mod remove_duplicates_from_sorted_array;
mod index_of_first_occurence_in_string;
mod remove_element;
mod sum_3;
mod container_with_most_water;
mod search_insert_position;
mod convert_sorted_array_to_binary_tree;
mod pascals_triangle;
mod best_time_to_buy_and_sell_stock;
mod inorder_bst_traversal;

pub fn run() {
    let mut input: Vec<i32> = vec![1, 2, 2, 3];
    remove_duplicates_from_sorted_array::Solution::remove_duplicates(&mut input);
    index_of_first_occurence_in_string::Solution::str_str(String::from("leetcode"), String::from("leeto"));
    remove_element::Solution::remove_element(&mut vec![3, 2, 2, 3], 3);
    sum_3::Solution::three_sum(vec![-1, -2, -5, 2, 0, 5, 3]);
    container_with_most_water::Solution::max_area(vec![1,8,6,2,5,4,8,3,7]);
    search_insert_position::Solution::search_insert(vec![1, 2, 3], 2);
    convert_sorted_array_to_binary_tree::Solution::sorted_arary_to_bst(vec![1, 2, 3, 4]);
    pascals_triangle::Solution::generate(4);
    best_time_to_buy_and_sell_stock::Solution::max_profit(vec![7 , 2, 3]);
    inorder_bst_traversal::Solution::inorder_traversal(Option::None);
}
