use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use sorting_algorithms::insertion_sort;

fn main() {
    let mut numbers: Vec<i32> = random_integers(20);
    println!("numbers = {:?}", numbers);
    insertion_sort(numbers.as_mut_slice());
    println!("sorted numbers = {:?}", numbers);

    let mut numbers: Vec<f32> = random_floats(20);
    println!("numbers = {:?}", numbers);
    insertion_sort(numbers.as_mut_slice());
    println!("sorted numbers = {:?}", numbers);
}

fn random_integers(num: i32) -> Vec<i32> {
    let mut vec: Vec<i32> = (0..num).collect();
    let mut rng: ThreadRng = rand::rng();
    vec.shuffle(&mut rng);

    vec
}

fn random_floats(num: i32) -> Vec<f32> {
    let mut vec: Vec<f32> = Vec::with_capacity(num as usize);
    for i in 0..num {
        vec.push(i as f32);
    }

    let mut rng: ThreadRng = rand::rng();
    vec.shuffle(&mut rng);

    vec
}
