import sys
import unittest

sys.path.insert(0, "python/ollama_mistral_instruct_chat/src")

from main import send


class TestOllamaMistralIntsructChatMethods(unittest.TestCase):
    def test_send(self):
        # arrange
        # act
        result = send("1 + 1")

        # assert
        self.assertGreaterEqual(result.find("2"), 0)


if __name__ == "__main__":
    unittest.main()
