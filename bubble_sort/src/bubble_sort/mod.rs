pub fn run(arr: &Vec<i64>) -> Vec<i64> {
    let mut new_arr: Vec<i64> = vec![0; arr.len()];
    new_arr.copy_from_slice(&arr);
    for x in 0..(new_arr.len() - 1) {
        for y in x + 1..new_arr.len() {
            if new_arr[x] > new_arr[y] {
                let aux = new_arr[x];
                new_arr[x] = new_arr[y];
                new_arr[y] = aux;
            }
        }
    }
    new_arr
}
