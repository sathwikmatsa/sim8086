use std::{fs::File, io::Read, process::Command};

use sim8086::{decode_8086, write_8086};

fn run_nasm(filename: &str) -> Result<bool, std::io::Error> {
    let status = Command::new("nasm").arg(filename).status()?;

    Ok(status.success())
}

fn check_diff(file1: &str, file2: &str) -> Result<bool, std::io::Error> {
    let status = Command::new("diff").arg(file1).arg(file2).status()?;

    Ok(status.success())
}

fn decode(file_path: &str) {
    let mut file = File::open(file_path).expect("Open file");

    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).expect("read file");

    let instructions = decode_8086(&bytes[..]);

    let out_filepath = format!("{}.sim8086.asm", file_path);
    let mut out_file = File::create(out_filepath).expect("Open output file");

    write_8086(instructions, &mut out_file).expect("Failed to write to output file");
}

fn test_fixture(name: &str) {
    let fullpath = format!("tests/artifacts/{}", name);
    let name = fullpath.as_str();
    assert!(run_nasm(format!("{}.asm", name).as_str()).unwrap_or(false));
    decode(name);
    assert!(run_nasm(format!("{}.sim8086.asm", name).as_str()).unwrap_or(false));
    assert!(check_diff(name, format!("{}.sim8086", name).as_str()).unwrap_or(false));
}

#[test]
fn more_movs() {
    test_fixture("listing_39");
}

#[test]
fn challenge_movs() {
    test_fixture("listing_40");
}

#[test]
fn add_sub_cmp() {
    test_fixture("listing_41_half");
}
