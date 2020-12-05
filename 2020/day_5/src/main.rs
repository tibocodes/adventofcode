// #![allow(unused)]
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

// Exo 1
fn main() -> std::io::Result<()> {
    let f = File::open("input")?;
    let f = BufReader::new(f);
    let mut max = 0;
    for line in f.lines() {
        let id = get_id(line.unwrap());
        if id > max {
            max = id;
        }
        println!("{}", id);
    }
    println!("{}", max);

    Ok(())
}


fn get_id(pass: String) -> u32 {
    let mut row_min: u32 = 0;
    let mut row_max: u32 = 127;
    let mut col_min: u32 = 0;
    let mut col_max: u32 = 7;
    for (i, c) in pass.chars().enumerate() {
        // println!("{}:{}", c, i);
        match c {
            // Adding +1 because the number of value is max num + 1 (128 values, max is 127)
            'F' => row_max = row_max - (row_max - row_min + 1) / 2,
            'B' => row_min = row_min + (row_max - row_min + 1) / 2,
            'L' => col_max = col_max - (col_max - col_min + 1) / 2,
            'R' => col_min = col_min + (col_max - col_min + 1) / 2,
            _ => continue,
        }
    }
    // println!("row max: {}", row_max);
    // println!("row min: {}", row_min);
    // println!("col max: {}", col_max);
    // println!("ID: {}", row_max * 8 + col_max);

    return row_max * 8 + col_max;
}
