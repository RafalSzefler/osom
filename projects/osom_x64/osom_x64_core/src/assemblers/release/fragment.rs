use crate::models::Instruction;

pub const NO_NEXT: u32 = 0xffffffu32;

pub enum FragmentKind {
    Static,
    DynamicInstruction(Instruction),
}

pub struct Fragment {
    pub next_position: u32,
    pub buffer_length: u32,
    pub offset: u32,
    pub kind: FragmentKind,
}

pub struct FragmentIterator<'a> {
    buffer: &'a Vec<u8>,
    current: u32,

    #[cfg(debug_assertions)]
    counter: u32,
}

impl<'a> FragmentIterator<'a> {
    #[inline(always)]
    pub fn new(buffer: &'a Vec<u8>) -> Self {
        Self {
            buffer,
            current: 0,
            counter: 0,
        }
    }
}

impl<'a> Iterator for FragmentIterator<'a> {
    type Item = &'a Fragment;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        if current == NO_NEXT {
            return None;
        }
        let fragment = unsafe {
            &*self
                .buffer
                .as_ptr()
                .byte_offset(current as isize)
                .cast::<Fragment>()
        };

        #[cfg(debug_assertions)]
        {
            self.current = fragment.next_position;
            self.counter += 1;
            if self.counter >= 500000 {
                panic!("Looped too many times to retrieve Fragments.");
            }
        }

        Some(fragment)
    }
}
