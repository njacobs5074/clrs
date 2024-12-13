use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use sorting_algorithms::{insertion_sort, merge_sort};
use std::f32::consts::PI;

fn main() {
    println!("Insertion sort:");
    run_insertion_sort();

    println!("\nMerge sort:");
    run_merge();
}

fn run_insertion_sort() {
    let mut numbers: Vec<i32> = random_integers(20);
    println!("integers = {:?}", numbers);
    insertion_sort(&mut numbers);
    println!("sorted integers = {:?}", numbers);

    let mut numbers: Vec<f32> = random_floats(20);
    println!("floats = {:?}", numbers);
    insertion_sort(&mut numbers);
    println!("sorted floats = {:?}", numbers);
}

fn run_merge() {
    let numbers = random_integers(20);

    println!("integers = {:?}", numbers);
    let sorted_numbers = merge_sort(&numbers);
    println!("sorted integers = {:?}", sorted_numbers);

    let numbers = random_floats(20);
    println!("floats = {:?}", numbers);
    let sorted_numbers = merge_sort(&numbers);
    println!("sorted floats = {:?}", sorted_numbers);
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
        vec.push(i as f32 * PI);
    }

    let mut rng: ThreadRng = rand::rng();
    vec.shuffle(&mut rng);

    vec
}
