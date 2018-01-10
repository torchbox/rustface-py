# Rustface bindings for Python

Fast, accurate and easy to install face detection for Python!

## Installation

Install with pip, wheel files are provided for Linux and macOS:

    pip install rustface

## Pillow usage example

    from PIL import Image
    from rustface import ImageData, Detector

    image = Image.open('foo.jpg')
    imagedata = ImageData.from_pillow_image(image)
    detector = Detector()

    for face in detector.detect(imagedata):
        print(face.x, face.y, face.width, face.height)

## Usage with Willow/Wagtail

This provides a Willow plugin that can be installed with the following code:

    from willow.registry import registry
    import rustface.willow

    registry.register_plugin(rustface.willow)

(put this somewhere where it will run on startup)

It provides the ``detect_faces`` operation that is usually provided by OpenCV
so you can use it as a drop-in replacement for that.

## Building from source

Install libffi and python3 headers and wheel. The following command installs these on Ubuntu:

    apt-get install libffi-dev python3-dev python3-setuptools python3-wheel

Check out the repository:

    git clone [GIT URL HERE]
    cd rustface-py
    git submodule update --init

In order to compile the Rust code, you need to have Rust nightly toolchain installed and enabled.

Use rustup to set this up, if you have got it already find installation instructions at https://www.rustup.rs/

To use Rust nightly, run the following commands in the project root:

    rustup update nightly
    rustup override add nightly

Now you can build the package:

    python3 setup.py bdist_wheel
