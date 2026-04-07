#!/usr/local/bin/python3 -tt
# -*- coding: utf-8 -*-

import argparse
import logging
from pathlib import Path
from typing import Any

from outetts import (
    Backend,
    GenerationConfig,
    GenerationType,
    Interface,
    LlamaCppQuantization,
    ModelConfig,
    Models,
    SamplerConfig,
)

logger = logging.getLogger(__name__)


def setup_logging():
    """
    Configures the logging settings
    """
    logging.basicConfig(
        level=logging.INFO,
        format="%(asctime)s.%(msecs)03d %(message)s",
        datefmt="%Y-%m-%d %H:%M:%S",
    )


def get_interface() -> Any:
    """
    Creates and returns an interface instance using the default configuration.

    Returns:
        Interface: An instance of outetts.Interface with auto-configured settings.
    """
    config = ModelConfig.auto_config(
        # Use 1B parameter OuteTTS model
        model=Models.VERSION_1_0_SIZE_1B,
        backend=Backend.LLAMACPP,
        # Use 8-bit quantised model variant
        quantization=LlamaCppQuantization.Q8_0,
    )

    return Interface(config=config)


def validate_file_path(file_path: str) -> None:
    """
    Validates the provided path exists.  Logs an error if it does not.

    Args:
        file_path (str): Path of file to check

    Raises:
        FileNotFoundError: If the file does not exist
    """
    path = Path(file_path)
    if not path.exists():
        logger.error("Error: The file `{file_path}` does not exist.")
        raise FileNotFoundError


def train_speaker(args: argparse.Namespace) -> None:
    """
    Trains a new speaker using the provided audio file and saves it to the specified
    output location.

    Args:
        args (argparse.Namespace): Parsed command-line arguments containing the input
        file path and output file path.
    """
    validate_file_path(args.file)

    interface = get_interface()
    new_speaker = interface.create_speaker(args.file)
    interface.save_speaker(new_speaker, args.output)
    logger.info(
        f"Model trained with input `{args.file}`, data written to `{args.output}`."
    )


def tts(args) -> None:
    """
    Converts text into speech using the specified speaker and saves the
    output audio file.

    Args:
        args (argparse.Namespace): Parsed command-line arguments containing the text to
        speak, optional speaker JSON file, and an optional speaker name.
    """
    interface = get_interface()
    if args.speaker:
        validate_file_path(args.speaker)
        speaker = interface.load_speaker(args.speaker)
        logger.info(f"Using speaker data from `{args.speaker}`.")
    else:
        speaker = interface.load_default_speaker("EN-FEMALE-1-NEUTRAL")
        logger.info("Using default speaker.")

    text = args.text
    output_path = "output.wav"
    output = interface.generate(
        config=GenerationConfig(
            text=text,
            generation_type=GenerationType.CHUNKED,
            speaker=speaker,
            sampler_config=SamplerConfig(
                temperature=0.4,
                repetition_penalty=1.1,
                repetition_range=64,
                top_k=40,
                top_p=0.9,
                min_p=0.05,
            ),
            max_length=8192,
        )
    )
    output.save(output_path)
    logger.info(f"Audio saved to `{output_path}`.")


def initialise_parser():
    """
    Initialises and returns an argument parser for the script, setting up subparsers for
    different modes ('tts' and 'train').

    Returns:
        argparse.ArgumentParser: An instance of argparse.ArgumentParser configured with
        subcommands.
    """
    parser = argparse.ArgumentParser(
        description="LLM text-to-speech (TTS) demo with voice training."
    )
    subparsers = parser.add_subparsers(
        title="mode", description="`tts` or `train` mode", help="subcommand help"
    )

    # tts subparser
    parser_tts = subparsers.add_parser("tts", help="tts help")
    parser_tts.add_argument(
        "-t",
        "--text",
        default="Use the --text flag to customise this spoken text",
        help="Specify the text that you want the model to speak.",
        type=str,
    )
    parser_tts.add_argument(
        "-s",
        "--speaker",
        default="",
        help="Specify the speaker JSON file to use for voicing the text.",
        type=str,
    )
    parser_tts.set_defaults(func=tts)

    # train subparser
    parser_train = subparsers.add_parser("train", help="train help")
    parser_train.add_argument(
        "-f",
        "--file",
        metavar="VOICE_AUDIO_FILE.wav",
        help="WAV file with audio to use for voice training.",
        type=str,
    )
    parser_train.add_argument(
        "-o",
        "--output",
        metavar="SPEAKER_DATA.json",
        help="location to save generated speaker JSON file to.",
        type=str,
    )
    parser_train.set_defaults(func=train_speaker)

    return parser


# Usage:
# - to train:
#     `uv run python/llamacpp_tts/src/main.py train \
#          -f python/llamacpp_tts/data/some_speaker.wav \
#          -o python/llamacpp_tts/speakers/some_speaker.json
# - to run tts:
#     `uv run python/llamacpp_tts/src/main.py tts -t "Hello! How do you do?"`
def main():
    """
    Main entry point of the script. Initialises an argument parser, parses user input,
    and runs either the 'train' or 'tts' function based on user input.
    """
    setup_logging()
    logger.info("local-ai-llm-playground/python/llamacpp-tts")

    # parser user input
    parser = initialise_parser()
    args = parser.parse_args()

    # run `train` or `tts`, based on parsed user input
    args.func(args)


if __name__ == "__main__":
    main()
