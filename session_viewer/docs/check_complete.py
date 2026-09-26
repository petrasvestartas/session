"""Every line lesson N adds to the viewer is shown on page N. Usage: check_complete.py [viewer] (viewer skips tests/, examples/, assets/)."""
import re, glob, os, sys, collections
NORM = re.compile(r'lessons/[0-9]{2,3}[a-z]?\b')
norm = lambda l: NORM.sub('lessons/N', l.strip())
V = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
D = f'{V}/docs'
MARK = re.compile(r'--8<--\s*\[(start|end):([^\]]+)\]')
SKIP = ('target', 'dist', 'Cargo.lock')

def section(lines, name):
    out, on = [], False
    for l in lines:
        m = MARK.search(l)
        if m and m.group(2) == name:
            on = m.group(1) == 'start'
            continue
        if on and not MARK.search(l):
            out.append(l)
    return out

def files(lesson):
    root = f'{D}/lessons/{lesson}'
    out = {}
    for f in glob.glob(f'{root}/**/*', recursive=True):
        r = os.path.relpath(f, root)
        if os.path.isfile(f) and not any(r.startswith(s) for s in SKIP) and not r.endswith(('.png', '.ttf', '.woff2', '.bin', '.glb')):
            out[r] = collections.Counter(norm(l) for l in open(f, errors='ignore').read().splitlines() if l.strip() and not MARK.search(l))
    return out

def shown(page):
    got = collections.defaultdict(collections.Counter)
    for m in re.finditer(r'--8<--\s*"([^"]+)"', open(page).read()):
        parts = m.group(1).split(':')
        path = next((p for p in (os.path.join(D, parts[0]), os.path.join(V, parts[0])) if os.path.exists(p)), None)
        if not path:
            continue
        lines = open(path, errors='ignore').read().splitlines()
        sel = [l for l in lines if not MARK.search(l)] if len(parts) == 1 else section(lines, parts[1]) if len(parts) == 2 else lines[int(parts[1] or 1) - 1:int(parts[2] or len(lines))]
        mm = re.search(r'lessons/[^/]+/(.*)', path)
        got[mm.group(1) if mm else os.path.relpath(path, V)].update(norm(l) for l in sel if l.strip())
    return got

series = [l.split()[0] for l in open(f'{D}/lessons/SERIES.txt') if l.strip()]
pages = {os.path.basename(p).split('-')[0]: p for p in glob.glob(f'{D}/[0-9]*.md')}
ONLY = sys.argv[1] if len(sys.argv) > 1 else ""
prev, tot_add, tot_miss = {}, 0, 0
rows = []
for lid in series:
    cur = files(lid)
    page = shown(pages[lid]) if lid in pages else {}
    add = miss = 0
    worst = []
    for f, c in cur.items():
        if ONLY == "viewer" and f.startswith(("tests/", "examples/", "assets/")):
            continue
        new = c - prev.get(f, collections.Counter())
        n = sum(new.values())
        if not n:
            continue
        gap = sum((new - page.get(f, collections.Counter())).values())
        add += n; miss += gap
        if gap:
            worst.append((gap, f))
    rows.append((lid, add, miss, sorted(worst, reverse=True)[:3]))
    tot_add += add; tot_miss += miss
    prev = cur
print(f'lines added across the chain {tot_add}, not shown on their own page {tot_miss} ({100*tot_miss/max(tot_add,1):.1f}%)')
for lid, add, miss, worst in rows:
    print(f'{lid:>4} added {add:6d} missing {miss:5d}  ' + ', '.join(f'{f}({g})' for g, f in worst))
