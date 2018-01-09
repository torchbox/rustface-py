from willow.image import Image
from willow.registry import registry
import rustface.willow

registry.register_plugin(rustface.willow)

def test():
    with open('scientists.jpg', 'rb') as f:
        faces = Image.open(f).detect_faces()

        print(list(faces))

test()
