// Insertion sort, p. 17
pub fn insertion_sort<T: PartialOrd + Clone>(array: &mut [T]) {
    for j in 1..array.len() {
        let key = array[j].clone();
        let mut i: i32 = (j - 1) as i32;
        while i >= 0 && array[i as usize] > key {
            array.swap((i + 1) as usize, i as usize);
            i -= 1;
        }
        array[(i + 1) as usize] = key;
    }
}
