use super::{
    Instruction::{self, *},
    execute::*,
    operands::ArithSource,
};

const ARITH_INSTR_MASK: u8 = 0b00111000;
const ARITH_INSTR_POS: u8 = 3;
const BLOCK_3_INSTR_MASK: u8 = 0b00000111;
const BLOCK_3_INSTR_POS: u8 = 0;

#[derive(Debug, PartialEq)]
pub enum DecodeError {
    Invalid,
}

pub trait Decode {
    type Error;
    fn decode(self, bytes: &[u8]) -> Result<Instruction, Self::Error>;
}

pub struct Block0;

impl Decode for Block0 {
    type Error = DecodeError;

    fn decode(self, _bytes: &[u8]) -> Result<Instruction, Self::Error> {
        todo!()
    }
}

pub struct Block1;

impl Decode for Block1 {
    type Error = DecodeError;

    fn decode(self, _bytes: &[u8]) -> Result<Instruction, Self::Error> {
        todo!()
    }
}

pub struct Block2;

impl Decode for Block2 {
    type Error = DecodeError;

    fn decode(self, bytes: &[u8]) -> Result<Instruction, Self::Error> {
        let [opcode, ..] = bytes else {
            return Err(DecodeError::Invalid);
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

impl Decode for Block3 {
    type Error = DecodeError;

    fn decode(self, bytes: &[u8]) -> Result<Instruction, Self::Error> {
        let [opcode, immediate, ..] = bytes else {
            return Err(DecodeError::Invalid);
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

impl Decode for Prefixed {
    type Error = DecodeError;

    fn decode(self, _bytes: &[u8]) -> Result<Instruction, Self::Error> {
        todo!()
    }
}

pub enum InstructionDecoder {
    Block0Decoder(Block0),
    Block1Decoder(Block1),
    Block2Decoder(Block2),
    Block3Decoder(Block3),
    PrefixedDecoder(Prefixed),
}

type Opcode = u8;

impl From<Opcode> for InstructionDecoder {
    fn from(opcode: Opcode) -> Self {
        let block = (opcode & 0xC0) >> 6;
        let is_prefixed = opcode == 0xCB;

        if is_prefixed {
            return Self::PrefixedDecoder(Prefixed);
        }

        match block {
            0 => Self::Block0Decoder(Block0),
            1 => Self::Block1Decoder(Block1),
            2 => Self::Block2Decoder(Block2),
            3 => Self::Block3Decoder(Block3),
            _ => unreachable!(),
        }
    }
}

impl Decode for InstructionDecoder {
    type Error = DecodeError;

    fn decode(self, rom_slice: &[u8]) -> Result<Instruction, Self::Error> {
        match self {
            Self::Block0Decoder(p) => p.decode(rom_slice),
            Self::Block1Decoder(p) => p.decode(rom_slice),
            Self::Block2Decoder(p) => p.decode(rom_slice),
            Self::Block3Decoder(p) => p.decode(rom_slice),
            Self::PrefixedDecoder(p) => p.decode(rom_slice),
        }
    }
}
