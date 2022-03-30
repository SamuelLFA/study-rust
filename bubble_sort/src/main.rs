mod bubble_sort;

fn main() {
    let arr: Vec<i64> = vec![1, 3, 1, 4, 5, 5, 2];
    let sorted = bubble_sort::run(&arr);
    println!("{:?}", sorted);
}
