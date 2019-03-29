use std::env;
use std::fs;
use std::io::{Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    if args.len() < 3 {
        eprintln!("{} <input_file> <output_file>", program);
        return;
    }
    let input_fname = &args[1];
    let output_fname = &args[2];

    let mut input_file = fs::File::open(input_fname)
        .unwrap_or_else(|err| panic!("error opening input {}: {}", input_fname, err));
    let mut contents = Vec::new();
    input_file.read_to_end(&mut contents)
        .unwrap_or_else(|err| panic!("read error: {}", err));

    let mut output_file = fs::File::create(output_fname)
        .unwrap_or_else(|err| panic!("error opening output {}: {}", output_fname, err));
    output_file.write_all(&contents)
        .unwrap_or_else(|err| panic!("write error: {}", err));
}
