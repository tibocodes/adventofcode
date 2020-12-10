// #![allow(unused)]
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;


// Exo1
// Take the input
// Sort it
// calculate 3 classes of jolt difference categories
fn main() {
    let input = read();
    let mut categories: HashMap<u32,u32>= HashMap::new();
    for el in &input {
        println!("{}", el);
    }
    &categories.insert(1, 1);
    &categories.insert(3, 1);
    for (i, el) in input[1..].iter().enumerate() {
        println!("{}", el - input[i]);
        let val = el - input[i];
        let current_number = &categories.get(&val).unwrap();

        &categories.insert(val, *current_number + 1);
        // match &categories.get(&val) {
        //     Some(current_number) => &categories.insert(val, *current_number + 1),
        //     None => &categories.insert(val, 1)
        // };
    }

    for (k, v) in &categories {
        println!("k:{} v:{}", k, v);
    }
    let big = &categories.get(&1).unwrap();
    let small = &categories.get(&3).unwrap();
    println!("result: {}", *big * *small);
}

fn read() -> Vec<u32> {
    let f = File::open("input").unwrap();
    let buf = BufReader::new(f);
    let mut input = Vec::new();
    for line in buf.lines() {
        let line = line.unwrap().parse().unwrap();
        input.push(line);
    }
    input.sort();
    return input;
}