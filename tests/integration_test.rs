use std::{fs::File, io::Read, process::Command};

use sim8086::{decode_8086, instruction::Inst, simulator::Simulator, write_8086};

fn run_nasm(filename: &str) -> Result<bool, std::io::Error> {
    let status = Command::new("nasm").arg(filename).status()?;

    Ok(status.success())
}

fn check_diff(file1: &str, file2: &str) -> Result<bool, std::io::Error> {
    let status = Command::new("diff").arg(file1).arg(file2).status()?;

    Ok(status.success())
}

fn decode(file_path: &str) -> Vec<Inst> {
    let mut file = File::open(file_path).expect("Open file");

    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).expect("read file");

    let instructions = decode_8086(&bytes[..]);

    let out_filepath = format!("{}.sim8086.asm", file_path);
    let mut out_file = File::create(out_filepath).expect("Open output file");

    write_8086(&instructions, &mut out_file).expect("Failed to write to output file");
    instructions
}

fn decode_test_fixture(name: &str) -> Vec<Inst> {
    let fullpath = format!("tests/artifacts/{}", name);
    let name = fullpath.as_str();
    assert!(run_nasm(format!("{}.asm", name).as_str()).unwrap_or(false));
    let instructions = decode(name);
    assert!(run_nasm(format!("{}.sim8086.asm", name).as_str()).unwrap_or(false));
    assert!(check_diff(name, format!("{}.sim8086", name).as_str()).unwrap_or(false));
    instructions
}

fn sim_test_fixture(name: &str) -> Simulator {
    let instructions = decode_test_fixture(name);
    let mut simulator = Simulator::default();
    let mut program = instructions.try_into().expect("decoded properly");
    simulator.exec(&mut program);
    simulator
}

#[test]
fn more_movs() {
    decode_test_fixture("listing_39");
}

#[test]
fn challenge_movs() {
    decode_test_fixture("listing_40");
}

#[test]
fn add_sub_cmp() {
    decode_test_fixture("listing_41_half");
}

#[test]
fn jumps() {
    decode_test_fixture("listing_41_otherhalf");
}

#[test]
fn completionist() {
    decode_test_fixture("listing_42");
}

#[test]
fn simulate_immediate_movs() {
    let output = sim_test_fixture("listing_0043_immediate_movs").to_string();
    let expected = r#"Final registers:
      ax: 0x0001 (1)
      bx: 0x0002 (2)
      cx: 0x0003 (3)
      dx: 0x0004 (4)
      sp: 0x0005 (5)
      bp: 0x0006 (6)
      si: 0x0007 (7)
      di: 0x0008 (8)"#;
    assert_eq!(output.trim(), expected);
}

#[test]
fn simulate_register_movs() {
    let output = sim_test_fixture("listing_0044_register_movs").to_string();
    let expected = r#"Final registers:
      ax: 0x0004 (4)
      bx: 0x0003 (3)
      cx: 0x0002 (2)
      dx: 0x0001 (1)
      sp: 0x0001 (1)
      bp: 0x0002 (2)
      si: 0x0003 (3)
      di: 0x0004 (4)"#;
    assert_eq!(output.trim(), expected);
}

#[test]
fn simulate_challenge_register_movs() {
    let output = sim_test_fixture("listing_0045_challenge_register_movs").to_string();
    let expected = r#"Final registers:
      ax: 0x4411 (17425)
      bx: 0x3344 (13124)
      cx: 0x6677 (26231)
      dx: 0x7788 (30600)
      sp: 0x4411 (17425)
      bp: 0x3344 (13124)
      si: 0x6677 (26231)
      di: 0x7788 (30600)
      ds: 0x3344 (13124)
      ss: 0x4411 (17425)
      es: 0x6677 (26231)"#;
    assert_eq!(output.trim(), expected);
}

#[test]
fn simulate_add_sub_cmp() {
    let output = sim_test_fixture("listing_0046_add_sub_cmp").to_string();
    let expected = r#"Final registers:
      bx: 0xe102 (57602)
      cx: 0x0f01 (3841)
      sp: 0x03e6 (998)
   flags: PZ"#;
    assert_eq!(output.trim(), expected);
}

#[test]
fn simulate_challenge_flags() {
    let output = sim_test_fixture("listing_0047_challenge_flags").to_string();
    let expected = r#"Final registers:
      bx: 0x9ca5 (40101)
      dx: 0x000a (10)
      sp: 0x0063 (99)
      bp: 0x0062 (98)
   flags: ACPS"#;
    assert_eq!(output.trim(), expected);
}

#[test]
fn simulate_ip_register() {
    let mut sim = sim_test_fixture("listing_0048_ip_register");
    sim.enable_ip_log();
    let output = sim.to_string();
    let expected = r#"Final registers:
      bx: 0x07d0 (2000)
      cx: 0xfce0 (64736)
      ip: 0x000e (14)
   flags: CS"#;
    assert_eq!(output.trim(), expected);
}

#[test]
fn simulate_conditional_jumps() {
    let mut sim = sim_test_fixture("listing_0049_conditional_jumps");
    sim.enable_ip_log();
    let output = sim.to_string();
    let expected = r#"Final registers:
      bx: 0x0406 (1030)
      ip: 0x000e (14)
   flags: PZ"#;
    assert_eq!(output.trim(), expected);
}
