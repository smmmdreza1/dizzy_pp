use std::collections::{BTreeMap, HashMap};

use std::fs;
use std::env::args;
// use std::io::prelude::*;

use std::error::Error;

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
        return self.parse::<usize>().unwrap_or_else(|err| panic!("Err: cannont convert the string slice to an uint becouse: {}", err));
    }
}

fn undizzy(data: &str) -> String {
    let parts: Vec<_> = data.split_whitespace().collect();
    let mut sorted_hashmap = BTreeMap::new();

    for part in parts {
        sorted_hashmap
            .entry(part.tail().to_uint())
            .or_insert(part.head());
    }

    let ret = String::from_iter(sorted_hashmap.values());
    return ret;
}

fn dizzy(data: &str) -> String {
    let mut unsorted_hashmap = HashMap::new();

    for (i, c) in data.chars().enumerate() {
        unsorted_hashmap.entry(i.to_string()).or_insert(c);
    }

    let my_vec: Vec<_> = unsorted_hashmap
        .iter()
        .map(|(k, v)| format!("{}{}", v, k))
        .collect();

    let ret = my_vec.join(" ");
    return ret;
}

fn read_file(path: &str) -> BoxResult<String> {
    return Ok(fs::read_to_string(path)?);
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() >= 3 {
        let command = args[1].as_str();

        if args[2] == "-f" {
            let file_name = &args[3];

            let file_content = read_file(file_name).expect(&format!("Err: Cannont find or read {}!", file_name));

            match command {
                "undizzy" => println!("Result: {}", undizzy(&file_content)),
                "dizzy"   => println!("Result: {}", dizzy(&file_content)),
                _         => println!("You have to type a command (dizzy/undizzy)!"),
            }
        } else {
            let data = &args[2..].join(" ");

            match command {
                "undizzy" => println!("Result: {}", undizzy(data)),
                "dizzy"   => println!("Result: {}", dizzy(data)),
                _         => println!("You have to type a command (dizzy/undizzy)!"),
            }
        }
    } else {
        println!("NOT ENOUGH ARGUMENTS!");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_undizzy_and_dizzy() {
        let need_to_undizzy = undizzy("T4 l16 _36 510 _27 s26 _11 320 414 {6 }39 C2 T0 m28 317 y35 d31 F1 m22 g19 d38 z34 423 l15 329 c12 ;37 19 h13 _30 F5 t7 C3 325 z33 _21 h8 n18 132 k24");

        let need_to_dizzy = dizzy("TFCCTF{th15_ch4ll3ng3_m4k3s_m3_d1zzy_;d}");

        assert_eq!(need_to_undizzy, undizzy(&need_to_dizzy));
    }

    #[test]
    fn is_working() {
        // let string = "77";
        // println!("{}", string.head());
        // println!("{}", string.to_uint());
    }
}
