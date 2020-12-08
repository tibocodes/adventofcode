// #![allow(unused)]
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    for line in file_to_vec() {
        println!("{} : {} : {}", line.0, line.1, line.2);
    }
    let mut vec = file_to_vec();
    let mut index: i32 = 0;
    let mut acc: i32 = 0;
    loop {
        if vec[index as usize].2 == true {
            break;
        }
        match vec[index as usize].0.as_ref() {
            "nop" => {
                vec[index as usize].2 = true;
                index += 1;
            },
            "acc" => {
                acc += vec[index as usize].1;
                vec[index as usize].2 = true;
                index += 1;
            },
            "jmp" => {
                vec[index as usize].2 = true;
                index += vec[index as usize].1;
            },
            &_ => {
                panic!("error");
            }
        }
    }
    println!("acc: {}", acc);
}


fn file_to_vec() -> Vec<(String, i32, bool)> {
    let file = File::open("input").unwrap();
    let buf = BufReader::new(file);
    let mut input = Vec::new();
    for line in buf.lines() {
        let line = line.unwrap();
        input.push((line[0..3].to_string(), line[4..].to_string().parse().unwrap(), false));
    }
    return input;
}