use std::boxed::Box;

use rustface::FaceInfo;

pub type Results = Vec<FaceInfo>;

#[repr(C)]
pub struct Face {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[no_mangle]
pub unsafe extern "C" fn results_get_count(results: *mut Results) -> usize {
    (*results).len()
}

#[no_mangle]
pub unsafe extern "C" fn results_get(results: *mut Results, id: usize) -> Face {
    let bbox = (*results)[id].bbox();

    Face {
        x: bbox.x(),
        y: bbox.y(),
        width: bbox.width(),
        height: bbox.height(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn results_destroy(results: *mut Results) {
    Box::<Results>::from_raw(results);
}
