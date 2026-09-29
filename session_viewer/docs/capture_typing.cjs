const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const {spawn} = require('node:child_process');
const {once} = require('node:events');
const readline = require('node:readline');
const {chromium} = require('playwright');
const data = JSON.parse(fs.readFileSync('docs/typing/manifest.json', 'utf8'));
const output = 'docs/screenshots/typing';
const previous = fs.existsSync(`${output}/captures.json`) ? JSON.parse(fs.readFileSync(`${output}/captures.json`, 'utf8')) : {};
const cached = previous.encoding === 'lossless-webp-v1' ? previous.lessons : {};
const hash = text => crypto.createHash('sha256').update(text).digest('hex');
const escape = text => text.replaceAll('&', '&amp;').replaceAll('<', '&lt;').replaceAll('>', '&gt;');
const lines = text => text.match(/[^\n]*\n|[^\n]+$/g) || [];
fs.mkdirSync(output, {recursive: true});

(async () => {
  const browser = await chromium.launch({channel: 'chrome', headless: true});
  const jobs = [];
  const worker = async () => {
  const encoder = spawn('python3', ['-u', '-c', 'import sys,json,io,base64\nfrom PIL import Image\nfor line in sys.stdin:\n row=json.loads(line)\n Image.open(io.BytesIO(base64.b64decode(row["png"]))).convert("RGB").save(row["path"],"WEBP",lossless=True,method=6)\n print("saved",flush=True)']);
  const replies = readline.createInterface({input: encoder.stdout});
  encoder.stderr.pipe(process.stderr);
  const page = await browser.newPage({viewport: {width: 1120, height: 900}, deviceScaleFactor: 1});
  const capture = async (id, name, before, code, result) => {
    await page.setContent(`<style>*{box-sizing:border-box}body{margin:0;padding:24px;background:#eff1f5;color:#172238;font:16px/1.5 system-ui}main{padding:24px;background:white;border:1px solid #d5dae2}h1{font-size:23px;margin:0 0 8px}h2{font-size:18px;margin:20px 0 6px}p{overflow-wrap:anywhere}pre{white-space:pre-wrap;overflow-wrap:anywhere;font:13px/1.5 monospace;background:#f5f6f8;padding:14px;border-left:3px solid #75829b;margin:8px 0}footer{font-size:13px;color:#536078}</style><main><h1>${escape(id)} · ${escape(name)}</h1><h2>Step 1 · Before this addition</h2><pre>${escape(before)}</pre><h2>Step 2 · Type the complete addition</h2><pre>${escape(code || '(empty file)')}</pre><h2>Step 3 · Recorded reference check</h2><pre>${escape(result)}</pre><footer>Reference workbook captured in Chrome. Use the selectable lesson listing to type; this image is for comparison.</footer></main>`);
    const png = await page.locator('main').screenshot();
    const done = once(replies, 'line');
    encoder.stdin.write(JSON.stringify({path: path.join(output, `${id}.webp`), png: png.toString('base64')}) + '\n');
    await done;
  };
  while (jobs.length) await capture(...jobs.shift());
  encoder.stdin.end();
  await once(encoder, 'exit');
  await page.close();
  };
  const records = {encoding: 'lossless-webp-v1', browser: browser.version(), captured: new Date().toISOString(), description: 'Browser workbook captures. Each contains three reference steps: insertion context, full typed addition, and actual source-replay comparison. Source equality is not a compiler or runtime result.', lessons: {}};
  const files = new Map();
  for (const lesson of data.lessons) {
    const before = files.get(lesson.path) || '';
    if (hash(before) !== lesson.before) throw Error(lesson.id + ': bad starting state');
    const source = fs.readFileSync(path.join('docs/typing', lesson.snippet), 'utf8');
    if (source !== lesson.code) throw Error(lesson.id + ': listing differs');
    const current = lines(before);
    current.splice(lesson.at, 0, ...lines(source));
    const after = current.join('');
    if (hash(after) !== lesson.after) throw Error(lesson.id + ': replay differs');
    files.set(lesson.path, after);
    const ready = (lesson.context || '(start of file)\n') + '\n[insert here]\n\n' + (lesson.after_context || '(end of file)');
    const result = `${lesson.id}: source replay matches the expected addition.\n${files.size} cumulative files tracked; all previous additions verified.\nNo source was supplied between steps.\nCompilation and behavior checks belong to the chapter checkpoint.`;
    records.lessons[lesson.id] = {source: hash(source), before: lesson.before, after: lesson.after};
    if (JSON.stringify(cached[lesson.id]) !== JSON.stringify(records.lessons[lesson.id]) || !fs.existsSync(`${output}/${lesson.id}.webp`))
      jobs.push([lesson.id, lesson.path, ready, source, result]);
  }
  for (const [name, expected] of Object.entries(data.files)) {
    if (hash(files.get(name)) !== expected) throw Error(name + ': final source differs');
  }
  for (const name of fs.readdirSync('docs/foundations/code').filter(name => name.endsWith('.rs')).sort()) {
    const id = name.slice(0, -3);
    const source = fs.readFileSync(`docs/foundations/code/${name}`, 'utf8');
    const log = fs.readFileSync(`target/full-course/foundations/${id}.json`, 'utf8');
    const result = JSON.parse(log);
    if (result.source !== hash(source) || result.compile !== 0 || result.run !== 0) throw Error(id + ': foundation check failed or stale');
    jobs.push([`rust-${id}`, name, 'Create an empty file in target/notebook.\nPredict the assertions before typing.', source, `$ rustc --edition 2024 ${name}\nCompiler exit: ${result.compile}\n$ ./${id}\nProgram exit: ${result.run}\nAll assertions passed. No output was expected.`]);
    records.lessons[`rust-${id}`] = {source: hash(source), compile: result.compile, run: result.run};
  }
  await Promise.all(Array.from({length: 4}, worker));
  fs.writeFileSync(path.join(output, 'captures.json'), JSON.stringify(records, null, 2) + '\n');
  await browser.close();
})().catch(error => { console.error(error); process.exit(1); });

// From session_viewer after foundation checks: NODE_PATH=target/course-tools/node_modules node docs/capture_typing.cjs.
