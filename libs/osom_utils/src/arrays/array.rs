use osom_alloc::{Allocator, StdAllocator};

use super::{internal::InternalArray, DynamicArray};

/// Represents immutable variant of [`DynamicArray`]. It is trivially created
/// from [`DynamicArray`], but once created there is no coming back to [`DynamicArray`].
/// Internally it has the same representation as [`DynamicArray`], except it cannot
/// be mutated in any safe way.
pub struct Array<T, TAllocator = StdAllocator>
where
    TAllocator: Allocator,
{
    internal: InternalArray<T, TAllocator>,
}

impl<T, TAllocator> AsRef<[T]> for Array<T, TAllocator>
where
    TAllocator: Allocator,
{
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, TAllocator> Array<T, TAllocator>
where
    TAllocator: Allocator,
{
    #[must_use]
    #[inline(always)]
    pub const fn from_dynamic_array(dynamic_array: DynamicArray<T, TAllocator>) -> Self {
        Self {
            internal: unsafe { dynamic_array.into_internal_array() },
        }
    }

    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> u32 {
        self.internal.length
    }

    #[must_use]
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.internal.length == 0
    }

    #[must_use]
    pub const fn as_slice(&self) -> &[T] {
        if self.internal.length == 0 {
            &[]
        } else {
            unsafe {
                core::slice::from_raw_parts(
                    self.internal.ptr.as_ptr(),
                    self.internal.length as usize,
                )
            }
        }
    }
}

impl<T, TAllocator> From<InternalArray<T, TAllocator>> for Array<T, TAllocator>
where
    TAllocator: Allocator,
{
    fn from(value: InternalArray<T, TAllocator>) -> Self {
        Self { internal: value }
    }
}

impl<T, TAllocator> From<DynamicArray<T, TAllocator>> for Array<T, TAllocator>
where
    TAllocator: Allocator,
{
    fn from(value: DynamicArray<T, TAllocator>) -> Self {
        Self::from_dynamic_array(value)
    }
}
