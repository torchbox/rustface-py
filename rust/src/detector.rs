use std::ffi::CStr;
use std::os::raw::{c_char, c_void};
use std::borrow::Borrow;
use std::boxed::Box;
use std::mem;

use rustface::{self, Detector, FuStDetector};
use rustface::model::ModelReader;
use libc;

use imagedata::ImageDataWrapper;
use results::Results;

// As the detector is a boxed trait, we need to wrap it with
// another boxed object as boxed traits require two pointers
type DetectorWrapper = Box<Box<rustface::Detector>>;

#[no_mangle]
pub unsafe extern "C" fn detector_create(model_data: *const c_char, model_data_len: usize) -> *mut DetectorWrapper {
    // Copy model data into our own buffer
    let mut buf = Vec::with_capacity(model_data_len);
    libc::memcpy(buf.as_ptr() as *mut libc::c_void, model_data as *const libc::c_void, model_data_len);
    buf.set_len(model_data_len);

    // Create model
    let model = ModelReader::new(buf).read().unwrap();

    // Create detector
    let detector = Box::new(FuStDetector::new(model));
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
