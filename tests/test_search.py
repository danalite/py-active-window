import doctest
import py_rust_search

def load_tests(loader, tests, ignore):
    tests.addTests(doctest.DocTestSuite(py_rust_search))
    return tests
