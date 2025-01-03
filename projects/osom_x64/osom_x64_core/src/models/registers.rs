use super::MachineSize;

macro_rules! reg {
    ( $name: ident, $size: ident, $index: literal ) => {
        pub const $name: Self =
            unsafe { Self::new_unchecked(MachineSize::$size, $index) };
    };
}

macro_rules! reg_class {
    ( $name: ident, $doc: literal ) => {
        #[doc = $doc]
        #[repr(transparent)]
        #[derive(PartialEq, Eq, Hash, Clone, Copy)]
        pub struct $name {
            val: u8,
        }

        impl $name {
            const _CHECK: () = {
                assert!(size_of::<Self>() == 1);
            };

            #[must_use]
            #[inline(always)]
            const unsafe fn new_unchecked(size: MachineSize, index: u8) -> Self {
                Self {
                    val: size.as_u8() | (index << 3),
                }
            }

            #[must_use]
            #[inline(always)]
            pub const fn size(&self) -> MachineSize {
                unsafe { MachineSize::from_u8(self.val & 7) }
            }

            #[must_use]
            #[inline(always)]
            pub const fn index(&self) -> u8 {
                self.val >> 3
            }

            #[must_use]
            #[inline(always)]
            pub const fn lower_index(&self) -> u8 {
                self.index() & 7
            }
        }
    };
}

reg_class!(GPR, "Represents general purpose registers.");

impl GPR {
    // Special.
    reg!(NO_REG, None, 16);
    reg!(RIP, None, 0);

    // Byte registers.
    reg!(AL, Byte, 0);
    reg!(CL, Byte, 1);
    reg!(DL, Byte, 2);
    reg!(BL, Byte, 3);
    reg!(SPL, Byte, 4);
    reg!(BPL, Byte, 5);
    reg!(SIL, Byte, 6);
    reg!(DIL, Byte, 7);
    reg!(R8B, Byte, 8);
    reg!(R9B, Byte, 9);
    reg!(R10B, Byte, 10);
    reg!(R11B, Byte, 11);
    reg!(R12B, Byte, 12);
    reg!(R13B, Byte, 13);
    reg!(R14B, Byte, 14);
    reg!(R15B, Byte, 15);

    // Word registers.
    reg!(AX, Word, 0);
    reg!(CX, Word, 1);
    reg!(DX, Word, 2);
    reg!(BX, Word, 3);
    reg!(SP, Word, 4);
    reg!(BP, Word, 5);
    reg!(SI, Word, 6);
    reg!(DI, Word, 7);
    reg!(R8W, Word, 8);
    reg!(R9W, Word, 9);
    reg!(R10W, Word, 10);
    reg!(R11W, Word, 11);
    reg!(R12W, Word, 12);
    reg!(R13W, Word, 13);
    reg!(R14W, Word, 14);
    reg!(R15W, Word, 15);

    // DWord registers.
    reg!(EAX, DWord, 0);
    reg!(ECX, DWord, 1);
    reg!(EDX, DWord, 2);
    reg!(EBX, DWord, 3);
    reg!(ESP, DWord, 4);
    reg!(EBP, DWord, 5);
    reg!(ESI, DWord, 6);
    reg!(EDI, DWord, 7);
    reg!(R8D, DWord, 8);
    reg!(R9D, DWord, 9);
    reg!(R10D, DWord, 10);
    reg!(R11D, DWord, 11);
    reg!(R12D, DWord, 12);
    reg!(R13D, DWord, 13);
    reg!(R14D, DWord, 14);
    reg!(R15D, DWord, 15);

    // QWord registers.
    reg!(RAX, QWord, 0);
    reg!(RCX, QWord, 1);
    reg!(RDX, QWord, 2);
    reg!(RBX, QWord, 3);
    reg!(RSP, QWord, 4);
    reg!(RBP, QWord, 5);
    reg!(RSI, QWord, 6);
    reg!(RDI, QWord, 7);
    reg!(R8, QWord, 8);
    reg!(R9, QWord, 9);
    reg!(R10, QWord, 10);
    reg!(R11, QWord, 11);
    reg!(R12, QWord, 12);
    reg!(R13, QWord, 13);
    reg!(R14, QWord, 14);
    reg!(R15, QWord, 15);
}

reg_class!(XMM, "Represents XMM registers.");

impl XMM {
    reg!(XMM0, XMMWord, 0);
    reg!(XMM1, XMMWord, 1);
    reg!(XMM2, XMMWord, 2);
    reg!(XMM3, XMMWord, 3);
    reg!(XMM4, XMMWord, 4);
    reg!(XMM5, XMMWord, 5);
    reg!(XMM6, XMMWord, 6);
    reg!(XMM7, XMMWord, 7);
    reg!(XMM8, XMMWord, 8);
    reg!(XMM9, XMMWord, 9);
    reg!(XMM10, XMMWord, 10);
    reg!(XMM11, XMMWord, 11);
    reg!(XMM12, XMMWord, 12);
    reg!(XMM13, XMMWord, 13);
    reg!(XMM14, XMMWord, 14);
    reg!(XMM15, XMMWord, 15);
}

reg_class!(YMM, "Represents YMM registers.");

impl YMM {
    reg!(YMM0, YMMWord, 0);
    reg!(YMM1, YMMWord, 1);
    reg!(YMM2, YMMWord, 2);
    reg!(YMM3, YMMWord, 3);
    reg!(YMM4, YMMWord, 4);
    reg!(YMM5, YMMWord, 5);
    reg!(YMM6, YMMWord, 6);
    reg!(YMM7, YMMWord, 7);
    reg!(YMM8, YMMWord, 8);
    reg!(YMM9, YMMWord, 9);
    reg!(YMM10, YMMWord, 10);
    reg!(YMM11, YMMWord, 11);
    reg!(YMM12, YMMWord, 12);
    reg!(YMM13, YMMWord, 13);
    reg!(YMM14, YMMWord, 14);
    reg!(YMM15, YMMWord, 15);
}
