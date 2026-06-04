import os

import pytest


@pytest.fixture
def is_ci() -> bool:
    return os.environ.get("CI", "0") != "0"
