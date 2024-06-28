mod decoder;
mod extractors;
mod program;

pub use extractors::*;
pub use program::*;

use std::io::{self, Write};

use crate::{
    instruction::{Inst, InstructionPrefix},
    ByteStream,
};
use decoder::{decode_instruction, DecoderOut};

fn join_prefix(prev: InstructionPrefix, curr: InstructionPrefix) -> InstructionPrefix {
    match (prev, curr) {
        (InstructionPrefix::Lock, InstructionPrefix::SegmentOverride(seg))
        | (InstructionPrefix::SegmentOverride(seg), InstructionPrefix::Lock) => {
            InstructionPrefix::LockSegmentOverride(seg)
        }
        _ => curr,
    }
}

pub fn decode_8086(byte_stream_raw: &[u8]) -> Vec<Inst> {
    let mut byte_stream = ByteStream::new(byte_stream_raw.iter());
    let mut instructions: Vec<Inst> = Vec::new();
    let mut first_instruction_byte = byte_stream.next_with_index();
    let mut inst_prefix: Option<InstructionPrefix> = None;
    let mut start_idx = 0;
    while let Some((end_idx, &first_byte)) = first_instruction_byte {
        if end_idx != start_idx {
            instructions
                .last_mut()
                .expect("atleast one instruction is decoded")
                .set_size(end_idx - start_idx);
            start_idx = end_idx;
        }
        let second_byte = byte_stream.peek().map(|&v| *v);
        match decode_instruction(first_byte, second_byte).unwrap() {
            DecoderOut::Inst(op, decoder) => {
                let mut inst = decoder.decode(first_byte, &mut byte_stream, op);
                if let Some(prefix) = inst_prefix.take() {
                    inst.add_instruction_prefix(prefix);
                }
                instructions.push(inst);
            }
            DecoderOut::Prefix(prefix) => {
                if inst_prefix.is_some() {
                    inst_prefix.replace(join_prefix(prefix, inst_prefix.unwrap()));
                } else {
                    inst_prefix.replace(prefix);
                }
            }
        }
        first_instruction_byte = byte_stream.next_with_index();
    }
    if start_idx != 0 || !byte_stream_raw.is_empty() {
        instructions
            .last_mut()
            .expect("atleast one instruction is decoded")
            .set_size(byte_stream.vended_count() - start_idx);
    }
    instructions
}

pub fn write_8086(instructions: &Vec<Inst>, f: &mut impl Write) -> Result<(), io::Error> {
    writeln!(f, "bits 16;")?;
    for instruction in instructions {
        writeln!(f, "{}", instruction)?;
    }
    Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn decode() {
        let bytes: [u8; 9] = [
            0b10100011, 0b00000000, 0b00000001, 0b11000111, 0b10000100, 0b00000100, 0b00000000,
            0b00000000, 0b00000001,
        ];
        let instructions = decode_8086(&bytes[..]);
        assert_eq!(instructions[0].to_string(), "mov word [256], ax");
        assert_eq!(instructions[1].to_string(), "mov word [si + 4], 256");
    }
}
