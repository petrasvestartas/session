#!/usr/bin/env python3
"""Build and run the existing shared CAD suites in Rust, C++ and Python (CPU only)."""
import argparse
import json
import os
from pathlib import Path
import re
import shlex
import subprocess
import sys


class Runner:
    """Record exact commands and logs, including a failed build, outside the repository."""

    def __init__(self, output):
        self.output = output
        self.commands = []

    def run(self, name, command, cwd, environment=None):
        """Run one dependency step and retain stdout/stderr before checking its status."""
        command = [str(value) for value in command]
        log_path = self.output / f"{name}.log"
        self.commands.append({"name": name, "argv": command, "cwd": str(cwd),
                              "environment_overrides": environment or {}, "log": str(log_path)})
        (self.output / "commands.json").write_text(json.dumps(self.commands, indent=2) + "\n")
        print(f"{name}: {log_path}", flush=True)
        with log_path.open("w") as log:
            result = subprocess.run(command, cwd=cwd, env=dict(os.environ, **(environment or {})),
                                    stdout=log, stderr=subprocess.STDOUT, check=False)
        if result.returncode:
            raise RuntimeError(f"{name} exited {result.returncode}; see {log_path}")
        return log_path.read_text()


def passed_count(text, language):
    """Reject missing summaries and test failures, even if a legacy runner returns zero."""
    matches = re.findall(rf"\[{language}-minitest\] (\d+)/(\d+) passed", text)
    if len(matches) != 1 or matches[0][0] != matches[0][1]:
        raise RuntimeError(f"invalid or failing {language} mini-test summary: {matches}")
    return int(matches[0][1])


def run_cpp(runner, source, build, jobs):
    """Reuse CMake's exact library link inputs with only the three requested test objects.

    The legacy C++ registry has no public filter. A generated main runs the existing
    registry; excluding unrelated registration objects selects the suites without
    changing assertions or production APIs. Recompile only its reporting implementation
    with a __FILE__ prefix map so its JSON reports land under the scratch output.
    """
    runner.run("cpp-configure", ["cmake", "-S", source, "-B", build, "-G", "Unix Makefiles",
                                "-DSESSION_REGEN_PROTO=OFF"], source)
    runner.run("cpp-build", ["cmake", "--build", build, "--target", "point_minitest",
                            "--parallel", str(jobs)], source)
    target = build / "CMakeFiles/point_minitest.dir"
    original_link = shlex.split((target / "link.txt").read_text())
    compiler = original_link[0]
    runner.run("cpp-version", [compiler, "--version"], build)
    flags = []
    for line in (target / "flags.make").read_text().splitlines():
        if line.startswith(("CXX_DEFINES =", "CXX_INCLUDES =", "CXX_FLAGS =")):
            flags.extend(shlex.split(line.split("=", 1)[1]))
    main = runner.output / "cad_main.cpp"
    main.write_text('#include "mini_test.h"\nint main() { return session_cpp::mini_test::run_all("cpp"); }\n')
    main_object = runner.output / "cad_main.o"
    runner.run("cpp-main", [compiler, *flags, "-c", main, "-o", main_object], build)
    registry_source = source / "src/mini_test.cpp"
    registry_object = runner.output / "cad_registry.o"
    mapped_source = runner.output / "cpp/session_cpp/src/mini_test.cpp"
    runner.run("cpp-registry", [compiler, *flags,
               f"-ffile-prefix-map={registry_source}={mapped_source}",
               "-c", registry_source, "-o", registry_object], build)
    selected = {"remesh_nurbssurface_grid_test.cpp.o", "nurbssurface_trimmed_test.cpp.o",
                "brep_test.cpp.o"}
    found = set()
    link = []
    for arg in original_link:
        if arg.startswith("-Wl,--dependency-file="):
            continue
        if arg.startswith("CMakeFiles/point_minitest.dir/") and arg.endswith(".o"):
            if Path(arg).name not in selected:
                continue
            found.add(Path(arg).name)
        link.append(arg)
    if found != selected:
        raise RuntimeError(f"CMake test object layout changed: expected {selected}, found {found}")
    binary = runner.output / "cpp-cad-tests"
    link[link.index("-o") + 1] = str(binary)
    link[1:1] = [str(main_object), str(registry_object)]
    runner.run("cpp-link", link, build)
    return passed_count(runner.run("cpp-tests", [binary], runner.output), "cpp")


def run_python(runner, source):
    """Import each original module with only its report destination redirected to scratch."""
    script = runner.output / "python_cad.py"
    script.write_text('''from pathlib import Path
import runpy
import sys
import session_py.mini_test as mini_test

def output_path(language, test_file):
    """Keep the existing runner's report serialization outside shared sources."""
    return Path(sys.argv[2]) / (test_file.stem + ".json")

mini_test._default_output_path = output_path
runpy.run_module("session_py." + sys.argv[1] + "_test", run_name="__main__")
''')
    report = runner.output / "python"
    report.mkdir(exist_ok=True)
    total = 0
    for module in ["remesh_nurbssurface_grid", "nurbssurface_trimmed", "brep"]:
        text = runner.run(f"python-{module}", [sys.executable, script, module, report], runner.output,
                          {"PYTHONPATH": str(source / "src")})
        total += passed_count(text, "py")
    return total


def main():
    """Rebuild the selected suites and record versions, commands, reports and test counts."""
    viewer = Path(__file__).resolve().parents[1]
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=Path("/tmp/session-viewer-parity"))
    parser.add_argument("--cpp-build", type=Path, default=viewer.parent / "session_cpp/build")
    parser.add_argument("--jobs", type=int, default=4)
    parser.add_argument("--rust-target", default="x86_64-unknown-linux-gnu")
    args = parser.parse_args()
    output = args.output.resolve()
    if output.is_relative_to(viewer.parent):
        parser.error("--output must be outside the shared Session source tree")
    output.mkdir(parents=True, exist_ok=True)
    runner = Runner(output)
    versions = {"python": sys.version,
                "rust": runner.run("rust-version", ["rustc", "--version"], output).strip(),
                "cmake": runner.run("cmake-version", ["cmake", "--version"], output).splitlines()[0]}
    rust = runner.run("rust-tests", ["cargo", "run", "--manifest-path", viewer / "Cargo.toml",
                     "--locked", "--target", args.rust_target, "--example", "check_shared_geometry"],
                      output, {"REGEN_PROTO": "0"})
    counts = {"rust": passed_count(rust, "rust"),
              "cpp": run_cpp(runner, viewer.parent / "session_cpp", args.cpp_build.resolve(), args.jobs),
              "python": run_python(runner, viewer.parent / "session_py")}
    versions["cpp"] = (output / "cpp-version.log").read_text().splitlines()[0]
    expected = {"rust": 53, "cpp": 61, "python": 53}
    summary = {"counts": counts, "expected": expected, "versions": versions,
               "groups": ["RemeshNurbsSurfaceGrid", "NurbsSurfaceTrimmed", "BRep"],
               "scope": "Existing suites; C++ includes eight additional preexisting trimmed-surface tests.",
               "passed": counts == expected}
    (output / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
    print(json.dumps(summary, indent=2))
    if counts != expected:
        raise RuntimeError("suite counts changed; inspect the registries before updating expectations")


if __name__ == "__main__":
    main()
