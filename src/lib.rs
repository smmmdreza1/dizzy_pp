/// fix the tests
/// add help command

use std::collections::{BTreeMap, HashMap};

use std::{fs, io::{self, Write}};
use std::env::args;
// use std::io::prelude::*;

use std::error::Error;

#[cfg(test)]
mod mod_tests;

type BoxResult<T> = Result<T, Box<dyn Error>>;

// this macro is actually useless for this program but ill keep it
// maybe. (keep in mind i need to learn more about macros in rust)
#[allow(unused_macros)]
macro_rules! tuple_to_vec {
    ($tuple:ident) => {
        {
            let mut vec = Vec::new();

            $(
                vec.push($tuple.$idx);
            )*

            vec
        }
    };
}

trait MyStr {
    fn head(&self) -> char;
    fn tail(&self) -> &Self;
    fn to_uint(&self) -> usize;
}

impl MyStr for str {
    fn head(&self) -> char {
        return self.chars().next()
            .expect("Err: cannot take the head of string slice beacouse the string slice is empty!");
    }

    fn tail(&self) -> &Self {
        return &self[1..];
    }

    fn to_uint(&self) -> usize {
        return self.parse::<usize>()
            .unwrap_or_else(|err|
                            panic!("Err: cannont convert the string slice `{}` to an uint becouse: {}", self, err)
            );
    }
}

fn read_file(path: &str) -> BoxResult<String> {
    return Ok(fs::read_to_string(path)?);
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

fn trim_newlines(s: &mut String) -> &mut String {
    if s.ends_with('\n') {
        s.pop();

        if s.ends_with('\r') {
            s.pop();
        }
    }

    return s;
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

fn perform_command(command: &str, data: &str) {
    match command {
        "undizzy" => println!("Result: {}\n", undizzy(data)),
        "dizzy"   => println!("Result: {}\n", dizzy(data)),
        _         => eprintln!("Err: You have to type a command (dizzy/undizzy)!\n"),
    }
}

fn user_input_run() {
    let stdin = io::stdin();
    let mut input = String::new();

    println!();

    loop {
        print!("_> ");
        let _ = io::stdout().flush();  // this line makes sure to
                                       // output the above print is
                                       // emitted immediately.

        stdin.read_line(&mut input)
            .expect("Err: Failed to read line!");

        match input.as_str() {
            "\r\n" => println!(),
            "quit\r\n" | "q\r\n" => {
                println!("You quited the program succsecfullty!");
                return;
            },
            _ => {
                let (command, data) = input.split_once(" ").unwrap();

                let mut data = data.to_string();
                trim_newlines(&mut data);

                if data.starts_with('\"') && data.ends_with( '\"') {
                    data.remove(0);
                    data.pop();
                }

                perform_command(command, &data);
            },
        }

        input.clear();
    }
}

fn cli_run(args: Vec<String>) {
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

pub fn run() {
    let args: Vec<String> = args().collect();

    if args.len() == 1 {
        user_input_run();
    } else if args.len() < 3 {
        eprintln!("Err: NOT ENOUGH ARGUMENTS!");
        return;
    } else {
        cli_run(args);
    }
}
