#[macro_export]
macro_rules! read(
    () => {{
        use std::io::stdin;
        use std::str::{FromStr};

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Could not read stdin");
        let trim = input.trim();
        FromStr::from_str(trim)
    }};
    ($text:expr) => {{
        use std::io::{stdout, Write};

        print!("{}", $text);
        stdout().flush().expect("Could not flush stdout.");
        read!()
    }
    }
);

#[test]
fn tests() {
    use std::str::FromStr;

    let input = String::from("20\r\n");
    let trim = input.trim();
    let num: i32 = FromStr::from_str(trim).unwrap();
    assert_eq!(20, num);

    let input = String::from("test\r\n\r\r\r\n");
    let trim = input.trim();
    let test: String = FromStr::from_str(trim).unwrap();
    assert_eq!("test", test);
}
