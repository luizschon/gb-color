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
            /// Stack pointer.
            sp: u16,
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
            pub fn sp(&self) -> u16 {
                self.sp
            }
            #[inline]
            pub fn set_sp(&mut self, val: u16) {
                self.sp = val
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

pub trait RwRegister<T> {
    fn read(&self, regs: &RawRegisters) -> T;
    fn write(&self, regs: &mut RawRegisters, val: T);
}

#[rustfmt::skip]
/// Represents a named CPU register.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Register {
    Acc, B, C, D, E, H, L,
    BC, DE, HL, SP, PC
}

#[rustfmt::skip]
/// Represents a named CPU 8-bit register.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Reg8 {
    Acc, B, C, D, E, H, L,
}

impl RwRegister<u8> for Reg8 {
    fn read(&self, regs: &RawRegisters) -> u8 {
        match *self {
            Self::Acc => regs.acc(),
            Self::B => regs.b(),
            Self::C => regs.c(),
            Self::D => regs.d(),
            Self::E => regs.e(),
            Self::H => regs.h(),
            Self::L => regs.l(),
        }
    }

    fn write(&self, regs: &mut RawRegisters, val: u8) {
        match *self {
            Self::Acc => regs.set_acc(val),
            Self::B => regs.set_b(val),
            Self::C => regs.set_c(val),
            Self::D => regs.set_d(val),
            Self::E => regs.set_e(val),
            Self::H => regs.set_h(val),
            Self::L => regs.set_l(val),
        }
    }
}

#[rustfmt::skip]
/// Represents a named CPU 16-bit register.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Reg16 {
    BC, DE, HL, SP
}

impl RwRegister<u16> for Reg16 {
    fn read(&self, regs: &RawRegisters) -> u16 {
        match *self {
            Self::BC => regs.bc(),
            Self::DE => regs.de(),
            Self::HL => regs.hl(),
            Self::SP => regs.sp(),
        }
    }

    fn write(&self, regs: &mut RawRegisters, val: u16) {
        match *self {
            Self::BC => regs.set_bc(val),
            Self::DE => regs.set_de(val),
            Self::HL => regs.set_hl(val),
            Self::SP => regs.set_sp(val),
        }
    }
}

/// Representation of the flags register of the GameBoy's CPU.
///
/// | Bit 7 | Bit 6       | Bit 5      | Bit 4   | Bits 3-0      |
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

            /// Reads byte value stored in the flags register.
            pub fn value(&mut self) -> u8 {
                self.0
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
            sp: 0xFF00, acc: 0xF0, b: 0xF1, c: 0x00, d: 0xF2,
            e: 0x01, h: 0xF3, l: 0x02,
        };
        assert_eq!(regs.sp(), 0xFF00);
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
    fn test_rw() {
        let mut regs = RawRegisters::new();
        Reg8::Acc.write(&mut regs, 0xFF);
        Reg16::BC.write(&mut regs, 0xCAFE);
        Reg8::H.write(&mut regs, 0xAB);
        assert!(regs.acc == 0xFF && regs.acc == Reg8::Acc.read(&regs));
        assert!(regs.b == 0xCA && regs.b == Reg8::B.read(&regs));
        assert!(regs.c == 0xFE && regs.c == Reg8::C.read(&regs));
        assert!(regs.bc() == 0xCAFE && regs.bc() == Reg16::BC.read(&regs));
        assert!(regs.h == 0xAB && regs.h == Reg8::H.read(&regs));
    }

    #[test]
    fn test_set_get_flags() {
        let mut f = Flags::new();
        f.set_zero(true);
        assert_eq!(f.value(), 0b10000000);
        f.set_carry(true);
        assert_eq!(f.value(), 0b10010000);
        f.set(0xFF);
        assert_eq!(f.value(), 0b11110000);
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
