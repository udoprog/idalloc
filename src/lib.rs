//! General purpose algorithms to generate unique identifiers.
//!
//! # Examples
//!
//! ```rust
//! let mut alloc = idalloc::Slab::<u32>::new();
//! assert_eq!(0u32, alloc.next());
//! assert_eq!(1u32, alloc.next());
//! alloc.free(0u32);
//! ```
#[deny(missing_docs)]
use std::fmt;

/// A type that can be used an allocator index.
pub trait Id: Copy + fmt::Display + fmt::Debug {
    /// Allocate the initial, unallocated value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use idalloc::Id as _;
    ///
    /// assert_eq!(0, u16::initial());
    /// assert_eq!(0, u16::initial());
    /// ```
    fn initial() -> Self;

    /// Get the index as a usize.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use idalloc::Id as _;
    ///
    /// assert_eq!(42, 42u16.as_usize());
    /// assert_eq!(42, 42u32.as_usize());
    /// ```
    fn as_usize(self) -> usize;

    /// Increment the index and return the incremented value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use idalloc::Id as _;
    ///
    /// assert_eq!(1, 0u16.increment());
    /// assert_eq!(1, 0u32.increment());
    /// ```
    fn increment(self) -> Self;

    /// Take the value and replace the existing value with the none variant.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use idalloc::Id as _;
    ///
    /// let mut v = 1u32;
    /// assert_eq!(1u32, v.take());
    /// assert_eq!(u32::none(), v);
    /// ```
    fn take(&mut self) -> Self;

    /// Test if the current value is none, and panic with the given message if
    /// if is.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use idalloc::Id as _;
    ///
    /// let mut v = 1u32;
    /// assert_eq!(1u32, v.expect("value must be defined"));
    /// ```
    fn expect(self, m: &str) -> Self;

    /// Construct the none sentinel value for this type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use idalloc::Id as _;
    ///
    /// assert!(u32::none().is_none());
    /// ```
    fn none() -> Self;

    /// Test if the value is the none sentinel value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use idalloc::Id as _;
    ///
    /// assert!(u32::none().is_none());
    /// ```
    fn is_none(self) -> bool;
}

macro_rules! impl_primitive_index {
    ($ty:ident) => {
        impl Id for $ty {
            #[inline(always)]
            fn initial() -> Self {
                0
            }

            #[inline(always)]
            fn as_usize(self) -> usize {
                self as usize
            }

            #[inline(always)]
            fn increment(self) -> Self {
                if self.is_none() {
                    panic!("index `{}` is out of bounds: 0-{}", self, std::$ty::MAX);
                }

                self + 1
            }

            #[inline(always)]
            fn take(&mut self) -> Self {
                std::mem::replace(self, Self::none())
            }

            #[inline(always)]
            fn expect(self, m: &str) -> Self {
                if self.is_none() {
                    panic!("{}", m);
                }

                self
            }

            #[inline(always)]
            fn none() -> Self {
                std::$ty::MAX
            }

            #[inline(always)]
            fn is_none(self) -> bool {
                self == Self::none()
            }
        }
    };
}

impl_primitive_index!(u8);
impl_primitive_index!(u16);
impl_primitive_index!(u32);
impl_primitive_index!(u64);
impl_primitive_index!(u128);

/// A slab-based id allocator which can deal with automatic reclamation as ids
/// are [freed][Slab::free].
///
/// # Examples
///
/// ```rust
/// use idalloc::Slab;
///
/// let mut alloc = Slab::<u32>::new();
///
/// let mut alloc = Slab::<u32>::new();
/// assert_eq!(0, alloc.next());
/// assert_eq!(1, alloc.next());
/// alloc.free(0);
/// assert_eq!(0, alloc.next());
/// assert_eq!(2, alloc.next());
/// alloc.free(0);
/// alloc.free(0);
/// alloc.free(1);
/// assert_eq!(1, alloc.next());
/// assert_eq!(0, alloc.next());
/// assert_eq!(3, alloc.next());
/// ```
pub struct Slab<I>
where
    I: Id,
{
    data: Vec<I>,
    next: I,
}

impl<I> Slab<I>
where
    I: Id,
{
    /// Construct a new slab allocator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use idalloc::Slab;
    ///
    /// let mut alloc = Slab::<u32>::new();
    ///
    /// let mut alloc = Slab::<u32>::new();
    /// assert_eq!(0, alloc.next());
    /// assert_eq!(1, alloc.next());
    /// alloc.free(0);
    /// assert_eq!(0, alloc.next());
    /// ```
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            next: I::initial(),
        }
    }

    /// Allocate the next id.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut alloc = idalloc::Slab::<u32>::new();
    /// assert_eq!(0u32, alloc.next());
    /// assert_eq!(1u32, alloc.next());
    /// ```
    pub fn next(&mut self) -> I {
        let index = self.next;

        self.next = if let Some(entry) = self.data.get_mut(self.next.as_usize()) {
            entry.take().expect("next index is null")
        } else {
            self.data.push(I::none());
            self.next.increment()
        };

        index
    }

    /// Free the specified id.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut alloc = idalloc::Slab::<u32>::new();
    /// let id = alloc.next();
    /// assert!(!alloc.free(id + 1));
    /// assert!(alloc.free(id));
    /// assert!(!alloc.free(id));
    /// ```
    pub fn free(&mut self, index: I) -> bool {
        if let Some(entry) = self.data.get_mut(index.as_usize()) {
            if entry.is_none() {
                *entry = self.next;
                self.next = index;
                return true;
            }
        }

        false
    }
}

impl<I> Default for Slab<I>
where
    I: Id,
{
    fn default() -> Self {
        Self::new()
    }
}
