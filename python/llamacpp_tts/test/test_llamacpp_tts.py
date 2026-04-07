import sys
import unittest
from tempfile import NamedTemporaryFile

sys.path.insert(0, "python/llamacpp_tts/src")

from main import validate_file_path


class TestLlamacppTtsMethods(unittest.TestCase):
    def test_validate_file_path_returns_none_for_valid_path(self):
        # arrange
        temp_file = self.enterContext(NamedTemporaryFile(mode="w+", suffix=".txt"))
        temp_file.write("This is a test file")
        path = str(temp_file.name)

        # act/assert
        outcome = validate_file_path(path)
        self.assertIsNone(outcome)

    def test_validate_file_path_raises_error_for_invalid_path(self):
        # arrange
        path = "./fixtures/does-not-exist.txt"

        # act/assert
        with self.assertRaises(FileNotFoundError):
            validate_file_path(path)


if __name__ == "__main__":
    unittest.main()
