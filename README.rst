py-active-window: Search files in target directory
============================================================

.. image:: https://github.com/danalite/py-active-window/workflows/Build/badge.svg?branch=main
    :target: https://github.com/danalite/py-active-window/actions?query=branch%3Amain

A package for searching files in a target directory.

This package provides python bindings for the rust crate
`Rust_Search <https://github.com/ParthJadhav/Rust_Search>`_ by building
a native Python extension using `PyO3 <https://github.com/pyO3/pyO3>`_.

Usage
-------------------

To install

.. code-block:: python

    pip install py-active-window


Building from source requires the nightly version of the rust compiler.

This module exposes a single function that transforms C++ linker symbols to a human readable
representation.

.. code-block:: python

    from py_active_win import get_active_win
    print(get_active_win())


Released under the MIT License
