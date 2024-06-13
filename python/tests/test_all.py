import pytest
from maturin_cargo_build_repro import *


def test_py_check():
    assert py_check_file("Cargo.toml")

def test_py_check_fail():
    with pytest.raises(Exception):
        assert py_check_file("Cargo.toml.NO")
