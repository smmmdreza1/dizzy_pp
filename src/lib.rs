use std::collections::{BTreeMap, HashMap};

use std::fs;
use std::env::args;
// use std::io::prelude::*;

use std::error::Error;

#[cfg(test)]
mod mod_tests;

type BoxResult<T> = Result<T, Box<dyn Error>>;

trait MyStr {
    fn head(&self) -> char;
    fn tail(&self) -> &Self;
    fn to_uint(&self) -> usize;
}

impl MyStr for str {
    fn head(&self) -> char {
        return self.chars().next().expect("Err: cannot take the head of string slice beacouse the string slice is empty!");
    }

    fn tail(&self) -> &Self {
        return &self[1..];
    }

    fn to_uint(&self) -> usize {
        return self.parse::<usize>().unwrap_or_else(|err| panic!("Err: cannont convert the string slice `{}` to an uint becouse: {}", self, err));
    }
}

fn split_single_whitespace(s: &str) -> Vec<String> {
    let mut ret = Vec::new();
    let mut current = String::new();

    for (i, c) in s.chars().enumerate() {
        if c == ' '  {
            if s.chars().nth(i-1).unwrap() == ' ' {
                current.push(c);
                continue;
            }

            ret.push(current.clone());
            current.clear();
        } else {

            current.push(c);
        }
    }

    if !current.is_empty() {
        ret.push(current);
    }

    return ret;
}

fn undizzy(data: &str) -> String {
    let sorted_hashmap: BTreeMap<usize, char> = split_single_whitespace(data)
        .iter()
        .map(|part| (part.tail().to_uint(), part.head()))
        .collect();

    let ret = sorted_hashmap.values().cloned().collect::<String>();
    return ret;
}

fn dizzy(data: &str) -> String {
    let unsorted_hashmap: HashMap<String, char> = data.chars().enumerate()
        .map(| (i, c) | (i.to_string(), c))
        .collect();

    let my_vec: Vec<String> = unsorted_hashmap.iter()
        .map(| (k, v) | format!("{}{}", v, k))
        .collect();

    let ret = my_vec.join(" ");
    return ret;
}

fn read_file(path: &str) -> BoxResult<String> {
    return Ok(fs::read_to_string(path)?);
}

fn perform_command(command: &str, data: &str) {
    match command {
        "undizzy" => println!("Result: {}", undizzy(data)),
        "dizzy"   => println!("Result: {}", dizzy(data)),
        _         => eprintln!("Err: You have to type a command (dizzy/undizzy)!"),
    }
}

pub fn run() {
    let args: Vec<String> = args().collect();

    if args.len() < 3 {
        eprintln!("Err: NOT ENOUGH ARGUMENTS!");
        return;
    }

    let command = args[1].as_str();

    match args.get(2) {
        Some(flag) if flag == "-f" => {
            if let Some(file_name) = args.get(3) {
                if let Ok(file_content) = read_file(file_name) {
                    perform_command(command, &file_content);
                } else {
                    eprintln!("Err: Cannont find or read {}!", file_name);
                }
            }
        }
        _ => {
            let data = &args[2..].join(" ");

            perform_command(command, data);
        }
    }
}
