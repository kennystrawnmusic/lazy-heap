//! # `lazy_heap`: A wrapper around the `slab_allocator_rs` crate that allows for lazy initialization
//!
//! Although there are plenty of global allocators in the crates.io repository, 3 years of tinkering with [my own kernel code](https://github.com/kennystrawnmusic/cryptos) have taught me that, sometimes, the more time saved, the better. As such, I came up with this in the code to my own kernel but decided, because of just how useful it really is, to actually open it up to the masses.
//!
//! ## Usage
//! Using this crate allows you to use a closure to initialize the heap automatically (lazily) on the first access attempt:
//!
//! ```rust
//! use lazy_heap::LazyHeap;
//!
//! #[global_allocator]
//! pub static ALLOC: LazyHeap = LazyHeap::new(|| {
//!    // allocator initialization code goes here
//! });
//! ```
//!
//! This is a much more seamless, much less error-prone, set-it-and-forget-it way to initialize heap allocation than any other approach, because, with it, you can be guaranteed that any first attempt to use `alloc` will automatically initialize the heap for you.

use core::alloc::{GlobalAlloc, Layout};
use spin::Lazy;

/// A re-export of `slab_allocator_rs::LockedHeap` for ease-of-use reasons.
pub use slab_allocator_rs::LockedHeap;

/// A wrapper around `slab_allocator_rs::LockedHeap` that initializes the heap lazily.
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
