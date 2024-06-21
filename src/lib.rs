mod cpu;
mod disasm;
mod fields;
mod handlers;
pub mod instruction;
mod operands;
pub mod simulator;

pub use disasm::{decode_8086, write_8086};
