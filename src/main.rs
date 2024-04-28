use std::collections::{BTreeMap};

fn main() {
    let data =
        "T4 l16 _36 510 _27 s26 _11 320 414 {6 }39 C2 T0 m28 317 y35 d31 F1 m22 g19 d38 z34 423 l15 329 c12 ;37 19 h13 _30 F5 t7 C3 325 z33 _21 h8 n18 132 k24";

    let parts: Vec<_> = data.split_whitespace().collect();
    let mut sorted_hashmap = BTreeMap::new();

    for part in parts {
        let first = part.chars().next().unwrap();
        let rest = part[1..].parse::<usize>().unwrap();
        sorted_hashmap.entry(rest).or_insert(first);
    }

    let answer = String::from_iter(sorted_hashmap.values());

    println!("answer is: {}", answer);
}
