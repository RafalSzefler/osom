use core::{alloc::Layout, ptr::NonNull};

#[cfg(not(feature = "std"))]
mod std_impl {
    extern crate alloc;

    use super::Layout;

    #[inline(always)]
    pub(super) unsafe fn allocate(layout: Layout) -> *mut u8 {
        alloc::alloc::alloc(layout)
    }

    #[inline(always)]
    pub(super) unsafe fn reallocate(
        ptr: *mut u8,
        layout: Layout,
        new_size: usize,
    ) -> *mut u8 {
        alloc::alloc::realloc(ptr, layout, new_size)
    }

    #[inline(always)]
    pub(super) unsafe fn dealloc(ptr: *mut u8, layout: Layout) {
        alloc::alloc::dealloc(ptr, layout);
    }
}

#[cfg(feature = "std")]
mod std_impl {
    use std::alloc;

    use super::Layout;

    #[inline(always)]
    pub(super) unsafe fn allocate(layout: Layout) -> *mut u8 {
        alloc::alloc(layout)
    }

    #[inline(always)]
    pub(super) unsafe fn reallocate(
        ptr: *mut u8,
        layout: Layout,
        new_size: usize,
    ) -> *mut u8 {
        alloc::realloc(ptr, layout, new_size)
    }

    #[inline(always)]
    pub(super) unsafe fn dealloc(ptr: *mut u8, layout: Layout) {
        alloc::dealloc(ptr, layout);
    }
}

use crate::allocator::{AllocationError, Allocator};

pub struct StdAllocator;

impl Default for StdAllocator {
    #[inline(always)]
    fn default() -> Self {
        Self {}
    }
}

unsafe impl Allocator for StdAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, AllocationError> {
        let ptr = unsafe { std_impl::allocate(layout) };
        if ptr.is_null() {
            return Err(AllocationError);
        }
        Ok(unsafe { NonNull::new_unchecked(ptr) })
    }

    fn reallocate(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_size: usize,
    ) -> Result<NonNull<u8>, AllocationError> {
        let result = unsafe { std_impl::reallocate(ptr.as_ptr(), old_layout, new_size) };
        if result.is_null() {
            return Err(AllocationError);
        }
        Ok(unsafe { NonNull::new_unchecked(result) })
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        std_impl::dealloc(ptr.as_ptr(), layout);
    }
}
