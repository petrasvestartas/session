<template>
  <div class="test-layout">
    <main class="test-main">
      <table v-if="groupedTests.length" :class="{ single: narrow }">
        <thead v-if="!narrow">
          <tr>
            <th>
              <a href="https://github.com/petrasvestartas/session_cpp" target="_blank" class="lang-link">
                <img :src="base + 'icons/lang_cpp.svg'" class="lang-icon" alt="C++" title="C++">
              </a>
            </th>
            <th>
              <a href="https://github.com/petrasvestartas/session_py" target="_blank" class="lang-link">
                <img :src="base + 'icons/lang_py.svg'" class="lang-icon" alt="Python" title="Python">
              </a>
            </th>
            <th>
              <a href="https://github.com/petrasvestartas/session_rust" target="_blank" class="lang-link">
                <img :src="base + 'icons/lang_rust.svg'" class="lang-icon" alt="Rust" title="Rust">
              </a>
            </th>
          </tr>
        </thead>
        <tbody>
          <template v-for="g in groupedTests" :key="g.name">
            <tr class="test-name-row" :id="'test-' + g.name">
              <td :colspan="narrow ? 1 : 3">
                <strong>{{ g.name }}</strong>
                <div v-if="narrow" class="lang-tabs" role="tablist" aria-label="Language" @keydown="onTabKey">
                  <button
                    v-for="l in LANGS" :key="l.id"
                    type="button"
                    role="tab"
                    class="lang-tab"
                    :class="{ active: lang === l.id, fail: g[l.id] && !g[l.id].passed }"
                    :aria-selected="lang === l.id"
                    :tabindex="lang === l.id ? 0 : -1"
                    @click="setLang(l.id)">
                    {{ l.label }}<span v-if="g[l.id] && !g[l.id].passed" aria-label="failed"> ✗</span>
                  </button>
                </div>
              </td>
            </tr>
            <tr>
            <!-- C++ column -->
            <td v-if="shows('cpp')" class="lang-col">
              <div v-if="g.cpp" class="test-card">
                <div :class="['tag', g.cpp.passed ? 'tag-pass' : 'tag-fail']" :style="timeStyle(g, 'cpp')">
                  {{ g.cpp.passed ? '✓' : '✗' }} {{ formatTime(g.cpp.time_ms) }} ms
                </div>
                <div v-if="g.cpp.code" class="code-shell">
                  <button
                    class="code-copy-btn"
                    type="button"
                    @click="copyCode(g.cpp)"
                    title="Copy code"
                    aria-label="Copy code"
                  >
                  </button>
                  <div v-html="highlightedCode(g.cpp)"></div>
                </div>
                <div class="failures" v-if="!g.cpp.passed">
                  <div><strong>Failing checks:</strong></div>
                  <ul>
                    <li v-for="c in failingChecks(g.cpp)" :key="'cpp-' + g.name + ':' + c.line">
                      line {{ c.line }}: <span class="inline-code" v-html="highlightedCheck(c, 'cpp')"></span>
                    </li>
                  </ul>

                  <div v-if="hasFailures(g.cpp)" class="exceptions">
                    <div><strong>Errors / Exceptions:</strong></div>
                    <ul>
                      <li
                        v-for="f in errorFailures(g.cpp)"
                        :key="'cpp-err-' + g.name + ':' + (f.line || 0) + ':' + (f.file || '')"
                      >
                        <div v-if="f.file">at {{ f.file }}<span v-if="f.line">:{{ f.line }}</span></div>
                        <div v-if="f.code_line">
                          <span class="inline-code" v-html="highlightedFailureCode(f, 'cpp')"></span>
                        </div>
                        <div class="error-message" v-if="f.error">{{ f.error }}</div>
                      </li>
                    </ul>
                  </div>
                </div>
              </div>
              <div v-else class="missing">–</div>
            </td>

            <!-- Python column -->
            <td v-if="shows('python')" class="lang-col">
              <div v-if="g.python" class="test-card">
                <div :class="['tag', g.python.passed ? 'tag-pass' : 'tag-fail']" :style="timeStyle(g, 'python')">
                  {{ g.python.passed ? '✓' : '✗' }} {{ formatTime(g.python.time_ms) }} ms
                </div>
                <div v-if="g.python.code" class="code-shell">
                  <button
                    class="code-copy-btn"
                    type="button"
                    @click="copyCode(g.python)"
                    title="Copy code"
                    aria-label="Copy code"
                  >
                  </button>
                  <div v-html="highlightedCode(g.python)"></div>
                </div>
                <div class="failures" v-if="!g.python.passed">
                  <div><strong>Failing checks:</strong></div>
                  <ul>
                    <li v-for="c in failingChecks(g.python)" :key="'py-' + g.name + ':' + c.line">
                      line {{ c.line }}: <span class="inline-code" v-html="highlightedCheck(c, 'python')"></span>
                    </li>
                  </ul>

                  <div v-if="hasFailures(g.python)" class="exceptions">
                    <div><strong>Errors / Exceptions:</strong></div>
                    <ul>
                      <li
                        v-for="f in errorFailures(g.python)"
                        :key="'py-err-' + g.name + ':' + (f.line || 0) + ':' + (f.file || '')"
                      >
                        <div v-if="f.file">at {{ f.file }}<span v-if="f.line">:{{ f.line }}</span></div>
                        <div v-if="f.code_line">
                          <span class="inline-code" v-html="highlightedFailureCode(f, 'python')"></span>
                        </div>
                        <div class="error-message" v-if="f.error">{{ f.error }}</div>
                      </li>
                    </ul>
                  </div>
                </div>
              </div>
              <div v-else class="missing">–</div>
            </td>

            <!-- Rust column -->
            <td v-if="shows('rust')" class="lang-col">
              <div v-if="g.rust" class="test-card">
                <div :class="['tag', g.rust.passed ? 'tag-pass' : 'tag-fail']" :style="timeStyle(g, 'rust')">
                  {{ g.rust.passed ? '✓' : '✗' }} {{ formatTime(g.rust.time_ms) }} ms
                </div>
                <div v-if="g.rust.code" class="code-shell">
                  <button
                    class="code-copy-btn"
                    type="button"
                    @click="copyCode(g.rust)"
                    title="Copy code"
                    aria-label="Copy code"
                  >
                  </button>
                  <div v-html="highlightedCode(g.rust)"></div>
                </div>
                <div class="failures" v-if="!g.rust.passed">
                  <div><strong>Failing checks:</strong></div>
                  <ul>
                    <li v-for="c in failingChecks(g.rust)" :key="'rs-' + g.name + ':' + c.line">
                      line {{ c.line }}: <span class="inline-code" v-html="highlightedCheck(c, 'rust')"></span>
                    </li>
                  </ul>

                  <div v-if="hasFailures(g.rust)" class="exceptions">
                    <div><strong>Errors / Exceptions:</strong></div>
                    <ul>
                      <li
                        v-for="f in errorFailures(g.rust)"
                        :key="'rs-err-' + g.name + ':' + (f.line || 0) + ':' + (f.file || '')"
                      >
                        <div v-if="f.file">at {{ f.file }}<span v-if="f.line">:{{ f.line }}</span></div>
                        <div v-if="f.code_line">
                          <span class="inline-code" v-html="highlightedFailureCode(f, 'rust')"></span>
                        </div>
                        <div class="error-message" v-if="f.error">{{ f.error }}</div>
                      </li>
                    </ul>
                  </div>
                </div>
              </div>
              <div v-else class="missing">–</div>
            </td>
          </tr>
          </template>
        </tbody>
      </table>

      <div v-else class="no-results">
        No kernel test results yet. They appear after the next <code>Session mini tests</code> run on main;
        locally, run <code>bash/minitest.sh</code>.
      </div>

      <!-- JSON Artifacts Section -->
      <div v-if="hasArtifacts" class="artifacts-section">
        <h3 class="section-title">Serialization JSON</h3>
        <table :class="{ single: narrow }">
          <thead>
            <tr>
              <th v-if="shows('cpp')">C++</th>
              <th v-if="shows('python')">Python</th>
              <th v-if="shows('rust')">Rust</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td v-if="shows('cpp')" class="lang-col">
                <div v-if="artifacts.cpp" class="artifact-card">
                  <button class="code-copy-btn" type="button" @click="copyJson(artifacts.cpp)" title="Copy JSON"></button>
                  <div v-html="formatJson(artifacts.cpp)"></div>
                </div>
                <div v-else class="missing">–</div>
              </td>
              <td v-if="shows('python')" class="lang-col">
                <div v-if="artifacts.python" class="artifact-card">
                  <button class="code-copy-btn" type="button" @click="copyJson(artifacts.python)" title="Copy JSON"></button>
                  <div v-html="formatJson(artifacts.python)"></div>
                </div>
                <div v-else class="missing">–</div>
              </td>
              <td v-if="shows('rust')" class="lang-col">
                <div v-if="artifacts.rust" class="artifact-card">
                  <button class="code-copy-btn" type="button" @click="copyJson(artifacts.rust)" title="Copy JSON"></button>
                  <div v-html="formatJson(artifacts.rust)"></div>
                </div>
                <div v-else class="missing">–</div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- Proto Schema Section -->
      <div v-if="hasProtoSchemas" class="artifacts-section">
        <h3 class="section-title">Serialization Protobuf</h3>
        <div class="artifact-card">
          <button class="code-copy-btn" type="button" @click="copyProto(protoSchemas[0]?.content)" title="Copy Proto"></button>
          <div v-html="formatProto(protoSchemas[0]?.content)"></div>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, onMounted, onBeforeUnmount } from 'vue'
import type { HighlighterCore } from 'shiki/core'
import { getHighlighter } from '../highlighter'
import { renderCode, uncommentUsing } from '../codeTheme'

const base = import.meta.env.BASE_URL

const props = defineProps({
  tests: { type: Array, required: true },
  activeSuite: { type: String, required: true }
})

defineEmits(['update:activeSuite'])

// Narrow screens show one language at a time; the choice holds for every test and is remembered.
const LANGS = [
  { id: 'cpp', label: 'C++' },
  { id: 'python', label: 'Python' },
  { id: 'rust', label: 'Rust' },
]
const LANG_KEY = 'session-docs-lang'
const narrowQuery = window.matchMedia('(max-width: 900px)')
const narrow = ref(narrowQuery.matches)
const onNarrow = (e: MediaQueryListEvent) => (narrow.value = e.matches)
onMounted(() => narrowQuery.addEventListener('change', onNarrow))
onBeforeUnmount(() => narrowQuery.removeEventListener('change', onNarrow))

const readLang = (): string => {
  try {
    const v = localStorage.getItem(LANG_KEY)
    return v && LANGS.some((l) => l.id === v) ? v : 'cpp'
  } catch {
    return 'cpp'
  }
}

const lang = ref(readLang())
const shows = (id: string) => !narrow.value || lang.value === id

const setLang = (id: string) => {
  lang.value = id
  try {
    localStorage.setItem(LANG_KEY, id)
  } catch { /* storage blocked */ }
}

// Arrow keys move between the tabs of one test, as in a tab list.
const onTabKey = (e: KeyboardEvent) => {
  if (e.key !== 'ArrowRight' && e.key !== 'ArrowLeft') return
  const i = LANGS.findIndex((l) => l.id === lang.value)
  const next = LANGS[(i + (e.key === 'ArrowRight' ? 1 : LANGS.length - 1)) % LANGS.length]
  setLang(next.id)
  const tabs = (e.currentTarget as HTMLElement).querySelectorAll<HTMLElement>('[role="tab"]')
  tabs[LANGS.indexOf(next)]?.focus()
  e.preventDefault()
}

// Syntax highlighting via Shiki (loaded once; replaces the tree-sitter wasm highlighter).
const ready = ref(false)
const hl = ref<HighlighterCore | null>(null)
const LANG_MAP: Record<string, string> = { cpp: 'cpp', python: 'python', rust: 'rust', json: 'json' }

const escapeHtml = (str: string): string => {
  return str.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;')
}

const KEYWORDS: Record<string, Set<string>> = {
  cpp: new Set(['if','else','for','while','do','return','class','struct','enum','namespace','using','template','typename','public','private','protected','virtual','const','static','inline','new','delete','try','catch','throw','void','int','double','float','bool','char','auto','sizeof','constexpr','override','explicit','extern','volatile','mutable','friend','operator','switch','case','default','break','continue','typedef','union','noexcept','nullptr','true','false','this','#include','#define','#ifdef','#ifndef','#endif','#if','co_await','co_return','co_yield','concept','requires','static_assert','static_cast','dynamic_cast','reinterpret_cast','const_cast']),
  python: new Set(['def','class','return','if','elif','else','for','while','break','continue','pass','import','from','as','with','try','except','finally','raise','yield','lambda','global','nonlocal','assert','del','in','not','and','or','is','async','await','True','False','None','self']),
  rust: new Set(['fn','let','mut','pub','struct','enum','impl','trait','use','mod','crate','super','match','if','else','for','while','loop','break','continue','return','as','const','static','type','where','unsafe','async','await','move','ref','dyn','extern','in','self','Self','true','false']),
  json: new Set(),
  proto: new Set(['syntax','message','enum','service','rpc','returns','option','import','package','repeated','optional','required','oneof','map','reserved','extensions','extend','stream','true','false','double','float','int32','int64','uint32','uint64','sint32','sint64','fixed32','fixed64','sfixed32','sfixed64','bool','string','bytes']),
}

// Context-aware gap tokenizer: detects functions, methods, keywords, types
const highlightGap = (text: string, lang: string): string => {
  const kw = KEYWORDS[lang] || new Set<string>()
  const re = /(#?\w+)|([(){}\[\]])|([,;])|([+\-*/%=!<>&|^~?:.@#]+)|(\s+)/g
  const tokens = []
  let m
  while ((m = re.exec(text)) !== null) {
    tokens.push({ text: m[0], word: m[1], bracket: m[2], delim: m[3], op: m[4], ws: m[5] })
  }
  let result = ''
  for (let i = 0; i < tokens.length; i++) {
    const t = tokens[i]
    if (t.ws) { result += t.ws; continue }
    if (t.bracket) { result += `<span class="ho">${escapeHtml(t.bracket)}</span>`; continue }
    if (t.delim) { result += `<span class="ho">${escapeHtml(t.delim)}</span>`; continue }
    if (t.op) { result += `<span class="ho">${escapeHtml(t.op)}</span>`; continue }
    if (t.word) {
      // Look ahead past whitespace for (
      let nextSym = null
      for (let j = i + 1; j < tokens.length; j++) {
        if (!tokens[j].ws) { nextSym = tokens[j]; break }
      }
      const followedByParen = nextSym && nextSym.bracket === '('
      // Look back past whitespace for . or :: or ->
      let prevSym = null
      for (let j = i - 1; j >= 0; j--) {
        if (!tokens[j].ws) { prevSym = tokens[j]; break }
      }
      const afterDot = prevSym && prevSym.op && /^(::|\.|->) *$/.test(prevSym.op)
      // Look back for import/module keywords (from X, use X, namespace X)
      let prevWord = null
      for (let j = i - 1; j >= 0; j--) {
        if (tokens[j].word) { prevWord = tokens[j].word; break }
        if (!tokens[j].ws) break
      }
      const MODULE_KW = new Set(['from','import','use','mod','crate','namespace','using','package'])
      const afterModuleKw = prevWord && MODULE_KW.has(prevWord)

      const isPascal = /^[A-Z][a-zA-Z0-9]+$/.test(t.word)

      if (kw.has(t.word)) {
        result += `<span class="hk">${escapeHtml(t.word)}</span>`
      } else if (afterModuleKw && !followedByParen) {
        result += `<span class="hu">${escapeHtml(t.word)}</span>`
      } else if (isPascal) {
        result += `<span class="ht">${escapeHtml(t.word)}</span>`
      } else if (followedByParen && afterDot) {
        result += `<span class="hf">${escapeHtml(t.word)}</span>`
      } else if (followedByParen) {
        result += `<span class="hf">${escapeHtml(t.word)}</span>`
      } else if (/^[A-Z][A-Z0-9_]+$/.test(t.word)) {
        result += `<span class="hn">${escapeHtml(t.word)}</span>`
      } else if (/^\d/.test(t.word)) {
        result += `<span class="hn">${escapeHtml(t.word)}</span>`
      } else if (lang === 'proto' && nextSym && nextSym.op === '=') {
        result += `<span class="hm">${escapeHtml(t.word)}</span>`
      } else {
        result += escapeHtml(t.word)
      }
      continue
    }
    result += escapeHtml(t.text)
  }
  return result
}

// Highlight a snippet → inner token HTML with the site code classes (codeTheme.ts). Falls back to
// escaped text until the highlighter loads.
const highlight = (code: string, lang: string): string => {
  const h = hl.value
  if (!h) return escapeHtml(code)
  try {
    const id = LANG_MAP[lang]
    return id ? renderCode(h, code, id) : escapeHtml(code)
  } catch {
    return escapeHtml(code)
  }
}

onMounted(async () => {
  try {
    hl.value = await getHighlighter()
    ready.value = true
  } catch (e) {
    console.error('shiki init failed:', e)
  }
})

const suites = computed(() => {
  const set = new Set()
  for (const t of props.tests) {
    if (t.suite) set.add(t.suite)
  }
  return Array.from(set.values())
})

const groupedTests = computed(() => {
  const byName = new Map()
  for (const t of props.tests) {
    if (t.suite !== props.activeSuite) continue
    const name = t.test_name || "(unnamed)"
    if (!byName.has(name)) {
      byName.set(name, { name, python: null, cpp: null, rust: null })
    }
    const entry = byName.get(name)
    if (t.language === "python") entry.python = t
    if (t.language === "cpp") entry.cpp = t
    if (t.language === "rust") entry.rust = t
  }
  return Array.from(byName.values())
})

const formatTime = (time_ms) => {
  return typeof time_ms === 'number' && time_ms.toFixed ? time_ms.toFixed(3) : time_ms
}

const normalizeForDisplay = (code) => {
  if (!code) return ""
  const lines = code.split('\n').map((line) => {
    const m = line.match(/^(\s*)\/\/\s*uncomment\s+(.*)$/)
    if (m) return m[1] + m[2]
    return line
  })
  let minIndent = Infinity
  for (const line of lines) {
    if (!line.trim()) continue
    const m = line.match(/^(\s*)/)
    const indent = m ? m[1].length : 0
    if (indent < minIndent) minIndent = indent
  }
  if (!Number.isFinite(minIndent) || minIndent === 0) return lines.join('\n').replace(/(\n\s*)+$/, '')
  return lines.map((line) => (line.length >= minIndent ? line.slice(minIndent) : line)).join('\n').replace(/(\n\s*)+$/, '')
}

// C++ bodies show their leading `// using session_cpp::X;` lines as real using statements.
const displayCode = (t) => normalizeForDisplay(t.language === 'cpp' ? uncommentUsing(t.code) : t.code)

const highlightedCode = (t) => {
  if (!t || !t.code) return ""
  const code = displayCode(t)
  const lang = t.language || ""
  if (!ready.value || !lang) return `<pre><code>${escapeHtml(code)}</code></pre>`
  return `<pre><code>${highlight(code, lang)}</code></pre>`
}

const highlightedCheck = (check, lang) => {
  if (!check || !check.code_line) return ""
  if (!ready.value) return escapeHtml(check.code_line)
  return highlight(check.code_line, lang)
}

const timeStyle = (group, lang) => {
  const t = group[lang]
  if (!t || typeof t.time_ms !== "number") return {}
  if (!t.passed) return { color: 'var(--fail)' }
  const times = [group.python, group.cpp, group.rust]
    .filter(x => x && typeof x.time_ms === "number" && x.passed)
    .map(x => x.time_ms)
  if (!times.length) return { color: '#000000' }
  const min = Math.min(...times)
  const max = Math.max(...times)
  if (max === min) return { color: '#000000' }
  const value = t.time_ms
  let ratio = (value - min) / (max - min)
  if (ratio < 0) ratio = 0
  if (ratio > 1) ratio = 1
  // Fastest black, slowest light grey.
  const v = Math.round(0x9a * ratio)
  return { color: `rgb(${v}, ${v}, ${v})` }
}

const failingChecks = (t) => {
  if (!t.checks) return []
  return t.checks.filter(c => c && c.passed === false)
}

const hasFailures = (t) => {
  return !!(t && Array.isArray(t.failures) && t.failures.length > 0)
}

const errorFailures = (t) => {
  if (!t || !Array.isArray(t.failures)) return []
  return t.failures
}

const highlightedFailureCode = (failure, lang) => {
  if (!failure || !failure.code_line) return ""
  if (!ready.value) return escapeHtml(failure.code_line)
  return highlight(failure.code_line, lang)
}

const copyCode = (t) => {
  if (!t || !t.code) return
  const text = displayCode(t)
  try {
    if (typeof navigator !== 'undefined' && navigator.clipboard && navigator.clipboard.writeText) {
      navigator.clipboard.writeText(text)
    }
  } catch (e) { /* ignore */ }
}

const artifacts = computed(() => {
  if (typeof window.TEST_DATA === 'undefined') return { python: null, cpp: null, rust: null }
  const data = window.TEST_DATA
  const suiteName = props.activeSuite.replace('_test', '')
  const artifactName = `test_${suiteName}`
  return {
    python: data[`artifact_${artifactName}_python`] || null,
    cpp: data[`artifact_${artifactName}_cpp`] || null,
    rust: data[`artifact_${artifactName}_rust`] || null
  }
})

const hasArtifacts = computed(() => {
  return artifacts.value.python || artifacts.value.cpp || artifacts.value.rust
})

const formatJson = (obj) => {
  if (!obj) return ''
  try {
    const jsonStr = JSON.stringify(obj, null, 2)
    if (!ready.value) return `<pre><code>${escapeHtml(jsonStr)}</code></pre>`
    return `<pre><code>${highlight(jsonStr, 'json')}</code></pre>`
  } catch (e) {
    return escapeHtml(String(obj))
  }
}

const protoSchemas = computed(() => {
  if (typeof window.TEST_DATA === 'undefined') return []
  const data = window.TEST_DATA
  const suiteName = props.activeSuite.replace('_test', '')
  const schemas = []
  if (data[`proto_${suiteName}`]) {
    schemas.push({ name: `${suiteName}.proto`, content: data[`proto_${suiteName}`] })
  }
  return schemas
})

const hasProtoSchemas = computed(() => protoSchemas.value.length > 0)

const formatProto = (content) => {
  if (!content) return ''
  const decoded = content.replace(/\\n/g, '\n').replace(/\\r/g, '')
  const highlighted = decoded.split('\n').map(line => {
    const commentIdx = line.indexOf('//')
    if (commentIdx >= 0) {
      const before = line.slice(0, commentIdx)
      const comment = line.slice(commentIdx)
      return highlightGap(before, 'proto') + `<span class="hc">${escapeHtml(comment)}</span>`
    }
    return highlightGap(line, 'proto')
  }).join('\n')
  return `<pre><code>${highlighted}</code></pre>`
}

const copyJson = (obj) => {
  if (!obj) return
  try {
    const text = JSON.stringify(obj, null, 2)
    if (typeof navigator !== 'undefined' && navigator.clipboard && navigator.clipboard.writeText) {
      navigator.clipboard.writeText(text)
    }
  } catch (e) { /* ignore */ }
}

const copyProto = (content) => {
  if (!content) return
  try {
    const decoded = content.replace(/\\n/g, '\n').replace(/\\r/g, '')
    if (typeof navigator !== 'undefined' && navigator.clipboard && navigator.clipboard.writeText) {
      navigator.clipboard.writeText(decoded)
    }
  } catch (e) { /* ignore */ }
}
</script>

<style scoped>
/* Test viewer styles: white page, black text, greys. */
.test-layout {
  display: flex;
  height: 100%;
}
.sidebar {
  width: 180px;
  flex-shrink: 0;
  background: #ffffff;
}
.sidebar-title {
  font-weight: 600;
  margin-bottom: 0.5rem;
  font-size: 16px;
  color: var(--fg);
}
.suite-pill {
  display: block;
  padding: 0.75rem 1rem;
  margin: 0;
  border-radius: 0;
  border: none;
  border-left: 3px solid transparent;
  background: transparent;
  color: var(--muted);
  font-weight: 600;
  cursor: pointer;
  transition: background 0.15s ease, color 0.15s ease, border-color 0.15s ease;
}
.suite-pill:hover {
  background: var(--hover);
  color: var(--fg);
}
.suite-pill.active {
  background: var(--hover);
  color: var(--fg);
  border-left-color: var(--fg);
}
.test-main {
  flex: 1;
  overflow-y: auto;
}

table {
  width: 100%;
  border-collapse: collapse;
  background: #ffffff;
  box-shadow: none;
  table-layout: fixed;
}
th, td {
  padding: 0.5rem 0.75rem;
  border: none;
  vertical-align: top;
  color: #1a1a1a;
}
th {
  background: #ffffff;
  text-align: left;
  font-weight: 600;
  font-size: 14px;
  color: var(--fg);
  border: none;
}

.lang-icon {
  width: 24px;
  height: 24px;
  font-size: 24px;
  color: var(--fg);
}

.lang-link {
  color: var(--fg);
  text-decoration: none;
  transition: color 0.2s;
}

.lang-link:hover {
  opacity: 0.5;
}

.lang-text {
  font-size: 24px;
  font-weight: 700;
  color: var(--fg);
}
.tag {
  display: inline-block;
  padding: 0.2rem 0;
  font-weight: 600;
  font-size: 0.85rem;
  background: none;
  border: none;
}
.tag-pass {
  color: var(--muted);
}
.tag-fail {
  color: var(--fail);
}
pre {
  margin: 0;
  background: transparent;
  border-radius: 0;
  padding: 0;
}
.test-card :deep(pre code) {
  white-space: pre-wrap;
  word-wrap: break-word;
  overflow-wrap: anywhere;
  color: #1a1a1a;
}
.code-shell {
  position: relative;
  margin: 0.25rem 0 0.5rem 0;
  background: var(--code-bg);
  border-radius: 4px;
  border: none;
}

.code-shell :deep(pre) {
  margin: 0;
  padding: 0.75rem;
  background: transparent !important;
}
.code-shell :deep(code) {
  white-space: pre-wrap;
  word-wrap: break-word;
  overflow-wrap: anywhere;
}

.code-copy-btn {
  position: absolute;
  top: 4px;
  right: 4px;
  width: 12px;
  height: 12px;
  padding: 0;
  border: none;
  border-radius: 50%;
  background: #d0d0d0;
  cursor: pointer;
}

.code-copy-btn:hover {
  background: #999999;
}
.failures {
  margin-top: 0.35rem;
  color: var(--fail);
}
.exceptions {
  margin-top: 0.35rem;
}
.error-message {
  margin-top: 0.15rem;
  font-family: var(--mono);
  color: var(--fail);
}
.test-card {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}
.test-card .tag {
  align-self: flex-end;
}
.missing {
  font-size: 0.8rem;
  color: var(--faint);
  font-style: italic;
}

.no-results {
  padding: 1rem 0.75rem;
  color: var(--muted);
}
.test-name-row td {
  padding-top: 0.75rem;
  font-weight: 600;
  color: var(--fg);
  background: #ffffff;
}
.lang-col {
  width: 33.33%;
}
.artifacts-section {
  margin-top: 2rem;
  padding-top: 1rem;
  border: none;
}
.section-title {
  background: #ffffff;
  color: var(--fg);
  padding: 0;
  margin: 0 0 0.5rem 0;
  font-size: 14px;
  font-weight: 600;
  border-radius: 0;
  border: none;
  text-align: center;
}
.artifacts-section h3 {
  margin: 0 0 1rem 0;
}
.artifact-card {
  position: relative;
  background: #ffffff;
  border-radius: 0;
  padding: 0.5rem 0;
  border: none;
}
.artifact-card :deep(pre) {
  margin: 0;
  font-size: 0.85rem;
  background: transparent !important;
}
.artifact-card :deep(code) {
  white-space: pre-wrap;
  word-wrap: break-word;
  color: #1a1a1a;
}

.lang-tabs {
  display: flex;
  gap: 0;
  margin-top: 0.4rem;
  border-bottom: 1px solid var(--rule);
}

.lang-tab {
  padding: 0.35rem 0.8rem;
  margin-bottom: -1px;
  background: transparent;
  border: none;
  border-bottom: 2px solid transparent;
  color: var(--muted);
  font-weight: 400;
  cursor: pointer;
}

.lang-tab.active {
  color: var(--fg);
  font-weight: 600;
  border-bottom-color: var(--fg);
}

.lang-tab.fail {
  color: var(--fail);
}

/* One language: full width, code keeps its lines and scrolls sideways inside its block. */
table.single th,
table.single td {
  padding-left: 0;
  padding-right: 0;
}

table.single .lang-col {
  width: 100%;
}

table.single .code-shell :deep(pre) {
  overflow-x: auto;
}

table.single .code-shell :deep(code),
table.single .artifact-card :deep(code) {
  white-space: pre;
  word-wrap: normal;
  overflow-wrap: normal;
}

table.single .artifact-card :deep(pre) {
  overflow-x: auto;
}
</style>

<style>
/* Proto schemas (highlightGap) use the site code colours in App.vue. */
.inline-code pre { display: inline; margin: 0; padding: 0; }
.inline-code code { display: inline; }
</style>
