use registers::{Flags, RawRegisters};

mod instructions;
mod registers;

#[derive(Debug)]
pub struct CpuState {
    // TODO add memory field
    pc: usize,
    sp: usize,
    flags: Flags,
    regs: RawRegisters,
}

impl CpuState {
    // fn fetch_instruction(&self) -> Instruction {
    //     Instruction::from_bytes()
    // }

    pub fn step(&mut self) {
        // let instr = self.fetch_instruction();
        // instr.execute(self);
    }
}
