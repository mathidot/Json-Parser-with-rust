use std::{iter::Peekable, str::Chars};

fn parse_key(chars: &mut Peekable<Chars>) -> Result<(), String> {
    chars.next();
    while let Some(c) = chars.peek() {
        match c {
            '"' => {
                chars.next();
                return Ok(())
            },
            _ => {
                chars.next();
            }
        }
    }
    
    Err(String::from("Invalid Json format, cannot find right double quotes"))
}

fn parse_value(chars: &mut Peekable<Chars>) -> Result<(), String> {
    if let Some(c) = chars.peek() {
        match c {
            '"' => {
                if let Err(e) = parse_key(chars) {
                    return Err(e);
                } else {
                    return Ok(());
                }
            },
            _  => {
                return Err(String::from("unkown value format"));
            }
        }       
    } else {
        return Err(String::from("fail to parse empty value"));
    }
}

pub fn parse_json(chars: &mut Peekable<Chars>) -> Result<(), String> {
    // Check the first character
    skip_whitespace(chars);

    if let Some('{') = chars.peek() {
        chars.next();
    } else {
        return Err(String::from("Invalid JSON: expected {"));
    }

    skip_whitespace(chars);

    if let Some('}') = chars.peek() {
        chars.next();
        skip_whitespace(chars);
        if let Some(_) = chars.next() {
            return Err(String::from("There are some characters after }"));
        } else {
            return Ok(());
        }
    }
    loop {
        if let Some('"') = chars.peek() {
            if let Err(e) = parse_key(chars) {
                return Err(e);
            }
        }

        skip_whitespace(chars);

        if let Some(':') = chars.peek() {
            chars.next();
        } else {
            return Err(format!("Invalid character after key: {:?}", chars.peek()));
        }

        skip_whitespace(chars);

        if let Err(e) = parse_value(chars){
            return Err(e);
        }

        skip_whitespace(chars);

        match chars.peek() {
            Some('}') => {
                chars.next();
                skip_whitespace(chars);
                if let Some(_) = chars.peek(){
                    return Err(format!("Invalid character after }}: {:?}", chars.peek()));
                }
                return Ok(());
            },
            Some(',') => {
                chars.next();
                skip_whitespace(chars);
                ()
            },
            Some(_) | None => {
                return Err(format!("Invalid character in the end of text: {:?}", chars.peek()));
            }
        }
    }
}

fn skip_whitespace(chars: &mut Peekable<Chars>) {
    match chars.peek() {
        Some(' ') | Some('\n') => {
            chars.next();
        },
        _ => {
            return;
        }
    }
}

mod tests{
    use super::*;
    #[test]
    fn test_empty_json() {
        let ts = String::from("{ }");
        let ret = parse_json(&mut ts.chars().peekable());
        assert_eq!(ret, Ok(()))
    }


    #[test]
    fn test_key_value_json() {
        let ts = String::from("{ \"key\" : \"value\" }");
        let ret = parse_json(&mut ts.chars().peekable());
        assert_eq!(ret, Ok(()))
    }


    #[test]
    fn test_mul_key_value_json() {
        let ts = String::from("{\"key1\" : \"value1\", \"key2\" : \"value2\"}");
        let ret = parse_json(&mut ts.chars().peekable());
        assert_eq!(ret, Ok(()))
    }

}