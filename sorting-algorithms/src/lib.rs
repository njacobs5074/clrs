// Insertion sort, p. 17
// Note that this is an in-place sort
pub fn insertion_sort<T: PartialOrd + Clone>(vec: &mut [T]) {
    for j in 1..vec.len() {
        let key = vec[j].clone();
        let mut i: i32 = (j - 1) as i32;
        while i >= 0 && vec[i as usize] > key {
            vec.swap((i + 1) as usize, i as usize);
            i -= 1;
        }
        vec[(i + 1) as usize] = key;
    }
}

// Merge sort pp. 29-33
// From https://mohitkarekar.com/posts/2020/merge-sort-in-rust/
fn merge<T: PartialOrd + Clone>(left: &Vec<T>, right: &Vec<T>) -> Vec<T> {
    let mut i = 0;
    let mut j = 0;
    let mut merged: Vec<T> = Vec::new();

    // Put the elements of left & right into the correct place in merged
    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged.push(left[i].clone());
            i += 1;
        } else {
            merged.push(right[j].clone());
            j += 1;
        }
    }

    // There might be some leftovers in the left vector - just copy them in to merged
    if i < left.len() {
        while i < left.len() {
            merged.push(left[i].clone());
            i += 1;
        }
    }

    // Ditto for the leftovers in the right vector
    if j < right.len() {
        while j < right.len() {
            merged.push(right[j].clone());
            j += 1;
        }
    }

    merged
}

// From https://mohitkarekar.com/posts/2020/merge-sort-in-rust/
pub fn merge_sort<T: PartialOrd + Clone>(vec: &Vec<T>) -> Vec<T> {
    if vec.len() < 2 {
        // Trivially sorted
        vec.to_vec()
    } else {
        // Otherwise we must divide & conquer, i.e., merge sort the
        // left hand vector and then merge sort the right hand vector
        let middle = vec.len() / 2;
        let left = merge_sort(&vec[0..middle].to_vec());
        let right = merge_sort(&vec[middle..].to_vec());

        // The left and right are sorted so now we just need to merge them
        // and return the result
        merge(&left, &right)
    }
}
