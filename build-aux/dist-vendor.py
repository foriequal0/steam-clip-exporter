import os
import subprocess
import sys


def main(args):
    source_root = os.environ["MESON_PROJECT_SOURCE_ROOT"]
    build_root = os.environ["MESON_PROJECT_BUILD_ROOT"]
    dist_root = os.environ["MESON_PROJECT_DIST_ROOT"]

    output = subprocess.run(
        ["cargo", "vendor",
         os.path.join(dist_root, "vendor"),
         *args],
        check=True, stdout=subprocess.PIPE)
    if output.returncode != 0:
        sys.exit(output.returncode)

    try:
        os.mkdir(os.path.join(dist_root, ".cargo"))
    except FileExistsError:
        pass

    with open(os.path.join(dist_root, ".cargo/config.toml"), "wb") as config:
        config.write(output.stdout)


if __name__ == "__main__":
    main(sys.argv[1:])
