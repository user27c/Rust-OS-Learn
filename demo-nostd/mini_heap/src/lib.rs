const HEAP_SIZE: usize = 128;
static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
static mut OFFSET: usize = 0;

#[allow(dead_code)]
unsafe fn alloc_simple(size: usize) -> *mut u8 {
    if OFFSET + size > HEAP_SIZE {
        return core::ptr::null_mut(); // 堆满了
    }
    let heap_start: usize = unsafe { core::ptr::addr_of!(HEAP) as usize };
    let ptr = (heap_start + OFFSET) as *mut u8;
    OFFSET += size;
    ptr
}

unsafe fn alloc(size: usize) -> *mut u8 {
    // 注意： 必须对其的是绝对地址，不是相对偏移
    // 因为Heap的起始地址不一定是对其的
    let heap_start = unsafe { core::ptr::addr_of!(HEAP) as usize };
    let current_addr = heap_start + OFFSET;
    // 对绝对地址做向上对其
    let aligned_addr = (current_addr + heap_start - 1) & !(heap_start - 1);
    let aligned_offset = aligned_addr - heap_start;

    if aligned_offset > HEAP_SIZE {
        return core::ptr::null_mut(); // 堆满了
    }
    OFFSET = aligned_offset + size;

    (aligned_addr as *mut u8)
}

unsafe fn reset() {
    OFFSET = 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alloc_basic() {
        unsafe {
            reset();
            let a = alloc(16, 1);
            let b = alloc(16, 1);
            // b 应该在 a 后面，差16字节
            assert!(!a.is_null());
            assert!(!b.is_null());
            assert_eq!(b as usize - a as usize, 16);
        }
    }

    #[test]
    fn test_alloc_full() {
        unsafe {
            reset();
            let a = alloc(HEAP_SIZE, 1);
            assert!(!a.is_null());
            let b = alloc(1, 1);
            assert!(b.is_null());
        }
    }

    #[test]
    fn test_alloc_aligned() {
        unsafe {
            reset();
            let _ = alloc(3, 1);
            // offset = 3
            let b = alloc(4, 4);
            assert!(!b.is_null());

            assert_eq!(b as usize % 4, 0, "地址{:p}不是4字节对齐的", b);
        }
    }
}
