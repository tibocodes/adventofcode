// #![allow(unused)]
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

// Exo1
// fn main() {
//     let values = values();
//     for (i, value) in values[25..].iter().enumerate() {
//         let window = &values[i..i + 25];
//         if sum_in(window, value) == false {
//             println!("result : {}", value);
//             break;
//         }
//     }
// }

// Exo2
// brute force, should at least opt out of loop when the sum is already > to the expected result
fn main() {
    let values = values();
    for (i, _) in values.iter().enumerate() {
        for (j, _) in values[i..].iter().enumerate() {
            let sum: u64 = values[i..(i + j)].iter().sum();
            if sum == 756008079 {
                let min = values[i..(i + j)].iter().min().unwrap();
                let max = values[i..(i + j)].iter().max().unwrap();
                println!("min: {} max:{}", min, max);
                println!("sum: {}", min + max);
                return;
            }
        }

    } 
}


fn values() -> Vec<u64> {
    let file = File::open("input").unwrap();
    let buf = BufReader::new(file);
    let mut input = Vec::new();
    for line in buf.lines() {
        let line = line.unwrap().parse().unwrap();
        input.push(line);
    }
    return input;
}

fn sum_in(arr: &[u64], sum: &u64) -> bool {
    for i in arr.iter() {
        for j in arr.iter() {
            if i + j == *sum {
                return true;
            }

        }
    }
    return false;
} 