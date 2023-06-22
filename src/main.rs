use std::io::{self};

fn reverse (val: &str) -> String { return val.chars().rev().collect::<String>(); }

fn clean_string(val: &str) -> String {
    let inpt = val.to_owned();
    return inpt.split_inclusive(':').map(|s|
        reverse(&trimfield(&reverse(&trimfield(s.trim()))))
    ).collect();
}

fn trimfield(val: &str) -> String {
    let mut st = String::new();
    let mut last_c = 'x';
    let mut trim_open : bool = false;
    for c in val.chars() {
        if (last_c == '"' && c == ' ')
        || (last_c == ' ' && c == ' ' && trim_open) {
            trim_open = true;
        } else {
            trim_open = false;
            st.push(c);
        }
        last_c = c;
    }
    return st;
}

fn main() {
    let mut buffer = String::new();
    loop {
        let bytes_read = io::stdin().read_line(&mut buffer).unwrap();
        if bytes_read == 0 { break; };
        print!("{}",clean_string(&buffer));
        buffer.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = r#"
        {
            "  key  ": "  true  ",
            "  empty array  ": [],
            "  empty object  ": {},
            "  empty string  ": "",
            "  null  ": null,
            "  nested  ": {
                "  key  ": "  false  ",
                "  empty array  ": [],
                "  empty object  ": {},
                "  empty string  ": "",
                "  null  ": null
            }
        }
        "#;
        let expected = r#"{"key":"true","empty array":[],"empty object":{},"empty string":"","null":null,"nested":{"key":"false","empty array":[],"empty object":{},"empty string":"","null":null}}"#;
        let cleaned = input.lines().map(|n| clean_string(&n)).collect::<String>();
        assert_eq!(cleaned, expected);
    }
}

