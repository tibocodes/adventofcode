// #![allow(unused)]
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use std::collections::HashMap;


// Exo 1

struct Card {
    byr: bool,
    iyr: bool,
    eyr: bool,
    hgt: bool,
    hcl: bool,
    ecl: bool,
    pid: bool,
    cid: bool,
}

// Exo 1
fn main() -> std::io::Result<()> {
    let f = File::open("input")?;
    let f = BufReader::new(f);
    let mut valid = 0;

    let re = Regex::new(r"(\w{3}):\S+").unwrap();

    let mut card: HashMap<String, bool> = new_card();
    for line in f.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            if valid_card(card) {
                valid += 1;
            }
            card = new_card();
            continue
        }
        for cap in re.captures_iter(&line) {
            card.insert((&cap[1]).to_string(), true);
        }
    }
    println!("{}", valid);
    Ok(())
}

fn valid_card(card: HashMap<String, bool>) -> bool {
    let fields = 0;
    for (k, v) in card.iter() {
        if k != "cid" && *v == false {
            return false;
        }
    }
    return true;
}

fn new_card() -> HashMap<String, bool> {
    let mut card: HashMap<String, bool> = HashMap::new();
    card.insert("byr".to_string(), false);
    card.insert("iyr".to_string(), false);
    card.insert("eyr".to_string(), false);
    card.insert("hgt".to_string(), false);
    card.insert("hcl".to_string(), false);
    card.insert("ecl".to_string(), false);
    card.insert("pid".to_string(), false);
    card.insert("cid".to_string(), false);
    return card;
}
