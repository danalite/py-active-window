from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(name='py-rust-search',
      author="vrteee",
      author_email="vrteee@github.com",
      url='http://github.com/danalites/py-rust-search',
      description="A package for fast file searching in Python using Rust",
      long_description=open("README.rst").read(),
      version="0.0.1",
      rust_extensions=[RustExtension('py_rust_search', 'Cargo.toml',  binding=Binding.PyO3)],
      test_suite="tests",
      license="GPLv3",
      classifiers=[
        "Development Status :: 3 - Alpha",
        "Programming Language :: Python :: 3",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: MIT License",
        "Topic :: Software Development :: Libraries",
        "Topic :: Utilities"],
      zip_safe=False)
