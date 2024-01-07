#![allow(irrefutable_let_patterns)]

use std::ops::{BitXor, BitOr};
use itertools::Itertools;

fn main() {
    let size = 5;
    let test_data: Vec<usize> = (1..10+1).collect();

    let results = test_all(size, test_data, &xor);
    //let results = find_minimal_for_size(test_data, &xor);
    // let result = find_minimal_for_top_val_xor(size);

    // println!("Needs 1..{} to solve for {}", size, result);

    for res in &results {
        println!("Found!");
        println!("{:?}", res)
    }

    println!("Solutions found: {}", results.len());
    if let Some(val) = results.first() {
        println!("Needs at least {} elements in the subset", val.len());
    }
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

fn test_all<T>(size: usize, test_data: Vec<T>, f: &dyn Fn(T, T) -> T) -> Vec<Vec<T>>
    where T: Clone + PartialEq + std::fmt::Debug {
    let sets = subsets(size, test_data.clone());

    println!("Solving for {} sets", sets.len());

    sets.into_iter().filter(|set| {
        let pairs = subsets(2, set.clone());

        //println!("Pairs to solve: {}", pairs.len());

        let output_set = pairs.into_iter().map(|pair| {
            let a = f(
                pair.get(0).expect("Malformed subset grouping 0").clone(), 
                pair.get(1).expect("Malformed subset grouping 1").clone()
            );
            a
        });
    
        test_data.clone().into_iter().all(|desired_val| 
            output_set.clone().chain(set.clone().into_iter()).clone().any(|c| c.eq(&desired_val)))
    }).collect::<Vec<Vec<T>>>()
}

// finds sets of a minimal size needed for a specific input set
fn find_minimal_for_size<T>(test_data: Vec<T>, f: &dyn Fn(T, T) ->T) -> Vec<Vec<T>> where T: Clone + PartialEq + std::fmt::Debug {
    let mut counter = 1;
    while let result = test_all(counter, test_data.clone(), f) {
        if result.len() != 0 {
            return result;
        }
        counter += 1;
    }
    return vec![];
}

// finds sets of a minimal size needed for a specific input set
// #[warn(while_true)]
// fn find_minimal_for_top_val_xor(size: usize) -> usize {
//     let mut counter = 1;
//     loop {
//         let test_data: Vec<usize> = (1..size+1).collect();
//         let result = test_all(counter, test_data, &xor);
//         if result.len() != 0 {
//             return counter;
//         }
//         counter += 1;
//     }
// }