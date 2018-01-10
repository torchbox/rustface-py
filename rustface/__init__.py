from __future__ import unicode_literals, absolute_import

import os

from . import _native


class ImageData(object):
    def __init__(self, buffer, width, height):
        # Buffer must be exactly width * height bytes in size
        if len(buffer) != width * height:
            raise Exception("Incorrect buffer size. Expected {} bytes. Got {} bytes", width * height, len(buffer))

        self._ptr = _native.lib.imagedata_create(buffer, width, height)

    @classmethod
    def from_pillow_image(self, image):
        greyscale_image = image.convert('L')

        return ImageData(greyscale_image.tobytes(), *greyscale_image.size)

    def __delete__(self):
        _native.lib.imagedata_destroy(self._ptr)


class Face(object):
    def __init__(self, cface):
        self.x = cface.x
        self.y = cface.y
        self.width = cface.width
        self.height = cface.height

    def __repr__(self):
        return '<Face x:{} y:{} size:{}x{}>'.format(
            self.x, self.y, self.width, self.height
        )


class Results(object):
    def __init__(self, ptr):
        self._ptr = ptr

    def __iter__(self):
        count = _native.lib.results_get_count(self._ptr)

        for i in range(count):
            yield Face(_native.lib.results_get(self._ptr, i))

    def __delete__(self):
        _native.lib.results_destroy(self._ptr)


class Detector(object):
    def __init__(self):
        with open(os.path.join(os.path.dirname(__file__), 'seeta_fd_frontal_v1.0.py'), 'rb') as f:
            buf = f.read()
        self._ptr = _native.lib.detector_create(buf, len(buf))

    def set_min_face_size(self, face_size):
        _native.lib.detector_set_min_face_size(self._ptr, face_size)

    def set_score_thresh(self, thresh):
        _native.lib.detector_set_score_thresh(self._ptr, thresh)

    def set_pyramid_scale_factor(self, scale_factor):
        _native.lib.detector_set_pyramid_scale_factor(self._ptr, scale_factor)

    def set_slide_window_step(self, step_x, step_y):
        _native.lib.detector_set_slide_window_step(self._ptr, step_x, step_y)

    def detect(self, image):
        return Results(_native.lib.detector_detect(self._ptr, image._ptr))

    def __delete__(self):
        _native.lib.detector_destroy(self._ptr)
