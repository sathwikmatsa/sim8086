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

fn sim_test_fixture_with_clock_estimation(name: &str) -> Simulator {
    let instructions = decode_test_fixture(name);
    let mut simulator = Simulator::default();
    simulator.enable_cycle_estimation();
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

#[test]
fn simulate_challenge_jumps() {
    let mut sim = sim_test_fixture("listing_0050_challenge_jumps");
    sim.enable_ip_log();
    let output = sim.to_string();
    let expected = r#"Final registers:
      ax: 0x000d (13)
      bx: 0xfffb (65531)
      ip: 0x001c (28)
   flags: ACS"#;
    assert_eq!(output.trim(), expected);
}

#[test]
fn simulate_memory_mov() {
    let mut sim = sim_test_fixture("listing_0051_memory_mov");
    sim.enable_ip_log();
    let output = sim.to_string();
    let expected = r#"Final registers:
      bx: 0x0001 (1)
      cx: 0x0002 (2)
      dx: 0x000a (10)
      bp: 0x0004 (4)
      ip: 0x0030 (48)"#;
    assert_eq!(output.trim(), expected);
}

#[test]
fn simulate_memory_add_loop() {
    let mut sim = sim_test_fixture("listing_0052_memory_add_loop");
    sim.enable_ip_log();
    let output = sim.to_string();
    let expected = r#"Final registers:
      bx: 0x0006 (6)
      cx: 0x0004 (4)
      dx: 0x0006 (6)
      bp: 0x03e8 (1000)
      si: 0x0006 (6)
      ip: 0x0023 (35)
   flags: PZ"#;
    assert_eq!(output.trim(), expected);
}

#[test]
fn simulate_memory_add_loop_challenge() {
    let mut sim = sim_test_fixture("listing_0053_add_loop_challenge");
    sim.enable_ip_log();
    let output = sim.to_string();
    let expected = r#"Final registers:
      bx: 0x0006 (6)
      dx: 0x0006 (6)
      bp: 0x03e6 (998)
      ip: 0x0021 (33)
   flags: PZ"#;
    assert_eq!(output.trim(), expected);
}

#[test]
fn draw_rectangle() {
    let mut sim = sim_test_fixture("listing_0054_draw_rectangle");
    sim.enable_ip_log();
    let output = sim.to_string();
    let expected = r#"Final registers:
      cx: 0x0040 (64)
      dx: 0x0040 (64)
      bp: 0x4000 (16384)
      ip: 0x0026 (38)
   flags: PZ"#;
    assert_eq!(output.trim(), expected);
    sim.dump_memory(File::create("draw_rectangle.data").unwrap())
        .unwrap();
}

#[test]
fn challenge_rectangle() {
    let mut sim = sim_test_fixture("listing_0055_challenge_rectangle");
    sim.enable_ip_log();
    let output = sim.to_string();
    let expected = r#"Final registers:
      bx: 0x4004 (16388)
      bp: 0x02fc (764)
      ip: 0x0044 (68)"#;
    assert_eq!(output.trim(), expected);
    sim.dump_memory(File::create("challenge_rectangle.data").unwrap())
        .unwrap();
}

#[test]
fn estimating_cycles() {
    let mut sim = sim_test_fixture_with_clock_estimation("listing_0056_estimating_cycles");
    sim.enable_ip_log();
    let output = sim.to_string();
    let expected = r#"Final registers:
      bx: 0x03e8 (1000)
      dx: 0x0032 (50)
      bp: 0x07d0 (2000)
      si: 0x0bb8 (3000)
      di: 0x0fa0 (4000)
      ip: 0x0037 (55)"#;
    assert_eq!(output.trim(), expected);
    assert_eq!(sim.clocks_8086(), 192);
    assert_eq!(sim.clocks_8088(), 236);
}

#[test]
fn challenge_cycles() {
    let mut sim = sim_test_fixture_with_clock_estimation("listing_0057_challenge_cycles");
    sim.enable_ip_log();
    let output = sim.to_string();
    let expected = r#"Final registers:
      bx: 0x03e8 (1000)
      bp: 0x07d0 (2000)
      si: 0x0bb8 (3000)
      di: 0x0fa0 (4000)
      ip: 0x0036 (54)
   flags: A"#;
    assert_eq!(output.trim(), expected);
    assert_eq!(sim.clocks_8086(), 289);
    assert_eq!(sim.clocks_8088(), 341);
}

#[test]
fn single_scalar() {
    let mut sim = sim_test_fixture_with_clock_estimation("listing_0059_SingleScalar");
    sim.enable_ip_log();
    let output = sim.to_string();
    let expected = r#"Final registers:
      ax: 0x00bf (191)
      cx: 0x0008 (8)
      bp: 0x03e8 (1000)
      si: 0x0008 (8)
      di: 0x0008 (8)
      ip: 0x0037 (55)
   flags: PZ"#;
    assert_eq!(output.trim(), expected);
    assert_eq!(sim.clocks_8086(), 463);
}
