py-rust-search: Search files in target directory
============================================================

.. image:: https://github.com/danalite/py-rust-search/workflows/Build/badge.svg?branch=master
    :target: https://github.com/danalite/py-rust-search/actions?query=branch%3Amaster

A package for searching files in a target directory.

This package provides python bindings for the rust crate
`Rust_Search <https://github.com/ParthJadhav/Rust_Search>`_ by building
a native Python extension using `PyO3 <https://github.com/pyO3/pyO3>`_.

Usage
-------------------

To install

.. code-block:: python

    pip install py-rust-search


Building from source requires the nightly version of the rust compiler.

This module exposes a single function that transforms C++ linker symbols to a human readable
representation.

.. code-block:: python

    from py_rust_search import get_similar_files
    print(get_similar_files('png', '~/Desktop'))


Released under the MIT License
