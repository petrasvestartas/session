#!/usr/bin/env python3
"""Rewrite what a checkpoint still says in the words of the viewer's old `math` module.

On 19 Sep 2026 the viewer's `math.rs` (an f32 `Aabb`, raw `Mat4` helpers, the camera facts
read off a view-projection) moved into the kernel as `session_rust::AABB` and `Xform`. The
production edit was folded into the steps that own its lines; every older wording of those
lines in the earlier checkpoints is rewritten here, mechanically, so each checkpoint compiles
against the maintained kernel and the final one is still production byte for byte. The rules
are idempotent: production is their fixed point.

Usage:
    python3 port_math.py <file or directory>...
"""

import argparse
from pathlib import Path
import re

KERNEL = {"Aabb": "AABB", "Mat4": "Xform"}
CAMERA = {"FOVY_DEG"}
DROPPED = {
    "mat_mul",
    "mat_to_f32",
    "mat_scale",
    "mat_mul_f32",
    "xform_point",
    "xform_point_f64",
    "eye_from_view_proj",
    "ortho_half_height",
}
IMPORT = re.compile(r"^(\s*)use crate::math::(?:\{([^}]*)\}|(\w+));\n", re.M)
GROW = re.compile(r"\.grow\(([^()]*(?:\([^()]*\))?[^()]*)\)")
BOX_LITERAL = re.compile(
    r"(?:crate::math::|math::)?(?:Aabb|AABB)\s*\{\s*min:\s*\[([^\]]*)\],\s*max:\s*\[([^\]]*)\],?\s*\}",
    re.S,
)
PLACEMENT_OLD = """    match world.get(guid) {
        Some(local) => mat_mul(place, &local.m),
        None => *place,
    }"""
PLACEMENT_NEW = """    match world.get(guid) {
        Some(local) => place * local,
        None => place.clone(),
    }"""


def imports(text):
    """`use crate::math::…` becomes the kernel and camera imports the file now needs."""
    needed = set()

    def replace(match):
        names = [
            n.strip()
            for n in (match.group(2) or match.group(3)).split(",")
            if n.strip()
        ]
        lines = []
        for name in names:
            if name in KERNEL:
                needed.add(KERNEL[name])
            elif name in CAMERA:
                lines.append(f"{match.group(1)}use crate::camera::{name};\n")
            elif name in DROPPED:
                needed.add("Xform")
        return "".join(lines)

    text = IMPORT.sub(replace, text)
    return text, needed


def imported(text, name):
    return f"use session_rust::{name};" in text or re.search(r"use session_rust::\{[^}]*\b" + name + r"\b", text) is not None


def add_imports(text, names):
    """`use session_rust::<name>;` for every name the file uses and does not import."""
    for name in sorted(names):
        if imported(text, name):
            continue
        line = f"use session_rust::{name};\n"
        if "use session_rust::" in text:
            text = text.replace("use session_rust::", line + "use session_rust::", 1)
        else:
            text = line + text
    return text


def coordinates(argument):
    """`p` or `[a, b, c]` as the three f64 coordinates `union_with_point` takes."""
    argument = argument.strip()
    if argument.startswith("[") and argument.endswith("]"):
        parts = [part.strip() for part in argument[1:-1].split(",")]
        return ", ".join(f"{part} as f64" for part in parts)
    if argument.startswith("*"):
        argument = argument[1:]
    return f"{argument}[0] as f64, {argument}[1] as f64, {argument}[2] as f64"


def point(triple):
    repeated = re.match(r"^\s*(.+?);\s*3\s*$", triple)
    if repeated:
        value = re.sub(r"\s+as f32$", "", repeated.group(1).strip())
        return f"Point::new({value}, {value}, {value})"
    parts = [
        re.sub(r"\s+as f32$", "", part.strip())
        for part in triple.split(",")
        if part.strip()
    ]
    return "Point::new(" + ", ".join(parts) + ")"


GIZMO_IMPORTS = """use session_rust::intersection::{line_line_parameters, line_plane};
use session_rust::{Line, Plane, Point, Vector, Xform};
"""


def gizmo(text):
    """The gumball's own vector and matrix helpers become kernel operators."""
    text = re.sub(r"^use session_rust::\{Point, Vector\};\n", GIZMO_IMPORTS, text, flags=re.M)
    text = re.sub(r"^use session_rust::Xform;\n", "", text, flags=re.M)
    text = re.sub(r"\balong\(&([\w.]+), &([\w.()]+), ([^()]+(?:\([^()]*\))?)\)", r"&\1 + &(&\2 * (\3))", text)
    text = re.sub(r"\bsub\(&([\w.]+), &([\w.]+)\)", r"&\1 - &\2", text)
    text = re.sub(r"\bdot\(&([\w.]+), &([\w.()]+)\)", r"\1.dot(&\2)", text)
    text = re.sub(r"\blength\(&([\w.]+)\)", r"\1.magnitude()", text)
    return text


def port(text, name=""):
    text, needed = imports(text)
    if name.endswith("app/gizmo.rs"):
        text = gizmo(text)
    text = text.replace(".and_then(|m| Xform::from_matrix(m).inverse())", ".and_then(|place| place.inverse())")
    text = re.sub(r"Xform::from_matrix\(self\.scene\.placement_of\((\w+(?:\.\w+)*)\)\?\)", r"self.scene.placement_of(\1)?", text)
    text = re.sub(r"Xform::from_matrix\(self\.placement_of\((\w+)\)\?\)", r"self.placement_of(\1)?", text)
    if name.endswith("state/edit.rs"):
        text = text.replace("base_place: [f64; 16],", "base_place: Xform,")
        text = text.replace("Xform::from_matrix(active.base_place)", "active.base_place.clone()")
        text = text.replace("&Xform::from_matrix(delta)", "&delta")
        text = text.replace("Xform::from_matrix(place)", "place")
    text = BOX_LITERAL.sub(
        lambda m: (
            f"AABB::from_points(&[{point(m.group(1))}, {point(m.group(2))}], 0.0)"
        ),
        text,
    )
    text = text.replace(PLACEMENT_OLD, PLACEMENT_NEW)
    text = text.replace("crate::math::Aabb", "AABB").replace("math::Aabb", "AABB")
    text = text.replace("crate::math::Mat4", "Xform").replace(
        "crate::math::FOVY_DEG", "crate::camera::FOVY_DEG"
    )
    text = re.sub(
        r"(?:crate::math::)?mat_mul\(&([\w.]+?)(?:\.m)?, &([\w.]+?)(?:\.m)?\)",
        r"&\1 * &\2",
        text,
    )
    text = re.sub(r"(?:crate::math::)?mat_to_f32\(&([\w.]+)\)", r"\1.to_f32()", text)
    text = re.sub(r"\bAabb::empty\(\)", "AABB::empty()", text)
    text = re.sub(r"\bAabb\b", "AABB", text)
    text = re.sub(r"\bMat4\b", "Xform", text)
    text = re.sub(r"\.placed\(&([\w.]+?)(?:\.m)?\)", r".transformed(&\1)", text)
    text = re.sub(r"\.union\(&", ".union_with(&", text)
    text = GROW.sub(
        lambda m: (
            f".union_with_point({coordinates(m.group(1))})"
            if m.group(1).count(",") in (0, 2) and "ctx" not in m.group(1)
            else m.group(0)
        ),
        text,
    )
    text = re.sub(
        r"\b(bounds|world|box_|local|object|extent_box)\.is_finite\(\)",
        r"\1.is_valid()",
        text,
    )
    text = text.replace("b.is_finite().then_some(b)", "b.is_valid().then_some(b)")
    text = re.sub(r"\b(\w+)\.min\[(\w+)\] as f64", r"\1.min_point()[\2]", text)
    text = re.sub(r"\b(\w+)\.max\[(\w+)\] as f64", r"\1.max_point()[\2]", text)
    text = re.sub(r"f64::from\((\w+)\.min\[(\w+)\]\)", r"\1.min_point()[\2]", text)
    text = re.sub(r"f64::from\((\w+)\.max\[(\w+)\]\)", r"\1.max_point()[\2]", text)
    text = re.sub(
        r"f64::from\((\w+)\.min\[(\d)\] \+ \1\.max\[\2\]\) \* 0\.5",
        lambda m: f"{m.group(1)}.c{'xyz'[int(m.group(2))]}",
        text,
    )
    text = re.sub(r"f64::from\((\w+)\.diagonal\(\)\)", r"\1.diagonal()", text)
    text = re.sub(
        r"\((\w+)\.min\[(\d)\] \+ \1\.max\[\2\]\) as f64 \* 0\.5",
        lambda m: f"{m.group(1)}.c{'xyz'[int(m.group(2))]}",
        text,
    )
    text = re.sub(r"\b(\w+(?:\.\w+)*\.place)\[(1[234])\]", r"\1.m[\2]", text)
    text = re.sub(r"(?<![\w.])place\[(1[234])\]", r"place.m[\1]", text)
    text = text.replace("Xform::identity().m", "Xform::identity()")
    text = re.sub(r"ObjectRow::new\(([\w.]+)\.m,", r"ObjectRow::new(\1.clone(),", text)
    text = re.sub(r"&([\w.]+?)\.m\)", r"&\1)", text)
    text = re.sub(r"let (\w+) = ([\w.]+)\.place\.m;", r"let \1 = \2.place.clone();", text)
    text = re.sub(r"let (\w+) = place\.m;", r"let \1 = place.clone();", text)
    text = re.sub(r"^(pub )?mod math;\n", "", text, flags=re.M)
    code = re.sub(r'"(?:[^"\\]|\\.)*"', '""', re.sub(r"//.*", "", text))
    for name in ("Point", "AABB", "Xform"):
        if re.search(r"(?<![\w:])" + name + r"(?:::|\b(?!\s*\())", code):
            needed.add(name)
    return add_imports(text, needed)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("paths", nargs="+", type=Path)
    args = parser.parse_args()
    files = []
    for path in args.paths:
        files.extend(sorted(path.rglob("*.rs")) if path.is_dir() else [path])
    for path in files:
        text = path.read_text()
        ported = port(text, path.as_posix())
        if ported != text:
            path.write_text(ported)
            print(f"ported {path}")


if __name__ == "__main__":
    main()
