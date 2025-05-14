use crate::cpu::CpuState;

use super::{
    Executable,
    operands::{ArithSource, Operand},
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
        let operand = self.0.get_value(state);
    }
}
