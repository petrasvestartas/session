import fs from 'node:fs';
import path from 'node:path';
import {course, docs, read, json, write} from './model.ts';
import {signature} from './verify.ts';

const language: Record<string, string> = {'.rs': 'rust', '.wgsl': 'wgsl', '.html': 'html', '.toml': 'toml', '.md': 'markdown', '.sh': 'sh', '.yaml': 'yaml'};
const command = 'npm --prefix ../session_tests run course --';

export function generate() {
    const steps = course().steps;
    const capture = path.join(docs, 'screenshots/journey/browser.json');
    const browser = fs.existsSync(capture) ? json(capture).steps : {};
    for (const [index, step] of steps.entries()) {
        const [low, high] = step.hours;
        const count = step.edits.reduce((n, edit) => n + read(path.join(docs, edit.snippet)).split('\n').length - 1, 0);
        const page = [`# ${String(index + 1).padStart(2, '0')} · ${step.title}`,
            `**Plan about ${low}–${high} hours.** ${count} lines to type, including comments and blank lines. Allow time to read, predict and experiment; this is an estimate, not a deadline.`,
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
        if (step.lock) page.push('After typing the manifest, run this from `session_viewer` to select the fixed dependency versions. It updates only Cargo.lock and preserves the previous lock:',
            `\`\`\`sh\n${command} dependencies ${step.id}\n\`\`\``);
        page.push('From `session_viewer`, enter your project folder:',
            '```sh\ncd workspace/journey\ncargo build --lib --locked --target wasm32-unknown-unknown -j4\nCARGO_BUILD_JOBS=4 trunk serve --port 8780\n```',
            'Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.', step.result);
        const evidence = browser[step.id];
        if (evidence && evidence.source === signature(step.id)) {
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
    const table = ['| Lesson | Time | Working result |', '| --- | --- | --- |',
        ...steps.map((step, i) => `| [${String(i + 1).padStart(2, '0')} · ${step.title}](${step.page}) | ${step.hours.join('–')} hours | ${step.goal} |`)];
    write(overview, read(overview)
        .replace(/\*\*Current release:[^\n]+/, `**Current release: ${steps.length} cumulative lessons, about ${hours.join('–')} active study hours.** Installation is extra. These estimates include reading, typing and experiments. Check the [release evidence](journey/release.md) before starting. The complete feature course is still being written.`)
        .replace(/\| Lesson \|[\s\S]*?(?=\n\n)/, table.join('\n')));
    console.log(`Generated ${steps.length} lessons from their source edits.`);
}
