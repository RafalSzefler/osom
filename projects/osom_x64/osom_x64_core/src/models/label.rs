/// Represents labels in assembly code to which certain instruction may refer,
/// in particular jump/call instructions.
#[repr(transparent)]
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Label {
    id: u32,
}

impl Label {
    const _CHECK: () = {
        assert!(size_of::<Self>() <= 4);
    };

    #[must_use]
    #[inline(always)]
    pub fn new(val: u32) -> Self {
        Self { id: val }
    }

    #[must_use]
    #[inline(always)]
    pub fn id(self) -> u32 {
        self.id
    }
}
