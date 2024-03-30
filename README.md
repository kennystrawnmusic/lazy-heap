# `lazy_heap`: A wrapper around the `slab_allocator_rs` crate that allows for lazy initialization

Although there are plenty of global allocators in the crates.io repository, 3 years of tinkering with [my own kernel code](https://github.com/kennystrawnmusic/cryptos) have taught me that, sometimes, the more time saved, the better. As such, I came up with this in the code to my own kernel but decided, because of just how useful it really is, to actually open it up to the masses.

Using this crate allows you to use a closure to initialize the heap automatically (lazily) on the first access attempt:

```rust
use lazy_heap::LazyHeap;

#[global_allocator]
pub static ALLOC: LazyHeap = LazyHeap::new(|| {
    // allocator initialization code goes here
})
```

This is a much more seamless, much less error-prone, set-it-and-forget-it way to initialize heap allocation than any other approach, because, with it, you can be guaranteed that any first attempt to use `alloc` will automatically initialize the heap for you.