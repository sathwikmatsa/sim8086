use std::{env, fs::File, io::Read};
use sim8086::{decode_8086, write_8086};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file_path>", args[0]);
        return;
    }

    let file_path = &args[1];
    let mut file = File::open(file_path).expect("Open file");

    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).expect("read file");

    let instructions = decode_8086(&mut bytes.iter());

    let out_filepath = format!("{}.8086.decoded", file_path);
    let mut out_file = File::create(out_filepath).expect("Open output file");

    write_8086(instructions, &mut out_file).expect("Failed to write to output file");
}
