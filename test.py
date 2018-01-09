from willow.image import Image, ImageBuffer
from willow.plugins.pillow import PillowImage
from willow.registry import registry
from rustface import Detector, ImageData

def convert(image, cls):
    path, _ = registry.find_shortest_path(image.__class__, cls)

    for converter, _ in path:
        image = converter(image)

    return image


class GreyScaleImageBuffer(ImageBuffer):
    @Image.converter_from(PillowImage)
    def from_pillow(pillow):
        greyscale_image = pillow.image.convert('L')
        return GreyScaleImageBuffer(greyscale_image.size, greyscale_image.tobytes())

registry.register_image_class(GreyScaleImageBuffer)

with open('scientists.jpg', 'rb') as f:
    w = convert(Image.open(f), GreyScaleImageBuffer)

    detector = Detector()
    image = ImageData(w.data, *w.size)

    detector.detect(image)
