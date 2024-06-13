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
        fn decode_instruction(first: u8, second: Option<u8>) -> Result<(Operation, Box<dyn InstructionDecoder>), String> {
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
            Err(format!("Unknown opcode: {:08b} {:08b}", first, second.unwrap_or(0)))
        }
    }
}

create_instruction_decoder!(
    (Mov, RegRM, [0b10001000], [0b11111100]),
    (Mov, RMImd, [0b11000110], [0b11111110]),
    (Mov, RegImd, [0b10110000], [0b11110000]),
    (Mov, AccDA, [0b10100000], [0b11111110]),
    (Mov, DAAcc, [0b10100010], [0b11111110]),
    (Mov, SRRM, [0b10001110], [0b11111111]),
    (Mov, SRRM, [0b10001100], [0b11111111]),
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
    (Cmp, AccImd, [0b00111100], [0b11111110]),
    (JE, Inc8, [0b01110100], [0b11111111]),
    (JL, Inc8, [0b01111100], [0b11111111]),
    (JLE, Inc8, [0b01111110], [0b11111111]),
    (JB, Inc8, [0b01110010], [0b11111111]),
    (JBE, Inc8, [0b01110110], [0b11111111]),
    (JP, Inc8, [0b01111010], [0b11111111]),
    (JO, Inc8, [0b01110000], [0b11111111]),
    (JS, Inc8, [0b01111000], [0b11111111]),
    (JNE, Inc8, [0b01110101], [0b11111111]),
    (JNL, Inc8, [0b01111101], [0b11111111]),
    (JNLE, Inc8, [0b01111111], [0b11111111]),
    (JNB, Inc8, [0b01110011], [0b11111111]),
    (JNBE, Inc8, [0b01110111], [0b11111111]),
    (JNP, Inc8, [0b01111011], [0b11111111]),
    (JNO, Inc8, [0b01110001], [0b11111111]),
    (JNS, Inc8, [0b01111001], [0b11111111]),
    (LOOP, Inc8, [0b11100010], [0b11111111]),
    (LOOPZ, Inc8, [0b11100001], [0b11111111]),
    (LOOPNZ, Inc8, [0b11100000], [0b11111111]),
    (JCXZ, Inc8, [0b11100011], [0b11111111]),
    (Push, RM, [0b11111111, 0b00110000], [0b11111111, 0b00111000]),
    (Push, Reg, [0b01010000], [0b11111000]),
    (Push, SR, [0b00000110], [0b11100111]),
    (Pop, RM, [0b10001111, 0b00000000], [0b11111111, 0b00111000]),
    (Pop, Reg, [0b01011000], [0b11111000]),
    (Pop, SR, [0b00000111], [0b11100111]),
    (XCHG, RegRM, [0b10000110], [0b11111110]),
    (XCHG, AccReg, [0b10010000], [0b11111000]),
    (IN, FixedPort, [0b11100100], [0b11111110]),
    (IN, VariablePort, [0b11101100], [0b11111110]),
    (OUT, FixedPort, [0b11100110], [0b11111110]),
    (OUT, VariablePort, [0b11101110], [0b11111110]),
    (XLAT, NoOps, [0b11010111], [0b11111111]),
    (LEA, RegRMW, [0b10001101], [0b11111111]),
    (LDS, RegRMW, [0b11000101], [0b11111111]),
    (LES, RegRMW, [0b11000100], [0b11111111])
);

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
        assert_eq!(instructions[0].to_string(), "mov [256], ax");
        assert_eq!(instructions[1].to_string(), "mov [si + 4], word 256");
    }
}
