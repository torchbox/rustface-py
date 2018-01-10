# Rustface bindings for Python

Fast, accurate and easy to install face detection for Python!

This module contains bindings to [Rustface](https://github.com/atomashpolskiy/rustface),
a face detection library written in [Rust](https://www.rust-lang.org/) which is derived
from [SeetaFace](https://github.com/seetaface/SeetaFaceEngine).

## Installation

Install with pip, wheel files are provided for Linux and macOS:

    pip install rustface

## Pillow usage example

    from PIL import Image
    from rustface import ImageData, Detector

    image = Image.open('image.jpg')
    imagedata = ImageData.from_pillow_image(image)

    detector = Detector()
    detector.set_min_face_size(20)
    detector.set_score_thresh(2.0)
    detector.set_pyramid_scale_factor(0.8)
    detector.set_slide_window_step(4, 4)

    for face in detector.detect(imagedata):
        print(face.x, face.y, face.width, face.height)

## Usage with Willow/Wagtail

This provides a [Willow](https://github.com/wagtail/Willow) plugin that can be installed with the following code:

    from willow.registry import registry
    import rustface.willow

    registry.register_plugin(rustface.willow)

(put this somewhere where it will run on startup)

To use this in Wagtail CMS, install the Willow plugin as per above and add the following into your project settings:

    WAGTAILIMAGES_FEATURE_DETECTION_ENABLED = True

## Building from source

Install libffi, python3 headers, setuptools and wheel. The following command will install these on Ubuntu:

    apt-get install libffi-dev python3-dev python3-setuptools python3-wheel

Check out the repository:

    git clone git@github.com:torchbox/rustface-py.git
    cd rustface-py
    git submodule update --init

In order to compile the Rust code, you'll need to have Rust nightly toolchain installed and enabled.

Use rustup to set this up, find installation instructions for rustup at https://www.rustup.rs/

To use Rust nightly, run the following commands in the project root:

    rustup update nightly
    rustup override add nightly

Now you can build the package:

    python3 setup.py bdist_wheel

## License

These bindings, [Rustface](https://github.com/atomashpolskiy/rustface/blob/master/LICENSE) and [SeetaFace](https://github.com/seetaface/SeetaFaceEngine/blob/master/LICENSE) are all released under the BSD 2-Clause license.
