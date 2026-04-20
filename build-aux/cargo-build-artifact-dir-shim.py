import dataclasses
from typing import List
import os
import shutil
import subprocess
import sys


@dataclasses.dataclass
class Args:
    cargo: List[str]
    output: str
    artifact_dir: str


def parse_args(argv) -> Args:
    cargo = []
    it = iter(argv)
    for arg in it:
        if arg == "--output":
            output = next(it)
            continue
        if arg == "--artifact-dir":
            artifact_dir = next(it)
            continue
        cargo.append(arg)
    return Args(cargo, output, artifact_dir)


def main(args):
    args = parse_args(args)
    output_name = os.path.basename(args.output)

    subprocess.run(args.cargo, check=True)
    shutil.copy(args.output, os.path.join(args.artifact_dir, output_name))


if __name__ == "__main__":
    main(sys.argv[1:])
