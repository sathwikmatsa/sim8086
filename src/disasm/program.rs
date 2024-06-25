use crate::{
    fields::{Operand, Operation},
    instruction::{Inst, InstructionPrefix},
};

pub struct Instruction {
    pub operation: Operation,
    pub first: Option<Operand>,
    pub second: Option<Operand>,
    pub prefix: Option<InstructionPrefix>,
    pub size: usize,
}

impl TryFrom<Inst> for Instruction {
    type Error = ();
    fn try_from(value: Inst) -> Result<Self, Self::Error> {
        if value.size().is_some() {
            Ok(Self {
                operation: value.operation,
                first: value.first,
                second: value.second,
                prefix: value.prefix,
                size: value.size().unwrap(),
            })
        } else {
            Err(())
        }
    }
}

pub struct Program {
    ip: usize,
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn next_instruction(&mut self) -> Option<&Instruction> {
        let instruction = self.instructions.get(self.ip);
        if instruction.is_some() {
            self.ip += 1;
        }
        instruction
    }
}

impl TryFrom<Vec<Inst>> for Program {
    type Error = ();
    fn try_from(value: Vec<Inst>) -> Result<Self, Self::Error> {
        let instructions: Vec<Instruction> = value
            .into_iter()
            .map(|x| x.try_into())
            .collect::<Result<Vec<Instruction>, ()>>()?;
        Ok(Self {
            ip: 0,
            instructions,
        })
    }
}
