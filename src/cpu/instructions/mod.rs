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

use execute::*;

use super::CpuState;

mod execute;
mod operands;
mod parsers;

pub trait Executable {
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

#[cfg(test)]
mod tests {
    use super::parsers::{Decode, InstructionDecoder};
    use super::*;
    use Instruction::*;

    #[test]
    fn test_add_parse() {
        use crate::cpu::registers::Reg8::*;
        use operands::ArithSource::*;

        // add a, r8
        let i = InstructionDecoder::from(0x80).decode(&[0x80]);
        assert_eq!(i, Ok(AddInstr(Add(Reg(B)))));
        let i = InstructionDecoder::from(0x87).decode(&[0x87]);
        assert_eq!(i, Ok(AddInstr(Add(Reg(Acc)))));
        let i = InstructionDecoder::from(0x86).decode(&[0x86]);
        assert_eq!(i, Ok(AddInstr(Add(Addr))));

        // add a, imm8
        let i = InstructionDecoder::from(0xC6).decode(&[0xC6, 0xFF]);
        assert_eq!(i, Ok(AddInstr(Add(Immediate(0xFF)))));
        let i = InstructionDecoder::from(0xC6).decode(&[0xC6, 0xAB]);
        assert_eq!(i, Ok(AddInstr(Add(Immediate(0xAB)))));
    }
}
