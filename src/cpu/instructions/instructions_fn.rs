use crate::cpu::CpuState;

use super::{Instruction, operands::Operand};

type InstructionFn = fn(&Instruction, &mut CpuState) -> ();

pub trait Executable {
    fn execute(&self, state: &mut CpuState) {}
}

fn add(instr: &Instruction, state: &mut CpuState) {
    let Instruction::Add(src) = instr else {
        panic!("Instruction {instr:?} shouldn't be in `add`!")
    };
    let operand = src.get_value(state);
}
