// #![allow(unused)]
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

// Exo 1
fn main() -> std::io::Result<()> {
   
    let f = File::open("input")?;
    let f = BufReader::new(f);
    let mut letters = init_empty_dic();
    let mut total = 0;
    
    for line in f.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            total += count_letters(letters);
            letters = init_empty_dic();
            continue;
        }
        // add_letters(&letters, line);
        for c in line.chars() {
            letters.insert(c as char, true);
        }
    }
    println!("{}", total);

    Ok(())
}

// Couldnt figure out how to make this one work
// Kept getting
// = note: expected type `&mut std::collections::HashMap<char, bool>`
//         found type `&std::collections::HashMap<char, bool>`
// fn add_letters(letters: &HashMap<char,bool>, line: String) {
//     for c in line.chars() {
//         letters.insert(c as char, true);
//     }
// }

fn init_empty_dic() -> HashMap<char,bool> {
    let mut map: HashMap<char,bool> = HashMap::new();
    for c in b'a'..=b'z' {
        // println!("{}", c as char);
        map.insert(c as char, false); 
    }
    return map;
}

fn count_letters(map: HashMap<char,bool>) -> u32 {
    let mut count = 0;
    for (_key, val) in map.iter() {
        if *val == true {
            count += 1 
        }
    }
    return count;
}