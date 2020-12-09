// #![allow(unused)]
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let values = values();
    for (i, value) in values[25..].iter().enumerate() {
        let window = &values[i..i + 25];
        if sum_in(window, value) == false {
            println!("result : {}", value);
            break;
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