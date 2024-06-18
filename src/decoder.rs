use instruction::InstructionPrefix;

use crate::fields::Operation;
use crate::operands::*;
use crate::*;
use std::str::FromStr;

pub enum DecoderOut {
    Inst(Operation, Box<dyn InstructionDecoder>),
    Prefix(InstructionPrefix),
}

macro_rules! create_instruction_decoder {
    (
        $(
            ($operation:ident, $operand_type:ty, $opcode:expr, $mask:expr)
        ),*
    ) => {
        pub fn decode_instruction(first: u8, second: Option<u8>) -> Result<DecoderOut, String> {
            $(
                let is_prefix = stringify!($operand_type) == "InstructionPrefix";
                if $opcode.len() == 1 && $mask.len() == 1 {
                    if (first & $mask[0]) == $opcode[0] {
                        if is_prefix {
                            return Ok(DecoderOut::Prefix(InstructionPrefix::from_str(stringify!($operation)).unwrap()));
                        }
                        return Ok(DecoderOut::Inst(Operation::$operation, Box::<$operand_type>::default()));
                    }
                } else if $opcode.len() == 2 && $mask.len() == 2 {
                    if let Some(second_byte) = second {
                        #[allow(unconditional_panic)]
                        #[allow(clippy::out_of_bounds_indexing)]
                        // https://github.com/rust-lang/rust/issues/90534
                        if (first & $mask[0]) == $opcode[0] && (second_byte & $mask[1]) == $opcode[1] {
                            if is_prefix {
                                return Ok(DecoderOut::Prefix(InstructionPrefix::from_str(stringify!($operation)).unwrap()));
                            }
                            return Ok(DecoderOut::Inst(Operation::$operation, Box::<$operand_type>::default()));
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
    (LES, RegRMW, [0b11000100], [0b11111111]),
    (LAHF, NoOps, [0b10011111], [0b11111111]),
    (SAHF, NoOps, [0b10011110], [0b11111111]),
    (PUSHF, NoOps, [0b10011100], [0b11111111]),
    (POPF, NoOps, [0b10011101], [0b11111111]),
    (ADC, RegRM, [0b00010000], [0b11111100]),
    (
        ADC,
        RMImdS,
        [0b10000000, 0b00010000],
        [0b11111100, 0b00111000]
    ),
    (ADC, AccImd, [0b00010100], [0b11111110]),
    (INC, RMW, [0b11111110, 0b00000000], [0b11111110, 0b00111000]),
    (INC, Reg, [0b01000000], [0b11111000]),
    (AAA, NoOps, [0b00110111], [0b11111111]),
    (DAA, NoOps, [0b00100111], [0b11111111]),
    (SBB, RegRM, [0b00011000], [0b11111100]),
    (
        SBB,
        RMImdS,
        [0b10000000, 0b00011000],
        [0b11111100, 0b00111000]
    ),
    (SBB, AccImd, [0b00011100], [0b11111110]),
    (DEC, RMW, [0b11111110, 0b00001000], [0b11111110, 0b00111000]),
    (DEC, Reg, [0b01001000], [0b11111000]),
    (NEG, RMW, [0b11110110, 0b00011000], [0b11111110, 0b00111000]),
    (AAS, NoOps, [0b00111111], [0b11111111]),
    (DAS, NoOps, [0b00101111], [0b11111111]),
    (MUL, RMW, [0b11110110, 0b00100000], [0b11111110, 0b00111000]),
    (
        IMUL,
        RMW,
        [0b11110110, 0b00101000],
        [0b11111110, 0b00111000]
    ),
    (
        AAM,
        NoOps2,
        [0b11010100, 0b00001010],
        [0b11111111, 0b11111111]
    ),
    (
        AAD,
        NoOps2,
        [0b11010101, 0b00001010],
        [0b11111111, 0b11111111]
    ),
    (DIV, RMW, [0b11110110, 0b00110000], [0b11111110, 0b00111000]),
    (
        IDIV,
        RMW,
        [0b11110110, 0b00111000],
        [0b11111110, 0b00111000]
    ),
    (CBW, NoOps, [0b10011000], [0b11111111]),
    (CWD, NoOps, [0b10011001], [0b11111111]),
    (NOT, RMW, [0b11110110, 0b00010000], [0b11111110, 0b00111000]),
    (
        SHL,
        RMVW,
        [0b11010000, 0b00100000],
        [0b11111100, 0b00111000]
    ),
    (
        SHR,
        RMVW,
        [0b11010000, 0b00101000],
        [0b11111100, 0b00111000]
    ),
    (
        SAR,
        RMVW,
        [0b11010000, 0b00111000],
        [0b11111100, 0b00111000]
    ),
    (
        ROL,
        RMVW,
        [0b11010000, 0b00000000],
        [0b11111100, 0b00111000]
    ),
    (
        ROR,
        RMVW,
        [0b11010000, 0b00001000],
        [0b11111100, 0b00111000]
    ),
    (
        RCL,
        RMVW,
        [0b11010000, 0b00010000],
        [0b11111100, 0b00111000]
    ),
    (
        RCR,
        RMVW,
        [0b11010000, 0b00011000],
        [0b11111100, 0b00111000]
    ),
    (AND, RegRM, [0b00100000], [0b11111100]),
    (
        AND,
        RMImd,
        [0b10000000, 0b00100000],
        [0b11111110, 0b00111000]
    ),
    (AND, AccImd, [0b00100100], [0b11111110]),
    (TEST, RegRM, [0b10000100], [0b11111100]),
    (
        TEST,
        RMImd,
        [0b11110110, 0b00000000],
        [0b11111110, 0b00111000]
    ),
    (TEST, AccImd, [0b10101000], [0b11111110]),
    (OR, RegRM, [0b00001000], [0b11111100]),
    (
        OR,
        RMImd,
        [0b10000000, 0b00001000],
        [0b11111110, 0b00111000]
    ),
    (OR, AccImd, [0b00001100], [0b11111110]),
    (XOR, RegRM, [0b00110000], [0b11111100]),
    (
        XOR,
        RMImd,
        [0b10000000, 0b00110000],
        [0b11111110, 0b00111000]
    ),
    (XOR, AccImd, [0b00110100], [0b11111110]),
    (MOVSB, NoOps, [0b10100100], [0b11111111]),
    (MOVSW, NoOps, [0b10100101], [0b11111111]),
    (LODSB, NoOps, [0b10101100], [0b11111111]),
    (LODSW, NoOps, [0b10101101], [0b11111111]),
    (STOSB, NoOps, [0b10101010], [0b11111111]),
    (STOSW, NoOps, [0b10101011], [0b11111111]),
    (CMPSB, NoOps, [0b10100110], [0b11111111]),
    (CMPSW, NoOps, [0b10100111], [0b11111111]),
    (SCASB, NoOps, [0b10101110], [0b11111111]),
    (SCASW, NoOps, [0b10101111], [0b11111111]),
    (Call, Inc16, [0b11101000], [0b11111111]),
    (Call, RM, [0b11111111, 0b00010000], [0b11111111, 0b00111000]),
    (Call, RM, [0b11111111, 0b00011000], [0b11111111, 0b00111000]),
    (Jmp, Inc16, [0b11101001], [0b11111111]),
    (Jmp, Inc8, [0b11101011], [0b11111111]),
    (Jmp, RM, [0b11111111, 0b00100000], [0b11111111, 0b00111000]),
    (Jmp, RM, [0b11111111, 0b00101000], [0b11111111, 0b00111000]),
    (Ret, NoOps, [0b11000011], [0b11111111]),
    (Ret, NoOps, [0b11001011], [0b11111111]),
    (Ret, Data16, [0b11000010], [0b11111111]),
    (Ret, Data16, [0b11001010], [0b11111111]),
    (INT, Data8, [0b11001101], [0b11111111]),
    (INT3, NoOps, [0b11001100], [0b11111111]),
    (INTO, NoOps, [0b11001110], [0b11111111]),
    (IRET, NoOps, [0b11001111], [0b11111111]),
    (CLC, NoOps, [0b11111000], [0b11111111]),
    (CMC, NoOps, [0b11110101], [0b11111111]),
    (STC, NoOps, [0b11111001], [0b11111111]),
    (CLD, NoOps, [0b11111100], [0b11111111]),
    (STD, NoOps, [0b11111101], [0b11111111]),
    (CLI, NoOps, [0b11111010], [0b11111111]),
    (STI, NoOps, [0b11111011], [0b11111111]),
    (HLT, NoOps, [0b11110100], [0b11111111]),
    (WAIT, NoOps, [0b10011011], [0b11111111]),
    (Rep, InstructionPrefix, [0b11110010], [0b11111110]),
    (Lock, InstructionPrefix, [0b11110000], [0b11111111]),
    (
        SegmentOverrideES,
        InstructionPrefix,
        [0b00100110],
        [0b11111111]
    ),
    (
        SegmentOverrideCS,
        InstructionPrefix,
        [0b00101110],
        [0b11111111]
    ),
    (
        SegmentOverrideSS,
        InstructionPrefix,
        [0b00110110],
        [0b11111111]
    ),
    (
        SegmentOverrideDS,
        InstructionPrefix,
        [0b00111110],
        [0b11111111]
    )
);
