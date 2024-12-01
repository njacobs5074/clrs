// Insertion sort, p. 17
pub fn insertion_sort(array: &mut [i32]) {
    for j  in 2..array.len() {
        let key: i32 = array[j];
        let mut i: i32 = (j - 1) as i32;
        while i >= 0 && array[i as usize] > key {
            array[(i + 1) as usize] = array[i as usize];
            i -= 1;
        }
        array[(i + 1) as usize] = key;
    }
}