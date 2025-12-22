#!/usr/bin/env python3
# Helper script for building/installing all examples.

import argparse
import logging
import os
from pathlib import Path, PurePosixPath
from subprocess import run
import sys

PYTHON = 'python'
TOOLS_PATH = '../tools'
INSTALL_PATH = PurePosixPath('/ext/apps/Examples')
ALL_EXAMPLES = {"bluetooth", "dialog", "embedded-graphics", "example_images", "gpio", "gui", "hello-rust", "notification", "serial-echo", "storage"}


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument('-i', '--install', action='store_true', help='Copy built projects to device')
    parser.add_argument('-a', '--all', action='store_true', help='Build all examples')
    parser.add_argument('example', nargs='*', help='Examples to build')

    return parser.parse_args()


def main():
    args = parse_args()

    logging.basicConfig(level=logging.INFO)

    selected_examples = ALL_EXAMPLES if args.all else args.example

    for example in selected_examples:
        logging.info('Building %s', example)
        run(['cargo', 'build', '--package', 'flipperzero', '--example', example, '--all-features', '--release'], check=True)

        if args.install:
            # Assume that the binary has the name as the example
            binary = Path.cwd() / 'target' / 'thumbv7em-none-eabihf' / 'release' / 'examples' / example
            target = INSTALL_PATH / f'{example}.fap'

            logging.info('Copying %s to %s', binary, target)
            run(['cargo', 'run', '--release', '--bin', 'storage', '--', 'send', os.fspath(binary), os.fspath(target)], cwd=TOOLS_PATH, check=True)


if __name__ == '__main__':
    main()
