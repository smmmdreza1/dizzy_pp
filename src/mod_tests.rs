use crate::*;

#[test]
fn test_undizzy() {
    struct Test {
        infile: &'static str,
        want: String,
    }

    let tests = vec![
        Test {
            infile: "testfiles/undizzy_test1.dat",
            want: "TFCCTF{th15_ch4ll3ng3_m4k3s_m3_d1zzy_;d}".to_string(),
        },
        Test {
            infile: "testfiles/undizzy_test2.dat",
            want: "rust test split single whitespace".to_string(),
        },
    ];

    for test in tests {
        let got = undizzy(&read_file(test.infile).unwrap()).unwrap();

        assert_eq!(got, test.want);
    }
}

#[test]
fn test_dizzy() {
    struct Test {
        infile: &'static str,
        want: String,
    }

    let tests = vec![
        Test {
            infile: "testfiles/dizzy_test1.dat",
            want: read_file("testfiles/undizzy_test1.dat").unwrap(),
        },
        Test {
            infile: "testfiles/dizzy_test2.dat",
            want: read_file("testfiles/undizzy_test2.dat").unwrap(),
        },
    ];

    for test in tests {
        let got = dizzy(&read_file(test.infile).unwrap());

        assert_eq!(undizzy(&got).unwrap(), undizzy(&test.want).unwrap());
    }
}

#[test]
fn test_split_single_whitespace() {
    struct Test {
        infile: &'static str,
        want: Vec<String>,
    }

    let tests = vec![
        Test {
            infile: "testfiles/split_sw_test1.dat",
            want: vec![
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
            ],
        },
        Test {
            infile: "testfiles/split_sw_test2.dat",
            want: vec![
                String::from("split"),
                String::from("single"),
                String::from("whitespace"),
                String::from("test"),
                String::from(" (forsen)"),
            ],
        },
    ];

    for test in tests {
        let file_content = read_file(test.infile).unwrap();

        let got = split_single_whitespace(&file_content);

        assert_eq!(got, test.want);
    }

}
