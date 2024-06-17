use std::io::{self, Write};

mod decoder;
mod extractors;
mod fields;
mod instruction;
mod operands;

use decoder::decode_instruction;
use instruction::{Inst, InstructionDecoder};

pub fn decode_8086(byte_stream: &[u8]) -> Vec<Inst> {
    let mut byte_stream = byte_stream.iter().peekable();
    let mut instructions = Vec::new();
    let mut first_instruction_byte = byte_stream.next();
    while let Some(&first_byte) = first_instruction_byte {
        let second_byte = byte_stream.peek().map(|&v| *v);
        let (op, decoder) = decode_instruction(first_byte, second_byte).unwrap();
        instructions.push(decoder.decode(first_byte, &mut byte_stream, op));
        first_instruction_byte = byte_stream.next();
    }
    instructions
}

pub fn write_8086(instructions: Vec<Inst>, f: &mut impl Write) -> Result<(), io::Error> {
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
