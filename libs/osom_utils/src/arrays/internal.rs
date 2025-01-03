use std::{alloc::Layout, ptr::NonNull};

use osom_traits::allocator::Allocator;

pub struct InternalArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    pub ptr: NonNull<T>,
    pub length: u32,
    pub capacity: u32,
    pub allocator: TAllocator,
}

impl<T, TAllocator> Drop for InternalArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    fn drop(&mut self) {
        if core::mem::needs_drop::<T>() {
            unsafe {
                let mut start = self.ptr.as_ptr();
                let end = start.add(self.length as usize);
                while start < end {
                    core::ptr::drop_in_place(start);
                    start = start.add(1);
                }
            }
        }

        let layout = unsafe {
            Layout::from_size_align_unchecked(
                size_of::<T>() * (self.capacity as usize),
                align_of::<T>(),
            )
        };

        unsafe { self.allocator.deallocate(self.ptr.cast(), layout) };
    }
}
