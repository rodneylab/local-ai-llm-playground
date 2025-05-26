#!/usr/local/bin/python3 -tt
# -*- coding: utf-8 -*-


import ollama

messages: list[dict[str, str]] = []


def send(chat: str) -> str:
    messages.append({"role": "user", "content": chat})
    stream = ollama.chat(model="mistral:instruct", messages=messages, stream=True)

    response = ""
    for chunk in stream:
        part = chunk["message"]["content"]
        print(part, end="", flush=True)
        response = response + part

    messages.append({"role": "assistant", "content": response})

    print("")

    return response


def main():
    print("trying-ai/python/ollama_mistral_instruct_chat")
    while True:
        chat = input(">>> ")

        if chat == "/exit":
            break
        elif len(chat) > 0:
            send(chat)


if __name__ == "__main__":
    main()
