use std::alloc::Layout;

use osom_traits::{
    allocator::{AllocationError, Allocator},
    default_impl::StdAllocator,
};

use super::{internal::InternalArray, Array};

/// Similar to Vec, but unlike Vec supports smaller range of lengths (
/// up to [`DynamicArray::max_length()`]), and has slightly different behaviour.
/// First of all always allocates, doesn't allow capacity 0. And growth is different,
/// by factor of 1.6.
pub struct DynamicArray<T, TAllocator = StdAllocator>
where
    TAllocator: Allocator,
{
    internal: InternalArray<T, TAllocator>,
}

impl<T, TAllocator> AsMut<[T]> for DynamicArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    fn as_mut(&mut self) -> &mut [T] {
        self.as_slice_mut()
    }
}

impl<T, TAllocator> AsRef<[T]> for DynamicArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

#[derive(Debug)]
pub enum DynamicArrayError {
    /// Underlying allocator could not allocate memory. Likely because of out of memory.
    AllocationError,

    /// Capacity exceeded [`DynamicArray::max_length()`].
    CapacityTooBig,

    /// Capacity smaller than [`DynamicArray::min_capacity()`].
    CapacityTooSmall,
}

impl From<AllocationError> for DynamicArrayError {
    fn from(_: AllocationError) -> Self {
        Self::AllocationError
    }
}

impl<T, TAllocator> DynamicArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    /// Returns the minimal capacity [`DynamicArray`] can have. Note that
    /// length can still be zero.
    #[must_use]
    #[inline(always)]
    pub const fn min_capacity() -> u32 {
        32
    }

    /// Returns the maximum capacity and length [`DynamicArray`] can have.
    #[must_use]
    #[inline(always)]
    pub const fn max_length() -> u32 {
        let size = size_of::<T>();
        let result = (i32::MAX as usize) / size;

        #[allow(clippy::cast_possible_truncation)]
        {
            result as u32
        }
    }

    /// Creates a new instance of [`DynamicArray`] with
    /// [`DynamicArray::min_capacity()`] capacity and concrete `allocator`.
    ///
    /// # Errors
    /// See [`DynamicArrayError`].
    #[inline(always)]
    pub fn new_with_allocator(allocator: TAllocator) -> Result<Self, DynamicArrayError> {
        Self::with_capacity_and_allocator(Self::min_capacity(), allocator)
    }

    /// Creates a new instance of [`DynamicArray`] with passed capacity and concrete `allocator`.
    ///
    /// # Errors
    /// See [`DynamicArrayError`].
    pub fn with_capacity_and_allocator(
        capacity: u32,
        allocator: TAllocator,
    ) -> Result<Self, DynamicArrayError> {
        if capacity > Self::max_length() {
            return Err(DynamicArrayError::CapacityTooBig);
        }

        if capacity < Self::min_capacity() {
            return Err(DynamicArrayError::CapacityTooSmall);
        }

        let layout = unsafe {
            Layout::from_size_align_unchecked(
                size_of::<T>() * (capacity as usize),
                align_of::<T>(),
            )
        };

        let ptr = allocator.allocate(layout)?;
        let internal = InternalArray {
            ptr: ptr.cast(),
            length: 0,
            capacity: capacity,
            allocator: allocator,
        };

        Ok(Self { internal })
    }

    /// Pushes new element to the [`DynamicArray`]. This will reallocate if
    /// length exceeds capacity.
    ///
    /// # Errors
    /// See [`DynamicArrayError`].
    pub fn push(&mut self, value: T) -> Result<(), DynamicArrayError> {
        self.resize_if_needed()?;

        unsafe {
            let address = self.internal.ptr.add(self.internal.length as usize);
            address.write(value);
        }

        self.internal.length += 1;

        Ok(())
    }

    /// Pops element from the end of [`DynamicArray`] and decrease its length by 1.
    /// Returns [`None`] if array is empty.
    #[must_use]
    pub fn pop(&mut self) -> Option<T> {
        if self.internal.length == 0 {
            return None;
        }

        self.internal.length -= 1;

        Some(unsafe {
            let address = self.internal.ptr.add(self.internal.length as usize);
            address.read()
        })
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
    pub const fn as_slice_mut(&mut self) -> &mut [T] {
        if self.internal.length == 0 {
            &mut []
        } else {
            unsafe {
                core::slice::from_raw_parts_mut(
                    self.internal.ptr.as_ptr(),
                    self.internal.length as usize,
                )
            }
        }
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

    fn resize_if_needed(&mut self) -> Result<(), DynamicArrayError> {
        if self.internal.length == self.internal.capacity {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let new_capacity = (f64::from(self.internal.capacity) * 1.6) as u32;

            if new_capacity > Self::max_length() {
                return Err(DynamicArrayError::CapacityTooBig);
            }

            let old_layout = unsafe {
                Layout::from_size_align_unchecked(
                    size_of::<T>() * (self.internal.capacity as usize),
                    align_of::<T>(),
                )
            };

            let new_size = size_of::<T>() * (new_capacity as usize);

            let new_ptr = self.internal.allocator.reallocate(
                self.internal.ptr.cast(),
                old_layout,
                new_size,
            )?;

            self.internal.ptr = new_ptr.cast();
            self.internal.capacity = new_capacity;
        }

        Ok(())
    }

    pub(super) const unsafe fn into_internal_array(
        self,
    ) -> InternalArray<T, TAllocator> {
        let internal = core::ptr::read(&self.internal);
        core::mem::forget(self);
        internal
    }

    #[must_use]
    #[inline(always)]
    pub const fn into_array(self) -> Array<T, TAllocator> {
        Array::from_dynamic_array(self)
    }
}

impl<T, TAllocator> From<InternalArray<T, TAllocator>> for DynamicArray<T, TAllocator>
where
    TAllocator: Allocator,
{
    fn from(value: InternalArray<T, TAllocator>) -> Self {
        Self { internal: value }
    }
}

impl<T> DynamicArray<T> {
    /// Creates a new instance of [`DynamicArray`] with
    /// [`DynamicArray::min_capacity()`] capacity and default [`StdAllocator`].
    ///
    /// # Errors
    /// See [`DynamicArrayError`].
    #[allow(clippy::default_constructed_unit_structs)]
    #[inline(always)]
    pub fn new() -> Result<Self, DynamicArrayError> {
        Self::new_with_allocator(StdAllocator::default())
    }

    /// Creates a new instance of [`DynamicArray`] with passed capacity and default
    /// [`StdAllocator`].
    ///
    /// # Errors
    /// See [`DynamicArrayError`].
    #[allow(clippy::default_constructed_unit_structs)]
    #[inline(always)]
    pub fn with_capacity(capacity: u32) -> Result<Self, DynamicArrayError> {
        Self::with_capacity_and_allocator(capacity, StdAllocator::default())
    }
}
