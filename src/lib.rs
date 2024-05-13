/// add help command
/// fix error handlings for user_input_run function (it hasnt to panic)

use std::collections::{BTreeMap, HashMap};

use std::{fs, io::{self, Write}};
use std::env::args;
// use std::io::prelude::*;

#[cfg(test)]
pub mod mod_tests;

type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

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

// This commented cuz i think its useless for this project but we can use
// this method too for this project.
// #[derive(Debug)]
// struct Cli {
//     command: String,
//     flag: Option<String>,
//     path: Option<std::path::PathBuf>,
// }

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

fn remove_first_and_last_char(s: &mut String) -> &mut String {
    s.pop();

    if s.len() > 0 {
        s.remove(0);
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
        "undizzy" => {
            println!("Result: {}\n", undizzy(data));
        },
        "dizzy"   => {
            println!("Result: {}\n", dizzy(data));
        },
        _ => println!("Err: command not found! Type (dizzy/undizzy)"),
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
                    remove_first_and_last_char(&mut data);
                }

                perform_command(command, &data);
            },
        }

        input.clear();
    }
}

fn cli_run(args: Vec<String>) {
    let command = args.get(1).expect("Err: You didnt pass the command!");

    // let mut flag = Option::default();
    // let mut path = Option::default();

    match args.get(2) {
        Some(f) if f == "-f" => {
            // flag = Some(String::from("-f"));

            if let Some(file_name) = args.get(3) {
                // path = Some(std::path::PathBuf::from(file_name));

                if let Ok(file_content) = read_file(file_name) {
                    perform_command(command, &file_content);
                } else {
                    eprintln!("Err: Cannont find or read {}!", file_name);
                }
            } else {
                // path = None;

                eprintln!("Err: Expected file path!");
            }
        }
        _ => {
            let data = &args[2..].join(" ");

            perform_command(command, data);
        }
    }

    // let cli = Cli {
    //     command: command.to_string(),
    //     flag,
    //     path,
    // };
}


// reminder: its okay the cli_run function panics but user_input_run will never be paniced.
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
