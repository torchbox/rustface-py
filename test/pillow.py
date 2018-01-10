from PIL import Image
from rustface import ImageData, Detector

image = Image.open('scientists.jpg')
imagedata = ImageData.from_pillow_image(image)
detector = Detector()
faces = detector.detect(imagedata)
print(list(faces))
