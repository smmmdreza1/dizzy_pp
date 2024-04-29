use std::collections::BTreeMap;

trait MyStr {
    fn first_char(&self) -> char;
    fn tail(&self) -> &Self;
    fn to_uint(&self) -> usize;
}

impl MyStr for str {
    fn first_char(&self) -> char {
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
            .or_insert(part.first_char());
    }

    let answer = String::from_iter(sorted_hashmap.values());
    return answer;
}

fn main() {
    let data =
        "T4 l16 _36 510 _27 s26 _11 320 414 {6 }39 C2 T0 m28 317 y35 d31 F1 m22 g19 d38 z34 423 l15 329 c12 ;37 19 h13 _30 F5 t7 C3 325 z33 _21 h8 n18 132 k24";

    dbg!(undizzy(data));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_undizzy() {
        let got = undizzy(
"T4 l16 _36 510 _27 s26 _11 320 414 {6 }39 C2 T0 m28 317 y35 d31 F1 m22 g19 d38 z34 423 l15 329 c12 ;37 19 h13 _30 F5 t7 C3 325 z33 _21 h8 n18 132 k24");

        let want = "TFCCTF{th15_ch4ll3ng3_m4k3s_m3_d1zzy_;d}";

        assert_eq!(got, want,
                   "got {}, wanted {}", got, want );
    }
}
