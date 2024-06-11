use std::io::{self, Write};
use std::slice::Iter;

use fields::Operation;
use instruction::{Inst, InstructionDecoder};
use operands::*;

mod extractors;
mod fields;
mod instruction;
mod operands;

macro_rules! create_instruction_decoder {
    (
        $(
            ($operation:ident, $operand_type:ty, $opcode:expr, $mask:expr)
        ),*
    ) => {
        fn decode_instruction<'a>(input: u8, iter: impl Iterator<Item = &'a u8>) -> Result<(Operation, Box<dyn InstructionDecoder>), ()> {
            let mut peekable_iter = iter.peekable();
            $(
                if $opcode.len() == 1 && $mask.len() == 1 {
                    if (input & $mask[0]) == $opcode[0] {
                        return Ok((Operation::$operation, Box::<$operand_type>::default()));
                    }
                } else if $opcode.len() == 2 && $mask.len() == 2 {
                    if let Some(&second_byte) = peekable_iter.peek() {
                        #[allow(unconditional_panic)]
                        #[allow(clippy::out_of_bounds_indexing)]
                        // https://github.com/rust-lang/rust/issues/90534
                        if (input & $mask[0]) == $opcode[0] && (second_byte & $mask[1]) == $opcode[1] {
                            return Ok((Operation::$operation, Box::<$operand_type>::default()));
                        }
                    }
                }
            )*
            Err(())
        }
    }
}

create_instruction_decoder!(
    (Mov, RegRM, [0b10001000], [0b11111100]),
    (Mov, RMImd, [0b11000110], [0b11111110]),
    (Mov, RegImd, [0b10110000], [0b11110000]),
    (Mov, AccDA, [0b10100000], [0b11111110]),
    (Mov, DAAcc, [0b10100010], [0b11111110])
);

pub fn decode_8086(byte_stream: &mut Iter<u8>) -> Vec<Inst> {
    let mut byte_stream = byte_stream;
    let mut instructions = Vec::new();
    let mut first_instruction_byte = byte_stream.next();
    while let Some(&first_byte) = first_instruction_byte {
        let (op, decoder) = decode_instruction(first_byte, &mut byte_stream).unwrap();
        instructions.push(decoder.decode(first_byte, byte_stream, op));
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
        let instructions = decode_8086(&mut bytes[..].iter());
        assert_eq!(instructions[0].to_string(), "mov [256], ax");
        assert_eq!(instructions[1].to_string(), "mov [si + 4], word 256");
    }
}
