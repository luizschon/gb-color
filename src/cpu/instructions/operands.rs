use crate::cpu::{
    CpuState,
    registers::{Reg8, RwRegister},
};

pub trait Source<T>: Sized {
    fn value(&self, state: &CpuState) -> T;
}

/// Source for 8-bit arithmetic operation, such as the
/// [block 2](https://gbdev.io/pandocs/CPU_Instruction_Set.html#block-2-8-bit-arithmetic)
/// and some [block 3](https://gbdev.io/pandocs/CPU_Instruction_Set.html#block-3)
/// instructions.
#[derive(Debug, PartialEq)]
pub enum ArithSource {
    /// An 8-bit register.
    Reg(Reg8),
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
            0 => Self::Reg(Reg8::B),
            1 => Self::Reg(Reg8::C),
            2 => Self::Reg(Reg8::D),
            3 => Self::Reg(Reg8::E),
            4 => Self::Reg(Reg8::H),
            5 => Self::Reg(Reg8::L),
            6 => Self::Addr,
            7 => Self::Reg(Reg8::Acc),
            _ => unreachable!(),
        }
    }

    pub fn from_literal(val: u8) -> Self {
        Self::Immediate(val)
    }
}

impl Source<u8> for ArithSource {
    fn value(&self, state: &CpuState) -> u8 {
        match *self {
            Self::Immediate(imm) => imm,
            Self::Reg(reg) => reg.read(&state.regs),
            Self::Addr => todo!(),
        }
    }
}
