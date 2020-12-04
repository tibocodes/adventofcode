// #![allow(unused)]
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use std::collections::HashMap;


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

    let re = Regex::new(r"(\w{3}):(\S+)").unwrap();

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
            // Exo 2, remove for exo 1
            if check_field(cap[1].to_string(), cap[2].to_string()) {
                card.insert((&cap[1]).to_string(), true);
            }
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

fn check_field(field_name: String, field_value: String) -> bool {
    match field_name.as_ref() {
        "byr" => {
            let re = Regex::new(r"(\d{4})").unwrap();
            if !re.is_match(&field_value) {
                return false;
            }
            let val: u32 = field_value.parse().unwrap();
            if val < 1920 || val > 2002 {
                return false;
            }
        },
        "iyr" => {
            let re = Regex::new(r"(\d{4})").unwrap();
            if !re.is_match(&field_value) {
                return false;
            }
            let val: u32 = field_value.parse().unwrap();
            if val < 2010 || val > 2020 {
                return false;
            }
        },
        "eyr" => {
            let re = Regex::new(r"(\d{4})").unwrap();
            if !re.is_match(&field_value) {
                return false;
            }
            let val: u32 = field_value.parse().unwrap();
            if val < 2020 || val > 2030 {
                return false;
            }
        },
        "hgt" => {
            let re = Regex::new(r"(\d{2,3})(cm|in)").unwrap();
            if !re.is_match(&field_value) {
                return false;
            }
            let caps = re.captures(&field_value).unwrap();
            let val: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
            if caps.get(2).unwrap().as_str() == "cm" {
                if val < 150 || val > 193 {
                    return false;
                }
            } else {
                if val < 59 || val > 76 {
                    return false;
                }
            }
        },
        "hcl" => {
            let re = Regex::new(r"#([0-9]|[a-f]){6}").unwrap();
            if !re.is_match(&field_value) {
                return false;
            }
        },
        "ecl" => {
            let re = Regex::new(r"amb|blu|brn|gry|grn|hzl|oth").unwrap();
            if !re.is_match(&field_value) {
                return false;
            }
        },
        "pid" => {
            let re = Regex::new(r"^\d{9}$").unwrap();
            if !re.is_match(&field_value) {
                return false;
            }
        },
        _ => {
            return false;
        },
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
