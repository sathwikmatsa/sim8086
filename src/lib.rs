use std::io::{self, Write};

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
        fn decode_instruction(first: u8, second: Option<u8>) -> Result<(Operation, Box<dyn InstructionDecoder>), ()> {
            $(
                if $opcode.len() == 1 && $mask.len() == 1 {
                    if (first & $mask[0]) == $opcode[0] {
                        return Ok((Operation::$operation, Box::<$operand_type>::default()));
                    }
                } else if $opcode.len() == 2 && $mask.len() == 2 {
                    if let Some(second_byte) = second {
                        #[allow(unconditional_panic)]
                        #[allow(clippy::out_of_bounds_indexing)]
                        // https://github.com/rust-lang/rust/issues/90534
                        if (first & $mask[0]) == $opcode[0] && (second_byte & $mask[1]) == $opcode[1] {
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
    (Mov, DAAcc, [0b10100010], [0b11111110]),
    (Add, RegRM, [0b00000000], [0b11111100]),
    (
        Add,
        RMImdS,
        [0b10000000, 0b00000000],
        [0b11111100, 0b00111000]
    ),
    (Add, AccImd, [0b00000100], [0b11111110]),
    (Sub, RegRM, [0b00101000], [0b11111100]),
    (
        Sub,
        RMImdS,
        [0b10000000, 0b00101000],
        [0b11111100, 0b00111000]
    ),
    (Sub, AccImd, [0b00101100], [0b11111110]),
    (Cmp, RegRM, [0b00111000], [0b11111100]),
    (
        Cmp,
        RMImdS,
        [0b10000000, 0b00111000],
        [0b11111100, 0b00111000]
    ),
    (Cmp, AccImd, [0b00111100], [0b11111110])
);

pub fn decode_8086(byte_stream: &[u8]) -> Vec<Inst> {
    let mut byte_stream = byte_stream.iter().peekable();
    let mut instructions = Vec::new();
    let mut first_instruction_byte = byte_stream.next();
    while let Some(&first_byte) = first_instruction_byte {
        let second_byte = byte_stream.peek().map(|&v| *v);
        let (op, decoder) = decode_instruction(first_byte, second_byte).expect("Opcode handled");
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
        assert_eq!(instructions[0].to_string(), "mov [256], ax");
        assert_eq!(instructions[1].to_string(), "mov [si + 4], word 256");
    }
}
