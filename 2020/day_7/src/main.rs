// // #![allow(unused)]
// use std::io::{self, BufReader};
// use std::io::prelude::*;
// use std::fs::File;
// use regex::Regex;
// use std::collections::HashMap;

// // Exo 1
// fn main() -> std::io::Result<()> {
//     let f = File::open("input")?;
//     let f = BufReader::new(f);
//     let bags: HashMap<String, String> = HashMap::new();
//     let mut child_bag = "shiny gold";
//     for line in f.lines() {
//         // let regexp_string = format!("^(.*) bags contain.*(\\d+ ({}) bag(s)?(\\.|,){{1}})+$", child_bag);
//         // let regexp_string = format!("^(.*) bags contain.*(\\d+ (\w+) bag(s)?(\\.|,){{1}})+$", child_bag);
//         // let re = Regex::new(&regexp_string).unwrap();
//         // let re = Regex::new(r"^(.*) bags contain.*(shiny gold) bag.*$").unwrap();
//         // let re = Regex::new(r"^(\w+\s\w+) bags contain.*(\d+ (\w+\s{1}\w+) bag[s]?[\.|,]{1})+$").unwrap();
//         let re = Regex::new(r"^(\w+\s\w+) bags contain(.*)").unwrap();
//         let line = line.unwrap();
//         if re.is_match(&line) {
//             println!("{}", line);
//             // let cap = re.captures(&line).unwrap();
//             // println!("cap 0 {}", &cap[0]);
//             // println!("cap 1 {}", &cap[1]);
//             // println!("cap 2 {}", &cap[2]);
//             // if cap.len() >= 4 {
//             //     println!("cap 3 {}", &cap[3]);
//             // }
//             // if cap.len() >= 5 {
//             //     println!("cap 4 {}", &cap[4]);
//             // }
//             let inside = cap[2];

//             for cap in re.captures_iter(&line) {
//                 println!("cap 0 {}", &cap[0]);
//                 println!("cap 1 {}", &cap[1]);
//                 println!("cap 2 {}", &cap[2]);
//             }
//         }
//     }
    
//     Ok(())
// }

// Solutions from : https://github.com/timvisee/advent-of-code-2020
// use regex::Regex;

// lazy_static::lazy_static! {
//     static ref RE_RULE: Regex = Regex::new(r#"^([a-z ]+) bags contain (.*)$"#).unwrap();
//     static ref RE_CONT: Regex = Regex::new(r#"\d ([a-z ]+) b"#).unwrap();
// }

// fn main() {
//     let data = std::fs::read_to_string("./input").unwrap();
//     let rules: Vec<_> = data.lines().map(parse_bag).collect();

//     let (mut bags, mut cursor) = (vec!["shiny gold"], 0);

//     while let Some(target) = bags.get(cursor) {
//         let extra = rules
//             .iter()
//             .filter(|(color, cont)| cont.contains(target) && !bags.contains(color))
//             .map(|(color, _)| *color)
//             .collect::<Vec<_>>();
//         bags.extend_from_slice(&extra);
//         cursor += 1;
//     }

//     println!("{}", bags.len() - 1);
// }

// /// Parse bag ruleset.
// #[inline(always)]
// fn parse_bag<'a>(rule: &'a str) -> (&'a str, Vec<&str>) {
//     let captures = RE_RULE.captures(rule).unwrap();
//     (
//         captures.get(1).unwrap().as_str(),
//         RE_CONT
//             .captures_iter(captures.get(2).unwrap().as_str())
//             .map(|cond| cond.get(1).unwrap().as_str())
//             .collect(),
//     )
// }

use regex::Regex;
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref RE_RULE: Regex = Regex::new(r#"^([a-z ]+) bags contain (.*)$"#).unwrap();
    static ref RE_CONT: Regex = Regex::new(r#"(\d) ([a-z ]+) b"#).unwrap();
}

fn main() {
    let data = std::fs::read_to_string("./input").unwrap();
    let rules: HashMap<_, _> = data.lines().map(parse_bag).collect();
    println!("{}", bags("shiny gold", &rules) - 1);
}

/// Parse bag ruleset.
#[inline(always)]
fn parse_bag<'a>(rule: &'a str) -> (&'a str, Vec<(&str, usize)>) {
    let captures = RE_RULE.captures(rule).unwrap();
    (
        captures.get(1).unwrap().as_str(),
        RE_CONT
            .captures_iter(captures.get(2).unwrap().as_str())
            .map(|cond| (cond.get(2).unwrap().as_str(), cond[1].parse().unwrap()))
            .collect(),
    )
}

/// Count bags in bags.
fn bags(color: &str, rules: &HashMap<&str, Vec<(&str, usize)>>) -> usize {
    1 + rules[color]
        .iter()
        .map(|(color, count)| bags(color, rules) * count)
        .sum::<usize>()
}