use paste::paste;

mod rom;

macro_rules! declare_memory_sections {
    ($($NAME:ident: $stt:literal to $end:literal),*) => {
        paste! {
            $(
                const [<$NAME _START>]: u16 = $stt;
                const [<$NAME _END>]: u16 = $end;
                const [<$NAME _SIZE>]: usize = $end - $stt + 1;
            )*
        }
    };
}

#[rustfmt::skip]
declare_memory_sections!(
    ROM_BANK_0:  0x0000 to 0x3FFF, ROM_BANK_N:  0x4000 to 0x7FFF,
    VRAM:        0x8000 to 0x9FFF, EXT_RAM:     0xA000 to 0xBFFF,
    WRAM_BANK_0: 0xC000 to 0xCFFF, WRAM_BANK_N: 0xD000 to 0xDFFF,
    ECHO_RAM:    0xE000 to 0xFDFF, OAM:         0xFE00 to 0xFE9F,
    _RESERVED:   0xFEA0 to 0xFEFF, IO_REGS:     0xFF00 to 0xFF7F,
    HRAM:        0xFF80 to 0xFFFE, INT_EN:      0xFFFF to 0xFFFF
);

pub enum MemoryError {
    ReadOnly,
    Prohibited,
}

#[derive(Debug)]
pub struct MemoryController {}

impl MemoryController {
    pub fn read(&self, addr: u16) -> Result<u8, MemoryError> {
        let data = match addr {
            ROM_BANK_0_START..=ROM_BANK_0_END => 0x00,
            ROM_BANK_N_START..=ROM_BANK_N_END => 0x00,
            VRAM_START..=VRAM_END => 0x00,
            EXT_RAM_START..=EXT_RAM_END => 0x00,
            WRAM_BANK_0_START..=WRAM_BANK_0_END => 0x00,
            WRAM_BANK_N_START..=WRAM_BANK_N_END => 0x00,
            ECHO_RAM_START..=ECHO_RAM_END => 0x00,
            OAM_START..=OAM_END => 0x00,
            _RESERVED_START..=_RESERVED_END => Err(MemoryError::Prohibited)?,
            IO_REGS_START..=IO_REGS_END => 0x00,
            HRAM_START..=HRAM_END => 0x00,
            INT_EN_START..=INT_EN_END => 0x00,
        };
        Ok(data)
    }

    pub fn write(&self, addr: u16, _data: u8) -> Result<(), MemoryError> {
        match addr {
            ROM_BANK_0_START..=ROM_BANK_0_END => 0x00,
            ROM_BANK_N_START..=ROM_BANK_N_END => 0x00,
            VRAM_START..=VRAM_END => 0x00,
            EXT_RAM_START..=EXT_RAM_END => 0x00,
            WRAM_BANK_0_START..=WRAM_BANK_0_END => 0x00,
            WRAM_BANK_N_START..=WRAM_BANK_N_END => 0x00,
            ECHO_RAM_START..=ECHO_RAM_END => 0x00,
            OAM_START..=OAM_END => 0x00,
            _RESERVED_START..=_RESERVED_END => Err(MemoryError::Prohibited)?,
            IO_REGS_START..=IO_REGS_END => 0x00,
            HRAM_START..=HRAM_END => 0x00,
            INT_EN_START..=INT_EN_END => 0x00,
        };
        Ok(())
    }
}
