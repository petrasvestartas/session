<template>
  <div class="test-layout">
    <main class="test-main">
      <h3 v-if="groupedTests.length" class="section-title">Tests</h3>
      <table v-if="groupedTests.length">
        <thead>
          <tr>
            <th>Python</th>
            <th>C++</th>
            <th>Rust</th>
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
              <!-- Python column -->
              <td class="lang-col">
              <div v-if="g.python" class="test-card">
                <div :class="['tag', g.python.passed ? 'tag-pass' : 'tag-fail']" :style="timeStyle(g, 'python')">
                  {{ g.python.passed ? 'passed' : 'failed' }} ({{ formatTime(g.python.time_ms) }} ms)
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
                  <pre><code :class="codeClass(g.python)" v-html="highlightedCode(g.python)"></code></pre>
                </div>
                <div class="failures" v-if="!g.python.passed">
                  <div><strong>Failing checks:</strong></div>
                  <ul>
                    <li v-for="c in failingChecks(g.python)" :key="'py-' + g.name + ':' + c.line">
                      line {{ c.line }}: <code :class="codeClass(g.python)" v-html="highlightedCheck(c, 'python')"></code>
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
                          <code :class="codeClass(g.python)" v-html="highlightedFailureCode(f, 'python')"></code>
                        </div>
                        <div class="error-message" v-if="f.error">{{ f.error }}</div>
                      </li>
                    </ul>
                  </div>
                </div>
              </div>
              <div v-else class="missing">–</div>
            </td>

            <!-- C++ column -->
            <td class="lang-col">
              <div v-if="g.cpp" class="test-card">
                <div :class="['tag', g.cpp.passed ? 'tag-pass' : 'tag-fail']" :style="timeStyle(g, 'cpp')">
                  {{ g.cpp.passed ? 'passed' : 'failed' }} ({{ formatTime(g.cpp.time_ms) }} ms)
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
                  <pre><code :class="codeClass(g.cpp)" v-html="highlightedCode(g.cpp)"></code></pre>
                </div>
                <div class="failures" v-if="!g.cpp.passed">
                  <div><strong>Failing checks:</strong></div>
                  <ul>
                    <li v-for="c in failingChecks(g.cpp)" :key="'cpp-' + g.name + ':' + c.line">
                      line {{ c.line }}: <code :class="codeClass(g.cpp)" v-html="highlightedCheck(c, 'cpp')"></code>
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
                          <code :class="codeClass(g.cpp)" v-html="highlightedFailureCode(f, 'cpp')"></code>
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
                  {{ g.rust.passed ? 'passed' : 'failed' }} ({{ formatTime(g.rust.time_ms) }} ms)
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
                  <pre><code :class="codeClass(g.rust)" v-html="highlightedCode(g.rust)"></code></pre>
                </div>
                <div class="failures" v-if="!g.rust.passed">
                  <div><strong>Failing checks:</strong></div>
                  <ul>
                    <li v-for="c in failingChecks(g.rust)" :key="'rs-' + g.name + ':' + c.line">
                      line {{ c.line }}: <code :class="codeClass(g.rust)" v-html="highlightedCheck(c, 'rust')"></code>
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
                          <code :class="codeClass(g.rust)" v-html="highlightedFailureCode(f, 'rust')"></code>
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
              <th>Python</th>
              <th>C++</th>
              <th>Rust</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td class="lang-col">
                <div v-if="artifacts.python" class="artifact-card">
                  <button class="code-copy-btn" type="button" @click="copyJson(artifacts.python)" title="Copy JSON"></button>
                  <pre><code class="hljs language-json" v-html="formatJson(artifacts.python)"></code></pre>
                </div>
                <div v-else class="missing">–</div>
              </td>
              <td class="lang-col">
                <div v-if="artifacts.cpp" class="artifact-card">
                  <button class="code-copy-btn" type="button" @click="copyJson(artifacts.cpp)" title="Copy JSON"></button>
                  <pre><code class="hljs language-json" v-html="formatJson(artifacts.cpp)"></code></pre>
                </div>
                <div v-else class="missing">–</div>
              </td>
              <td class="lang-col">
                <div v-if="artifacts.rust" class="artifact-card">
                  <button class="code-copy-btn" type="button" @click="copyJson(artifacts.rust)" title="Copy JSON"></button>
                  <pre><code class="hljs language-json" v-html="formatJson(artifacts.rust)"></code></pre>
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
          <pre><code class="hljs language-protobuf" v-html="formatProto(protoSchemas[0]?.content)"></code></pre>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup>
import { computed } from 'vue'
import hljs from 'highlight.js/lib/core'
import python from 'highlight.js/lib/languages/python'
import cpp from 'highlight.js/lib/languages/cpp'
import rust from 'highlight.js/lib/languages/rust'
import json from 'highlight.js/lib/languages/json'
import protobuf from 'highlight.js/lib/languages/protobuf'

// Register languages
hljs.registerLanguage('python', python)
hljs.registerLanguage('cpp', cpp)
hljs.registerLanguage('rust', rust)
hljs.registerLanguage('json', json)
hljs.registerLanguage('protobuf', protobuf)

const props = defineProps({
  tests: { type: Array, required: true },
  activeSuite: { type: String, required: true }
})

defineEmits(['update:activeSuite'])

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

  // First, apply any inline transformations (e.g. uncomment markers)
  const lines = code.split('\n').map((line) => {
    const m = line.match(/^(\s*)\/\/\s*uncomment\s+(.*)$/)
    if (m) {
      const indent = m[1]
      const rest = m[2]
      return indent + rest
    }
    return line
  })

  // Then, strip the common leading indentation so code is visually flush-left
  let minIndent = Infinity
  for (const line of lines) {
    if (!line.trim()) continue
    const m = line.match(/^(\s*)/)
    const indent = m ? m[1].length : 0
    if (indent < minIndent) minIndent = indent
  }

  if (!Number.isFinite(minIndent) || minIndent === 0) {
    return lines.join('\n')
  }

  return lines.map((line) => (line.length >= minIndent ? line.slice(minIndent) : line)).join('\n')
}

const codeClass = (t) => {
  if (!t || !t.language) return "hljs"
  if (t.language === "python") return "hljs language-python"
  if (t.language === "cpp") return "hljs language-cpp"
  if (t.language === "rust") return "hljs language-rust"
  return "hljs"
}

const highlightedCode = (t) => {
  if (!t || !t.code) return ""
  const displayCode = normalizeForDisplay(t.code)
  try {
    const lang = t.language || ""
    if (!lang) return displayCode
    const result = hljs.highlight(displayCode, { language: lang })
    return result.value
  } catch (e) {
    return displayCode
  }
}

const highlightedCheck = (check, lang) => {
  if (!check || !check.code_line) return ""
  try {
    const result = hljs.highlight(check.code_line, { language: lang })
    return result.value
  } catch (e) {
    return check.code_line
  }
}

const timeStyle = (group, lang) => {
  const t = group[lang]
  if (!t || typeof t.time_ms !== "number") return {}

  const times = [group.python, group.cpp, group.rust]
    .filter(x => x && typeof x.time_ms === "number")
    .map(x => x.time_ms)
  if (!times.length) return {}

  const min = Math.min(...times)
  const max = Math.max(...times)

  if (max === min) {
    return { background: "hsl(140, 60%, 92%)" }
  }

  const value = t.time_ms
  let ratio = (value - min) / (max - min)
  if (ratio < 0) ratio = 0
  if (ratio > 1) ratio = 1

  const start = { h: 140, s: 60, l: 90 }
  const end = { h: 210, s: 80, l: 88 }

  const h = start.h + (end.h - start.h) * ratio
  const s = start.s + (end.s - start.s) * ratio
  const l = start.l + (end.l - start.l) * ratio

  return { background: `hsl(${h}, ${s}%, ${l}%)` }
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
  try {
    const result = hljs.highlight(failure.code_line, { language: lang })
    return result.value
  } catch (e) {
    return failure.code_line
  }
}

const copyCode = (t) => {
  if (!t || !t.code) return
  const text = normalizeForDisplay(t.code)
  try {
    if (typeof navigator !== 'undefined' && navigator.clipboard && navigator.clipboard.writeText) {
      navigator.clipboard.writeText(text)
    }
  } catch (e) {
    console.error('Failed to copy code', e)
  }
}

// JSON artifacts (test_point.json files from each language)
const artifacts = computed(() => {
  if (typeof window.TEST_DATA === 'undefined') return { python: null, cpp: null, rust: null }
  const data = window.TEST_DATA
  
  // Get artifact based on active suite (e.g., point_test -> test_point)
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
    const result = hljs.highlight(jsonStr, { language: 'json' })
    return result.value
  } catch (e) {
    return String(obj)
  }
}

// Proto schemas (shared across all languages)
const protoSchemas = computed(() => {
  if (typeof window.TEST_DATA === 'undefined') return []
  const data = window.TEST_DATA
  
  // Get proto schema based on active suite (e.g., point_test -> point.proto)
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
  try {
    const result = hljs.highlight(content, { language: 'protobuf' })
    return result.value
  } catch (e) {
    return content
  }
}

const copyJson = (obj) => {
  if (!obj) return
  try {
    const text = JSON.stringify(obj, null, 2)
    if (typeof navigator !== 'undefined' && navigator.clipboard && navigator.clipboard.writeText) {
      navigator.clipboard.writeText(text)
    }
  } catch (e) {
    console.error('Failed to copy JSON', e)
  }
}

const copyProto = (content) => {
  if (!content) return
  try {
    if (typeof navigator !== 'undefined' && navigator.clipboard && navigator.clipboard.writeText) {
      navigator.clipboard.writeText(content)
    }
  } catch (e) {
    console.error('Failed to copy proto', e)
  }
}
</script>

<style scoped>
/* Test viewer styles */
.test-layout {
  display: flex;
  height: 100%;
}
.sidebar {
  width: 180px;
  flex-shrink: 0;
  background: #2563eb;
}
.sidebar-title {
  font-weight: 600;
  margin-bottom: 0.5rem;
  font-size: 16px;
}
.suite-pill {
  display: block;
  padding: 0.75rem 1rem;
  margin: 0;
  border-radius: 0;
  border: none;
  border-left: 3px solid transparent;
  background: transparent;
  color: rgba(255, 255, 255, 0.8);
  font-weight: 600;
  cursor: pointer;
  transition: background 0.15s ease, color 0.15s ease, border-color 0.15s ease;
}
.suite-pill:hover {
  background: rgba(255, 255, 255, 0.08);
  color: #ffffff;
}
.suite-pill.active {
  background: rgba(255, 255, 255, 0.12);
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
  background: #fff;
  box-shadow: 0 1px 4px rgba(0,0,0,0.08);
  table-layout: fixed;
}
th, td {
  padding: 0.5rem 0.75rem;
  border-bottom: 1px solid #e0e0e0;
  border-right: 1px solid #e0e0e0;
  vertical-align: top;
}
th {
  background: #fafafa;
  text-align: left;
  font-weight: 600;
}
th:last-child, td:last-child {
  border-right: none;
}
.tag {
  display: inline-block;
  padding: 0.1rem 0.4rem;
  border-radius: 0.25rem;
  font-weight: 600;
}
.tag-pass {
  background: #e3f9e5;
  color: #000;
}
.tag-fail {
  background: #fdecea;
  color: #b00020;
}
pre {
  margin: 0;
  background: transparent;
  border-radius: 0;
  padding: 0;
}
.test-card pre code {
  white-space: pre-wrap;      /* allow wrapping of long lines */
  word-wrap: break-word;
  overflow-wrap: anywhere;
}
.code-shell {
  position: relative;
  margin: 0.25rem 0 0.5rem 0;
  background: #ffffff;
  border-radius: 4px;
}

.code-shell pre {
  margin: 0;
  padding: 0;
}

.code-copy-btn {
  position: absolute;
  top: 4px;
  right: 4px;
  width: 15px;
  height: 15px;
  padding: 0;
  border: none;
  border-radius: 0;
  background: #2563eb; /* same blue as sidebar/top layout */
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  font-size: 0;
  line-height: 0;
}

.code-copy-btn:hover {
  background: #1d4ed8; /* slightly darker blue on hover */
}
.failures {
  margin-top: 0.35rem;
  color: #b00020;
}
.exceptions {
  margin-top: 0.35rem;
}
.error-message {
  margin-top: 0.15rem;
  font-family: monospace;
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
  color: #bbb;
  font-style: italic;
}
.test-name-row td {
  padding-top: 0.75rem;
  font-weight: 600;
}
.lang-col {
  width: 33.33%;
}
.artifacts-section {
  margin-top: 2rem;
  padding-top: 1rem;
  border-top: 2px solid #e0e0e0;
}
.section-title {
  background: #2563eb;
  color: white;
  padding: 0.5rem 0.75rem;
  margin: 0 0 1rem 0;
  font-size: 1.1rem;
  font-weight: 600;
  border-radius: 4px;
}
.artifacts-section h3 {
  margin: 0 0 1rem 0;
}
.artifact-card {
  position: relative;
  background: #fafafa;
  border-radius: 4px;
  padding: 0.5rem;
}
.artifact-card pre {
  margin: 0;
  font-size: 0.85rem;
  white-space: pre-wrap;
  word-wrap: break-word;
}
</style>

<style>
/* Highlight.js overrides (global) */
.hljs {
  color: #002050 !important;
}
.hljs-keyword, .hljs-built_in, .hljs-type, .hljs-title,
.hljs-title.class_, .hljs-title.function_, .hljs-variable.language_ {
  color: #0050d7 !important;
  font-weight: 600;
}
.hljs-property, .hljs-variable, .hljs-attribute, .hljs-attr,
.hljs-params, .hljs-symbol, .hljs-meta {
  color: #003c99 !important;
}
.hljs-string, .hljs-char, .hljs-template-variable {
  color: #1a66cc !important;
}
.hljs-number, .hljs-literal, .hljs-operator {
  color: #3399ff !important;
}
.hljs-comment {
  color: #5c6f91 !important;
  font-style: italic;
}
</style>
