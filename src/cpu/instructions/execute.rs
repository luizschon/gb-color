use crate::cpu::CpuState;

use super::{
    Executable,
    operands::{ArithSource, Source},
};

#[derive(Debug, PartialEq)]
pub struct Add(pub ArithSource);

impl Add {
    pub fn with_source(src: ArithSource) -> Self {
        Self(src)
    }
}

impl Executable for Add {
    fn execute(&self, state: &mut CpuState) {
        let operand = self.0.value(state);
        let acc = state.regs.acc();
        let (result, did_overflow) = acc.overflowing_add(operand);

        state.flags.set_zero(result == 0);
        state.flags.set_carry(did_overflow);
        state.flags.clear_subtract();
        // If a half carry occured, the resulting nibble will be less than the
        // operand nibble.
        state.flags.set_half_carry(result & 0x0F < operand & 0x0F);

        // Increment program counter
        state.pc += match self.0 {
            ArithSource::Reg(_) | ArithSource::Addr => 1,
            ArithSource::Immediate(_) => 2,
        };
        // Update Acc
        state.regs.set_acc(result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::registers::Reg8::*;
    use ArithSource::*;

    #[test]
    fn test_add() {
        let mut state = CpuState::new();
        state.regs.set_b(0xE1);
        state.regs.set_e(0x0F);
        state.regs.set_hl(0x0000);

        Add(Reg(B)).execute(&mut state);
        assert_eq!(state.regs.acc(), 0xE1);

        Add(Reg(E)).execute(&mut state);
        assert_eq!(state.regs.acc(), 0xF0);
        assert!(state.flags.half_carry() == true);

        Add(Immediate(0x0F)).execute(&mut state);
        assert_eq!(state.regs.acc(), 0xFF);
        assert!(state.flags.half_carry() == false);

        Add(Immediate(0x01)).execute(&mut state);
        assert_eq!(state.regs.acc(), 0x00);
        assert!(state.flags.zero() == true);
        assert!(state.flags.carry() == true);
        assert!(state.flags.half_carry() == true);

        Add(Addr).execute(&mut state);
        assert_eq!(state.regs.acc(), 0x00);
        assert!(state.flags.zero() == true);
    }
}
