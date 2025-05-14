use super::{
    Instruction::{self, *},
    instructions_fn::*,
    operands::ArithSource,
};

const ARITH_INSTR_MASK: u8 = 0b00111000;
const ARITH_INSTR_POS: u8 = 3;
const BLOCK_3_INSTR_MASK: u8 = 0b00000111;
const BLOCK_3_INSTR_POS: u8 = 0;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Invalid,
}
pub trait Parse {
    type Error;
    fn decode(self, bytes: &[u8]) -> Result<Instruction, Self::Error>;
}

pub struct Block0;

impl Parse for Block0 {
    type Error = ParseError;

    fn decode(self, _bytes: &[u8]) -> Result<Instruction, Self::Error> {
        todo!()
    }
}

pub struct Block1;

impl Parse for Block1 {
    type Error = ParseError;

    fn decode(self, _bytes: &[u8]) -> Result<Instruction, Self::Error> {
        todo!()
    }
}

pub struct Block2;

impl Parse for Block2 {
    type Error = ParseError;

    fn decode(self, bytes: &[u8]) -> Result<Instruction, Self::Error> {
        let [opcode, ..] = bytes else {
            return Err(ParseError::Invalid);
        };

        // The opcode without the bits encoding the block and the source register.
        let instr = (opcode & ARITH_INSTR_MASK) >> ARITH_INSTR_POS;

        let parsed = match instr {
            0 => AddInstr(Add::with_source(ArithSource::from_opcode(*opcode))),
            _ if instr > 7 => unreachable!(),
            _ => todo!(),
        };
        Ok(parsed)
    }
}

pub struct Block3;

impl Parse for Block3 {
    type Error = ParseError;

    fn decode(self, bytes: &[u8]) -> Result<Instruction, Self::Error> {
        let [opcode, immediate, ..] = bytes else {
            return Err(ParseError::Invalid);
        };

        // Arithmetic operations in block 3 ends with 110.
        let is_arithmetic = (opcode & BLOCK_3_INSTR_MASK) >> BLOCK_3_INSTR_POS == 0b110;

        let parsed = if is_arithmetic {
            // The opcode without the bits encoding the block and the arithmetic
            // instruction.
            let instr = (opcode & ARITH_INSTR_MASK) >> ARITH_INSTR_POS;

            match instr {
                0 => AddInstr(Add::with_source(ArithSource::from_literal(*immediate))),
                _ if instr > 7 => unreachable!(),
                _ => todo!(),
            }
        } else {
            todo!()
        };
        Ok(parsed)
    }
}

pub struct Prefixed;

impl Parse for Prefixed {
    type Error = ParseError;

    fn decode(self, _bytes: &[u8]) -> Result<Instruction, Self::Error> {
        todo!()
    }
}
