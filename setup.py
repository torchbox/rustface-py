from setuptools import setup, find_packages
from glob import glob

def build_native(spec):
    # build an example rust library
    build = spec.add_external_build(
        cmd=['cargo', 'build', '--release'],
        path='./rust'
    )

    spec.add_cffi_module(
        module_path='rustface._native',
        dylib=lambda: build.find_dylib('rustface', in_path='target/release'),
        header_filename=lambda: build.find_header('rustface.h', in_path='target')
    )

setup(
    name='rustface',
    version='0.1.0',
    packages=find_packages(),
    zip_safe=False,
    platforms='any',
    setup_requires=['milksnake'],
    install_requires=['milksnake'],
    milksnake_tasks=[
        build_native
    ]
)
