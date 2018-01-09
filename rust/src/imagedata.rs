use std::ffi::CStr;
use std::os::raw::{c_char, c_void};
use std::borrow::Borrow;
use std::boxed::Box;
use std::mem;

use rustface::{Detector, FaceInfo, ImageData};

pub struct ImageDataWrapper {
    pub imagedata: ImageData,
}

#[no_mangle]
pub unsafe extern "C" fn imagedata_create(bytes: *const c_char, width: u32, height: u32) -> *mut ImageDataWrapper {
    let imagedata = ImageData::new(bytes as *const u8, width, height);
    Box::<ImageDataWrapper>::into_raw(Box::new(ImageDataWrapper { imagedata }))
}

#[no_mangle]
pub unsafe extern "C" fn imagedata_destroy(imagedata: *mut ImageDataWrapper) {
    Box::<ImageDataWrapper>::from_raw(imagedata);
}
