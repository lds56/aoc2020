use regex::Regex;

use std::collections::HashMap;
use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<HashMap<String, String>> {

    input
        .split("\n\n")
        .map(|case| {

            let mut input = HashMap::new();
            
            case
                .replace('\n', " ")
                .split(' ')
                .for_each(|kv: &str| {
                    let kv_split: Vec<&str> = kv.splitn(2, ':').collect();
                    input.insert(kv_split[0].to_string(), kv_split[1].to_string());
                });

            input

        }).collect::<Vec<HashMap<String, String>>>()
}

#[aoc(day4, part1)]
pub fn solvor(inputs: &[HashMap<String, String>]) -> usize {

    /*
    for input in inputs.iter() {
        for (k, v) in input.iter() {
            println!("{}, {}", k, v);
        }
        println!("========");
    }
    */

    let required_keys: Vec<String> = vec![
        "byr".to_string(), "iyr".to_string(), "eyr".to_string(), "hgt".to_string(),
        "hcl".to_string(), "ecl".to_string(), "pid".to_string()
    ];
    let _optional_key = "cid".to_string();

    let key_set: HashSet<String> = required_keys.into_iter().collect();

    inputs
        .iter()
        .filter(|input| {
            for k in key_set.iter() {
                if !input.contains_key(k) {
                    return false;
                }
            }
            true
        })
        .count()
}

#[aoc(day4, part2)]
pub fn solvor_part2(inputs: &[HashMap<String, String>]) -> usize {

    let mut validators: HashMap<String, Box<dyn Fn(&str) -> bool>> = HashMap::new();

    validators.insert("byr".to_string(), Box::new(move |s| Regex::new(r"^(19[2-9]\d|200[0-2])$").unwrap().is_match(&s) ));
    validators.insert("iyr".to_string(), Box::new(move |s| Regex::new(r"^(201\d|2020)$").unwrap().is_match(&s) ));
    validators.insert("eyr".to_string(), Box::new(move |s| Regex::new(r"^(202\d|2030)$").unwrap().is_match(&s) ));
    validators.insert("hgt".to_string(), Box::new(move |s| Regex::new(r"^((1[5-8][0-9]|19[0-3])cm|(59|6[0-9]|7[0-6])in)$").unwrap().is_match(&s) ));
    validators.insert("hcl".to_string(), Box::new(move |s| Regex::new(r"^#([0-9]|[a-f]){6}$").unwrap().is_match(&s) ));
    validators.insert("ecl".to_string(), Box::new(move |s| Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap().is_match(&s) ));
    validators.insert("pid".to_string(), Box::new(move |s| Regex::new(r"^\d{9}$").unwrap().is_match(&s) ));

    inputs
        .iter()
        .filter(|input| {

            for (k, validator) in validators.iter() {
                if let Some(val) = input.get(k) {
                    if !validator(val) { return false; }
                } else {
                    return false;
                }
            }
            true
        })
        .count()

}
