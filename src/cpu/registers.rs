use paste::paste;

/// Macro that builds the [RawRegisters] struct and implements `setter` and `getter`
/// methods for it's fields.
macro_rules! make_registers_struct {
    ($($high:ident $low:ident),*) => {
        /// A representation of the readable and writable CPU registers of the
        /// GameBoy, commonly denominaded as the A, B, C, D, E, H and L registers.
        ///
        /// The `A` register is an 8 bit accumulator. The remaining registers can
        /// be thought of as the higher/lower parts of a 16 bit word, forming the
        /// following registers:
        ///
        /// | 16 bit register | Higher byte | Lower byte |
        /// |-----------------|-------------|------------|
        /// | `BC`            | `B`         | `C`        |
        /// | `DE`            | `D`         | `E`        |
        /// | `HL`            | `H`         | `L`        |
        #[derive(Debug, Default)]
        pub struct RawRegisters {
            /// Accumulator register, i.e. the `A` register.
            acc: u8,
            $(
                $high: u8,
                $low: u8,
            )*
        }

        impl RawRegisters {
            /// Construct as new [RawRegisters] struct with all values zeroed.
            pub fn new() -> Self {
                Default::default()
            }

            #[inline]
            pub fn acc(&self) -> u8 {
                self.acc
            }

            #[inline]
            pub fn set_acc(&mut self, val: u8) {
                self.acc = val
            }

            paste! {
                $(
                    #[inline]
                    pub fn $high(&self) -> u8 {
                        self.$high
                    }

                    #[inline]
                    pub fn [<set_ $high>](&mut self, val: u8) {
                        self.$high = val;
                    }

                    #[inline]
                    pub fn $low(&self) -> u8 {
                        self.$low
                    }

                    #[inline]
                    pub fn [<set_ $low>](&mut self, val: u8) {
                        self.$low = val;
                    }

                    #[inline]
                    pub fn [<$high $low>](&self) -> u16 {
                        ((self.$high as u16) << 8) | self.$low as u16
                    }

                    #[inline]
                    pub fn [<set_ $high $low>](&mut self, val: u16) {
                        let (low, high) = ((val & 0xFF) as u8, (val >> 8) as u8);
                        self.$low = low;
                        self.$high = high;
                    }
                )*
            }
        }
    };
}

make_registers_struct!(b c, d e, h l);

#[rustfmt::skip]
/// Represents a named CPU register.
#[derive(Debug, Clone, Copy)]
pub enum Register {
    Acc, B, C, D, E, H, L,
    BC, DE, HL
}

#[rustfmt::skip]
/// Represents a named CPU 8-bit register.
#[derive(Debug, Clone, Copy)]
pub enum Register8 {
    Acc, B, C, D, E, H, L,
}

#[rustfmt::skip]
/// Represents a named CPU 16-bit register.
#[derive(Debug, Clone, Copy)]
pub enum Register16 {
    BC, DE, HL
}

/// Representation of the flags register of the GameBoy's CPU.
///
/// | Bit 7 | Bit 6       | Bit 5      | Bit 5   | Bits 4-0      |
/// |-------|-------------|------------|---------|---------------|
/// | Zero  | Subtraction | Half-carry | Carry   | Unused (zero) |
///
/// - `Zero` flag: this flag is set if the result of an operation is 0.
/// - `Subtraction` flag: ***this flag is only used by the DAA operation***. Indicates
/// if the last operation was a substraction.
/// - `Half-carry` flag: ***this flag is only used by the DAA operation***. Indicates
/// a carry in the lower 4 bits.
/// - `Carry` flag: this flag is set if the result of the previous operation
/// over(under)flowed or if a previous rotate/shift shifted a `1` out. ***Used by
/// conditional jumps and some other instructions***.
#[derive(Debug, Default)]
pub struct Flags(u8);

/// Macro that implements `setter`, `getter` and `clear` methods for the flags
/// in the [Flags] struct.
macro_rules! impl_flags_struct {
    ($($flag:ident: $pos:literal),*) => {
        #[allow(dead_code)]
        impl Flags {
            /// Constructs a new [Flags] struct, with zeroed values
            pub fn new() -> Self {
                Default::default()
            }

            /// Sets the flags register to a [u8] value. It's required that the
            /// lower 4 bits of the flags register are always zeroed, so these
            /// bits are ignored.
            #[inline]
            pub fn set(&mut self, val: u8) {
                // The first 4 bits should always be zeroed.
                self.0 = val & 0xF0;
            }

            #[inline]
            pub fn clear(&mut self) {
                self.0 = 0x00;
            }

            paste! {
                $(
                    #[inline]
                    pub fn $flag(&self) -> bool {
                        (self.0 >> $pos) & 0b1 == 0b1
                    }

                    #[inline]
                    pub fn [<set_ $flag>](&mut self, val: bool) {
                        self.0 = self.0 & !(1 << $pos) | ((val as u8) << $pos)
                    }

                    #[inline]
                    pub fn [<clear_ $flag>](&mut self) {
                        self.0 = self.0 & !(1 << $pos)
                    }
                )*
            }
        }
    };
}

impl_flags_struct!(zero: 7, subtract: 6, half_carry: 5, carry: 4);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getters() {
        #[rustfmt::skip]
        let regs = RawRegisters {
            acc: 0xF0, b: 0xF1, c: 0x00, d: 0xF2,
            e: 0x01, h: 0xF3, l: 0x02,
        };
        assert_eq!(regs.acc(), 0xF0);
        assert_eq!(regs.b(), 0xF1);
        assert_eq!(regs.c(), 0x00);
        assert_eq!(regs.bc(), 0xF100);
        assert_eq!(regs.d(), 0xF2);
        assert_eq!(regs.e(), 0x01);
        assert_eq!(regs.de(), 0xF201);
        assert_eq!(regs.h(), 0xF3);
        assert_eq!(regs.l(), 0x02);
        assert_eq!(regs.hl(), 0xF302);
    }

    #[test]
    fn test_setters() {
        let mut regs = RawRegisters::new();
        regs.set_acc(0xFF);
        regs.set_bc(0xCAFE);
        regs.set_h(0xAB);
        assert_eq!(regs.acc, 0xFF);
        assert_eq!(regs.b, 0xCA);
        assert_eq!(regs.c, 0xFE);
        assert_eq!(regs.h, 0xAB);
    }

    #[test]
    fn test_set_get_flags() {
        let mut f = Flags::new();
        f.set_zero(true);
        assert_eq!(f.0, 0b10000000);
        f.set_carry(true);
        assert_eq!(f.0, 0b10010000);
        f.set(0xFF);
        assert_eq!(f.0, 0b11110000);
        assert!(
            [f.zero(), f.subtract(), f.half_carry(), f.carry()]
                .iter()
                .all(|&x| x)
        );
    }

    #[test]
    fn test_clear_flags() {
        let mut f = Flags::new();
        f.set(0xF0);
        f.clear_zero();
        assert!(!f.zero());
        f.clear_subtract();
        assert!(!f.subtract());
        f.clear_half_carry();
        assert!(!f.half_carry());
        f.clear_carry();
        assert!(!f.carry());
        f.set(0xF0);
        f.clear();
        assert_eq!(f.0, 0x00);
    }
}
