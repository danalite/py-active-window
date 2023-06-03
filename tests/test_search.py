import doctest
import py_active_win

def load_tests(loader, tests, ignore):
    tests.addTests(doctest.DocTestSuite(py_active_win))
    return tests
