use std::io::{self};

fn clean_string(val: &str) -> String {
    let inpt = val.to_owned();
    return inpt.split_inclusive(':').map(|s|
        trimfield(&s.trim())
    ).collect();
}

fn trimfield(val: &str) -> String {
    let mut chars = val.chars();
    let first_c = chars.next();
    let last_c = chars.next_back();
    if first_c == None || last_c == None { return val.to_string(); }
    chars.next_back();
    if last_c != None && last_c.unwrap() == ':' {
        return format!("\"{}\":", chars.as_str().trim());
    } else if first_c == None || first_c.unwrap() == '"' {
        return format!("\"{}\",", chars.as_str().trim());
    } else {
        return val.to_string();
    }
}

fn main() {
    let mut buffer = String::with_capacity(1000);
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

