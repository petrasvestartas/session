import {changedBlocks, lines, lineContext} from './line-diff.ts';
import fs from 'node:fs';
import path from 'node:path';
import {createRequire} from 'node:module';
import {course, docs, read, json, write, hash, typingLoad, edited, expected, withExpectedSequence, type Step, type Edit} from './model.ts';
import {signature} from './verify.ts';
import {courseBook, readingBook} from '../../src/courseBook.ts';

const bookFor = (steps: Step[]) => courseBook(steps, json(path.join(docs, 'journey/book.json')));

const language: Record<string, string> = {'.rs': 'rust', '.wgsl': 'wgsl', '.html': 'html', '.toml': 'toml', '.md': 'markdown', '.sh': 'sh', '.yaml': 'yaml', '.js': 'javascript', '.json': 'json'};
const command = 'npm --prefix ../session_tests run course --';
const browserFingerprint = createRequire(import.meta.url)(path.join(docs, 'journey/checks/fingerprint.cjs'));

export function instructions(edits: Edit[], files: Record<string, string>, load = (edit: Edit) => read(path.join(docs, edit.snippet))) {
    const current = {...files}, result: {edit: Edit; code: string; inline: boolean; references: Edit[]}[] = [];
    const typed = (edit: Edit) => !edit.copy && !edit.rename_field && !edit.reuse;
    for (let i = 0; i < edits.length;) {
        const first = edits[i], group = [first], before = current[first.path];
        i++;
        while (typed(first) && i < edits.length && typed(edits[i]) && edits[i].path === first.path) group.push(edits[i++]);
        for (const edit of group) current[edit.path] = edited(current[edit.path], edit, load(edit));
        result.push(group.length === 1 ? {edit: first, code: load(first), inline: false, references: group} : {
            edit: {...first, before: before ?? null, why: [...new Set(group.map(edit => edit.why))].join(' ')},
            code: current[first.path], inline: true, references: group,
        });
    }
    return result;
}

export function dailyPlan(steps: Step[]) {
    const book = bookFor(steps);
    const reading = readingBook(steps, json(path.join(docs, 'journey/book.json')));
    const days: {steps: Step[]; minutes: number[]}[] = [];
    let day = {steps: [] as Step[], minutes: [0, 0]};
    for (const step of steps) {
        if (reading.aliases.has(step.id)) continue;
        if (step.id === '97-finish-with-the-maintained-browser-runtime') continue;
        const minutes = typingLoad(step).minutes;
        if (minutes[1] > 60) throw Error(`${step.id}: split the coding lesson before scheduling it`);
        if (day.steps.length && (day.minutes[1] + minutes[1] > 60 || day.steps.length === 6)) {
            days.push(day); day = {steps: [], minutes: [0, 0]};
        }
        day.steps.push(step);
        day.minutes = day.minutes.map((value, index) => value + minutes[index]);
    }
    if (day.steps.length) days.push(day);
    return ['# Plan your coding sessions',
        ...reading.chapters.filter(chapter => chapter.sections.some(section => section.lessons.some(lesson => /^\d+\.\d+$/.test(lesson.number)))).flatMap(chapter => [
            `## Chapter ${chapter.number}: consolidated lessons`,
            'Work through one coherent lesson at a time. Use its retained implementation checkpoints to save a stopping point; the reading structure is not limited by a typing-character budget.',
            ...chapter.sections.flatMap(section => section.lessons.map(lesson => `- [${lesson.number} · ${lesson.title}](${path.posix.relative('journey', lesson.page)})`)),
        ]),
        ...(days.length ? ['## Chapters awaiting consolidation',
        'Follow each row from left to right in the same browser project. Small consecutive checkpoints share a day; their code and explanations remain in their original lessons. Build and inspect the browser at each checkpoint before continuing.',
        'The estimates count typing at 50–100 characters per minute. Reading, reasoning, debugging and experiments take additional time. Stop at a completed checkpoint when your hour ends; resume from your own saved project.',
        'This route covers the authored learner implementation. [Current-viewer parity remains unfinished](completion-contract.md). The maintained-runtime reference comparison is excluded from the coding route.',
        '| Day | Checkpoints in order | Estimated coding |', '| ---: | --- | --- |',
        ...days.map((day, index) => `| ${index + 1} | ${day.steps.map(step => `[${book.byId.get(step.id)?.number || 'Reference comparison'} · ${step.title}](${step.id}.md)`).join(' → ')} | ${day.minutes.join('–')} min |`)] : []),
        ''].join('\n\n').replace(/(\|[^\n]*)\n\n(?=\|)/g, '$1\n');
}

export function replaceImplementation(page: string, id: string, title: string, body: string) {
    const anchor = page.indexOf(`<a id="checkpoint-${id}"></a>`);
    const start = page.indexOf('<details>', anchor);
    if (anchor < 0 || start < 0) throw Error(`Missing implementation block: ${id}`);
    let depth = 0, fence = '', end = start;
    for (const line of page.slice(start).split('\n')) {
        end += line.length + 1;
        const marker = line.match(/^\s*(`{3,}|~{3,})/);
        if (marker) {
            if (!fence) fence = marker[1];
            else if (marker[1][0] === fence[0] && marker[1].length >= fence.length) fence = '';
            continue;
        }
        if (fence) continue;
        depth += [...line.matchAll(/<details(?:\s[^>]*)?>/g)].length - [...line.matchAll(/<\/details>/g)].length;
        if (depth === 0) {
            const label = title.replaceAll('&', '&amp;').replaceAll('<', '&lt;').replaceAll('>', '&gt;');
            return page.slice(0, start) + `<details>\n<summary>Implementation ${id}: ${label}</summary>\n\n${body.trim()}\n\n</details>\n` + page.slice(end);
        }
    }
    throw Error(`Unclosed implementation block: ${id}`);
}

function implementationLinks(text: string, from: string, to: string) {
    let fence = '';
    return text.split('\n').map(line => {
        const marker = line.match(/^\s*(`{3,}|~{3,})/);
        if (marker) {
            if (!fence) fence = marker[1];
            else if (marker[1][0] === fence[0] && marker[1].length >= fence.length) fence = '';
            return line;
        }
        if (fence) return line;
        return line.replace(/\]\(([^)\s]+)\)/g, (match, href: string) => {
            if (/^[a-z][a-z0-9+.-]*:|^\/|^#/i.test(href)) return match;
            const [file, anchor] = href.split('#');
            const relative = path.posix.relative(path.posix.dirname(to), path.posix.join(path.posix.dirname(from), file));
            return `](${relative}${anchor ? '#' + anchor : ''})`;
        });
    }).join('\n');
}

export function generate(ids?: string[]) {
    return withExpectedSequence(() => generatePages(ids ? new Set(ids) : undefined));
}

function generatePages(selected?: Set<string>) {
    const steps = course().steps;
    const stepsById = new Map(steps.map(step => [step.id, step]));
    if (selected) for (const id of selected) if (!stepsById.has(id)) throw Error(`Unknown checkpoint: ${id}`);
    const implementations = new Map<string, string>();
    const book = bookFor(steps);
    const reading = readingBook(steps, json(path.join(docs, 'journey/book.json')));
    const readerCount = reading.byId.size;
    const consolidatedCount = json(path.join(docs, 'journey/book.json')).chapters.filter((chapter: {consolidated?: unknown[]}) => chapter.consolidated).length;
    const sources = new Map<string, string>();
    const files: Record<string, string> = {};
    const pictureFiles = new Map<string, string>();
    const resultImage = (file: string, caption: string) => {
        const digest = hash(fs.readFileSync(path.join(docs, 'screenshots/journey', file)));
        const shared = pictureFiles.get(digest) || file;
        pictureFiles.set(digest, shared);
        return `![${caption}](../screenshots/journey/${shared})`;
    };
    const sourceFor = (id: string) => {
        if (!sources.has(id)) sources.set(id, signature(id));
        return sources.get(id)!;
    };
    const capture = path.join(docs, 'screenshots/journey/browser.json');
    const browser = fs.existsSync(capture) ? json(capture).steps : {};
    const buildFile = path.join(docs, '../target/course-checks/results.json');
    const builds = fs.existsSync(buildFile) ? json(buildFile) : {};
    const visualReviews = json(path.join(docs, 'journey/visual-review.json'));
    const visualFor = (step: Step) => visualReviews.steps[step.id] || visualReviews.frames?.[step.frame || ''];
    const captureHashes = new Map(['capture_journey.cjs', 'capture_current.cjs'].map(file => [file, hash(read(path.join(docs, file)))]));
    const currentChrome = (step: Step) => {
        const proof = browser[step.id];
        return proof?.source === sourceFor(step.id)
            && proof.checker === captureHashes.get(step.browser_check === 'journey/checks/browser/final.cjs' ? 'capture_current.cjs' : 'capture_journey.cjs')
            && (!step.browser_check || proof.extraChecker === browserFingerprint(step.browser_check));
    };
    const reviewedBrowserScene = (step: Step) => {
        const review = visualFor(step), proof = browser[step.id];
        return review?.status === 'accepted' && currentChrome(step)
            && review.source === proof.source && review.checker === proof.extraChecker && review.file === proof.file
            && review.images?.[proof.file]
            && Object.entries(review.images).every(([file, digest]) => {
                const image = path.join(docs, 'screenshots/journey', file);
                return fs.existsSync(image) && digest === hash(fs.readFileSync(image));
            });
    };

    for (const [index, step] of steps.entries()) {
        if (selected && !selected.has(step.id)) {
            for (const edit of step.edits) files[edit.path] = edited(files[edit.path], edit, read(path.join(docs, edit.snippet)), step.id);
            continue;
        }
        const [low, high] = step.hours;
        const typing = typingLoad(step);
        const buildEnv = expected(step.id)['Cargo.toml'].includes('session_rust') ? 'REGEN_PROTO=0 ' : '';
        const paragraphs = step.story.split('\n\n');
        const diagrams = paragraphs.filter(text => text.startsWith('!['));
        const explanation = paragraphs.filter(text => !text.startsWith('!['));
        const concise = explanation[0].trim().split(/\s+/).length <= 45;
        const introduction = [concise ? explanation[0] : step.goal];
        const supportingExplanation = [
            ...(concise ? [] : [explanation[0]]),
            ...explanation.slice(index === 0 ? 2 : 1),
        ];
        const lesson = book.byId.get(step.id);
        const checkpoint = lesson?.number || step.id;
        const page = [`# ${lesson?.number || 'Reference comparison'} · ${step.title}`,
            ...(lesson ? [`**Chapter ${lesson.chapter} · Section ${lesson.section}** · [Contents](contents.md)`] : []),
            `**Typing: ${typing.minutes.join('–')} minutes.** [Estimate](typing-load.md).`,
            ...(typing.minutes[1] > 60 ? ['**Splitting required:** this verified checkpoint exceeds the one-hour typing target. Smaller runnable lessons are still being prepared.'] : []),
            ...introduction, '## Type'];
        if (index === 0) page.push(explanation[1]);
        if (index) {
            const previous = steps[index - 1];
            page.push(`Continue from [${previous.title}](${previous.id}.md). [Save or recover your work](recovery.md).`);
        }
        for (const [number, instruction] of instructions(step.edits, files).entries()) {
            const {edit} = instruction;
            let code = instruction.code;
            const original = files[edit.path] || '';
            const why = edit.why === explanation[0] ? [] : [edit.why];
            if (edit.reuse && (!edit.copy || edit.before !== null || files[edit.reuse] !== code)) {
                throw Error(`${step.id}: ${edit.path} must exactly reuse ${edit.reuse}`);
            }
            files[edit.path] = edited(files[edit.path], edit, code, step.id);
            if (edit.rename_field) {
                page.push(`### ${number + 1}. \`${edit.path}\``, ...why,
                    'Use your editor’s **whole-word Replace All** in this file. Find this field reference:',
                    `\`\`\`rust\n${edit.before}\n\`\`\``,
                    'Replace every match with:', `\`\`\`rust\n--8<-- "${edit.snippet}"\n\`\`\``,
                    'Keep longer field names such as `self.depth_pyramid` unchanged. The checkpoint replay validates this exact rename; no implementation code is supplied.');
                continue;
            }
            if (edit.reuse) {
                page.push(`### ${number + 1}. \`${edit.path}\``, ...why,
                    'Reuse the shader you have already written:',
                    `\`\`\`sh\ncp ${edit.reuse} ${edit.path}\n\`\`\``,
                    `<details>\n<summary>Reused source reference</summary>\n\n\`\`\`wgsl\n--8<-- "${edit.snippet}"\n\`\`\`\n\n</details>`);
                continue;
            }
            let before = edit.before, expanded = instruction.inline;
            if (before && !edit.copy) {
                const context = lineContext(original, before, code);
                expanded ||= context.before !== before || context.after !== code;
                before = context.before;
                code = context.after;
            }
            const lang = edit.copy ? 'text' : language[path.extname(edit.path)] || 'text';
            const added = before && code.startsWith(before) ? code.slice(before.length) : code;
            const append = !!before && code.startsWith(before) && added.length > 0;
            const author = edit.copy && (/^examples\/[^/]+\.rs$/.test(edit.path)
                || /^src\/(reference_example|inventory_example|reference_frame)\.rs$/.test(edit.path));
            const data = edit.copy && edit.path.endsWith('.json');
            if (edit.supplied && !edit.copy) throw Error(`${step.id}: supplied code must be excluded from typing`);
            const example = edit.copy && edit.supplied === 'example';
            const moved = edit.copy && edit.supplied === 'move';
            const fixture = author || edit.copy && (/_tests\.rs$/.test(edit.path)
                || edit.supplied === 'validation' || added.trimStart().startsWith('#[cfg(test)]') || added.trimStart().startsWith('#[test]'));
            if (moved) page.push(`### ${number + 1}. \`${edit.path}\``, ...why,
                'Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.');
            if (fixture || data || example || moved) page.push(`<details>\n<summary>${moved ? 'Previously typed code reference' : data ? 'Supplied data' : example ? 'Supplied example data' : author ? 'Optional authoring helper' : 'Supplied validation fixture'}</summary>`,
                moved ? 'Use this reference when moving the existing code; it is excluded from the typing estimate.' : data || example ? 'Copy this supplied data once; it is excluded from the typing estimate.' : author ? 'Copy this supplied authoring helper once. Use the linked data or recorded comparison for the browser exercise; you do not need to compile or run this native helper.'
                    : 'Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.');
            const fence = '`'.repeat(Math.max(3, ...[...(code + (before || '') + instruction.references.map(edit => read(path.join(docs, edit.snippet))).join('\n')).matchAll(/^\s*(`{3,})/gm)].map(match => match[1].length + 1)));
            const references = () => {
                if (expanded) page.push('<details>\n<summary>Optional patch references</summary>',
                    'Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.',
                    ...instruction.references.filter(edit => read(path.join(docs, edit.snippet)))
                        .map(edit => `${fence}text\n--8<-- "${edit.snippet}"\n${fence}`), '</details>');
            };
            if (!moved) page.push(`### ${number + 1}. \`${edit.path}\``, ...why);
            const blocks = before && !edit.copy && !append && lines(before).length >= 60
                ? changedBlocks(before, code, original) : null;
            if (blocks && blocks.reduce((count, block) => count + block.before.length + block.after.length, 0) < (before!.length + code.length) / 2) {
                page.push('Edit these blocks from bottom to top. Keep the surrounding lines unchanged.');
                for (const block of blocks) {
                    page.push(`<details>\n<summary>Locate this block</summary>\n\n${fence}${lang}\n${block.before.trimEnd()}\n${fence}\n\n</details>`);
                    if (block.after) page.push('Replace it with:', `${fence}${lang}\n${block.after.trimEnd()}\n${fence}`);
                    else page.push('Delete this block.');
                }
                page.push(`<details>\n<summary>Complete updated block reference</summary>\n\n${fence}${lang}\n${expanded ? code.trimEnd() : `--8<-- "${edit.snippet}"`}\n${fence}\n\n</details>`);
                references();
                continue;
            }
            if (before === null) page.push(moved ? 'Create the destination by moving the existing code:' : edit.copy ? 'Copy this supplied file:' : 'Create the file and type:');
            else if (code) page.push(`<details>\n<summary>Locate the existing block</summary>\n\n${fence}${lang}\n${before.trimEnd()}\n${fence}\n\n</details>`, append
                ? moved ? 'Move the existing code after that block:' : edit.copy ? 'After that block, append this supplied code:' : 'After that block, type:'
                : moved ? 'Reuse the existing code in this destination:' : edit.copy ? 'Update the existing block with this supplied code; keep unchanged lines:' : 'Edit the existing block to match this code; keep unchanged lines:');
            else page.push('Find this exact block:', `${fence}${lang}\n${before.trimEnd()}\n${fence}`, 'Delete this block.');
            if (code) page.push(`${fence}${lang}\n${expanded ? (append ? code.slice(before!.length) : code).trimEnd() : `--8<-- "${edit.snippet}"${append ? ` skip=${before!.length}` : ''}`}\n${fence}`);
            references();
            if (fixture || data || example || moved) page.push('</details>');
        }
        implementations.set(step.id, page.slice(page.indexOf('## Type') + 1).join('\n\n'));
        page.push('## Run and check');
        if (step.lock) page.push('Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:',
            `\`\`\`sh\n${command} dependencies ${checkpoint}\n\`\`\``);
        page.push('In your project:',
            `\`\`\`sh\ncd workspace/journey\n${buildEnv}cargo build --lib --locked --target wasm32-unknown-unknown -j1\n${buildEnv}CARGO_BUILD_JOBS=1 trunk serve --port 8780\n\`\`\``,
            'Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.', step.result);
        if (visualFor(step)?.issue) page.push(`**Visual review: issue found.** ${visualFor(step).issue}`, '[Visual review findings](visual-review.md).');
        const evidence = browser[step.id];
        const source = sourceFor(step.id);
        const reviewed = reviewedBrowserScene(step);
        if (currentChrome(step)) {
            page.push('**Verified checkpoint in Chrome.**',
                ...(reviewed ? ['**Critical browser-scene review accepted.**', visualFor(step).caption] : step.browser_check ? ['**Feature-specific visual review pending.** Scripted acceptance and the final main-page capture are separate evidence. A checker may restore its general fixture after closing the feature-test page; this image does not establish the feature’s visual correctness.'] : []),
                resultImage(evidence.file, reviewed ? visualFor(step).caption : step.browser_check ? `General browser fixture after ${step.title}; feature-scene visual acceptance pending.` : `Actual browser result: ${step.title}.`),
                '[Verification scope](release.md).');
        } else if (index && fs.existsSync(path.join(docs, 'screenshots/journey', `${step.id === '04-input' ? '04-input-light' : step.id}.png`))) {
            const picture = step.id === '04-input' ? '04-input-light' : step.id;
            const current = builds[step.id]?.source === source;
            page.push('### Recorded visual result',
                current ? '**Native render check — not a browser screenshot.**' : '**Previous native render — current verification pending.**',
                ...(step.image_result ? [step.image_result] : []),
                resultImage(`${picture}.png`, `Native renderer output for ${step.title.toLowerCase()}.`),
                current ? '*Read directly from this checkpoint’s GPU texture. Browser controls and event delivery remain unverified until the browser check passes.*'
                    : '*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*');
        } else if (index) {
            page.push('**Verification pending.** Canonical build and browser acceptance have not yet been recorded for this checkpoint.');
        }
        if (step.image_same_as) {
            const parent = book.byId.get(step.image_same_as);
            page.push(`This reference image is shared with [${parent?.number || ''} · ${parent?.title || step.image_same_as}](${step.image_same_as}.md). It does not demonstrate a new feature by itself.`);
        }
        page.push(`<details>\n<summary>Code explanation and diagram</summary>\n\n${supportingExplanation.join('\n\n')}\n\n${step.trace}.\n\n${diagrams.join('\n\n')}\n\n${step.question}\n\n${step.answer}\n\nStudy estimate, including typing and experiments: ${low}–${high} hours.\n\n</details>`,
            `<details>\n<summary>Optional experiment</summary>\n\n${step.experiment}\n\n</details>`,
            `<details>\n<summary>Check and save your work</summary>\n\nRestore experimental edits, then run from \`session_viewer\`:\n\n` +
            `\`\`\`sh\n${command} check ${checkpoint}\n${command} save ${checkpoint}\n\`\`\`\n\n` +
            'Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).\n\n</details>');
        const supporting = [step.checks, step.browser_caption].filter(Boolean);
        if (step.production || supporting.length || step.browser_check) {
            page.push(`<details>\n<summary>Viewer coverage and verification</summary>\n\n${step.production}\n\n${supporting.join('\n\n')}\n\n[Full validation scope](release.md).\n\n` +
                (step.browser_check ? `To reproduce the scripted acceptance of the reference checkpoint, run from \`session_viewer\` with the [course bundle server](release.md#reproduce) running on port 8781:\n\n\`\`\`sh\n${command} capture ${checkpoint}\n\`\`\`\n\nThis uses the verified reference bundle; it does not check or change your typed project.\n\n` : '') +
                '</details>');
        }
        write(path.join(docs, step.page), page.join('\n\n') + '\n');
    }
    for (const chapter of reading.chapters) for (const section of chapter.sections) for (const lesson of section.lessons) {
        if (!/^\d+\.\d+$/.test(lesson.number)) continue;
        const file = path.join(docs, lesson.page);
        let page = read(file);
        for (const step of steps) {
            if (reading.aliases.get(step.id)?.page !== lesson.page || !implementations.has(step.id)) continue;
            page = replaceImplementation(page, step.id, step.title, implementationLinks(implementations.get(step.id)!, step.page, lesson.page));
        }
        if (page !== read(file)) write(file, page);
    }
    if (selected) {
        console.log(`Generated ${selected.size} changed checkpoint pages and refreshed their consolidated implementations.`);
        return;
    }
    const hours = steps.reduce((sum, step) => sum.map((n, i) => n + step.hours[i]), [0, 0]);
    const overview = path.join(docs, 'journey.md');
    const planned = [...read(path.join(docs, 'journey/roadmap.md'))
        .matchAll(/^- \[(?:x| )\] (?:\d+[a-z]*|\d+(?:\.\d+)+|Reference comparison) · /gm)].length;
    if (planned < steps.length) throw Error('Roadmap must include every available lesson');
    const roadmap = path.join(docs, 'journey/roadmap.md');
    const verifiedIds = new Set(steps.filter(currentChrome).map(step => step.id));
    const verified = verifiedIds.size;
    const progress = verified === steps.length
        ? `${steps.length} current checkpoints have fresh build and Chrome evidence.`
        : `${steps.length} current checkpoints; ${verified} have fresh build and Chrome evidence; ${steps.length - verified} await fresh Chrome acceptance.`;
    write(roadmap, read(roadmap)
        .replace(/^\*\*(?:Draft plan|Course status):[^\n]*?\*\*/m,
            `**Course status: ${progress}${planned > steps.length ? ` ${planned - steps.length} planned lessons remain.` : ''}**`)
        .replace(/all \d+ current checkpoints/g, `all ${steps.length} current checkpoints`)
        .split('\n').map(line => {
            if (!/^- \[[ x]\] (?:\d+[a-z]*|\d+(?:\.\d+)+|Reference comparison) · /.test(line)) return line;
            const id = line.match(/\]\(([^)]+)\.md\)/)?.[1];
            return id && steps.some(step => step.id === id)
                ? line.replace(/^- \[[ x]\] [^·]+ · /, `- [${verifiedIds.has(id) ? 'x' : ' '}] ${book.byId.get(id)?.number || 'Reference comparison'} · `) : line;
        }).join('\n'));
    const release = path.join(docs, 'journey/release.md');
    write(release, read(release).replace(/^\*\*[^\n]*?\*\*/m, verified === steps.length
        ? `**All ${steps.length} current checkpoints build and pass their scripted checks in visible Chrome ${json(capture).browser}.**`
        : `**${verified} of ${steps.length} current checkpoints have fresh build and Chrome evidence. Remaining browser acceptance is pending.**`));
    const table = ['| Lesson | Working result |', '| --- | --- |',
        ...steps.map(step => `| [${book.byId.get(step.id)?.number || 'Reference comparison'} · ${step.title}](${step.page}) | ${step.goal} |`)];
    write(overview, read(overview)
        .replace(/^The \[complete (?:draft )?lesson checklist\].+$/m,
            `The [complete lesson checklist](journey/roadmap.md) lists **${steps.length} authored checkpoints${planned > steps.length ? ` and ${planned - steps.length} planned lessons` : ''}**. Use the [daily coding route](journey/daily-plan.md) to group small checkpoints. Its lesson checkboxes track fresh build and browser acceptance.`)
        .replace(/Remaining chapters must meet the same limit as they are written\./, 'Reading and experiments are estimated separately.')
        .replace(/Show all \d+ cumulative checkpoints/, `Show all ${steps.length} cumulative checkpoints`)
        .replace(/\*\*Current release:[^\n]+/, `**Current release: ${readerCount.toLocaleString('en-US')} reader-facing lessons and ${steps.length.toLocaleString('en-US')} preserved implementation checkpoints.** ${consolidatedCount} of ${book.chapters.length} chapters are consolidated, retaining ${reading.aliases.size} original checkpoint IDs. The [source and coverage map](journey/production-course-map.md) tracks the retained lessons and remaining implementation coverage. Check the [release evidence](journey/release.md) before starting; consolidation does not revalidate historical GPU evidence.`)
        .replace(/\| Lesson \|[\s\S]*?(?=\n\n)/, table.join('\n')));
    const contents = ['# Course contents',
        'Read in chapter order. Consolidated lessons use **chapter.lesson** numbering, such as **1.1**. Original chapter.section.checkpoint numbers and URLs remain available for recovery and source comparison. The [daily plan](daily-plan.md) keeps stopping points within each lesson.',
        `[Production source and coverage map](production-course-map.md) · **${consolidatedCount} / ${book.chapters.length} chapters consolidated; ${readerCount.toLocaleString('en-US')} reader-facing lessons on the current route**. All ${steps.length.toLocaleString('en-US')} old checkpoint IDs remain available; the final maintained-runtime comparison is a reference appendix.`,
        '[Progress and remaining work](roadmap.md) · [Visual review and known issues](visual-review.md). Existing lesson links and saved checkpoints remain valid.',
        'Each chapter shows a recorded result from one linked checkpoint. **Current Chrome check** means that checkpoint’s source and browser checks match the recorded proof. **Recorded Chrome screenshot** means fresh acceptance is pending. **Native render** shows GPU output without proving browser interaction. These pictures do not establish that every lesson in a chapter works.'];
    for (const chapter of book.chapters) {
        contents.push(`## ${chapter.number} · ${chapter.title}`);
        const lessons = chapter.sections.flatMap(section => section.lessons).reverse();
        const checked = lessons.filter(lesson => verifiedIds.has(lesson.id)).length;
        contents.push(`**Automated Chrome checks: ${checked} of ${lessons.length} checkpoints current.**${checked < lessons.length ? ' Remaining checkpoints need fresh Chrome acceptance.' : ''}`);
        const reviewed = lessons.filter(lesson => reviewedBrowserScene(stepsById.get(lesson.id)!)).length;
        contents.push(`**Critical browser-scene review: ${reviewed} of ${lessons.length} checkpoints recorded.** A supporting image is a reference, not approval of the entire chapter.`);
        const imageExists = (file: string) => fs.existsSync(path.join(docs, 'screenshots/journey', file));
        const nativeFile = (id: string) => `${id === '04-input' ? '04-input-light' : id}.png`;
        const stepFor = (id: string) => stepsById.get(id)!;
        const reviewedCheckpoint = lessons.find(lesson => reviewedBrowserScene(stepFor(lesson.id)));
        const nativeCheckpoint = lessons.find(lesson => imageExists(nativeFile(lesson.id)));
        const checkpoint = reviewedCheckpoint || nativeCheckpoint
            || lessons.find(lesson => verifiedIds.has(lesson.id) && browser[lesson.id]?.file && imageExists(browser[lesson.id].file))
            || lessons.find(lesson => browser[lesson.id]?.file && imageExists(browser[lesson.id].file));
        if (checkpoint) {
            const proof = browser[checkpoint.id];
            const isBrowser = Boolean(reviewedCheckpoint || !nativeCheckpoint && proof?.file && imageExists(proof.file));
            const file = isBrowser ? proof.file : nativeFile(checkpoint.id);
            const reviewed = reviewedBrowserScene(stepFor(checkpoint.id));
            let status = 'Previous native render — fresh verification pending';
            if (isBrowser) status = reviewed ? 'Current Chrome capture — critically reviewed scene'
                : verifiedIds.has(checkpoint.id) ? 'Current scripted Chrome check — critical image review pending'
                : 'Recorded Chrome screenshot — fresh verification pending';
            else if (builds[checkpoint.id]?.source === sourceFor(checkpoint.id)) status = 'Current supporting native render — critical image review pending';
            if (visualFor(stepFor(checkpoint.id))?.issue) contents.push(`**Visual review: issue found.** ${visualFor(stepFor(checkpoint.id)).issue}`);
            else if (reviewed) contents.push(`**Critical browser-scene review accepted.** ${visualFor(stepFor(checkpoint.id)).caption}`);
            else contents.push('**Critical visual review pending.** This recorded reference is not an accepted visual-quality result.');
            contents.push(`**${status}.** [${checkpoint.number} · ${checkpoint.title}](${checkpoint.id}.md).${isBrowser && proof.captured ? ` Captured ${proof.captured.slice(0, 10)}.` : ''}`,
                `![Chapter ${chapter.number}: ${checkpoint.title}. ${status}.](../screenshots/journey/${file})`);
        } else {
            contents.push('**Visual verification pending.** No recorded image is available for this chapter yet.');
        }
        for (const section of reading.chapters[chapter.number - 1].sections) {
            contents.push(`### ${section.number} · ${section.title}`,
                section.lessons.map(lesson => `- [${lesson.number} · ${lesson.title}](${path.posix.relative('journey', lesson.page)})`).join('\n'));
        }
    }
    if (book.comparison) contents.push('## Reference comparison', `[${book.comparison.title}](${book.comparison.id}.md). This comparison remains outside the learner coding route while implementation is unfinished.`);
    write(path.join(docs, 'journey/contents.md'), contents.join('\n\n') + '\n');
    write(path.join(docs, 'journey/daily-plan.md'), dailyPlan(steps));
    const audit = path.join(docs, 'journey/typing-load.md');
    const rows = steps.map(step => {
        const typing = typingLoad(step);
        return `| [${book.byId.get(step.id)?.number || 'Reference comparison'} · ${step.title}](${step.id}.md) | ${typing.lines} | ${typing.characters} | ${typing.minutes.join('–')} min | ${step.hours.join('–')} h | ${typing.minutes[1] <= 60 ? 'Within planning limit' : 'Split required'} |`;
    });
    write(audit, read(audit)
        .replace(/all \d+ current checkpoints/, `all ${steps.length} current checkpoints`)
        .replace(/the current \d+ slots/, `the current ${planned} slots`)
        .replace(/\| Checkpoint \|[\s\S]*?(?=\n\n)/,
        ['| Checkpoint | Added/changed lines | Characters | Typing estimate | Existing combined study estimate | Typing limit |',
            '| --- | ---: | ---: | --- | --- | --- |', ...rows].join('\n')));
    console.log(`Generated ${steps.length} lessons from their source edits.`);
}
