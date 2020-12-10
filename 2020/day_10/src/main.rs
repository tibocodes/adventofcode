// #![allow(unused)]
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;


// Exo1
// Take the input
// Sort it
// calculate 3 classes of jolt difference categories
// fn main() {
//     let input = read();
//     let mut categories: HashMap<u32,u32>= HashMap::new();
//     for el in &input {
//         println!("{}", el);
//     }
//     &categories.insert(1, 1);
//     &categories.insert(3, 1);
//     // let mut diff: Vec<u32> = Vec::new();
//     for (i, el) in input[1..].iter().enumerate() {
//         println!("{}", el - input[i]);
//         // diff.push(el - input[i]);
//         let val = el - input[i];
//         let current_number = &categories.get(&val).unwrap();

//         &categories.insert(val, *current_number + 1);
//         // match &categories.get(&val) {
//         //     Some(current_number) => &categories.insert(val, *current_number + 1),
//         //     None => &categories.insert(val, 1)
//         // };
//     }

//     for (k, v) in &categories {
//         println!("k:{} v:{}", k, v);
//     }
//     let big = &categories.get(&1).unwrap();
//     let small = &categories.get(&3).unwrap();
//     println!("result: {}", *big * *small);
//     droppable_values(input);
// }

// everytime there is a step of 1, check if it can be replaced with a step of 2 or 3 and drop the intermediate values
fn droppable_values(input: Vec<u32>) -> u32 {
    let mut res = 0;
    let len = input.len();

    println!("printing steps");
    for (i, el) in input.iter().enumerate() {
        println!("{}", el);
        println!("el: {}", el);
        if i + 2 < len && input[i + 2] - el < 3 {
        println!("input + 2: {}", input[i + 2]);
            res += 1;
        }
        // if i + 3 < len && input[i + 3] - el < 3 {
        // println!("input + 3: {}", input[i + 3]);
        //     res += 1;
        // }
        // if i + 1 < len && input[i + 1] - el < 3 {
        //     println!("input + 1: {}", input[i + 1]);
        //         res += 1;
        // }
    }
    println!("droppable values: {}", res);
    // je reconnais une somme de coefficients binomiaux
    println!("2 puissance {}: {}", res, u32::pow(2, res));
    return res;
}

// https://www.reddit.com/r/rust/comments/ka9nre/advent_of_code_2020_day_10/gf9m754/?utm_source=reddit&utm_medium=web2x&context=3
pub fn main() {
    let mut nums: Vec<usize> = include_str!("../input")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect();
    nums.sort_unstable();

    println!(
        "{}",
        2 * nums
            .windows(2)
            .collect::<Vec<_>>()
            .split(|n| n[1] - n[0] == 3)
            .map(|n| match n.len() {
                4 => 7,
                3 => 4,
                2 => 2,
                _ => 1,
            })
            .product::<usize>()
    );
}


fn read() -> Vec<u32> {
    let f = File::open("example2").unwrap();
    let buf = BufReader::new(f);
    let mut input = Vec::new();
    for line in buf.lines() {
        let line = line.unwrap().parse().unwrap();
        input.push(line);
    }
    input.sort();
    return input;
}