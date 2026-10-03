import fs from 'node:fs';
import path from 'node:path';
import {createRequire} from 'node:module';
import {course, docs, read, json, write, hash, typingLoad} from './model.ts';
import {signature} from './verify.ts';

const language: Record<string, string> = {'.rs': 'rust', '.wgsl': 'wgsl', '.html': 'html', '.toml': 'toml', '.md': 'markdown', '.sh': 'sh', '.yaml': 'yaml'};
const command = 'npm --prefix ../session_tests run course --';
const browserFingerprint = createRequire(import.meta.url)(path.join(docs, 'journey/checks/fingerprint.cjs'));

export function generate() {
    const steps = course().steps;
    const capture = path.join(docs, 'screenshots/journey/browser.json');
    const browser = fs.existsSync(capture) ? json(capture).steps : {};
    for (const [index, step] of steps.entries()) {
        const [low, high] = step.hours;
        const typing = typingLoad(step);
        const page = [`# ${step.id.split('-')[0]} · ${step.title}`,
            `**Combined study estimate: ${low}–${high} hours.** Includes reading, typing, reasoning and experiments.`,
            `**Typing estimate: ${typing.minutes.join('–')} minutes.** ${typing.lines} added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).`,
            ...(typing.minutes[1] > 60 ? ['**Splitting required:** this verified checkpoint exceeds the one-hour typing target. Smaller runnable lessons are still being prepared.'] : []),
            `**Today:** ${step.goal}`,
            '**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).',
            `**Follow:** ${step.trace}.`, `**Before you finish, explain:** ${step.question}`, step.story, '## Type the change'];
        if (index) {
            const previous = steps[index - 1];
            page.push(`Continue [${previous.title}](${previous.id}.md). From \`session_viewer\`, save your files with \`${command} save before-${step.id}\`. A save keeps your own work; it does not fill in the next lesson.`);
        }
        for (const [number, edit] of step.edits.entries()) {
            const code = read(path.join(docs, edit.snippet));
            const lang = language[path.extname(edit.path)] || 'text';
            const fence = '`'.repeat(Math.max(3, ...[...(code + (edit.before || '')).matchAll(/^\s*(`{3,})/gm)].map(match => match[1].length + 1)));
            page.push(`### ${number + 1}. \`${edit.path}\``, edit.why);
            if (edit.before === null) page.push('Create the file and type:');
            else page.push('Find this exact block:', `${fence}${lang}\n${edit.before.trimEnd()}\n${fence}`, code ? 'Replace that block with:' : 'Delete this block.');
            if (code) page.push(`${fence}${lang}\n--8<-- "${edit.snippet}"\n${fence}`);
        }
        page.push('## Run and look');
        if (step.lock) page.push('After typing the manifest, run this from `session_viewer` to select the fixed dependency versions. It updates Cargo.lock, preserves the previous lock, and installs any supplied binary font assets. It does not write implementation code:',
            `\`\`\`sh\n${command} dependencies ${step.id}\n\`\`\``);
        page.push('From `session_viewer`, enter your project folder:',
            '```sh\ncd workspace/journey\ncargo build --lib --locked --target wasm32-unknown-unknown -j4\nCARGO_BUILD_JOBS=4 trunk serve --port 8780\n```',
            'Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.', step.result);
        const evidence = browser[step.id];
        if (evidence && evidence.source === signature(step.id)
            && evidence.checker === hash(read(path.join(docs, 'capture_journey.cjs')))
            && (!step.browser_check || evidence.extraChecker === browserFingerprint(step.browser_check))) {
            page.push('**Actual Chrome screenshot.**',
                ...(step.browser_caption ? [step.browser_caption] : []),
                `![Actual browser result: ${step.title}.](../screenshots/journey/${evidence.file})`,
                '*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*');
        } else if (index) {
            const picture = step.id === '04-input' ? '04-input-light' : step.id;
            page.push('**Native render check — not a browser screenshot.**',
                ...(step.image_result ? [step.image_result] : []),
                `![Native renderer output for ${step.title.toLowerCase()}.](../screenshots/journey/${picture}.png)`,
                '*Read directly from this checkpoint’s GPU texture. Browser controls and event delivery remain unverified until the browser check passes.*');
        }
        if (step.tests) page.push('Run the state checks from your project folder:', '```sh\ncargo test --lib --locked -j4\n```');
        page.push('## Try one small experiment', step.experiment, '## Explain it in your own words',
            'Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.',
            `<details>\n<summary>Compare your explanation</summary>\n\n${step.answer}\n\n</details>`,
            '## Keep your working result', 'Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:',
            `\`\`\`sh\n${command} check ${step.id}\n${command} save ${step.id}\n\`\`\``,
            'The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.',
            '## Where this grows', step.production, '[Validation status and course release](release.md).');
        write(path.join(docs, step.page), page.join('\n\n') + '\n');
    }
    const hours = steps.reduce((sum, step) => sum.map((n, i) => n + step.hours[i]), [0, 0]);
    const overview = path.join(docs, 'journey.md');
    const planned = [...read(path.join(docs, 'journey/roadmap.md'))
        .matchAll(/^- \[(?:x| )\] \d+[a-z]? · /gm)].length;
    if (planned < steps.length) throw Error('Roadmap must include every available lesson');
    const table = ['| Lesson | Time | Working result |', '| --- | --- | --- |',
        ...steps.map((step, i) => `| [${step.id.split('-')[0]} · ${step.title}](${step.page}) | ${step.hours.join('–')} hours | ${step.goal} |`)];
    write(overview, read(overview)
        .replace(/^The \[complete draft lesson checklist\].+$/m,
            `The [complete draft lesson checklist](journey/roadmap.md) has **${planned} proposed slots: ${steps.length} current checkpoints and ${planned - steps.length} later slots planned**. This is a teaching plan, not a fixed final count or percentage of engineering work.`)
        .replace(/\*\*Current release:[^\n]+/, `**Current release: ${steps.length} cumulative lessons, about ${hours.join('–')} active study hours.** Installation is extra. These estimates include reading, typing and experiments. Check the [release evidence](journey/release.md) before starting. The complete feature course is still being written.`)
        .replace(/\| Lesson \|[\s\S]*?(?=\n\n)/, table.join('\n')));
    const audit = path.join(docs, 'journey/typing-load.md');
    const rows = steps.map(step => {
        const typing = typingLoad(step);
        return `| [${step.id}](${step.id}.md) | ${typing.lines} | ${typing.characters} | ${typing.minutes.join('–')} min | ${step.hours.join('–')} h | ${typing.minutes[1] <= 60 ? 'Within planning limit' : 'Split required'} |`;
    });
    write(audit, read(audit)
        .replace(/all \d+ current checkpoints/, `all ${steps.length} current checkpoints`)
        .replace(/the current \d+ slots/, `the current ${planned} slots`)
        .replace(/\| Checkpoint \|[\s\S]*?(?=\n\n)/,
        ['| Checkpoint | Added/changed lines | Characters | Typing estimate | Existing combined study estimate | Typing limit |',
            '| --- | ---: | ---: | --- | --- | --- |', ...rows].join('\n')));
    console.log(`Generated ${steps.length} lessons from their source edits.`);
}
