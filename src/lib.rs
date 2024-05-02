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

pub fn run() {
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
    fn test_split_single_whitespace() {
        let file_content = read_file("src/split_sw.xyz").expect("Err: Cannont find or read `split.sw.xyz`!");

        let got = split_single_whitespace(&file_content);

        let want = vec![
            String::from("s16"), String::from("n18"), String::from("i25"),
            String::from("s10"), String::from("e32"), String::from("t8") ,
            String::from("l20"), String::from("i13"), String::from("s28"),
            String::from("t3") , String::from("a30"), String::from(" 15"),
            String::from(" 4") , String::from("e27"), String::from("h24"),
            String::from("p29"), String::from("e6") , String::from(" 9") ,
            String::from("i17"), String::from("t5") , String::from("s2") ,
            String::from("g19"), String::from("s7") , String::from("w23"),
            String::from("l12"), String::from("r0") , String::from("p11"),
            String::from("t14"), String::from(" 22"), String::from("t26"),
            String::from("u1") , String::from("c31"), String::from("e21"),
        ];

        assert_eq!(got, want);
    }
}
