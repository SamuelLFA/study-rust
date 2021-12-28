fn main() {
    let arr = [5, 10, 15, 5, 5];
    get_max_sum_and_min_sum(arr)
}

fn get_max_sum_and_min_sum(arr: [i32;5]) {
    let mut min_value = std::i32::MAX;
    let mut min_index = 0;
    let mut max_value = 0;
    let mut max_index = 0;
    for (i, elem) in arr.iter().enumerate() {
        if elem < &min_value {
            min_value = *elem;
            min_index = i;
        }
        if elem > &max_value {
            max_value = *elem;
            max_index = i;
        }
    }
    let mut max_sum = 0;
    let mut min_sum = 0;

    for (i, elem) in arr.iter().enumerate() {
        if i != min_index {
            max_sum += elem
        }
        if i != max_index {
            min_sum += elem
        }
    }

    println!("{} {}", min_sum, max_sum)
}
