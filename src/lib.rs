pub use slab_allocator_rs::LockedHeap;
use core::alloc::{Layout,GlobalAlloc};
use spin::Lazy;

// A wrapper around `slab_allocator_rs::LockedHeap` that initializes the heap lazily.
pub struct LazyHeap(Lazy<LockedHeap>);

impl LazyHeap {
    /// Create a new `LazyHeap` with the given initialization function.
    pub const fn new(init: fn() -> LockedHeap) -> Self {
        Self(Lazy::new(init))
    }

    /// Create a new `LazyHeap` with the default initialization function.
    pub const fn empty() -> Self {
        Self(Lazy::new(|| LockedHeap::empty()))
    }

    /// Initialize the heap with the given range.
    pub fn init(&self, begin: usize, len: usize) {
        unsafe {
            self.0.init(begin, len);
        }
    }
}

unsafe impl GlobalAlloc for LazyHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { self.0.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { self.0.dealloc(ptr, layout) }
    }
}