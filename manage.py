#!/usr/bin/env python3

import sys
from manager import *

def process_args():
    binary_args_started = False
    args = set()
    binary_args = set()
    for arg in sys.argv:
        if binary_args_started:
            binary_args.add(arg)
        else:
            if arg == "--":
                binary_args_started = True
                continue
            args.add(arg)
    return args, binary_args

def main():
    FEATURE_SETS_CHECK = [
        [],
        ["default"],
        ["nnue_reader"],
        ["nnue_reader", "speed"],
        ["inbuilt_nnue"],
        ["inbuilt_nnue", "speed"],
        ["binary"],
        ["binary", "speed"],
        ["binary", "serde"],
        ["binary", "speed", "serde"],
    ]

    if sys.platform == "linux":
        home_dir = os.path.expanduser("~")
        possible_cargo_path = os.path.join(home_dir, ".cargo", "bin")
        sys.path.append(possible_cargo_path)

    if which("cargo") is None:
        print("Cargo not found. Please install Rust from https://www.rust-lang.org/tools/install")
        return

    args, binary_args = process_args()

    if "check" in args:
        if check_errors(FEATURE_SETS_CHECK):
            return

    if "test" in args:
        if test_package("--release" in args):
            return

    if "build" in args:
        if os.system(f"{RUST_FLAGS_STRING} cargo build --release"):
            return

    if "run" in args:
        run_package(os.path.dirname(__file__), args = args, binary_args = binary_args)

    if "backup" in args:
        backup_code("--noconfirm" in args)

    if "publish" in args:
        assert_publish_condition()
        if not check_errors(FEATURE_SETS_CHECK):
            if not test_package():
                backup_code("--noconfirm" in args)
                publish_package()

if __name__ == "__main__":
    main()