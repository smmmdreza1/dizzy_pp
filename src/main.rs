use std::collections::{BTreeMap, HashMap};

trait MyStr {
    fn head(&self) -> char;
    fn tail(&self) -> &Self;
    fn to_uint(&self) -> usize;
}

impl MyStr for str {
    fn head(&self) -> char {
        return self.chars().next().unwrap();
    }

    fn tail(&self) -> &Self {
        return &self[1..];
    }

    fn to_uint(&self) -> usize {
        return self.parse::<usize>().unwrap();
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

fn main() {
    let data = "T4 l16 _36 510 _27 s26 _11 320 414 {6 }39 C2 T0 m28 317 y35 d31 F1 m22 g19 d38 z34 423 l15 329 c12 ;37 19 h13 _30 F5 t7 C3 325 z33 _21 h8 n18 132 k24";

    dbg!(undizzy(data));

    let data2 = "TFCCTF{th15_ch4ll3ng3_m4k3s_m3_d1zzy_;d}";

    dbg!(dizzy(data2));
    dbg!(undizzy(&dizzy(data2)));
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
}
