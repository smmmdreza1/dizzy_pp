use crate::*;

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
