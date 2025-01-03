use std::collections::HashMap;

use crate::models::{Assembler, Label};

use super::{
    fragment::Fragment, EmitError, PreassembleError, SetLabelError,
    X64ReleasePreassembledModule,
};

struct LabelPosition {
    pub fragment_position: u32,
    pub relative_offset: u32,
}

pub struct X64ReleaseAssembler {
    buffer: Vec<u8>,
    label_counter: u32,
    labels: HashMap<String, Label>,
    labels_position: HashMap<Label, LabelPosition>,
    current_fragment_position: u32,
}

macro_rules! get_fragment {
    ( $assembler: expr, $position: expr ) => {
        unsafe {
            let ptr = $assembler
                .buffer
                .as_mut_ptr()
                .byte_offset({ $position } as isize);
            &mut *ptr.cast::<Fragment>()
        }
    };
}

macro_rules! get_current_fragment {
    ( $assembler: expr ) => {{
        let pos = $assembler.current_fragment_position;
        get_fragment!($assembler, pos)
    }};
}

impl X64ReleaseAssembler {
    #[inline(always)]
    fn current_len(&self) -> u32 {
        self.buffer.len() as u32
    }
}

impl Default for X64ReleaseAssembler {
    fn default() -> Self {
        Self {
            buffer: Vec::with_capacity(2048),
            label_counter: 0,
            labels: HashMap::with_capacity(16),
            labels_position: HashMap::with_capacity(16),
            current_fragment_position: 0,
        }
    }
}

impl Assembler for X64ReleaseAssembler {
    type EmitError = EmitError;

    type SetLabelError = SetLabelError;

    type PreassembledModule = X64ReleasePreassembledModule;

    type PreassembleError = PreassembleError;

    fn get_or_create_label<T: AsRef<str>>(&mut self, name: &T) -> Label {
        if let Some(label) = self.labels.get(name.as_ref()) {
            *label
        } else {
            let counter = self.label_counter;
            self.label_counter += 1;
            let label = Label::new(counter);
            self.labels.insert(name.as_ref().to_string(), label);
            label
        }
    }

    fn add_instruction(
        &mut self,
        instr: &crate::models::Instruction,
    ) -> Result<usize, Self::EmitError> {
        todo!()
    }

    fn add_bytes<T: AsRef<[u8]>>(
        &mut self,
        buffer: &T,
    ) -> Result<usize, Self::EmitError> {
        let slice = buffer.as_ref();
        let len = slice.len();
        self.buffer.extend_from_slice(slice);
        let fragment = get_current_fragment!(self);
        fragment.buffer_length += len as u32;
        Ok(len)
    }

    fn set_label(&mut self, label: &Label) -> Result<(), Self::SetLabelError> {
        if self.labels_position.contains_key(label) {
            return Err(SetLabelError::LabelAlreadyExists);
        }

        let offset = {
            let fragment = get_current_fragment!(self);
            fragment.offset
        };

        let current_len = self.current_len();
        let position = LabelPosition {
            fragment_position: self.current_fragment_position,
            relative_offset: current_len - offset,
        };
        self.labels_position.insert(*label, position);
        Ok(())
    }

    fn preassemble(
        mut self,
    ) -> Result<Self::PreassembledModule, Self::PreassembleError> {
        // TODO: process offsets
        let mut inverse_map = {
            let labels = self.labels;
            let mut map = HashMap::with_capacity(labels.len());
            for kvp in labels.into_iter() {
                map.insert(kvp.1, kvp.0);
            }
            map
        };

        let mut absolute_labels = HashMap::with_capacity(self.labels_position.len());
        let labels_positions = &self.labels_position;
        for (label, pos) in labels_positions.iter() {
            let absolute_position = {
                let fragment = get_fragment!(self, self.current_fragment_position);
                fragment.offset + pos.relative_offset
            };
            if let Some(text_label) = inverse_map.remove(label) {
                absolute_labels.insert(text_label, absolute_position);
            } else {
                unreachable!("All labels have to have text representation.");
            }
        }

        let module = X64ReleasePreassembledModule::new(absolute_labels, self.buffer);
        Ok(module)
    }
}
