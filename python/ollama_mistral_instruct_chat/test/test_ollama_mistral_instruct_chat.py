import respx
import sys
import unittest

sys.path.insert(0, "python/ollama_mistral_instruct_chat/src")

from main import send


class TestOllamaMistralIntsructChatMethods(unittest.TestCase):
    @respx.mock
    def test_send_makes_ollama_request(self):
        json_body = {
            "model": "llama3.2",
            "created_at": "2023-08-04T10:22:45.499127Z",
            "message": {"role": "assistant", "content": ""},
            "done": True,
            "total_duration": 4883583458,
            "load_duration": 1334875,
            "prompt_eval_count": 26,
            "prompt_eval_duration": 342546000,
            "eval_count": 282,
            "eval_duration": 4535599000,
        }

        my_route = respx.post("http://127.0.0.1:11434/api/chat").respond(
            status_code=200,
            json=json_body,
        )

        result = send("How hot is the sun?")
        assert my_route.called
        assert result == ""


if __name__ == "__main__":
    unittest.main()
