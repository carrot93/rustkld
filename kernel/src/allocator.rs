use core::{mem, ptr};
use libc::{c_int, c_void};
use crate::bindings::imports::*;

pub fn kld_alloc<T>(pool: *mut malloc_type, flags: c_int) -> Result<ptr::NonNull<T>, c_int>
{
    let size = mem::size_of::<T>();
    
    let raw = unsafe { malloc(size, pool, flags) } as *mut T;

    ptr::NonNull::new(raw).ok_or(ENOMEM)
}

pub fn kld_free<T>(ptr: ptr::NonNull<T>, pool: *mut malloc_type) 
{
    unsafe {
        free(ptr.as_ptr() as *mut c_void, pool);
    }
}
