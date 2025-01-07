use super::MachineSize;

macro_rules! reg_field {
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
            const _CHECK: () = const {
                assert!(size_of::<Self>() == 1);
            };

            /// Creates a new instance.
            ///
            /// # Safety
            /// `index` has to be in `0..=15`. Not all combinations with `MachineSize` are valid.
            /// It is advised to use const fields on this struct instead.
            #[must_use]
            #[inline(always)]
            pub const unsafe fn new_unchecked(size: MachineSize, index: u8) -> Self {
                let size_u8 = size as u8;
                Self {
                    val: (size_u8 << 4) | index,
                }
            }

            #[must_use]
            #[inline(always)]
            pub const fn size(&self) -> MachineSize {
                unsafe { core::mem::transmute(self.val >> 4) }
            }

            #[must_use]
            #[inline(always)]
            pub const fn index(&self) -> u8 {
                self.val
            }
        }
    };
}

reg_class!(GPR, "Represents general purpose registers.");

impl GPR {
    // Special.
    reg_field!(NO_REG, None, 16);
    reg_field!(RIP, None, 0);

    // Byte registers.
    reg_field!(AL, Byte, 0);
    reg_field!(CL, Byte, 1);
    reg_field!(DL, Byte, 2);
    reg_field!(BL, Byte, 3);
    reg_field!(SPL, Byte, 4);
    reg_field!(BPL, Byte, 5);
    reg_field!(SIL, Byte, 6);
    reg_field!(DIL, Byte, 7);
    reg_field!(R8B, Byte, 8);
    reg_field!(R9B, Byte, 9);
    reg_field!(R10B, Byte, 10);
    reg_field!(R11B, Byte, 11);
    reg_field!(R12B, Byte, 12);
    reg_field!(R13B, Byte, 13);
    reg_field!(R14B, Byte, 14);
    reg_field!(R15B, Byte, 15);

    // Word registers.
    reg_field!(AX, Word, 0);
    reg_field!(CX, Word, 1);
    reg_field!(DX, Word, 2);
    reg_field!(BX, Word, 3);
    reg_field!(SP, Word, 4);
    reg_field!(BP, Word, 5);
    reg_field!(SI, Word, 6);
    reg_field!(DI, Word, 7);
    reg_field!(R8W, Word, 8);
    reg_field!(R9W, Word, 9);
    reg_field!(R10W, Word, 10);
    reg_field!(R11W, Word, 11);
    reg_field!(R12W, Word, 12);
    reg_field!(R13W, Word, 13);
    reg_field!(R14W, Word, 14);
    reg_field!(R15W, Word, 15);

    // DWord registers.
    reg_field!(EAX, DWord, 0);
    reg_field!(ECX, DWord, 1);
    reg_field!(EDX, DWord, 2);
    reg_field!(EBX, DWord, 3);
    reg_field!(ESP, DWord, 4);
    reg_field!(EBP, DWord, 5);
    reg_field!(ESI, DWord, 6);
    reg_field!(EDI, DWord, 7);
    reg_field!(R8D, DWord, 8);
    reg_field!(R9D, DWord, 9);
    reg_field!(R10D, DWord, 10);
    reg_field!(R11D, DWord, 11);
    reg_field!(R12D, DWord, 12);
    reg_field!(R13D, DWord, 13);
    reg_field!(R14D, DWord, 14);
    reg_field!(R15D, DWord, 15);

    // QWord registers.
    reg_field!(RAX, QWord, 0);
    reg_field!(RCX, QWord, 1);
    reg_field!(RDX, QWord, 2);
    reg_field!(RBX, QWord, 3);
    reg_field!(RSP, QWord, 4);
    reg_field!(RBP, QWord, 5);
    reg_field!(RSI, QWord, 6);
    reg_field!(RDI, QWord, 7);
    reg_field!(R8, QWord, 8);
    reg_field!(R9, QWord, 9);
    reg_field!(R10, QWord, 10);
    reg_field!(R11, QWord, 11);
    reg_field!(R12, QWord, 12);
    reg_field!(R13, QWord, 13);
    reg_field!(R14, QWord, 14);
    reg_field!(R15, QWord, 15);
}

reg_class!(XMM, "Represents XMM registers.");

impl XMM {
    reg_field!(XMM0, XMMWord, 0);
    reg_field!(XMM1, XMMWord, 1);
    reg_field!(XMM2, XMMWord, 2);
    reg_field!(XMM3, XMMWord, 3);
    reg_field!(XMM4, XMMWord, 4);
    reg_field!(XMM5, XMMWord, 5);
    reg_field!(XMM6, XMMWord, 6);
    reg_field!(XMM7, XMMWord, 7);
    reg_field!(XMM8, XMMWord, 8);
    reg_field!(XMM9, XMMWord, 9);
    reg_field!(XMM10, XMMWord, 10);
    reg_field!(XMM11, XMMWord, 11);
    reg_field!(XMM12, XMMWord, 12);
    reg_field!(XMM13, XMMWord, 13);
    reg_field!(XMM14, XMMWord, 14);
    reg_field!(XMM15, XMMWord, 15);
}

reg_class!(YMM, "Represents YMM registers.");

impl YMM {
    reg_field!(YMM0, YMMWord, 0);
    reg_field!(YMM1, YMMWord, 1);
    reg_field!(YMM2, YMMWord, 2);
    reg_field!(YMM3, YMMWord, 3);
    reg_field!(YMM4, YMMWord, 4);
    reg_field!(YMM5, YMMWord, 5);
    reg_field!(YMM6, YMMWord, 6);
    reg_field!(YMM7, YMMWord, 7);
    reg_field!(YMM8, YMMWord, 8);
    reg_field!(YMM9, YMMWord, 9);
    reg_field!(YMM10, YMMWord, 10);
    reg_field!(YMM11, YMMWord, 11);
    reg_field!(YMM12, YMMWord, 12);
    reg_field!(YMM13, YMMWord, 13);
    reg_field!(YMM14, YMMWord, 14);
    reg_field!(YMM15, YMMWord, 15);
}
