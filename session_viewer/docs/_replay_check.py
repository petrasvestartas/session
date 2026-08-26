"""Replay lesson markdown against a working tree.
Verbs: Find + (Replace with | Add below it | Add above it | Delete), plus Create `path`.
Both the Find and the argument may be a fenced block OR an inline `code span`."""
import re, sys, pathlib, shutil

FILE_RE = re.compile(r'`([\w/_.-]+\.(?:rs|wgsl|toml|html|json))`')
SPAN_RE = re.compile(r'`([^`]+)`')
CREATE_RE = re.compile(r'\*\*Create `([\w/_.-]+)`')

def spans(line, after):
    """Code spans on `line` occurring after index `after`, excluding file paths."""
    return [m.group(1) for m in SPAN_RE.finditer(line)
            if m.start() >= after and not FILE_RE.fullmatch(m.group(0))]

def fences(lines, start):
    """The run of consecutive fenced blocks at/after `start`, separated only by blank lines.
    A Find (or its argument) written as two blocks is ONE anchor with two parts."""
    j = start
    while j < len(lines) and not lines[j].startswith("```"):
        j += 1
    out = []
    while j < len(lines) and lines[j].startswith("```"):
        k = j + 1; body = []
        while k < len(lines) and not lines[k].startswith("```"):
            body.append(lines[k]); k += 1
        out.append("\n".join(body))
        j = k + 1
        while j < len(lines) and not lines[j].strip():
            j += 1
        if not (j < len(lines) and lines[j].startswith("```")):
            return out, k + 1
    return out, start + 1

def fence(lines, start):
    j = start
    while j < len(lines) and not lines[j].startswith("```"):
        j += 1
    if j >= len(lines):
        return None, start + 1
    k = j + 1; body = []
    while k < len(lines) and not lines[k].startswith("```"):
        body.append(lines[k]); k += 1
    return "\n".join(body), k

VERBS = [("**Replace the first line with", "replace_first"),
         ("**Replace with", "replace"),
         ("**Add below", "below"),
         ("**Add above", "above"),
         ("**Add** ", "after_field"),
         ("**Delete", "delete")]

def ops(doc):
    lines = pathlib.Path(doc).read_text().split("\n")
    cur = None; i = 0; out = []
    while i < len(lines):
        line = lines[i]
        c = CREATE_RE.search(line)
        if c:
            body, k = fence(lines, i)
            out.append(("create", c.group(1), [body], None, i + 1)); i = k; continue
        m = FILE_RE.search(line)
        if m: cur = m.group(1)
        if "**find" in line.lower():
            f = FILE_RE.search(line); tgt = f.group(1) if f else cur
            idx = line.lower().index("**find")
            # An inline Find only when the verb rides the SAME line
            # ("**Find** `x` - **add below it:**"); otherwise the anchor is the fenced block.
            # A verb reached before any fence means the anchor was written inline.
            verb_first = False
            for jj in range(i, len(lines)):
                if lines[jj].startswith("```"): break
                if jj > i and "**find" in lines[jj].lower(): break
                if any(t.lower() in lines[jj].lower() for t, _ in VERBS):
                    verb_first = True; break
            # The LAST span before the verb is the anchor: "**Find** in `Fn`'s parameter list
            # `        glyph_layout: ...`" names the function first and the anchor second.
            vpos = next((line.lower().index(t.lower()) for t, _ in VERBS if t.lower() in line.lower()), len(line))
            inline = [sp for sp in spans(line, idx) if line.index("`" + sp + "`") < vpos]
            if verb_first and inline:
                find, k = [inline[-1]], i + 1
            else:
                fl, k = fences(lines, i)
                find = fl
            verb, arg = None, None
            j = i  # the verb can sit on the Find line itself ("— **add below it:**")
            while j < len(lines):
                if j > i and "**find" in lines[j].lower(): break
                low = lines[j]
                pair = next(((tag, v) for tag, v in VERBS if tag.lower() in low.lower()), None)
                if pair:
                    tag, hit = pair
                    verb = hit
                    if hit == "delete":
                        arg = [""]
                    else:
                        # A fenced block on the next non-blank line wins over any span in the
                        # verb's own prose ("**Replace with** (the `point_buffer` NAME survives):").
                        nb = j + 1
                        while nb < len(lines) and not lines[nb].strip():
                            nb += 1
                        if nb < len(lines) and lines[nb].startswith("```"):
                            arg = fences(lines, nb)[0]
                        else:
                            pos = low.lower().index(tag.lower())
                            sp = spans(low, pos)
                            arg = [sp[0]] if sp else fences(lines, max(j, k))[0]
                    break
                j += 1
            out.append((verb or "?", tgt, find, arg, i + 1))
            i = max(k, j + 1) if verb in ("below", "above", "replace", "replace_first", "delete") else k
            continue
        i += 1
    return out

def apply(root, doc):
    fails = []
    for verb, tgt, a, b, ln in ops(doc):
        if verb == "create":
            body = a[0] if isinstance(a, list) and a else (a or "")
            p = root / tgt; p.parent.mkdir(parents=True, exist_ok=True); p.write_text(body + "\n"); continue
        if verb == "?" or tgt is None:
            fails.append((ln, tgt, "no verb recognised")); continue
        p = root / tgt
        if not p.exists():
            fails.append((ln, tgt, "file not in snapshot")); continue
        if verb == "after_field":
            fails.append((ln, tgt, "prose op: 'Add X after the Y field' (not a Find/Replace/Add pair)")); continue
        finds = a if isinstance(a, list) else [a]
        args = b if isinstance(b, list) else [b]
        if verb != "delete" and len(args) < len(finds):
            args = args + [args[-1] if args else ""] * (len(finds) - len(args))
        txt = p.read_text(); bad = False
        for idx_f, fa in enumerate(finds):
            n = txt.count(fa)
            if n != 1:
                fails.append((ln, tgt, f"Find block {idx_f+1}/{len(finds)} matches {n}x")); bad = True; break
            ar = "" if verb == "delete" else (args[idx_f] or "")
            if verb in ("replace", "delete"): txt = txt.replace(fa, ar)
            elif verb == "replace_first":
                first = fa.split("\n")[0]
                txt = txt.replace(fa, fa.replace(first, ar, 1))
            elif verb == "below":            txt = txt.replace(fa, fa + "\n" + ar)
            else:                            txt = txt.replace(fa, ar + "\n" + fa)
        if not bad:
            p.write_text(txt)
    return fails

if __name__ == "__main__":
    snap = pathlib.Path(sys.argv[1]); work = pathlib.Path(sys.argv[2]); docs = sys.argv[3:]
    if work.exists(): shutil.rmtree(work)
    shutil.copytree(snap, work)
    bad = 0
    for d in docs:
        f = apply(work, d); total = len(ops(d))
        print(f"{d}: {total} ops, {len(f)} failed")
        for ln, tgt, why in f[:14]:
            print(f"   !! doc line {ln:>4}  {tgt}  — {why}")
        bad += len(f)
    sys.exit(1 if bad else 0)
