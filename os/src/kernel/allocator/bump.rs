/* ╔═════════════════════════════════════════════════════════════════════════╗
 *   ║ Module: bump                                                            ║
 *   ╟─────────────────────────────────────────────────────────────────────────╢
 *   ║ Descr.: Implementing a basic heap allocator which cannot use            ║
 *   ║         deallocated memory. Thus it is only for learning and testing.   ║
 *   ╟─────────────────────────────────────────────────────────────────────────╢
 *   ║ Author: Philipp Oppermann                                               ║
 *   ║         https://os.phil-opp.com/allocator-designs/                      ║
 *   ╚═════════════════════════════════════════════════════════════════════════╝
 */
use super::{align_up, Locked};
use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr;

/// A simple bump allocator that allocates memory in a linear fashion.
pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
    allocations: usize,
}

impl BumpAllocator {
    /// Create a new empty bump allocator.
    pub const fn new(heap_start: usize, heap_size: usize) -> BumpAllocator {
        BumpAllocator {
            heap_start,
            heap_end: heap_start + heap_size,
            next: heap_start,
            allocations: 0,
        }
    }

    /// Initialize the bump allocator.
    /// No-op for this allocator, but required by the kernel.
    pub unsafe fn init(&mut self) {}

    /// Dump free memory for debugging purposes.
    pub fn dump_free_list(&mut self) {
        let used = self.next - self.heap_start;
        let total = self.heap_end - self.heap_start;
        let free = self.heap_end - self.next;
    
        println!("Bump Allocator Debug Info:");
        println!("  Heap start:   {:#x} Heap end:     {:#x}  Next pointer: {:#x}", self.heap_start, self.heap_end, self.next);
        println!("  Used:         {} bytes", used);
        println!("  Free:         {} bytes", free);
        println!("  Total:        {} bytes", total);
        println!("  Allocations:  {}", self.allocations);

    }

    /// Allocate memory of the given size and alignment.
    pub unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        let alloc_start = align_up(self.next, layout.align());
        let alloc_end = match alloc_start.checked_add(layout.size()) {
            Some(end) => end,
            None => return ptr::null_mut(),
        };

        if alloc_end > self.heap_end {
            ptr::null_mut() // out of memory
        } else {
            self.next = alloc_end;
            self.allocations += 1;
            alloc_start as *mut u8
        }
    }

    /// Deallocate memory (not supported by bump allocator).
    pub unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        println!("Bump Allocator does not support deallocation")
    }
}

// Trait required by the Rust runtime for heap allocations
unsafe impl GlobalAlloc for Locked<BumpAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
            self.lock().alloc(layout)
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            self.lock().dealloc(ptr, layout);
        }
    }
}
