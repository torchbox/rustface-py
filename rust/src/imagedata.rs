use std::os::raw::c_char;
use std::boxed::Box;
use std::mem;

use libc;
use rustface::ImageData;

pub struct ImageDataWrapper {
    pub data: *mut u8,
    pub data_size: usize,
    pub imagedata: ImageData,
}

fn alloc(count: usize) -> *mut u8 {
    let mut v = Vec::with_capacity(count);
    let ptr = v.as_mut_ptr();
    mem::forget(v);
    ptr
}

unsafe fn free(ptr: *mut u8, count: usize) {
    mem::drop(Vec::from_raw_parts(ptr, 0, count));
}

#[no_mangle]
pub unsafe extern "C" fn imagedata_create(bytes: *const c_char, width: u32, height: u32) -> *mut ImageDataWrapper {
    // We need to copy the image data into our own buffer so that we can access it later
    // It's up to the caller to make sure that the buffer is at least `width * height` bytes in size
    let data_size = (width * height) as usize;
    let data = alloc(data_size);
    libc::memcpy(data as *mut libc::c_void, bytes as *const libc::c_void, data_size);

    let imagedata = ImageData::new(data as *const u8, width, height);
    Box::<ImageDataWrapper>::into_raw(Box::new(ImageDataWrapper { data, data_size, imagedata }))
}

#[no_mangle]
pub unsafe extern "C" fn imagedata_destroy(imagedata: *mut ImageDataWrapper) {
    let imagedata = Box::<ImageDataWrapper>::from_raw(imagedata);

    // Free pixel data
    free(imagedata.data, imagedata.data_size);
}
