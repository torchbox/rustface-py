from __future__ import unicode_literals, absolute_import

from willow.image import Image, ImageBuffer
from willow.plugins.pillow import PillowImage
from rustface import Detector, ImageData


class RustFaceImage(Image):
    def __init__(self, imagedata):
        self.imagedata = imagedata

    @classmethod
    @Image.converter_from(PillowImage)
    def from_pillow(cls, image):
        return RustFaceImage(ImageData.from_pillow_image(image.image))

    @Image.operation
    def detect_faces(self):
        detector = Detector()
        faces = detector.detect(self.imagedata)

        return [(face.x,
                 face.y,
                 face.x + face.width,
                 face.y + face.height,
                ) for face in faces]

    # Prevent Wagtail from crashing if OpenCV is not installed
    # (when Wagtail feature detection is enabled it expects both methods to be implemented)
    @Image.operation
    def detect_features(self):
        return []

willow_image_classes = [RustFaceImage]
