from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(name='py-active-window',
      author="vrteee",
      author_email="vrteee@github.com",
      url='http://github.com/danalite/py-active-window',
      description="A package for getting active window using Rust",
      long_description=open("README.rst").read(),
      version="0.0.1",
      rust_extensions=[RustExtension('py_active_win', 'Cargo.toml',  binding=Binding.PyO3)],
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
