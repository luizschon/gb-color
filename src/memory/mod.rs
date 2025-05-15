use paste::paste;

macro_rules! declare_memory_sections {
    ($($NAME:ident: $stt:literal to $end:literal),*) => {
        paste! {
            $(
                const [<$NAME _START>]: u16 = $stt;
                const [<$NAME _END>]: u16 = $end;
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

#[derive(Debug)]
struct MemoryController;
