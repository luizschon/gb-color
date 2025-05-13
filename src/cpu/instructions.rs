//! The CPU instructions for the GameBoy can be organized in
//! (blocks)[https://gbdev.io/pandocs/CPU_Instruction_Set.html], where the
//! block number is encoded in the 2 MSB's of the opcode:
//!
//! | Block | Opcode bit 7 | Opcode bit 6 | Opcode bits 5-0 |
//! |-------|--------------|--------------|-----------------|
//! | 0     | 0            | 0            | X X X X X X     |
//! | 1     | 0            | 1            | X X X X X X     |
//! | 2     | 1            | 0            | X X X X X X     |
//! | 3     | 1            | 1            | X X X X X X     |

use super::registers::Register8;

/// Source for 8-bit arithmetic operation, such as the
/// (block 2)[https://gbdev.io/pandocs/CPU_Instruction_Set.html#block-2-8-bit-arithmetic]
/// and some (block 3)[https://gbdev.io/pandocs/CPU_Instruction_Set.html#block-3]
/// instructions.
#[derive(Debug, PartialEq)]
pub enum ArithSource {
    /// An 8-bit register.
    Reg(Register8),
    /// An 16-bit address into the GameBoy's memory, read from the HL register.
    Addr,
    /// A 8-bit literal.
    Immediate(u8),
}

impl ArithSource {
    pub fn from_opcode(opcode: u8) -> Self {
        // The three last bits of the opcode
        let reg_idx = opcode & 0b00000111;

        match reg_idx {
            0 => Self::Reg(Register8::B),
            1 => Self::Reg(Register8::C),
            2 => Self::Reg(Register8::D),
            3 => Self::Reg(Register8::E),
            4 => Self::Reg(Register8::H),
            5 => Self::Reg(Register8::L),
            6 => Self::Addr,
            7 => Self::Reg(Register8::Acc),
            _ => unreachable!(),
        }
    }

    pub fn from_literal(val: u8) -> Self {
        Self::Immediate(val)
    }
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Add(ArithSource),
}

#[derive(Debug, PartialEq)]
pub enum InstructionError {
    Invalid,
}

impl Instruction {
    pub fn from_bytes(mem_slice: &[u8]) -> Result<Self, InstructionError> {
        let [first, rest @ ..] = mem_slice else {
            return Err(InstructionError::Invalid);
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
            3 => parse_block_3_instr(opcode, rest.first().copied()),
            _ => unreachable!(),
        }
    }
}

fn parse_block_2_instr(opcode: u8) -> Result<Instruction, InstructionError> {
    // The opcode without the bits representing the block and the
    // source register.
    let instr = (opcode & 0b00111000) >> 3;

    let parsed = match instr {
        0 => Instruction::Add(ArithSource::from_opcode(opcode)),
        _ if instr > 7 => unreachable!(),
        _ => todo!(),
    };
    Ok(parsed)
}

fn parse_block_3_instr(opcode: u8, next: Option<u8>) -> Result<Instruction, InstructionError> {
    let is_arithmetic = opcode & 0b00000111 == 0b110;

    let parsed = if is_arithmetic {
        // The opcode without the bits representing the block and the
        // arithmetic instruction bits
        let instr = (opcode & 0b00111000) >> 3;
        let immediate = next.ok_or(InstructionError::Invalid)?;

        match instr {
            0 => Instruction::Add(ArithSource::from_literal(immediate)),
            _ if instr > 7 => unreachable!(),
            _ => todo!(),
        }
    } else {
        todo!()
    };
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_2_instr_builder() {
        use Instruction::*;
        use Register8::*;

        // ADD A, r8
        assert_eq!(
            Instruction::from_bytes(&[0x80]),
            Ok(Add(ArithSource::Reg(B)))
        );
        assert_eq!(
            Instruction::from_bytes(&[0x81]),
            Ok(Add(ArithSource::Reg(C)))
        );
        assert_eq!(Instruction::from_bytes(&[0x86]), Ok(Add(ArithSource::Addr)));
        assert_eq!(
            Instruction::from_bytes(&[0x87]),
            Ok(Add(ArithSource::Reg(Acc)))
        );
    }

    #[test]
    fn test_block_3_instr() {
        use Instruction::*;

        // ADD A, imm8
        assert_eq!(
            Instruction::from_bytes(&[0xC6, 0xFF]),
            Ok(Add(ArithSource::Immediate(0xFF)))
        );
        assert_eq!(
            Instruction::from_bytes(&[0xC6, 0xAB]),
            Ok(Add(ArithSource::Immediate(0xAB)))
        );
    }
}
