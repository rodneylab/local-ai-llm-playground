import json
import sys
import unittest
import httpretty

sys.path.insert(0, "python/ollama_mistral_instruct_chat/src")

from main import send


class TestOllamaMistralIntsructChatMethods(unittest.TestCase):
    def test_send_makes_ollama_request(self):
        httpretty.enable(verbose=True, allow_net_connect=True)
        json_body = json.dumps(
            {
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
        )
        httpretty.register_uri(
            httpretty.POST, "http://127.0.0.1:11434/api/chat", body=json_body
        )

        send("How hot is the sun?")

        httpretty.disable()
        httpretty.reset()


if __name__ == "__main__":
    unittest.main()
