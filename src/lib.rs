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
    fn to_uint(&self) -> BoxResult<usize>;
}

impl MyStr for str {
    fn head(&self) -> char {
        return self.chars().next()
            .expect("Err: cannot take the head of string slice beacouse the string slice is empty!");
    }

    fn tail(&self) -> &Self {
        return &self[1..];
    }

    fn to_uint(&self) -> BoxResult<usize> {
        return Ok(self.parse::<usize>()?);
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
            if ( i == 0 && c == ' ' ) ||
               ( i >= 1 && s.chars().nth(i-1).unwrap() == ' ' ) {
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

fn flush_stdout() {
    io::stdout().flush().expect("Failed to flush stdout");
}

fn undizzy(data: &str) -> BoxResult<String> {
    let sorted_hashmap: BTreeMap<_, _> = split_single_whitespace(data)
        .iter()
        .map(|part| -> BoxResult<(_, _)>{
            Ok( ( part.tail().to_uint()? , part.head() ) )
        })
        .collect::<BoxResult<_>>()?;

    let ret: String = sorted_hashmap.values().cloned().collect();
    return Ok(ret);
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
    match command.to_lowercase().as_str() {
        "undizzy" => {
            if data.trim().is_empty() {
                eprintln!("Err: expected text after `{}` command! (use help or ? command for more infos :\\)\n", command);
            } else {
                match undizzy(data) {
                    Ok(val) => println!("Result: {}\n", val),
                    Err(_) => eprintln!("Err: cannot undizzy `{}` the undizzy form is like this: `t11 z4  7 i3 e9 u0 d2 s10 z5 t8 y6 n1`.\n", data),
                }
            }
        },
        "dizzy"   => {
            if data.trim().is_empty() {
                eprintln!("Err: expected text after `{}` command! (use help or ? command for more infos :\\)\n", command);
            } else {
                println!("Result: {}\n", dizzy(data));
            }
        },
        "help" | "?" => {
            if !data.trim().is_empty() {
                eprintln!("Err: why the fuck did you type `{}` ? ...Nevermind here is the help dumbfuck:", data);
            }

            let help_msg = read_file("help.txt").unwrap();

            println!("{help_msg}");
        },
        _ => eprintln!("Err: wtf is this `{}`, seems like you are retarded type `help` :\\\n", command),
    }
}

fn user_input_run() {
    let stdin = io::stdin();
    let mut input = String::new();

    println!();

    loop {
        print!(">_ ");

        flush_stdout();  // this line makes sure to output the above print
                         // is emitted immediately.

        stdin.read_line(&mut input)
            .expect("Err: Failed to read line!");

        match input.to_lowercase().trim() {
            "" => println!(),
            "quit" | "q" => {
                println!("You quited the program succsecfullty!");
                return;
            },
            _ => {
                if let Some( (command, data) ) = input.split_once(" ") {
                    let mut data = data.to_string();
                    trim_newlines(&mut data);

                    if data.starts_with('\"') && data.ends_with( '\"') {
                        remove_first_and_last_char(&mut data);
                    }

                    perform_command(command, &data);
                } else {
                    perform_command(trim_newlines(&mut input), "");
                }
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
    } else if args.len() == 2 {
        if let Some(command) = args.get(1) {
            perform_command(command, "");
        }
    } else if args.len() < 3 {
        eprintln!("Err: NOT ENOUGH ARGUMENTS!");
        return;
    } else {
        cli_run(args);
    }
}
