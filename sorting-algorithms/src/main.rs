use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use sorting_algorithms::insertion_sort;

fn main() {
    let mut numbers: Vec<i32> = random_numbers(100);
    println!("numbers = {:?}", numbers);
    insertion_sort(numbers.as_mut_slice());
    println!("sorted numbers = {:?}", numbers);
}

fn random_numbers(num: i32) -> Vec<i32> {
    let mut vec: Vec<i32> = (0..num).collect();
    let mut rng: ThreadRng = rand::rng();
    vec.shuffle(&mut rng);
    
    vec 
}
