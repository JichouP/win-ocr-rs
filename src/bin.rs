use std::env;
use win_ocr::*;

#[derive(Debug)]
struct Cli {
    input: String,
}

fn main() {
    let input = env::args().nth(1);

    let input = match input {
        Some(input) => input,
        None => {
            println!("Error: no input path given");
            return;
        }
    };

    let res = ocr(&input).expect("ocr failed");

    println!("{}", res);
}
