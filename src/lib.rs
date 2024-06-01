use std::io::{self, Write};

use instruction::Instruction;
use mov::{
    accumulator_to_memory::AccumulatorToMemory, immediate_to_register::ImmediateToRegister,
    immediate_to_register_memory::ImmediateToRegisterMemory,
    memory_to_accumulator::MemoryToAccumulator,
    register_memory_to_from_register::RegisterMemoryToFromRegister,
};

pub mod instruction;
pub mod mov;

macro_rules! decode {
    ($first_byte:expr, $instructions:expr, $byte_stream:expr, $($instruction:ty),+ $(,)?) => {
        let mut matched = false;
        $(
            if <$instruction>::match_opcode($first_byte) && !matched {
                $instructions.push(Box::new(<$instruction>::new($first_byte, &mut $byte_stream)));
                matched = true;
            }
        )+
        if !matched {
            unimplemented!("Unknown opcode");
        }
    };
}

pub fn decode_8086<'a, I>(byte_stream: I) -> Vec<Box<dyn Instruction>>
where
    I: Iterator<Item = &'a u8>,
{
    let mut byte_stream = byte_stream;
    let mut instructions: Vec<Box<dyn Instruction>> = Vec::new();
    let mut first_instruction_byte = byte_stream.next();
    while let Some(&first_byte) = first_instruction_byte {
        decode!(
            first_byte,
            instructions,
            byte_stream,
            AccumulatorToMemory,
            ImmediateToRegisterMemory,
            ImmediateToRegister,
            MemoryToAccumulator,
            RegisterMemoryToFromRegister,
        );
        first_instruction_byte = byte_stream.next();
    }
    return instructions;
}

pub fn write_8086(instructions: Vec<Box<dyn Instruction>>, f: &mut impl Write) -> Result<(), io::Error> {
    writeln!(f, "bits 16;")?;
    for instruction in instructions {
        writeln!(f, "{}", instruction.to_string())?;
    }
    return Ok(());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn decode() {
        let bytes: [u8; 9] = [0b10100011, 0b00000000, 0b00000001, 0b11000111, 0b10000100, 0b00000100, 0b00000000, 0b00000000, 0b00000001];
        let instructions = decode_8086(bytes[..].iter());
        assert_eq!(instructions[0].to_string(), "mov [256], ax");
        assert_eq!(instructions[1].to_string(), "mov [si + 4], 256");
    }
}