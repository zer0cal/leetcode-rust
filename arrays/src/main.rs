fn main() {
    let res = ::arrays::median_of_2_sorted_arrays::Solution::find_median_sorted_arrays(
        vec![1, 2, 5],
        vec![3, 4, 6],
    );
    println!("{:?}", res);
}
