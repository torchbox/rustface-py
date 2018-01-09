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
pub struct DetectorWrapper {
    pub detector: Box::<rustface::Detector>,
}

#[no_mangle]
pub unsafe extern "C" fn detector_create(model_filename: *const c_char) -> *mut DetectorWrapper {
    let model_filename = CStr::from_ptr(model_filename);

    let detector = rustface::create_detector(model_filename.to_string_lossy().borrow()).unwrap();
    Box::<DetectorWrapper>::into_raw(Box::new(DetectorWrapper { detector }))
}

#[no_mangle]
pub unsafe extern "C" fn detector_set_min_face_size(detector: *mut DetectorWrapper, min_face_size: u32) {
    let mut detector  = Box::<DetectorWrapper>::from_raw(detector);
    detector.detector.set_min_face_size(min_face_size);
    mem::forget(detector);
}

#[no_mangle]
pub unsafe extern "C" fn detector_set_score_thresh(detector: *mut DetectorWrapper, thresh: f64) {
    let mut detector  = Box::<DetectorWrapper>::from_raw(detector);
    detector.detector.set_score_thresh(thresh);
    mem::forget(detector);
}

#[no_mangle]
pub unsafe extern "C" fn detector_set_pyramid_scale_factor(detector: *mut DetectorWrapper, scale_factor: f32) {
    let mut detector  = Box::<DetectorWrapper>::from_raw(detector);
    detector.detector.set_pyramid_scale_factor(scale_factor);
    mem::forget(detector);
}

#[no_mangle]
pub unsafe extern "C" fn detector_set_slide_window_step(detector: *mut DetectorWrapper, step_x: u32, step_y: u32) {
    let mut detector  = Box::<DetectorWrapper>::from_raw(detector);
    detector.detector.set_slide_window_step(step_x, step_y);
    mem::forget(detector);
}

#[no_mangle]
pub unsafe extern "C" fn detector_detect(detector: *mut DetectorWrapper, imagedata: *mut ImageDataWrapper) -> *mut Results {
    let mut detector  = Box::<DetectorWrapper>::from_raw(detector);
    let mut imagedata  = Box::<ImageDataWrapper>::from_raw(imagedata);

    let results = Results {
        results: detector.detector.detect(&mut imagedata.imagedata).into_iter().collect(),
    };

    mem::forget(detector);
    mem::forget(imagedata);

    Box::<Results>::into_raw(Box::new(results))
}


#[no_mangle]
pub unsafe extern "C" fn detector_destroy(detector: *mut DetectorWrapper) {
    Box::<DetectorWrapper>::from_raw(detector);
}
