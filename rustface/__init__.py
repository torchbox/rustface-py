import os

from . import _native


class ImageData:
    def __init__(self, buffer, width, height):
        self._ptr = _native.lib.imagedata_create(buffer, width, height)

    def __delete__(self):
        _native.lib.imagedata_destroy(self._ptr)


class Detector:
    def __init__(self):
        self._ptr = _native.lib.detector_create(os.path.join(os.path.dirname(__file__), 'model/seeta_fd_frontal_v1.0.bin').encode('UTF-8'))

    def set_min_face_size(self, face_size):
        _native.lib.detector_set_min_face_size(self._ptr, face_size)

    def set_score_thresh(self, thresh):
        _native.lib.detector_set_score_thresh(self._ptr, thresh)

    def set_pyramid_scale_factor(self, scale_factor):
        _native.lib.detector_set_pyramid_scale_factor(self._ptr, scale_factor)

    def set_slide_window_step(self, step_x, step_y):
        _native.lib.detector_set_slide_window_step(self._ptr, step_x, step_y)

    def detect(self, image):
        _native.lib.detector_detect(self._ptr, image._ptr)

    def __delete__(self):
        _native.lib.detector_destroy(self._ptr)


def test():
    detector = Detector()
    detector.set_min_face_size(20)
    detector.set_score_thresh(2.0)
    detector.set_pyramid_scale_factor(0.8)
    detector.set_slide_window_step(4, 4)
