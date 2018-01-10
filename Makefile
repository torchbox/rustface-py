wheel:
	python setup.py bdist_wheel

sdist:
	python setup.py sdist --format=zip

wheel-manylinux:
	docker run --rm -it -v $(CURDIR):/work -w /work $(IMAGE) sh manylinux.sh

.PHONY: wheel sdist wheel-manylinux
