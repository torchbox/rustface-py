# Rustface bindings for Python

Hopefully, this would lead to faster, more accurate, and easier to install face detection in Wagtail!

## Building from source

Requires Rust nightly, use rustup to install that.

If you haven't got rustup, find installation instructions at https://www.rustup.rs/

To use Rust nightly, run the following in the project folder

    rustup update nightly
    rustup override add nightly

Install libffi and python3 headers and wheel:

    apt-get install libffi-dev python3-dev python3-setuptools python3-wheel
    
Build package:

    python3 setup.py bdist_wheel


## Usage with Willow/Wagtail

This provides a Willow plugin that can be installed with the following code:

    from willow.registry import registry
    import rustface.willow

    registry.register_plugin(rustface.willow)
    
(put this somewhere where it will run on startup)

It provides the ``detect_faces`` operation that is usually provided by OpenCV
so you can use it as a drop-in replacement for that.
