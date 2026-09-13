"""Render and replay independent and integrated lessons from checkpoint 21 patches."""
import argparse
import hashlib
import json
import os
import re
from pathlib import Path
import shutil
import subprocess
import tempfile

import course_pages as course

HERE = Path(__file__).resolve().parent
REPO = HERE.parent
DATA = HERE / "extensions"
BASE = REPO / "target/docs/course-cache/snapshots/21"


def run(command, cwd, log):
    with log.open("w") as output:
        result = subprocess.run(command, cwd=cwd, env={**os.environ, "REGEN_PROTO": "0", "CARGO_TARGET_DIR": str(REPO / "target")}, stdout=output, stderr=subprocess.STDOUT)
    if result.returncode:
        raise RuntimeError(f"{command} failed; see {log}\n{log.read_text()[-4000:]}")


def prepare(output, isolated=False):
    if output.exists():
        raise ValueError(f"choose a new directory: {output}")
    if not BASE.exists():
        raise ValueError("run docs/serve.sh build first to populate the verified checkpoint cache")
    output.mkdir(parents=True)
    shutil.copytree(BASE / "session_viewer", output / "session_viewer")
    if isolated:
        manifest = output / "session_viewer/Cargo.toml"
        manifest.write_text(manifest.read_text().replace('path = "../session_rust"', f'path = "{BASE / "session_rust"}"'))
    else:
        shutil.copytree(BASE / "session_rust", output / "session_rust")
    return output / "session_viewer"


def instructions(step, working):
    changes = course.parse_patch((DATA / step["patch"]).read_text())
    parts = []
    for name, change in changes.items():
        old = working.get(name)
        if old is None and (BASE / "session_viewer" / name).exists():
            old = (BASE / "session_viewer" / name).read_text()
        label = "COPY" if name == "Cargo.lock" else "TYPE THIS"
        parts.append(f"### `{name}`\n")
        if change.kind == "create":
            new = "\n".join(change.hunks[0].new) + "\n"
            parts.append(f"**NEW FILE · {label}**\n\n" + course.fence(name, new))
        else:
            new = old
            for hunk in change.hunks:
                parts.append(f"**{label}**\n\n" + course.render_hunk(name, hunk, old, new))
                new = course.apply_hunk(name, hunk, old, new)
            new = course.finish_newline(new, old)
        working[name] = new
    return "\n".join(parts)


def render(lesson):
    key = lesson["id"]
    parts = [f'# {lesson["title"]}\n\n## You are building\n\n{lesson["intro"]}\n',
        f'![Running viewer: {lesson["title"]}.](screenshots/{lesson["shot"]})\n\nActual maintained viewer output. [Capture setup and five browser rounds](extensions/README.md).\n',
        '## Starting point\n\nStart from a fresh checkpoint **21**, not from another extension lesson. The lessons can be implemented separately. Every code block below is complete; there are no omitted method bodies. Execute every edit within one step before its check.\n',
        'Use the tools installed in [00 · Environment](00-environment.md). From the maintained `session_viewer` repository, create your learning workspace once:\n\n' + course.fence('setup.sh', f'export COURSE_REPO="$PWD"\nbash "$COURSE_REPO/docs/serve.sh" build --quiet\npython3 "$COURSE_REPO/docs/extensions.py" --prepare "$HOME/viewer-{key}"\ncd "$HOME/viewer-{key}/session_viewer"\nexport REGEN_PROTO=0\ncargo check -j4 --lib\n'),
        'The build prepares the frozen checkpoint cache. The initializer copies its viewer and kernel into a new folder; it does **not** install the feature. Expected: `Finished` with no compiler errors. Keep this terminal in the new `session_viewer` directory. If the destination exists, use a new folder name.\n\nFor **CURRENT → REPLACE WITH**, find the complete CURRENT block in the named file and replace it once. For **ADD BELOW**, keep the shown anchor and insert the new block directly after it. For **NEW FILE**, create the named path and paste its complete block. Apply blocks in page order; compile only at the check marker. All required code and answers are visible here.\n']
    if key in ('ui', 'gumball'):
        parts.insert(1, f'![CPU/GPU flow and resource lifetime](illustrations/extend-{key}.svg)\n')
    working = {}
    evidence = DATA / "verification.json"
    checks = json.loads(evidence.read_text()).get(key, {}) if evidence.exists() else {}
    for number, step in enumerate(lesson["steps"], 1):
        parts.append(f'## Step {number} · {step["title"]}\n\n{step["why"]}\n')
        parts.append(instructions(step, working))
        status = "**Verified:** the complete step compiles for WebAssembly." if checks.get('steps', {}).get(str(number)) == hashlib.sha256((DATA/step['patch']).read_bytes()).hexdigest() else "**Verification pending:** run the check below before continuing."
        parts.append(f'### Check step {number}\n\n{status}\n\n'+course.fence('check.sh','cargo check -j4 --lib\n'))
    parts.extend(['## Check\n\n'+course.fence('check.sh', 'cargo xtest -j4 --lib' + (' ' + lesson['test'] if lesson['test'] else '') + '\ntrunk serve --port 8780\n'),
        'Open <http://localhost:8780/?data=off&inspect=1>. Stop the server with **Ctrl+C**.\n',
        '### Reproduce the screenshots\n\nThe screenshots use the small [nested fixture](extensions/nested.pb) and [manifest](extensions/nested.yaml), not private project files. Save both into your workspace:\n\n'+course.fence('fixture.sh','cp "$COURSE_REPO/docs/extensions/nested.pb" assets/extension-nested.pb\ncp "$COURSE_REPO/docs/extensions/nested.yaml" assets/extension-nested.yaml\n')+'\nOpen <http://localhost:8780/?scene=extension-nested.yaml&data=off&inspect=1>.\n',
        '## What changed\n\n'+lesson['limits']+'\n',
        '## Try\n\n'+lesson['try']+'\n',
        '## Questions and answers\n\n**What goes to the GPU?** Modeling rebuilds existing geometry lanes; panels change object flags; controls upload a small preview. The solid gumball owns a fixed mesh, an unlit shader and a bounded antialiasing tile.\n\n**Why clear row selection after rebuilding?** Row numbers are upload addresses, not permanent identities. A rebuild can assign the same number to a different object.\n\n**Where is the exact patch?** '+', '.join(f'[step {i}](extensions/{step["patch"]})' for i,step in enumerate(lesson['steps'],1))+'. The patch and these visible instructions are generated from the same changes.\n'])
    return '\n'.join(parts)


def chapters(lesson):
    body = render(lesson)
    sections = re.split(r'(?m)^## Step (\d+) · ([^\n]+)\n', body)
    pages = {}
    links = []
    for at in range(1, len(sections), 3):
        number, title, content = sections[at:at + 3]
        previous = 'extend-integrated-tutorial.md' if number == '1' else f'current-{int(number)-1}.md'
        following = f'current-{int(number)+1}.md' if int(number) < len(lesson['steps']) else 'command-line-walkthrough.md'
        step = lesson['steps'][int(number)-1]
        visual = {1: '16-01.svg', 2: '20-03.svg', 3: 'extend-controls.svg', 4: 'extend-gumball.svg', 5: 'README-01.svg', 6: 'extend-ui.svg', 7: 'README-02.svg'}[int(number)]
        picture = f'\n![Ownership and data flow](illustrations/{visual})\n'
        if number == '4': picture += '\n![Unlit cylindrical gumball in the maintained viewer](screenshots/extensions-gumball.png)\n'
        if number == '6': picture += '\n![The white command window and black text](screenshots/extensions-command-interface.png)\n'
        content = picture + content
        content += '\n## Answers and next action\n\n' + step['answers'] + '\n\n'
        content += '**Run now**, in the same learning workspace:\n\n' + course.fence('check.sh', 'cargo check -j4 --lib\ntrunk serve --port 8780\n')
        content += '\nExpected compiler result: `Finished` with no errors. Open <http://localhost:8780/?data=off&inspect=1>. ' + step['expected'] + '\n\nStop the server with **Ctrl+C** before editing the next checkpoint. '
        content += f'Then open [{lesson["steps"][int(number)]["title"]}]({following}) and apply its blocks in order.\n' if int(number) < len(lesson['steps']) else f'Then follow [Use the command line]({following}) to exercise the finished interface.\n'
        navigation = f'[Previous]({previous}) · [Sequence](extend-integrated-tutorial.md) · [Next]({following})'
        pages[f'current-{number}.md'] = f'# {number} · {title}\n\n{navigation}\n\nContinue in the same checkpoint workspace. Complete the edits below before compiling.\n' + content + f'\n{navigation}\n'
        links.append(f'{number}. [{title}](current-{number}.md)')
    pages['extend-integrated-tutorial.md'] = sections[0] + '## Follow these checkpoints in order\n\n' + '\n'.join(links) + '\n\nThe final check compares every runtime source file, Cargo manifest, lockfile and browser entry point with the maintained viewer. Each checkpoint compiles for WebAssembly; the final one runs native library tests.\n'
    return pages


def current_sources():
    paths = sorted(path for path in (REPO / 'src').rglob('*') if path.is_file())
    paths += [REPO/name for name in ('Cargo.toml', 'Cargo.lock', 'index.html')]
    return {str(path.relative_to(REPO)): path.read_text() for path in paths}


def check_current(lesson):
    working = {}
    for step in lesson['steps']:
        instructions(step, working)
    expected = current_sources()
    for name, source in expected.items():
        actual = working.get(name, (BASE/'session_viewer'/name).read_text() if (BASE/'session_viewer'/name).exists() else None)
        if actual != source:
            raise ValueError(f'integrated lessons differ from maintained source: {name}')
    extra = set(working) - set(expected)
    if extra:
        raise ValueError(f'integrated lessons contain extra runtime files: {sorted(extra)}')
    print(f'PASS integrated lessons match {len(expected)} maintained runtime/build files')


def verify(lessons):
    report = json.loads((DATA/"verification.json").read_text()) if (DATA/"verification.json").exists() else {}
    logs = REPO / "target/docs/extension-checks"
    logs.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="viewer-extension-check-") as temp:
        for lesson in lessons:
            key = lesson['id']
            viewer = prepare(Path(temp) / key, isolated=True)
            working = {}
            result = {'steps': {}, 'kernel': 'frozen checkpoint 21'}
            for number, step in enumerate(lesson['steps'],1):
                instructions(step, working)
                run(['git','apply',str(DATA/step['patch'])],viewer,logs/f'{key}-{number}-apply.log')
                for name, source in working.items():
                    actual = (viewer/name).read_text()
                    if name == 'Cargo.toml':
                        actual = actual.replace(str(BASE / 'session_rust'), '../session_rust')
                    if actual != source:
                        raise ValueError(f'visible instructions differ from applied patch: {key}/{number}/{name}')
                run(['cargo','check','--locked','--lib','--target','wasm32-unknown-unknown','-j4'],viewer,logs/f'{key}-{number}-check.log')
                result['steps'][str(number)] = hashlib.sha256((DATA/step['patch']).read_bytes()).hexdigest()
                print(f'PASS {key} step {number}: visible snippets match patch; WASM check',flush=True)
            run(['cargo','test','--locked','--lib','--target','x86_64-unknown-linux-gnu','-j4'],viewer,logs/f'{key}-tests.log')
            result['tests'] = 'passed'
            report[key] = result
    (DATA/'verification.json').write_text(json.dumps(report,indent=2)+'\n')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--prepare',type=Path)
    parser.add_argument('--verify',action='store_true')
    parser.add_argument('--write',action='store_true')
    parser.add_argument('--lesson')
    args = parser.parse_args()
    lessons = json.loads((DATA/'lessons.json').read_text())['lessons']
    if args.prepare:
        print(prepare(args.prepare.expanduser().resolve()))
        return
    if args.verify:
        verify([lesson for lesson in lessons if not args.lesson or lesson['id'] == args.lesson])
    for lesson in lessons:
        if lesson.get('chapters'):
            check_current(lesson)
            pages = chapters(lesson)
        else:
            pages = {f'extend-{lesson["id"]}-tutorial.md': render(lesson)}
        for name, body in pages.items():
            page = HERE / name
            if args.write:
                page.write_text(body)
            elif not page.exists() or page.read_text() != body:
                raise ValueError(f'{page.name} is stale; run docs/extensions.py --write')
    print('PASS extension pages match their exact code instructions')


if __name__=='__main__':
    main()
