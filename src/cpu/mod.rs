use registers::{Flags, RawRegisters};

mod instructions;
mod registers;

#[derive(Debug, Default)]
pub struct CpuState {
    // TODO add memory field
    pc: u16,
    flags: Flags,
    regs: RawRegisters,
}

impl CpuState {
    pub fn new() -> Self {
        Default::default()
    }
    // fn fetch_instruction(&self) -> Instruction {
    //     Instruction::from_bytes()
    // }

    pub fn step(&mut self) {
        // let instr = self.fetch_instruction();
        // instr.execute(self);
    }
}
