from willow.image import Image, ImageBuffer
from willow.plugins.pillow import PillowImage
from rustface import Detector, ImageData


class GreyScaleImageBuffer(ImageBuffer):
    @Image.converter_from(PillowImage)
    def from_pillow(pillow):
        greyscale_image = pillow.image.convert('L')
        return GreyScaleImageBuffer(greyscale_image.size, greyscale_image.tobytes())

    # TODO: Converter for wand


class RustFaceImage(Image):
    def __init__(self, imagedata):
        self.imagedata = imagedata

    @Image.converter_from(GreyScaleImageBuffer)
    def from_greyscale_image(gs_image):
        return RustFaceImage(ImageData(gs_image.data, *gs_image.size))

    @Image.operation
    def detect_faces(self):
        detector = Detector()
        faces = detector.detect(self.imagedata)

        return [(face.x - face.width / 2,
                 face.y - face.height / 2,
                 face.x + face.width / 2,
                 face.y + face.height / 2,
                ) for face in faces]

willow_image_classes = [GreyScaleImageBuffer, RustFaceImage]
