use std::ffi::CStr;
use std::os::raw::{c_char, c_void};
use std::borrow::Borrow;
use std::boxed::Box;
use std::mem;

use libc;
use rustface::{Detector, FaceInfo, ImageData};

pub struct Results {
    pub results: Vec<FaceInfo>,
}

#[repr(C)]
pub struct Face {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[no_mangle]
pub unsafe extern "C" fn results_get_count(results: *mut Results) -> usize {
    let results = Box::<Results>::from_raw(results);
    let count = results.results.len();

    // Prevent deallocation
    mem::forget(results);

    count
}

#[no_mangle]
pub unsafe extern "C" fn results_get(results: *mut Results, id: usize) -> Face {
    let results = Box::<Results>::from_raw(results);
    let result = {
        let bbox = results.results[id].bbox();

        Face {
            x: bbox.x(),
            y: bbox.y(),
            width: bbox.width(),
            height: bbox.height(),
        }
    };

    // Prevent deallocation
    mem::forget(results);

    result
}

#[no_mangle]
pub unsafe extern "C" fn results_destroy(results: *mut Results) {
    Box::<Results>::from_raw(results);
}
