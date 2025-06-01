#![no_std]

extern crate axlog;

use allocator::{AllocError, BaseAllocator, ByteAllocator, PageAllocator};
use axlog::info;
use core::ptr::NonNull;

/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///
pub struct EarlyAllocator<const PAGE_SIZE: usize> {
    b_pos: usize,
    p_pos: usize,
    count: usize,
    start: usize,
    size: usize,
}

impl<const PAGE_SIZE: usize> EarlyAllocator<PAGE_SIZE> {
    pub const fn new() -> Self {
        Self {
            b_pos: 0,
            p_pos: 0,
            count: 0,
            start: 0,
            size: 0,
        }
    }
}

impl<const PAGE_SIZE: usize> BaseAllocator for EarlyAllocator<PAGE_SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        self.b_pos = start;
        self.p_pos = start + size;
        self.count = 0;
        self.start = start;
        self.size = size;
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> Result<(), AllocError> {
        Err(AllocError::NoMemory)
    }
}

impl<const PAGE_SIZE: usize> ByteAllocator for EarlyAllocator<PAGE_SIZE> {
    fn alloc(&mut self, layout: core::alloc::Layout) -> Result<NonNull<u8>, AllocError> {
        let align = layout.align();
        let start = (self.b_pos + align - 1) & !(align - 1);
        let end = start + layout.size();

        if end > self.p_pos {
            return Err(AllocError::NoMemory);
        }

        self.b_pos = end;
        self.count += 1;
        unsafe { Ok(NonNull::new_unchecked(start as *mut u8)) }
    }

    fn dealloc(&mut self, _pos: NonNull<u8>, _layout: core::alloc::Layout) {
        self.count -= 1;
        if self.count == 0 {
            self.b_pos = self.start;
        }
    }

    fn total_bytes(&self) -> usize {
        self.size
    }
    fn used_bytes(&self) -> usize {
        self.b_pos - self.start
    }
    fn available_bytes(&self) -> usize {
        self.p_pos - self.b_pos
    }
}

impl<const PAGE_SIZE: usize> PageAllocator for EarlyAllocator<PAGE_SIZE> {
    const PAGE_SIZE: usize = PAGE_SIZE;

    fn alloc_pages(&mut self, num_pages: usize, align_pow2: usize) -> Result<usize, AllocError> {
        let size = num_pages * PAGE_SIZE;
        let start = (self.p_pos - size) & !(align_pow2 - 1);

        if start < self.b_pos {
            return Err(AllocError::NoMemory);
        }

        self.p_pos = start;
        Ok(start)
    }

    fn dealloc_pages(&mut self, _pos: usize, _num_pages: usize) {
        // 页分配器不释放内存
    }

    fn total_pages(&self) -> usize {
        self.size / PAGE_SIZE
    }
    fn used_pages(&self) -> usize {
        (self.size - (self.p_pos - self.start)) / PAGE_SIZE
    }
    fn available_pages(&self) -> usize {
        (self.p_pos - self.b_pos) / PAGE_SIZE
    }
}
