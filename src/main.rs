mod parse;
use std::process::exit;
/*
example 1:
{
    "key": value
}
example 2:
{
    "key1": value1,
    "key2": value2
}
example 3:
{}
*/

use parse::parse_json;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("There is no filename");
        exit(0);
    }

    let file_name = &args[1];
    if let Ok(contents) = std::fs::read_to_string(&file_name) {
        let ret = parse_json(&mut contents.chars().peekable());
    } else {
        println!("fail to open file: {}", file_name);
        exit(0);
    }


    println!("Hello, world!");
}
