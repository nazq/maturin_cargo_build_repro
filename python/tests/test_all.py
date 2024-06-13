import pytest
import maturin_cargo_build_repro


def test_sum_as_string():
    assert maturin_cargo_build_repro.sum_as_string(1, 1) == "2"
