use std::ops::{BitXor, BitOr};
use itertools::Itertools;

fn main() {
    let size = 6;
    let test_data: Vec<usize> = (1..18+1).collect();

    test_all(size, test_data, &xor);
}

fn xor<T>(a: T, b: T) -> T where T: BitXor<T, Output = T> {
    a ^ b
}

fn or<T>(a: T, b: T) -> T where T: BitOr<T, Output = T> {
    a | b
}

// generate subsets of "size"
fn subsets<T: Clone>(size: usize, items: Vec<T>) -> Vec<Vec<T>> {
    items.into_iter().combinations(size).collect_vec()
}

// generate function results

fn test_all<T>(size: usize, test_data: Vec<T>, f: &dyn Fn(T, T) -> T) where T: Clone + PartialEq + std::fmt::Debug {
    let sets = subsets(size, test_data.clone());

    let results = sets.into_iter().filter(|set| {
        let pairs = subsets(2, set.clone());

        let output_set = pairs.into_iter().map(|pair| {
            let a = f(
                pair.get(0).expect("Malformed subset grouping 0").clone(), 
                pair.get(1).expect("Malformed subset grouping 1").clone()
            );
            a
        });
    
        test_data.clone().into_iter().all(|desired_val| 
            output_set.clone().chain(set.clone().into_iter()).clone().any(|c| c.eq(&desired_val)))
    }).collect::<Vec<Vec<T>>>();

    for res in &results {
        println!("Found!");
        println!("{:?}", res)
    }

    println!("Solutions found: {}", results.len());
}