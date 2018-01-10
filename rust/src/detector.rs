use std::ffi::CStr;
use std::os::raw::{c_char, c_void};
use std::borrow::Borrow;
use std::boxed::Box;
use std::mem;

use rustface::{self, Detector};

use imagedata::ImageDataWrapper;
use results::Results;

// As the detector is a boxed trait, we need to wrap it with
// another boxed object as boxed traits require two pointers
type DetectorWrapper = Box<Box<rustface::Detector>>;

#[no_mangle]
pub unsafe extern "C" fn detector_create(model_filename: *const c_char) -> *mut DetectorWrapper {
    let model_filename = CStr::from_ptr(model_filename);

    let detector = rustface::create_detector(model_filename.to_string_lossy().borrow()).unwrap();
    Box::into_raw(Box::new(Box::new(detector)))
}

#[no_mangle]
pub unsafe extern "C" fn detector_set_min_face_size(detector: *mut DetectorWrapper, min_face_size: u32) {
    (*detector).set_min_face_size(min_face_size);
}

#[no_mangle]
pub unsafe extern "C" fn detector_set_score_thresh(detector: *mut DetectorWrapper, thresh: f64) {
    (*detector).set_score_thresh(thresh);
}

#[no_mangle]
pub unsafe extern "C" fn detector_set_pyramid_scale_factor(detector: *mut DetectorWrapper, scale_factor: f32) {
    (*detector).set_pyramid_scale_factor(scale_factor);
}

#[no_mangle]
pub unsafe extern "C" fn detector_set_slide_window_step(detector: *mut DetectorWrapper, step_x: u32, step_y: u32) {
    (*detector).set_slide_window_step(step_x, step_y);
}

#[no_mangle]
pub unsafe extern "C" fn detector_detect(detector: *mut DetectorWrapper, imagedata: *mut ImageDataWrapper) -> *mut Results {
    let results = (*detector).detect(&mut (*imagedata).imagedata).into_iter().collect();
    Box::<Results>::into_raw(Box::new(results))
}

#[no_mangle]
pub unsafe extern "C" fn detector_destroy(detector: *mut DetectorWrapper) {
    Box::<DetectorWrapper>::from_raw(detector);
}
