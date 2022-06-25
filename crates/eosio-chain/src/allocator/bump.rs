//code borrow from https://github.com/paritytech/ink/blob/master/crates/allocator/src/bump.rs
//with a fix of head_base
use core::alloc::{
    GlobalAlloc,
    Layout,
};

/// A page in Wasm is `64KiB`
const PAGE_SIZE: usize = 64 * 1024;

static mut INNER: InnerAlloc = InnerAlloc::new();

/// A bump allocator suitable for use in a Wasm environment.
pub struct BumpAllocator;

unsafe impl GlobalAlloc for BumpAllocator {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match INNER.alloc(layout) {
            Some(start) => start as *mut u8,
            None => core::ptr::null_mut(),
        }
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // A new page in Wasm is guaranteed to already be zero initialized, so we can just use our
        // regular `alloc` call here and save a bit of work.
        //
        // See: https://webassembly.github.io/spec/core/exec/modules.html#growing-memories
        self.alloc(layout)
    }

    #[inline]
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[cfg_attr(feature = "std", derive(Debug, Copy, Clone))]
struct InnerAlloc {
    /// Points to the start of the next available allocation.
    next: usize,

    /// The address of the upper limit of our heap.
    upper_limit: usize,
}

extern "C" {
    static __heap_base: u8;
}

fn get_heap_base() -> usize {
    unsafe {
        return &__heap_base as *const u8 as usize;
    }
}

impl InnerAlloc {
    const fn new() -> Self {
        Self {
            next: 0,
            upper_limit: 0,
        }
    }

    cfg_if::cfg_if! {
        if #[cfg(test)] {
            /// Request a `pages` number of page sized sections of Wasm memory. Each page is `64KiB` in size.
            ///
            /// Returns `None` if a page is not available.
            ///
            /// This implementation is only meant to be used for testing, since we cannot (easily)
            /// test the `wasm32` implementation.
            fn request_pages(&mut self, _pages: usize) -> Option<usize> {
                Some(self.upper_limit)
            }
        } else if #[cfg(feature = "std")] {
            fn request_pages(&mut self, _pages: usize) -> Option<usize> {
                unreachable!(
                    "This branch is only used to keep the compiler happy when building tests, and
                     should never actually be called outside of a test run."
                )
            }
        } else if #[cfg(target_arch = "wasm32")] {
            /// Request a `pages` number of pages of Wasm memory. Each page is `64KiB` in size.
            ///
            /// Returns `None` if a page is not available.
            fn request_pages(&mut self, pages: usize) -> Option<usize> {
                let prev_page = core::arch::wasm32::memory_grow(0, pages);
                if prev_page == usize::MAX {
                    return None;
                }

                prev_page.checked_mul(PAGE_SIZE)
            }
        } else {
            compile_error! {
                "only supports compilation as `std` or `no_std` + `wasm32-unknown`"
            }
        }
    }

    /// Tries to allocate enough memory on the heap for the given `Layout`. If there is not enough
    /// room on the heap it'll try and grow it by a page.
    ///
    /// Note: This implementation results in internal fragmentation when allocating across pages.
    fn alloc(&mut self, layout: Layout) -> Option<usize> {
        if self.next == 0 {
            self.next = get_heap_base();
            self.upper_limit = required_pages(self.next).unwrap()  * 65536;
        }

        let alloc_start = self.next;

        let aligned_size = layout.pad_to_align().size();
        let alloc_end = alloc_start.checked_add(aligned_size)?;

        if alloc_end > self.upper_limit {
            let required_pages = required_pages(aligned_size)?;
            let page_start = self.request_pages(required_pages)?;

            self.upper_limit = required_pages
                .checked_mul(PAGE_SIZE)
                .and_then(|pages| page_start.checked_add(pages))?;
            self.next = page_start.checked_add(aligned_size)?;

            Some(page_start)
        } else {
            self.next = alloc_end;
            Some(alloc_start)
        }
    }
}

/// Calculates the number of pages of memory needed for an allocation of `size` bytes.
///
/// This function rounds up to the next page. For example, if we have an allocation of
/// `size = PAGE_SIZE / 2` this function will indicate that one page is required to satisfy
/// the allocation.
#[inline]
fn required_pages(size: usize) -> Option<usize> {
    size.checked_add(PAGE_SIZE - 1)
        .and_then(|num| num.checked_div(PAGE_SIZE))
}


