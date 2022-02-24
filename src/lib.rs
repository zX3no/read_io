#[macro_export]
macro_rules! read(
    () => {{
        use std::io::stdin;
        use std::str::{FromStr};

        let mut s = String::new();
        stdin().read_line(&mut s).expect("Could not flush stdout");
        s.pop();
        s.pop();
        FromStr::from_str(&s).unwrap()
    }};
    ($text:expr) => {{
        use std::io::{stdout, Write};

        print!("{}", $text);
        stdout().flush().expect("Could not flush stdout.");
        read!()
    }
    }
);
