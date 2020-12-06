import datetime
import pathlib
import subprocess

import requests
import toml


MAIN = """fn main() {{
    let (part1, part2) = {crate}::solve();
    println!("{{}}", part1);
    println!("{{}}", part2);
}}"""

LIB = """#[inline]
pub fn solve() -> (T, T) {{
    unimplemented!()
}}"""


def main() -> None:
    now = datetime.datetime.now()
    day = now.day
    year = now.year

    crate = f"day{day:02}"
    crate_path = pathlib.Path(crate)

    if crate_path.exists():
        print(f"{crate} already exists.")
        return

    with open("Cargo.toml") as manifest_f:
        manifest = toml.load(manifest_f)

    manifest["workspace"]["members"].append(crate)

    with open("Cargo.toml", "w") as manifest_f:
        toml.dump(manifest, manifest_f)

    subprocess.run(["cargo", "new", "--bin", crate], check=True)

    src = crate_path / "src"

    with (src / "main.rs").open("w") as main:
        main.write(MAIN.format(crate=crate))

    with (src / "lib.rs").open("w") as lib:
        lib.write(LIB.format(crate=crate))

    with open("session.txt") as session_f:
        session = session_f.read().strip()

    with (src / "input.txt").open("w", newline="\n") as input:
        resp = requests.get(
            f"https://adventofcode.com/{year}/day/{day}/input",
            cookies={"session": session},
        )
        resp.raise_for_status()
        input.write(resp.text)


if __name__ == "__main__":
    main()
