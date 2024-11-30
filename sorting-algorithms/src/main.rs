use rand::distr::StandardUniform;
use rand::Rng;
use sorting_algorithms::insertion_sort;

fn main() {
    let mut vec: Vec<i32> = rand::rng().sample_iter(StandardUniform).take(10).collect();
    insertion_sort(vec.as_mut_slice());
    println!("{:?}", vec);
}
