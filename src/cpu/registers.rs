use paste::paste;

/// Macro that builds the `Register` struct and implements `setter` and `getter`
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
        #[allow(dead_code)]
        pub struct Registers {
            /// Accumulator register, i.e. the A register.
            acc: u8,
            $(
                $high: u8,
                $low: u8,
            )*
        }

        #[allow(dead_code)]
        impl Registers {
            /// Construct as new `Registers` struct with all values zeroed.
            pub fn new() -> Self {
                Default::default()
            }

            /// Gets the `acc` register value. Commonly refered "register A".
            #[inline]
            pub fn get_acc(&self) -> u8 {
                self.acc
            }

            /// Sets the `acc` register value. Commonly refered as "register A".
            #[inline]
            pub fn set_acc(&mut self, val: u8) {
                self.acc = val
            }

            paste! {
                $(
                    #[doc = concat!("Gets the `", stringify!($high), "` register value.")]
                    #[inline]
                    pub fn [<get_ $high>](&self) -> u8 {
                        self.$high
                    }

                    #[doc = concat!("Sets the `", stringify!($high), "` register value.")]
                    #[inline]
                    pub fn [<set_ $high>](&mut self, val: u8) {
                        self.$high = val;
                    }

                    #[doc = concat!("Gets the `", stringify!($low), "` register value.")]
                    #[inline]
                    pub fn [<get_ $low>](&self) -> u8 {
                        self.$low
                    }

                    #[doc = concat!("Sets the `", stringify!($low), "` register value.")]
                    #[inline]
                    pub fn [<set_ $low>](&mut self, val: u8) {
                        self.$low = val;
                    }

                    #[doc = concat!("Sets the `", stringify!($high), stringify!($low), "` 16 bit register value.")]
                    #[inline]
                    pub fn [<get_ $high $low>](&self) -> u16 {
                        ((self.$high as u16) << 8) | self.$low as u16
                    }

                    #[doc = concat!("Sets the `", stringify!($high), stringify!($low), "` 16 bit register value.")]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getters() {
        let regs = Registers {
            acc: 0xF0,
            b: 0xF1,
            c: 0x00,
            d: 0xF2,
            e: 0x01,
            h: 0xF3,
            l: 0x02,
        };
        assert_eq!(regs.get_acc(), 0xF0);
        assert_eq!(regs.get_b(), 0xF1);
        assert_eq!(regs.get_c(), 0x00);
        assert_eq!(regs.get_bc(), 0xF100);
        assert_eq!(regs.get_d(), 0xF2);
        assert_eq!(regs.get_e(), 0x01);
        assert_eq!(regs.get_de(), 0xF201);
        assert_eq!(regs.get_h(), 0xF3);
        assert_eq!(regs.get_l(), 0x02);
        assert_eq!(regs.get_hl(), 0xF302);
    }

    #[test]
    fn test_setters() {
        let mut regs = Registers::new();
        regs.set_acc(0xFF);
        regs.set_bc(0xCAFE);
        regs.set_h(0xAB);
        assert_eq!(regs.acc, 0xFF);
        assert_eq!(regs.get_bc(), 0xCAFE);
        assert_eq!(regs.b, 0xCA);
        assert_eq!(regs.c, 0xFE);
        assert_eq!(regs.h, 0xAB);
    }
}
