"""Replay lesson markdown against a working tree.
Verbs: Find + (Replace with | Add below it | Add above it | Delete), plus Create `path`.
Both the Find and the argument may be a fenced block OR an inline `code span`.
Region verbs (lessons 40a+): every part is an inline `span` on ONE line, or the same parts as
consecutive fenced blocks right below it, in order:
  **Move** `file A` `first line` **through** `last line` **to** `file B` **after** `anchor line`
  **Move** ... **to** `file B` **at the end**   (or **at the start**)
  **Remove** `file A` `first line` **through** `last line`
  **Replace-all** `file` `old` → `new` (N hits)          whole-word, count asserted
`first line` must be unique in A; `last line` is the first match AFTER it (a closing `}` is fine);
the region is whole lines, cut with its trailing newline and pasted after `anchor line` (unique in B).

Usage:
  python3 docs/_replay_check.py <snap> <work> <doc.md>...            replay, report failed ops
  python3 docs/_replay_check.py --moves <snap> <work> <doc.md>...    replay + prove every **Move**
                                                                    moved its lines BYTE-IDENTICALLY
  python3 docs/_replay_check.py --stale <tree> docs/*.md            enumerate every op against a
                                                                    tree: does the target exist and
                                                                    does each Find match exactly once
  python3 docs/_replay_check.py --render docs/*.md                  fence parity + duplicate Create
                                                                    bodies — what a broken page looks like
  python3 docs/_replay_check.py --audit [--max N] docs/*.md         every fenced code block that NO
                                                                    op reached — the code a replay
                                                                    silently never types
"""
import re, sys, pathlib, shutil, collections

FILE_RE = re.compile(r'`([\w/_.-]+\.(?:rs|wgsl|toml|html|json))`')
SPAN_RE = re.compile(r'`([^`]+)`')
CREATE_RE = re.compile(r'\*\*Create `([\w/_.-]+)`')
# The counterpart of Create. Requires an EXTENSION so it can never be confused with the
# content-level `**Delete**` that follows a Find, or with a `**Delete** `some_expr`` span.
DELETE_FILE_RE = re.compile(r'\*\*Delete `([\w/_.-]+\.(?:rs|wgsl|toml|json|md))`\*\*')

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
        d = DELETE_FILE_RE.search(line)
        if d:
            out.append(("delete_file", d.group(1), None, None, i + 1)); i += 1; continue
        c = CREATE_RE.search(line)
        if c:
            body, k = fence(lines, i)
            out.append(("create", c.group(1), [body], None, i + 1)); i = k; continue
        r = region_op(lines, i)
        if r:
            out.append(r[0]); i = r[1]; continue
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
                        # A fenced block belonging to this verb wins over any span in the verb's
                        # own prose ("**Replace with** (the `point_buffer` NAME survives):").
                        # Scan PAST a multi-line explanation, not just one blank line: lesson 36's
                        # "**Add below it** - `push_cloud` writes STRAIGHT into..." runs six lines
                        # before its fence, and taking the span wrote the literal token
                        # `push_cloud` into scene.rs where 58 lines of function belonged - an op
                        # that REPORTED SUCCESS. Stop at the next op so an unrelated later fence
                        # is never stolen.
                        nb = j + 1
                        while nb < len(lines):
                            l = lines[nb]
                            if l.startswith("```"): break
                            if l.startswith("#") or "**find" in l.lower(): nb = -1; break
                            if l.startswith("**") and nb > j: nb = -1; break
                            nb += 1
                        if 0 <= nb < len(lines) and lines[nb].startswith("```"):
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


HITS_RE = re.compile(r"\((\d+) hits?\)")
CONT = ("**through**", "**to**", "**after**", "**at the end**", "**at the start**", "**up to**")

def _window(lines, i):
    """Lines i..j of one region verb: until the next top-level verb/heading (continuation
    keywords **through**/**to**/**after**/**at the end**/**at the start** belong to this op)."""
    j = i + 1
    while j < len(lines):
        l = lines[j]
        if l.startswith("#") or (l.startswith("**") and not l.lower().startswith(CONT)):
            break
        j += 1
    return j

def _parts(lines, i, j):
    """(files, code parts in order) over the window: inline spans and fenced blocks alike."""
    files, parts, k = [], [], i
    while k < j:
        l = lines[k]
        if l.startswith("```"):
            body, k = fence(lines, k); parts.append(body); k += 1; continue
        files += [m.group(1) for m in FILE_RE.finditer(l)]
        parts += [m.group(1) for m in SPAN_RE.finditer(l) if not FILE_RE.fullmatch(m.group(0))]
        k += 1
    return files, parts

def region_op(lines, i):
    low = lines[i].lower()
    if not (low.startswith("**move**") or low.startswith("**remove**") or low.startswith("**append**")
            or low.startswith("**replace-all**")):
        return None
    j = _window(lines, i)
    files, parts = _parts(lines, i, j)
    text = "\n".join(lines[i:j]).lower()
    bad = (("?", files[0] if files else None, None, None, i + 1), j)
    if low.startswith("**move**"):
        where = "end" if "**at the end**" in text else ("start" if "**at the start**" in text else None)
        n = 2 if where else 3
        if len(files) < 2 or len(parts) < n: return bad
        # `**up to**` is exclusive here exactly as it is for Remove: a body is named by the thing
        # that follows it, and moving a method's body must not take the method's closing brace.
        return (("move", files[0], [parts[0], parts[1], "**up to**" in text],
                 [files[1], where or parts[2]], i + 1), j)
    if low.startswith("**append**"):
        # `**Create` covers a new file and `**Add below` needs an anchor; adding a whole new item
        # at the END of an existing file had no verb at all, so lesson 36's resolve pipeline sat
        # in the doc as unreachable prose.
        if not files or not parts: return bad
        return (("append", files[0], [parts[0]], None, i + 1), j)
    if low.startswith("**remove**"):
        if not files or len(parts) < 2: return bad
        # "**up to**" makes the second anchor EXCLUSIVE. Deleting a whole function needs it:
        # its own closing brace is not a unique line, but the next item's first line is.
        return (("remove", files[0], [parts[0], parts[1], "**up to**" in text], None, i + 1), j)
    h = HITS_RE.search(text)
    if not files or len(parts) < 2 or not h: return bad
    return (("replace_all", files[0], [parts[0]], [parts[1], int(h.group(1))], i + 1), j)

def cut_region(txt, first, last, exclusive=False):
    """Whole-line exact matches: `first` unique, `last` = first line == last at/after it.
    `exclusive` stops one line SHORT of `last`, which is how a whole function is named: by the
    thing that follows it. Returns (remaining text, region) or (None, why)."""
    ls = txt.split("\n")
    hits = [k for k, l in enumerate(ls) if l == first]
    if len(hits) != 1: return None, f"first line matches {len(hits)}x (whole-line, exact)"
    a = hits[0]
    b = next((k for k in range(a, len(ls)) if ls[k] == last), None)
    if b is None: return None, "last line not found at/after the first (whole-line, exact)"
    if exclusive:
        if b <= a: return None, "the exclusive end line sits at or before the first line"
        b -= 1
    return "\n".join(ls[:a] + ls[b + 1:]), "\n".join(ls[a:b + 1]) + "\n"

def paste_region(txt, region, where):
    if where == "end":
        return (txt if txt.endswith("\n") else txt + "\n") + "\n" + region, None
    if where == "start":
        return region + "\n" + txt, None
    ls = txt.split("\n")
    hits = [k for k, l in enumerate(ls) if l == where]
    if len(hits) != 1: return None, f"anchor matches {len(hits)}x (whole-line, exact)"
    k = hits[0] + 1
    return "\n".join(ls[:k]) + "\n\n" + region + "\n".join(ls[k:]), None

def replace_all_words(txt, old, new):
    pat = re.compile(r"(?<![A-Za-z0-9_])" + re.escape(old) + r"(?![A-Za-z0-9_])")
    return pat.subn(new, txt)

# A lesson may edit the KERNEL as well as the viewer (lesson 36 adds `PointCloud::normals`).
# Those paths live outside the staged viewer tree, so a replay cannot apply them - and must not
# report them as broken anchors either, or the gate carries a permanent false failure that
# trains you to ignore it. They are counted and named instead.
OUT_OF_TREE = ("session_rust/", "session_cpp/", "session_py/", "session_proto/")

def apply(root, doc):
    fails, skipped = [], []
    for verb, tgt, a, b, ln in ops(doc):
        if tgt and tgt.startswith(OUT_OF_TREE):
            skipped.append((ln, tgt)); continue
        if verb == "append":
            p = root / tgt
            if not p.exists(): fails.append((ln, tgt, "file not in snapshot")); continue
            old = p.read_text()
            p.write_text(old + ("" if old.endswith("\n") else "\n") + a[0] + "\n"); continue
        if verb == "delete_file":
            p = root / tgt
            if not p.exists(): fails.append((ln, tgt, "file not in snapshot")); continue
            p.unlink(); continue
        if verb == "create":
            body = a[0] if isinstance(a, list) and a else (a or "")
            p = root / tgt; p.parent.mkdir(parents=True, exist_ok=True); p.write_text(body + "\n"); continue
        if verb == "?" or tgt is None:
            fails.append((ln, tgt, "no verb recognised")); continue
        if verb in ("move", "remove"):
            p = root / tgt
            if not p.exists(): fails.append((ln, tgt, "file not in snapshot")); continue
            rest, region = cut_region(p.read_text(), a[0], a[1], len(a) > 2 and bool(a[2]))
            if rest is None: fails.append((ln, tgt, f"Move/Remove: {region}")); continue
            if verb == "move":
                q = root / b[0]
                if not q.exists():
                    q.parent.mkdir(parents=True, exist_ok=True); q.write_text("")
                new, why = paste_region(q.read_text(), region.strip("\n") + "\n", b[1])
                if new is None: fails.append((ln, b[0], f"Move: {why}")); continue
                q.write_text(new)
            p.write_text(rest)
            continue
        if verb == "replace_all":
            p = root / tgt
            if not p.exists(): fails.append((ln, tgt, "file not in snapshot")); continue
            new, n = replace_all_words(p.read_text(), a[0], b[0])
            if n != b[1]: fails.append((ln, tgt, f"Replace-all: {n} hits, doc says {b[1]}")); continue
            p.write_text(new)
            continue
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
            # An INLINE replacement ("**Replace with** `scene.add_file(..)`") is written in prose,
            # where leading whitespace cannot survive. Take the indentation from the line being
            # replaced. Without this the op silently un-indents its line and every later lesson
            # that anchors on it misses.
            if verb in ("replace", "replace_first") and "\n" not in ar and ar[:1] not in ("", " ", "\t"):
                head = fa.split("\n")[0]
                pad = head[:len(head) - len(head.lstrip())]
                if pad:
                    ar = pad + ar
            if verb in ("replace", "delete"): txt = txt.replace(fa, ar)
            elif verb == "replace_first":
                first = fa.split("\n")[0]
                txt = txt.replace(fa, fa.replace(first, ar, 1))
            elif verb == "below":            txt = txt.replace(fa, fa + "\n" + ar)
            else:                            txt = txt.replace(fa, ar + "\n" + fa)
        if not bad:
            p.write_text(txt)
    return fails, skipped

# ----------------------------------------------------------------- --moves
# A **Move** is the one op the compiler and the pixel goldens cannot police: a line dropped
# inside a `#[cfg(...)]` arm compiles on the default target and renders the same frame. So
# compare the MULTISET of stripped, non-blank lines over {source} u {destinations} BEFORE the
# doc against the same set AFTER it. Destinations are counted on both sides, so moving into a
# file that already exists is not read as a gain.

def _bag(p):
    """Sorted multiset of stripped, non-blank lines; a missing file is empty."""
    if not p.exists():
        return []
    return [l.strip() for l in p.read_text().splitlines() if l.strip()]

def _lines(blocks):
    out = []
    for b in blocks or []:
        if b:
            out += [l.strip() for l in b.split("\n") if l.strip()]
    return out

def move_map(oplist):
    """source file -> the set of files its **Move** ops send lines to."""
    m = {}
    for verb, tgt, a, b, ln in oplist:
        if verb == "move" and tgt and b:
            m.setdefault(tgt, set()).add(b[0])
    return m

def declared_new_lines(oplist):
    """Every line the doc says it ADDS: Create bodies and the arguments of Replace/Add."""
    out = []
    for verb, tgt, a, b, ln in oplist:
        if verb == "create":                                   out += _lines(a)
        elif verb in ("replace", "replace_first", "below", "above"): out += _lines(b)
    return out

def declared_removed_lines(oplist):
    """Every line the doc SPELLS OUT as dropped: Delete/Replace anchors. **Remove** is NOT here —
    it names two anchors and never its content, so everything it eats stays undeclared on purpose."""
    out = []
    for verb, tgt, a, b, ln in oplist:
        if verb in ("replace", "delete"):
            out += _lines(a)
        elif verb == "replace_first":
            out += [f.split("\n")[0].strip() for f in (a or []) if f and f.strip()]
    return out

def declared_subs(oplist):
    return [(a[0], b[0]) for verb, tgt, a, b, ln in oplist
            if verb == "replace_all" and a and b]

def moves(snap, work, doc):
    """[(src, files, kind, lines)] — kind is LOST, GAINED or (informational) LOST-declared.
    No LOST/GAINED rows means every **Move** in the doc moved its lines byte-identically."""
    oplist = ops(doc)
    subs = declared_subs(oplist)
    new, gone = collections.Counter(declared_new_lines(oplist)), collections.Counter(declared_removed_lines(oplist))
    fails = []
    for src, dsts in sorted(move_map(oplist).items()):
        files = [src] + sorted(dsts)
        b = collections.Counter(l for f in files for l in _bag(snap / f))
        a = collections.Counter(l for f in files for l in _bag(work / f))
        lost, gained = b - a, a - b
        for l in list(lost.elements()):           # a declared Replace-all rename is not a loss:
            n = l                                 # cancel the old spelling against the new one
            for old, nw in subs:
                n = replace_all_words(n, old, nw)[0]
            if n != l and gained[n] > 0:
                gained[n] -= 1; lost[l] -= 1
        lost, gained = +lost, (+gained) - new
        told = lost & gone                       # a Delete/Replace op spelled these out
        lost -= told
        if lost:   fails.append((src, files, "LOST", sorted(lost.elements())))
        if gained: fails.append((src, files, "UNDECLARED", sorted(gained.elements())))
        if told:   fails.append((src, files, "lost-declared", sorted(told.elements())))
    return fails

# ----------------------------------------------------------------- --stale
# Read-only enumeration of every op against one tree. Ops that depend on an earlier op in the
# same doc read as STALE here by construction; this is the re-anchor worklist, not a verdict
# on the doc.

def stale(tree, docs):
    bad = 0
    for d in docs:
        lesson = pathlib.Path(d).stem.split("-")[0]
        for verb, tgt, a, b, ln in ops(d):
            if verb == "?" or tgt is None:
                v = "STALE: no verb recognised"
            elif verb == "create":
                v = "EXISTS ALREADY" if (tree / tgt).exists() else "n/a (creates it)"
            elif not (tree / tgt).exists():
                v = "STALE: no such file"
            else:
                txt = (tree / tgt).read_text()
                if verb in ("move", "remove"):
                    rest, why = cut_region(txt, a[0], a[1])
                    v = "ok" if rest is not None else f"STALE: {why}"
                    if rest is not None and verb == "move" and b and b[1] not in ("end", "start"):
                        q = tree / b[0]
                        n = q.read_text().split("\n").count(b[1]) if q.exists() else 0
                        if n != 1: v = f"STALE: anchor in {b[0]} matches {n}x"
                elif verb == "replace_all":
                    n = replace_all_words(txt, a[0], b[0])[1]
                    v = "ok" if n == b[1] else f"STALE: {n} hits, doc says {b[1]}"
                elif verb == "after_field":
                    v = "STALE: prose op, no Find anchor"
                else:
                    miss = [f"{i+1}/{len(a)} matches {txt.count(f)}x"
                            for i, f in enumerate(a or []) if f is None or txt.count(f) != 1]
                    v = "ok" if (a and not miss) else "STALE: find " + ", ".join(miss or ["is empty"])
            bad += v.startswith("STALE")
            print(f"{lesson} · {ln} · {verb} · {tgt} · {v}")
    print(f"# {bad} stale op(s)")
    return bad

# ----------------------------------------------------------------- --audit
# The replay only ever touches a fenced block that a PARSED op reached. A lesson whose verb was
# written as prose ("in `gpu/mod.rs`, add this to the struct:"), or whose verb hides inside a bold
# span that does not START with it ("**1a. Find the `Instance` struct**"), parses to nothing — the
# block is never applied and the run still prints "0 failed". Lesson 43 delivered 27% of its code
# that way. --audit closes the hole from the other side: enumerate EVERY fenced block, subtract the
# ones an op claimed, and report the remainder.
#
# HEURISTIC, in one paragraph. A block is CODE if its language tag is a source language, or it is
# untagged and reads like source and not like a shell transcript. A code block is CLAIMED if its
# exact body text appears among the Find anchors / arguments / Create bodies / region parts that
# `ops()` produced for that doc. An unclaimed code block is ILLUSTRATIVE (the lesson is quoting,
# not dictating) when it sits under a heading that OPENS with an explanatory word — Goal, Why,
# Design, Mental model, How it works, Recap, Next, Expected state, Files we touch, Troubleshoot —
# or when the last prose line above it carries a quote cue ("looks like", "currently reads",
# "for reference", "you should see", "is unchanged", "it prints", "the error"). A bold verb on that
# same line always wins over a quote cue, because "**1b. `SceneTables` becomes `ArenaUpload`.**
# Find:" is an instruction that happens to contain the word "becomes". Everything else is ORPHANED.
#
# LIMITS, measured over the 128 docs in this directory:
#  * It only sees FENCED blocks. A prose op whose payload is an inline `span` is invisible here —
#    lesson 43 had 3 such sites on top of the 13 fenced ones this mode reports.
#  * Claiming is by exact body text, so two identical blocks are both claimed when one op took one
#    of them, and a one-line block is claimed by an inline span of the same text. Both err toward
#    silence, never toward a false alarm.
#  * A block claimed as an op's ARGUMENT is counted even when the op itself is broken; the normal
#    replay is what reports those.
#  * Measured false-alarm rate on a 12-block hand-check of the best docs: 1 in 12 (37-cloud-memory
#    line 83, a two-line quote of `append_rows` introduced by prose with no cue word). Blocks that
#    illustrate mid-Step, with no cue and under a Step heading, are the class that leaks through.
#  * Unknown language tags (anything outside the two sets below) are treated as prose and never
#    reported. Add the tag to SOURCE_LANGS when a lesson starts using one.

SOURCE_LANGS = {"rust", "rs", "wgsl", "toml", "html", "json", "python", "py", "cpp", "proto", "css"}
PROSE_LANGS  = {"bash", "sh", "shell", "console", "text", "txt", "output", "log", "diff",
                "md", "markdown", "yaml", "yml", "csv", "tsv", "ini"}
# An UNTAGGED block counts as code only if it reads like source and not like a shell transcript.
CODE_RE  = re.compile(r"(^|\n)\s*(pub |fn |let |use |impl |struct |enum |const |#\[|//|@group|@binding)")
SHELL_RE = re.compile(r"(^|\n)\s*(\$ |# |cargo |trunk |python3? |\./|git |ls |cd |RUST|VIEWER_)")

ILLUS_RE = re.compile(r"(looks? like|read like|reads like|for reference|as a reminder|"
                      r"you (should )?see|it prints|prints:|the error|error message|for context|"
                      r"currently reads|today reads|shipped as|quoted here|is (currently|today)|"
                      r"unchanged|excerpt)", re.I)
INSTR_RE = re.compile(r"\*\*(find|replace|add|delete|create|move|remove|insert|new|then|change)", re.I)
# Whole sections that hold explanation, transcripts or listings — never something to type. The
# keyword must OPEN the heading, so "## Why the lane is compute" counts and
# "## Step 3 — why the cache is keyed on the guid" does not.
QUOTE_HEAD_RE = re.compile(r"^#+\s+(the\s+)?(expected state|recap|next|what you should see|"
                           r"troubleshoot|compare to the archive|files we touch|mental model|"
                           r"how it works|goal|design|why)\b", re.I)

def blocks(doc):
    """Every fenced block: (1-based line of its opening fence, lang, body). Same fence convention
    as fence()/fences(): any line starting with ``` opens or closes."""
    lines = pathlib.Path(doc).read_text().split("\n")
    out, i = [], 0
    while i < len(lines):
        if lines[i].startswith("```"):
            lang = lines[i][3:].strip().lower()
            k, body = i + 1, []
            while k < len(lines) and not lines[k].startswith("```"):
                body.append(lines[k]); k += 1
            out.append((i + 1, lang, "\n".join(body)))
            i = k + 1
        else:
            i += 1
    return out, lines

def claimed(oplist):
    """Every block body an op took hold of — Find anchor, argument, Create body, region part."""
    out = set()
    for verb, tgt, a, b, ln in oplist:
        for part in list(a or []) + list(b or []):
            if isinstance(part, str):
                out.add(part)
    return out

def is_code(lang, body):
    if lang in SOURCE_LANGS: return True
    if lang in PROSE_LANGS:  return False
    if lang:                 return False          # unknown tag: prose (see LIMITS)
    return bool(CODE_RE.search(body)) and not SHELL_RE.search(body)

def context(lines, ln):
    """(last non-blank prose line above the fence at `ln`, nearest heading at/above it)."""
    i = ln - 2
    while i >= 0 and not lines[i].strip():
        i -= 1
    prev = lines[i] if i >= 0 else ""
    h = ""
    for k in range(ln - 1, -1, -1):
        if lines[k].startswith("#"):
            h = lines[k]; break
    return prev, h

def preamble(lines, ln):
    """The whole prose paragraph above the fence at `ln`: back to the previous fence or heading."""
    out, i = [], ln - 2
    while i >= 0 and not lines[i].startswith("```") and not lines[i].startswith("#"):
        out.append(lines[i]); i -= 1
    return "\n".join(reversed(out))

def illustrative(prev, head):
    if QUOTE_HEAD_RE.search(head): return True
    if INSTR_RE.search(prev):      return False
    return bool(ILLUS_RE.search(prev))

VERB_WORD_RE = re.compile(r"\b(find|replace|add|delete|create|insert|move|remove)\b", re.I)
BOLD_RE = re.compile(r"\*\*(.+?)\*\*", re.S)

def why_orphan(pre):
    """Best guess at WHY no op reached the block — the repair differs per class."""
    for m in BOLD_RE.finditer(pre):
        v = VERB_WORD_RE.search(m.group(1))
        if v and v.start() > 0:
            return "verb-not-at-bold-start"   # "**1a. Find the X**" — the parser needs "**Find"
    if VERB_WORD_RE.search(pre):
        return "verb-unbolded"                # "In `f`, find:" / "**add:**" — no parsable verb
    return "no-verb"                          # pure prose, or a payload with no instruction at all

def all_lesson_docs():
    """Every lesson doc, for a mode invoked without an explicit list.

    Handed no documents, these modes used to iterate an empty list and print a summary that
    reads exactly like a pass - `--audit` and `--links` both reported clean while checking
    nothing at all. A verification tool that answers "fine" to a question it never asked is
    worse than one that errors, so an omitted list now means ALL of them.
    """
    here = pathlib.Path(__file__).resolve().parent
    return [str(f) for f in sorted(here.glob("[0-9]*-*.md"))]


def audit(docs, threshold):
    """Exit-code rule: only a doc that parses at least ONE op can fail. A doc with zero parsed ops
    is a full-listing lesson (everything before 34c) or one whose verbs are all invisible — the
    replay already prints a visibly vacuous "0 ops, 0 failed" for it, so it is reported and tagged,
    never counted. Default threshold 0: the five docs authored in the current verb style
    (38, 39, 40, 42, 44) all score exactly 0 orphans, so 0 is a reachable bar, not an aspiration."""
    bad = noops = 0
    for d in docs:
        oplist = ops(d)
        bs, lines = blocks(d)
        have = claimed(oplist)
        code = [(ln, lg, body) for ln, lg, body in bs if is_code(lg, body)]
        orph = []
        for ln, lg, body in code:
            if body in have: continue
            prev, head = context(lines, ln)
            if illustrative(prev, head): continue
            orph.append((ln, lg, body, why_orphan(preamble(lines, ln))))
        nl = sum(len(b.split("\n")) for _, _, b, _ in orph)
        if not oplist: noops += 1
        tag = "" if oplist else "   [NO OPS PARSED — informational]"
        print(f"{d}: {len(code)} fenced code blocks, {len(code) - len(orph)} claimed by ops, "
              f"{len(orph)} orphaned ({nl} lines){tag}")
        for ln, lg, body, why in orph:
            first = next((l for l in body.split("\n") if l.strip()), "")
            print(f"   ?? {ln:>4} · {lg or 'untagged':<8} · {len(body.split(chr(10))):>3} lines · "
                  f"{first.strip()[:66]} · {why}")
        if oplist and len(orph) > threshold:
            bad += 1
    print(f"# {bad} doc(s) with parsed ops over the orphan threshold ({threshold}), "
          f"{len(docs)} audited, {noops} with NO ops parsed")
    return bad

HEAVY = ("assets", "target", ".git", "dist", "node_modules", "__pycache__")

def links(docs):
    """Every cross-reference resolves, and every number in one names a lesson that exists.

    Inserting a lesson renumbers every file after it. The rename pass sees `](45-foo.md)` and the
    `[45]` label in front of it; it does NOT see `lesson 45` written in prose, or inside a code
    comment the lesson tells you to type. Those silently keep pointing one lesson short, and no
    other mode here looks at them - the replay is happy either way, because a number in prose is
    not an anchor. Inserting lesson 38 left 84 of them stale across 22 files.
    """
    import collections
    here = pathlib.Path(docs[0]).parent if docs else pathlib.Path("docs")
    have = {}
    for f in here.glob("*.md"):
        m = re.match(r"^(\d+[a-z]?)-", f.name)
        if m: have[m.group(1)] = f.name
    # `34` is a real reference when the lessons on disk are 34b..34h - the number names the
    # GROUP. Both the exact key and its numeric prefix count as known.
    known = {k.lstrip("0") or "0" for k in have}
    known |= {re.match(r"\d+", k).group(0).lstrip("0") or "0" for k in have}
    bad = 0
    for d in docs:
        s = pathlib.Path(d).read_text()
        miss, wrong, unknown, mistopic = [], [], [], []
        for m in re.finditer(r"\]\((\d+[a-z]?-[a-z0-9-]+\.md)\)", s):
            if not (here / m.group(1)).exists(): miss.append(m.group(1))
        for m in re.finditer(r"\[(\d+)\]\((\d+)-[a-z0-9-]+\.md\)", s):
            if m.group(1) != m.group(2): wrong.append(m.group(0))
        for m in re.finditer(r"\b[Ll]essons? ((?:\b\d{2,3}\b)(?:\s*(?:,|and|&|/|[-\u2013\u2014])\s*\b\d{2,3}\b)*)", s):
            for n in re.findall(r"\b\d{2,3}\b", m.group(1)):
                if (n.lstrip("0") or "0") not in known: unknown.append(n)
        # A bare `lesson 57` cannot be checked - 57 exists, and nothing says which 57 was meant.
        # But prose usually names the topic too: "lesson 57 (isocurves)", "lesson 57's isocurves".
        # When it does, the topic must appear in THAT lesson's filename. This is what catches a
        # renumber that moved the file and left the sentence behind - the number still resolves,
        # so the check above is happy, and the reader is sent to the wrong lesson.
        for m in re.finditer(r"\b[Ll]essons? (\d{1,3})(?:'s)? \(([a-z][a-z0-9 /-]{2,40})\)", s):
            num, topic = (m.group(1).lstrip("0") or "0"), m.group(2)
            fn = next((v for k, v in have.items() if (k.lstrip("0") or "0") == num), None)
            if not fn: continue
            words = [w for w in re.split(r"[ /-]+", topic) if len(w) > 3]
            if words and not any(w in fn for w in words):
                mistopic.append(f"{m.group(0)} -> {fn}")

        n = len(miss) + len(wrong) + len(unknown) + len(mistopic)
        if n:
            bad += n
            print(f"{d}: {len(miss)} dead link(s), {len(wrong)} mislabelled, "
                  f"{len(unknown)} naming no lesson, {len(mistopic)} naming the WRONG lesson")
            for x in miss[:4]:    print(f"   !! dead link      {x}")
            for x in wrong[:4]:   print(f"   !! label mismatch {x}")
            for x in sorted(set(unknown))[:6]: print(f"   ?? no such lesson  {x}")
            for x in sorted(set(mistopic))[:6]: print(f"   !! wrong lesson    {x}")
    print(f"# {bad} cross-reference problem(s) over {len(docs)} doc(s)")
    return bad


def render(docs):
    """Checks the other three modes are blind to: does the page RENDER as intended?

    A mistyped fence (```text where a closing ``` belongs) flips every following block from code
    to prose and back for the rest of the file, and NOTHING else here sees it: no op reaches the
    stray block, so `--audit` stays clean, and the replay stays byte-perfect because the ops
    themselves are untouched. Lesson 47 shipped ~900 lines rendered inside-out that way.
    """
    bad = False
    for d in docs:
        lines = pathlib.Path(d).read_text().split("\n")
        depth, opened = 0, None
        for i, l in enumerate(lines, 1):
            if l.startswith("```"):
                if depth == 0: depth, opened = 1, (i, l.strip())
                else: depth, opened = 0, None
        if depth:
            print(f"   !! {d}: fence opened at line {opened[0]} ({opened[1]}) is never closed"); bad = True
        heads = collections.Counter(re.findall(r"\*\*Create `([^`]+)`\*\*", "\n".join(lines)))
        for f, n in heads.items():
            if n > 1:
                print(f"   !! {d}: `{f}` is Created {n}x — a stale duplicate body?"); bad = True
        blocks = re.findall(r"```[a-z]*\n(.*?)\n```", "\n".join(lines), re.S)
        for b, n in collections.Counter(x for x in blocks if x.count("\n") > 40).items():
            if n > 1:
                print(f"   !! {d}: a {b.count(chr(10))+1}-line block appears {n}x verbatim"); bad = True
        print(f"{d}: fences balanced, no duplicate Create body")
    return bad


def copy_tree(src, dst, link_heavy=True):
    """Copy a snapshot without duplicating its bulk: heavy dirs are symlinked, not copied.
    A snapshot's assets/ is ~1.7 GB — copying it per replay filled /tmp."""
    src = pathlib.Path(src).resolve()
    shutil.copytree(src, dst, symlinks=True, ignore=shutil.ignore_patterns(*HEAVY))
    if not link_heavy: return
    for p in src.iterdir():
        if p.name in HEAVY and p.is_dir() and p.name != "__pycache__":
            (pathlib.Path(dst) / p.name).symlink_to(p, target_is_directory=True)

if __name__ == "__main__":
    argv = sys.argv[1:]
    if argv and argv[0] == "--audit":
        argv = argv[1:]
        thr = 0
        if argv and argv[0] == "--max":
            thr = int(argv[1]); argv = argv[2:]
        elif argv and argv[0].startswith("--max="):
            thr = int(argv[0].split("=", 1)[1]); argv = argv[1:]
        sys.exit(1 if audit(argv or all_lesson_docs(), thr) else 0)
    if argv and argv[0] == "--links":
        sys.exit(1 if links(argv[1:] or all_lesson_docs()) else 0)
    if argv and argv[0] == "--render":
        sys.exit(1 if render(argv[1:] or all_lesson_docs()) else 0)
    if argv and argv[0] == "--stale":
        sys.exit(1 if stale(pathlib.Path(argv[1]), argv[2:]) else 0)
    check_moves = bool(argv) and argv[0] == "--moves"
    if check_moves: argv = argv[1:]
    snap = pathlib.Path(argv[0]); work = pathlib.Path(argv[1]); docs = argv[2:]
    if work.exists(): shutil.rmtree(work)
    copy_tree(snap, work)
    prev = work.parent / (work.name + ".prev")
    bad = 0
    for d in docs:
        if check_moves:
            if prev.exists(): shutil.rmtree(prev)
            copy_tree(work, prev, link_heavy=False)
        f, skipped = apply(work, d); total = len(ops(d))
        out = f"{d}: {total} ops, {len(f)} failed"
        if skipped:
            out += f", {len(skipped)} kernel op(s) outside the viewer tree"
        print(out)
        for ln, tgt in skipped:
            print(f"   .. doc line {ln:>4}  {tgt}  — out of tree, not applied")
        for ln, tgt, why in f[:14]:
            print(f"   !! doc line {ln:>4}  {tgt}  — {why}")
        bad += len(f)
        if check_moves:
            mv = moves(prev, work, d)
            hard = [m for m in mv if m[2] != "lost-declared"]
            print(f"{d}: {len(move_map(ops(d)))} move source(s), {len(hard)} not byte-identical")
            for src, files, kind, ls in mv:
                mark = "!!" if kind != "lost-declared" else "..."
                print(f"   {mark} {kind} {src} (over {', '.join(files)}) — {len(ls)} line(s)")
                for l in ls[:8]:
                    print(f"        {l}")
            bad += len(hard)
    if prev.exists(): shutil.rmtree(prev)
    sys.exit(1 if bad else 0)
