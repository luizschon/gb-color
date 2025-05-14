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
use operands::ArithSource;

use super::CpuState;

mod instructions_fn;
mod operands;

pub trait Executable: Sized {
    fn execute(&self, state: &mut CpuState);
}

#[rustfmt::skip]
#[derive(Debug, PartialEq)]
pub enum Instruction {
    AddInstr(Add),
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Invalid,
}

impl Instruction {
    fn from_bytes(rom_slice: &[u8]) -> Result<Self, ParseError> {
        let [first, rest @ ..] = rom_slice else {
            return Err(ParseError::Invalid);
        };

        let block = dbg!((*first & 0xC0) >> 6);
        let is_prefixed = *first == 0xCB;
        let opcode = if is_prefixed { todo!() } else { *first };

        // Opcodes can be organized in blocks 00-03, as in
        // https://gbdev.io/pandocs/CPU_Instruction_Set.html
        match block {
            // Block 0
            0 => todo!(),
            // Block 1: register to register load + halt
            1 => todo!(),
            // Block 2: 8-bit arithmetic instructions with registers.
            2 => parse_block_2_instr(opcode),
            // Block 2: 8-bit immediate-mode arithmetic, jumps, stack-pointer
            // manipulation, etc.
            3 => parse_block_3_instr(opcode, rest.first().copied()),
            _ => unreachable!(),
        }
    }
}

fn parse_block_2_instr(opcode: u8) -> Result<Instruction, ParseError> {
    use Instruction::*;
    // The opcode without the bits encoding the block and the source register.
    let instr = (opcode & 0b00111000) >> 3;

    let parsed = match instr {
        0 => AddInstr(Add::with_source(ArithSource::from_opcode(opcode))),
        _ if instr > 7 => unreachable!(),
        _ => todo!(),
    };
    Ok(parsed)
}

fn parse_block_3_instr(opcode: u8, next: Option<u8>) -> Result<Instruction, ParseError> {
    use Instruction::*;
    let is_arithmetic = opcode & 0b00000111 == 0b110;

    let parsed = if is_arithmetic {
        // The opcode without the bits encoding the block and the arithmetic
        // instruction.
        let instr = (opcode & 0b00111000) >> 3;
        let immediate = next.ok_or(ParseError::Invalid)?;

        match instr {
            0 => AddInstr(Add::with_source(ArithSource::from_literal(immediate))),
            _ if instr > 7 => unreachable!(),
            _ => todo!(),
        }
    } else {
        todo!()
    };
    Ok(parsed)
}

impl Executable for Instruction {
    fn execute(&self, state: &mut CpuState) {
        use Instruction::*;

        match self {
            AddInstr(i) => i.execute(state),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::registers::Register8::*;
    use Instruction::*;
    use operands::*;

    #[test]
    fn test_block_2_instr_builder() {
        // ADD A, r8
        assert_eq!(
            Instruction::from_bytes(&[0x80]),
            Ok(AddInstr(Add(ArithSource::Reg(B))))
        );
        assert_eq!(
            Instruction::from_bytes(&[0x81]),
            Ok(AddInstr(Add(ArithSource::Reg(C))))
        );
        assert_eq!(
            Instruction::from_bytes(&[0x86]),
            Ok(AddInstr(Add(ArithSource::Addr)))
        );
        assert_eq!(
            Instruction::from_bytes(&[0x87]),
            Ok(AddInstr(Add(ArithSource::Reg(Acc))))
        );
    }

    #[test]
    fn test_block_3_instr() {
        // ADD A, imm8
        assert_eq!(
            Instruction::from_bytes(&[0xC6, 0xFF]),
            Ok(AddInstr(Add(ArithSource::Immediate(0xFF))))
        );
        assert_eq!(
            Instruction::from_bytes(&[0xC6, 0xAB]),
            Ok(AddInstr(Add(ArithSource::Immediate(0xAB))))
        );
    }
}
