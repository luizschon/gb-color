//! The CPU instructions for the GameBoy can be organized in
//! [blocks](https://gbdev.io/pandocs/CPU_Instruction_Set.html), where the
//! block number is encoded in the 2 MSB's of the opcode:
//!
//! | Block | Opcode bit 7 | Opcode bit 6 | Opcode bits 5-0 |
//! |-------|--------------|--------------|-----------------|
//! | 0     | 0            | 0            | X X X X X X     |
//! | 1     | 0            | 1            | X X X X X X     |
//! | 2     | 1            | 0            | X X X X X X     |
//! | 3     | 1            | 1            | X X X X X X     |

use instructions_fn::*;
use parsers::Parse;

use super::CpuState;

mod instructions_fn;
mod operands;
mod parsers;

type Opcode = u8;

pub trait Executable: Sized {
    fn execute(&self, state: &mut CpuState);
}

#[rustfmt::skip]
#[derive(Debug, PartialEq)]
pub enum Instruction {
    AddInstr(Add),
}

impl Executable for Instruction {
    fn execute(&self, state: &mut CpuState) {
        use Instruction::*;

        state.flags.clear();
        match self {
            AddInstr(i) => i.execute(state),
        }
    }
}

pub enum InstructionParser {
    Block0Parser(parsers::Block0),
    Block1Parser(parsers::Block1),
    Block2Parser(parsers::Block2),
    Block3Parser(parsers::Block3),
    PrefixedParser(parsers::Prefixed),
}

impl From<Opcode> for InstructionParser {
    fn from(opcode: Opcode) -> Self {
        let block = (opcode & 0xC0) >> 6;
        let is_prefixed = opcode == 0xCB;

        if is_prefixed {
            return Self::PrefixedParser(parsers::Prefixed);
        }

        match block {
            0 => Self::Block0Parser(parsers::Block0),
            1 => Self::Block1Parser(parsers::Block1),
            2 => Self::Block2Parser(parsers::Block2),
            3 => Self::Block3Parser(parsers::Block3),
            _ => unreachable!(),
        }
    }
}

impl Parse for InstructionParser {
    type Error = parsers::ParseError;

    fn decode(self, rom_slice: &[u8]) -> Result<Instruction, Self::Error> {
        match self {
            Self::Block0Parser(p) => p.decode(rom_slice),
            Self::Block1Parser(p) => p.decode(rom_slice),
            Self::Block2Parser(p) => p.decode(rom_slice),
            Self::Block3Parser(p) => p.decode(rom_slice),
            Self::PrefixedParser(p) => p.decode(rom_slice),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use Instruction::*;

    #[test]
    fn test_add_parse() {
        use crate::cpu::registers::Register8::*;
        use operands::ArithSource::*;

        // add a, r8
        let i = InstructionParser::from(0x80).decode(&[0x80]);
        assert_eq!(i, Ok(AddInstr(Add(Reg(B)))));
        let i = InstructionParser::from(0x87).decode(&[0x87]);
        assert_eq!(i, Ok(AddInstr(Add(Reg(Acc)))));
        let i = InstructionParser::from(0x86).decode(&[0x86]);
        assert_eq!(i, Ok(AddInstr(Add(Addr))));

        // add a, imm8
        let i = InstructionParser::from(0xC6).decode(&[0xC6, 0xFF]);
        assert_eq!(i, Ok(AddInstr(Add(Immediate(0xFF)))));
        let i = InstructionParser::from(0xC6).decode(&[0xC6, 0xAB]);
        assert_eq!(i, Ok(AddInstr(Add(Immediate(0xAB)))));
    }
}
