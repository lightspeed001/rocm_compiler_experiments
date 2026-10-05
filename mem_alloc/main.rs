use hip_sys::{hipMallocAsync, hipMallocPinned, hipFreeAsync};

pub fn allocate_device_memory_async(size: usize, stream: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    let mut ptr = std::ptr::null_mut();
    unsafe {
        hip_sys::hipMallocAsync(&mut ptr, size, stream);
    }
    ptr
}

pub fn allocate_pinned_host_memory(size: usize) -> *mut std::ffi::c_void {
    let mut ptr = std::ptr::null_mut();
    unsafe {
        hip_sys::hipMallocPinned(&mut ptr, size);
    }
    ptr
}
