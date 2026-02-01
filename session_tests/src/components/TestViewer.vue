<template>
  <div class="test-layout">
    <main class="test-main">
      <table v-if="groupedTests.length">
        <thead>
          <tr>
            <th>
              <a href="https://github.com/petrasvestartas/session_cpp" target="_blank" class="lang-link">
                <img src="/icons/session_cpp_white.png" class="lang-icon" alt="C++" title="C++">
              </a>
            </th>
            <th>
              <a href="https://github.com/petrasvestartas/session_py" target="_blank" class="lang-link">
                <img src="/icons/session_py_white.png" class="lang-icon" alt="Python" title="Python">
              </a>
            </th>
            <th>
              <a href="https://github.com/petrasvestartas/session_rust" target="_blank" class="lang-link">
                <img src="/icons/session_rust_white.png" class="lang-icon" alt="Rust" title="Rust">
              </a>
            </th>
          </tr>
        </thead>
        <tbody>
          <template v-for="g in groupedTests" :key="g.name">
            <tr class="test-name-row">
              <td :colspan="3">
                <strong>{{ g.name }}</strong>
              </td>
            </tr>
            <tr>
            <!-- C++ column -->
            <td class="lang-col">
              <div v-if="g.cpp" class="test-card">
                <div :class="['tag', g.cpp.passed ? 'tag-pass' : 'tag-fail']" :style="timeStyle(g, 'cpp')">
                  <i :class="g.cpp.passed ? 'fa-solid fa-check' : 'fa-solid fa-xmark'"></i> {{ formatTime(g.cpp.time_ms) }} ms
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
            <td class="lang-col">
              <div v-if="g.python" class="test-card">
                <div :class="['tag', g.python.passed ? 'tag-pass' : 'tag-fail']" :style="timeStyle(g, 'python')">
                  <i :class="g.python.passed ? 'fa-solid fa-check' : 'fa-solid fa-xmark'"></i> {{ formatTime(g.python.time_ms) }} ms
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
            <td class="lang-col">
              <div v-if="g.rust" class="test-card">
                <div :class="['tag', g.rust.passed ? 'tag-pass' : 'tag-fail']" :style="timeStyle(g, 'rust')">
                  <i :class="g.rust.passed ? 'fa-solid fa-check' : 'fa-solid fa-xmark'"></i> {{ formatTime(g.rust.time_ms) }} ms
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

      <div v-else>
        No test results loaded yet. Make sure you ran <code>minitest.sh</code>.
      </div>

      <!-- JSON Artifacts Section -->
      <div v-if="hasArtifacts" class="artifacts-section">
        <h3 class="section-title">Serialization JSON</h3>
        <table>
          <thead>
            <tr>
              <th>C++</th>
              <th>Python</th>
              <th>Rust</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td class="lang-col">
                <div v-if="artifacts.cpp" class="artifact-card">
                  <button class="code-copy-btn" type="button" @click="copyJson(artifacts.cpp)" title="Copy JSON"></button>
                  <div v-html="formatJson(artifacts.cpp)"></div>
                </div>
                <div v-else class="missing">–</div>
              </td>
              <td class="lang-col">
                <div v-if="artifacts.python" class="artifact-card">
                  <button class="code-copy-btn" type="button" @click="copyJson(artifacts.python)" title="Copy JSON"></button>
                  <div v-html="formatJson(artifacts.python)"></div>
                </div>
                <div v-else class="missing">–</div>
              </td>
              <td class="lang-col">
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

<script setup>
import { computed, ref, onMounted } from 'vue'
import Parser from 'web-tree-sitter'

const props = defineProps({
  tests: { type: Array, required: true },
  activeSuite: { type: String, required: true }
})

defineEmits(['update:activeSuite'])

// Tree-sitter state
let parser = null
const languages = {}
const queries = {}
const ready = ref(false)

const LANG_MAP = { cpp: 'cpp', python: 'python', rust: 'rust', json: 'json' }
const BASE = import.meta.env.BASE_URL || '/'
const WASM_PATHS = {
  cpp: `${BASE}tree-sitter-cpp.wasm`,
  python: `${BASE}tree-sitter-python.wasm`,
  rust: `${BASE}tree-sitter-rust.wasm`,
  json: `${BASE}tree-sitter-json.wasm`,
}

// Minimal reliable tree-sitter queries — only unambiguous node types
// Everything else (keywords, functions, methods) handled by highlightGap
const QUERIES = {
  cpp: `
    (comment) @comment
    (number_literal) @number
    (string_literal) @string
    (raw_string_literal) @string
    (char_literal) @string
    (system_lib_string) @string
    (type_identifier) @type
    (primitive_type) @type.builtin
    (sized_type_specifier) @type.builtin
    (namespace_identifier) @module
  `,
  python: `
    (comment) @comment
    (integer) @number
    (float) @number
    (string) @string
    (type (identifier) @type)
  `,
  rust: `
    (line_comment) @comment
    (block_comment) @comment
    (string_literal) @string
    (raw_string_literal) @string
    (char_literal) @string
    (integer_literal) @number
    (float_literal) @number
    (boolean_literal) @constant.builtin
    (type_identifier) @type
    (primitive_type) @type.builtin
    (attribute_item) @decorator
    (macro_invocation macro: (identifier) @macro)
  `,
  json: `
    (string) @string
    (number) @number
    (null) @constant.builtin
    (true) @constant.builtin
    (false) @constant.builtin
    (pair key: (string) @property)
  `
}

const CSS = {
  'comment': 'ts-c', 'number': 'ts-n', 'string': 'ts-s',
  'type': 'ts-ty', 'type.builtin': 'ts-tyb', 'type.def': 'ts-tyd',
  'property': 'ts-pr', 'module': 'ts-mod', 'macro': 'ts-mc',
  'constant.builtin': 'ts-cb', 'decorator': 'ts-dec',
  'function': 'ts-fn', 'function.def': 'ts-fnd', 'method': 'ts-mt',
  'keyword': 'ts-kw', 'variable': 'ts-v', 'variable.builtin': 'ts-vb',
  'parameter': 'ts-pm', 'operator': 'ts-op',
  'punctuation.bracket': 'ts-pb', 'punctuation.delimiter': 'ts-pd',
}

const escapeHtml = (str) => {
  return str.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;')
}

const KEYWORDS = {
  cpp: new Set(['if','else','for','while','do','return','class','struct','enum','namespace','using','template','typename','public','private','protected','virtual','const','static','inline','new','delete','try','catch','throw','void','int','double','float','bool','char','auto','sizeof','constexpr','override','explicit','extern','volatile','mutable','friend','operator','switch','case','default','break','continue','typedef','union','noexcept','nullptr','true','false','this','#include','#define','#ifdef','#ifndef','#endif','#if','co_await','co_return','co_yield','concept','requires','static_assert','static_cast','dynamic_cast','reinterpret_cast','const_cast']),
  python: new Set(['def','class','return','if','elif','else','for','while','break','continue','pass','import','from','as','with','try','except','finally','raise','yield','lambda','global','nonlocal','assert','del','in','not','and','or','is','async','await','True','False','None','self']),
  rust: new Set(['fn','let','mut','pub','struct','enum','impl','trait','use','mod','crate','super','match','if','else','for','while','loop','break','continue','return','as','const','static','type','where','unsafe','async','await','move','ref','dyn','extern','in','self','Self','true','false']),
  json: new Set(),
  proto: new Set(['syntax','message','enum','service','rpc','returns','option','import','package','repeated','optional','required','oneof','map','reserved','extensions','extend','stream','true','false','double','float','int32','int64','uint32','uint64','sint32','sint64','fixed32','fixed64','sfixed32','sfixed64','bool','string','bytes']),
}

// Context-aware gap tokenizer: detects functions, methods, keywords, types
const highlightGap = (text, lang) => {
  const kw = KEYWORDS[lang] || new Set()
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
    if (t.bracket) { result += `<span class="ts-pb">${escapeHtml(t.bracket)}</span>`; continue }
    if (t.delim) { result += `<span class="ts-pd">${escapeHtml(t.delim)}</span>`; continue }
    if (t.op) { result += `<span class="ts-op">${escapeHtml(t.op)}</span>`; continue }
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
        result += `<span class="ts-kw">${escapeHtml(t.word)}</span>`
      } else if (afterModuleKw && !followedByParen) {
        result += `<span class="ts-mod">${escapeHtml(t.word)}</span>`
      } else if (isPascal) {
        result += `<span class="ts-ty">${escapeHtml(t.word)}</span>`
      } else if (followedByParen && afterDot) {
        result += `<span class="ts-mt">${escapeHtml(t.word)}</span>`
      } else if (followedByParen) {
        result += `<span class="ts-fn">${escapeHtml(t.word)}</span>`
      } else if (/^[A-Z][A-Z0-9_]+$/.test(t.word)) {
        result += `<span class="ts-cb">${escapeHtml(t.word)}</span>`
      } else {
        result += escapeHtml(t.word)
      }
      continue
    }
    result += escapeHtml(t.text)
  }
  return result
}

// Core highlight function using tree-sitter
const highlight = (code, lang) => {
  if (!parser || !languages[lang]) return escapeHtml(code)
  try {
    return _highlight(code, lang)
  } catch (e) {
    console.error('highlight error', lang, e)
    return escapeHtml(code)
  }
}

const _highlight = (code, lang) => {
  parser.setLanguage(languages[lang])
  const tree = parser.parse(code)
  if (!tree) return escapeHtml(code)

  const query = queries[lang]
  if (!query) { tree.delete(); return escapeHtml(code) }

  const captures = query.captures(tree.rootNode)

  // Build interval list: for each byte position, track the best (most specific) capture
  const best = new Map()
  for (const cap of captures) {
    const s = cap.node.startIndex
    const e = cap.node.endIndex
    if (s === e) continue
    const key = `${s}:${e}`
    const existing = best.get(key)
    // More dots = more specific; equal dots = prefer later (more contextual) pattern
    if (!existing || cap.name.split('.').length >= existing.name.split('.').length) {
      best.set(key, cap)
    }
  }

  // Sort intervals by start, then by shortest span (most specific)
  const intervals = Array.from(best.values())
    .map(c => ({ s: c.node.startIndex, e: c.node.endIndex, cls: CSS[c.name] || 'ts-v' }))
    .sort((a, b) => a.s - b.s || (a.e - a.s) - (b.e - b.s))

  // Render: walk through code, emit spans for intervals, skip overlaps
  let result = ''
  let pos = 0
  for (const iv of intervals) {
    if (iv.s < pos) continue
    if (iv.s > pos) result += highlightGap(code.slice(pos, iv.s), lang)
    const text = code.slice(iv.s, iv.e)
    result += `<span class="${iv.cls}">${escapeHtml(text)}</span>`
    pos = iv.e
  }
  if (pos < code.length) result += highlightGap(code.slice(pos), lang)

  tree.delete()
  return result
}

onMounted(async () => {
  try {
    await Parser.init({ locateFile: (f) => `${BASE}${f}` })
    parser = new Parser()
  } catch (e) {
    console.error('tree-sitter init failed:', e)
    return
  }
  for (const lang of Object.keys(WASM_PATHS)) {
    try {
      const language = await Parser.Language.load(WASM_PATHS[lang])
      languages[lang] = language
      queries[lang] = language.query(QUERIES[lang])
    } catch (e) {
      console.error(`tree-sitter ${lang} query failed:`, e.message)
    }
  }
  ready.value = true
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
  if (!Number.isFinite(minIndent) || minIndent === 0) return lines.join('\n')
  return lines.map((line) => (line.length >= minIndent ? line.slice(minIndent) : line)).join('\n')
}

const highlightedCode = (t) => {
  if (!t || !t.code) return ""
  const code = normalizeForDisplay(t.code)
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
  if (!t.passed) return { color: '#ff5555' }
  const times = [group.python, group.cpp, group.rust]
    .filter(x => x && typeof x.time_ms === "number" && x.passed)
    .map(x => x.time_ms)
  if (!times.length) return { color: '#ffffff' }
  const min = Math.min(...times)
  const max = Math.max(...times)
  if (max === min) return { color: '#ffffff' }
  const value = t.time_ms
  let ratio = (value - min) / (max - min)
  if (ratio < 0) ratio = 0
  if (ratio > 1) ratio = 1
  const r = Math.round(255 + (0x55 - 255) * ratio)
  const g = Math.round(255 + (0x88 - 255) * ratio)
  const b = Math.round(255 + (0xff - 255) * ratio)
  return { color: `rgb(${r}, ${g}, ${b})` }
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
  const text = normalizeForDisplay(t.code)
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
      return highlightGap(before, 'proto') + `<span class="ts-c">${escapeHtml(comment)}</span>`
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
/* Test viewer styles - Pure black theme */
.test-layout {
  display: flex;
  height: 100%;
}
.sidebar {
  width: 180px;
  flex-shrink: 0;
  background: #000000;
}
.sidebar-title {
  font-weight: 600;
  margin-bottom: 0.5rem;
  font-size: 16px;
  color: #ffffff;
}
.suite-pill {
  display: block;
  padding: 0.75rem 1rem;
  margin: 0;
  border-radius: 0;
  border: none;
  border-left: 3px solid transparent;
  background: transparent;
  color: #aaaaaa;
  font-weight: 600;
  cursor: pointer;
  transition: background 0.15s ease, color 0.15s ease, border-color 0.15s ease;
}
.suite-pill:hover {
  background: #111111;
  color: #ffffff;
}
.suite-pill.active {
  background: #111111;
  color: #ffffff;
  border-left-color: #ffffff;
}
.test-main {
  flex: 1;
  overflow-y: auto;
}

table {
  width: 100%;
  border-collapse: collapse;
  background: #000000;
  box-shadow: none;
  table-layout: fixed;
}
th, td {
  padding: 0.5rem 0.75rem;
  border: none;
  vertical-align: top;
  color: #aaaaaa;
}
th {
  background: #000000;
  text-align: left;
  font-weight: 600;
  font-size: 14px;
  color: #ffffff;
  border: none;
}

.lang-icon {
  width: 24px;
  height: 24px;
  font-size: 24px;
  color: #ffffff;
}

.lang-link {
  color: #ffffff;
  text-decoration: none;
  transition: color 0.2s;
}

.lang-link:hover {
  color: #aaaaaa;
}

.lang-text {
  font-size: 24px;
  font-weight: 700;
  color: #ffffff;
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
  color: #50fa7b;
}
.tag-fail {
  color: #ff5555;
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
  color: #abb2bf;
}
.code-shell {
  position: relative;
  margin: 0.25rem 0 0.5rem 0;
  background: #0f0f0f;
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
  background: #444444;
  cursor: pointer;
}

.code-copy-btn:hover {
  background: #666666;
}
.failures {
  margin-top: 0.35rem;
  color: #ff5555;
}
.exceptions {
  margin-top: 0.35rem;
}
.error-message {
  margin-top: 0.15rem;
  font-family: monospace;
  color: #ff5555;
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
  color: #666666;
  font-style: italic;
}
.test-name-row td {
  padding-top: 0.75rem;
  font-weight: 600;
  color: #ffffff;
  background: #000000;
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
  background: #000000;
  color: #ffffff;
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
  background: #000000;
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
  color: #aaaaaa;
}
</style>

<style>
/* Tree-sitter highlight theme */
.ts-kw  { color: #c678dd; }
.ts-dir { color: #c678dd; }
.ts-ty  { color: #00e5ff; }
.ts-tyb { color: #56b6c2; }
.ts-tyd { color: #00e5ff; font-weight: 600; }
.ts-fn  { color: #61afef; }
.ts-fnd { color: #61afef; font-weight: 600; }
.ts-mt  { color: #61afef; }
.ts-mc  { color: #61afef; font-weight: 600; }
.ts-v   { color: #abb2bf; }
.ts-vb  { color: #e06c75; font-style: italic; }
.ts-pm  { color: #e5e54b; font-style: italic; }
.ts-pl  { color: #e5e54b; }
.ts-pr  { color: #e06c75; }
.ts-cb  { color: #e5e54b; }
.ts-s   { color: #98c379; }
.ts-n   { color: #e5e54b; }
.ts-c   { color: #5c6370; font-style: italic; }
.ts-op  { color: #56b6c2; }
.ts-mod { color: #00e5ff; }
.ts-lb  { color: #00e5ff; font-style: italic; }
.ts-dec { color: #00e5ff; }
.ts-pb  { color: #ff79c6; }
.ts-pd  { color: #ff79c6; }
.inline-code pre { display: inline; margin: 0; padding: 0; }
.inline-code code { display: inline; }
</style>
