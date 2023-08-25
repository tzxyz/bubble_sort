fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut ve1 = vec![9, -3, 6, 3, 1, 2, 2, 18, -5, 7];
    println!("排序前: {:?}", ve1);
    bubble_sort(&mut ve1);
    println!("排序后：{:?}", ve1);

    let mut ve2 = vec!["a", "k", "4", "7"];

    println!("排序前: {:?}", ve2);
    bubble_sort(&mut ve2);
    println!("排序后：{:?}", ve2);
}
