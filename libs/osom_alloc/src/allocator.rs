use core::{alloc::Layout, ptr::NonNull};

/// Represents generic allocation error. Mostly means out of memory.
pub struct AllocationError;

/// Represents generic allocator.
///
/// # Safety
/// Both [`Allocator::allocate()`] and [`Allocator::reallocate()`] are safe to use.
/// [`Allocator::deallocate()`] is not though, since you could potentially pass
/// the same pointer twice ([`NonNull`] is unfortunately copyable).
pub unsafe trait Allocator {
    /// Returns newly allocated memory with specified layout.
    ///
    /// # Errors
    /// On error returns generic [`AllocationError`].
    fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, AllocationError>;

    /// Returns newly reallocated memory with old layout but new size. It might be
    /// NOP if new size is not larger than the old size. The old ptr is invalidated
    /// by this method, and you should use the returned ptr instead.
    ///
    /// # Errors
    /// On error returns generic [`AllocationError`].
    fn reallocate(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_size: usize,
    ) -> Result<NonNull<u8>, AllocationError>;

    /// Deallocates raw ptr.
    ///
    /// # Safety
    /// Safe as long as it is not called twice on the same pointer.
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout);
}
