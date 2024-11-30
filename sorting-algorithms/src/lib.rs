// Insertion sort, p. 17
pub fn insertion_sort(array: &mut [i32]) {
    for j  in 1..array.len() {
        let key = array[j];
        let mut i = (j - 1) as i32;
        while i >= 0 && array[i as usize] > key {
            array[(i + 1) as usize] = array[i as usize];
            i -= 1;
        }
        array[(i + 1) as usize] = key;
    }
}