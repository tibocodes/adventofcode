// #![allow(unused)]
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

// Exo 1
// fn main() -> io::Result<()> {
//     let f = File::open("input")?;
//     let f = BufReader::new(f);
//     let mut valid = 0;

//     for line in f.lines() {
//         let line = line.unwrap();
//         let v: Vec<&str> = line.rsplit(' ').collect();
//         // println!("v0 : {}", v[0]);
//         // println!("v1 : {}", v[1]);
//         // println!("v2 : {}", v[2]);
//         // v0 : abcde
//         // v1 : a:
//         // v2 : 1-3
//         let letter: char; 
//         match v[1].chars().nth(0) {
//            Some(p) => letter = p,
//            None => panic!(),
//         }
//         // println!("letter {}", letter);
//         let min_max: Vec<&str> = v[2].rsplit('-').collect();
//         // println!("min max 0: {}", min_max[0]);
//         // println!("min max 1: {}", min_max[1]);
//         let max: u32 = min_max[0].parse().unwrap();
//         let min: u32 = min_max[1].parse().unwrap();
//         // println!("min: {}", min);
//         // println!("max 1: {}", max);
        
//         let pw = v[0];
//         let mut count = 0;
//         for c in pw.chars() {
//             // println!("{}", c)
//             if c == letter {
//                 count = count + 1;
//             }
//         }
        
//         if count >= min && count <= max {
//             valid += 1;
//         }
//     }
//     println!("valid: {}", valid);
//     Ok(())
// }

// Exo 2
fn main() -> io::Result<()> {
    let f = File::open("input")?;
    let f = BufReader::new(f);
    let mut valid = 0;

    for line in f.lines() {
        let line = line.unwrap();
        let v: Vec<&str> = line.rsplit(' ').collect();
        // println!("v0 : {}", v[0]);
        // println!("v1 : {}", v[1]);
        // println!("v2 : {}", v[2]);
        // v0 : abcde
        // v1 : a:
        // v2 : 1-3
        let letter: char; 
        match v[1].chars().nth(0) {
           Some(p) => letter = p,
           None => panic!(),
        }
        let min_max: Vec<&str> = v[2].rsplit('-').collect();
        let max: usize = min_max[0].parse().unwrap();
        let min: usize = min_max[1].parse().unwrap();

        
        let pw = v[0];
        let mut count = 0;
        // -1 because index = pos - 1
        if (pw.chars().nth(min - 1).unwrap() == letter) ^ (pw.chars().nth(max - 1).unwrap() == letter) {
            valid += 1;
        } 
    }
    println!("valid: {}", valid);
    Ok(())
}