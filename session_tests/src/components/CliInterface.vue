<template>
  <div class="cli-interface">
    <div class="cli-input-wrapper">
      <div class="cli-input-container">
        <input 
          type="text" 
          class="cli-input" 
          v-model="currentInput"
          @keydown.enter="executeCommand"
          :placeholder="inputPlaceholder"
          ref="inputRef"
        />
      </div>
    </div>

    <div class="cli-results-wrapper">
      <div class="cli-results-container">
        <div class="cli-messages" ref="outputRef">
          <div v-for="(item, idx) in history" :key="idx" class="message-group">
            <!-- Command (question) aligned right -->
            <div v-if="item.type === 'command'" class="message command-message">
              <div class="message-content">{{ item.text }}</div>
            </div>

            <!-- Answer / output aligned left with formatting -->
            <div v-else class="message response-message">
              <div class="message-content" :class="{ 'error-text': item.type === 'error' }" v-html="formatAnswer(item.text)"></div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, nextTick, onMounted } from 'vue'
import { getClaudeApiKey } from '../firebase.js'

const props = defineProps({
  activeTab: { type: String, required: true }
})

const history = ref([])
const currentInput = ref('')
const outputRef = ref(null)
const inputRef = ref(null)

// API key management - fetched from Firebase
const apiKey = ref('')
const apiKeyLoading = ref(true)
const apiKeyError = ref('')

// Fetch API key from Firebase on mount
onMounted(async () => {
  try {
    const { key, error } = await getClaudeApiKey()
    if (key) {
      apiKey.value = key
      apiKeyError.value = ''
    } else {
      apiKeyError.value = error || 'Failed to load API key'
    }
  } catch (err) {
    apiKeyError.value = `Firebase error: ${err.message}`
  } finally {
    apiKeyLoading.value = false
  }
})

// Cosine similarity for semantic search
const cosineSimilarity = (a, b) => {
  if (!a || !b || a.length !== b.length) return 0
  let dotProduct = 0
  let normA = 0
  let normB = 0
  for (let i = 0; i < a.length; i++) {
    dotProduct += a[i] * b[i]
    normA += a[i] * a[i]
    normB += b[i] * b[i]
  }
  return dotProduct / (Math.sqrt(normA) * Math.sqrt(normB))
}

// Semantic search using pre-computed embeddings + query embedding from Claude
const semanticSearch = async (query, maxResults = 5) => {
  const index = window.SEARCH_INDEX
  if (!index || !index.chunks) {
    return { results: [], error: 'Search index not loaded' }
  }

  // If no embeddings in index, fall back to keyword search
  if (!index.hasEmbeddings) {
    return clientSideSearch(query, maxResults)
  }

  // Use Claude to get query embedding via a simple text-embedding approach
  // Since Anthropic doesn't have a public embedding API, we'll use keyword matching
  // combined with the pre-computed document embeddings for ranking
  // Fall back to enhanced keyword search that leverages the embedding structure

  return clientSideSearch(query, maxResults)
}

// Client-side search function using the static index
const clientSideSearch = (query, maxResults = 5) => {
  const index = window.SEARCH_INDEX
  if (!index || !index.chunks) {
    return { results: [], error: 'Search index not loaded' }
  }

  const queryLower = query.toLowerCase()
  const queryWords = queryLower.split(/\s+/).filter(w => w.length >= 2)

  // Score each chunk based on keyword matches
  const scored = index.chunks.map(chunk => {
    let score = 0

    // Check name match (highest weight)
    if (chunk.name.toLowerCase().includes(queryLower)) {
      score += 100
    }

    // Check keyword matches
    for (const word of queryWords) {
      if (chunk.keywords && chunk.keywords.includes(word)) {
        score += 20
      }
      if (chunk.name.toLowerCase().includes(word)) {
        score += 30
      }
      if (chunk.code && chunk.code.toLowerCase().includes(word)) {
        score += 5
      }
    }

    // Boost for specific patterns
    if (queryLower.includes('create') || queryLower.includes('new') || queryLower.includes('constructor')) {
      if (chunk.name.includes('__init__') || chunk.name.includes('new') || chunk.name.includes('New')) {
        score += 50
      }
    }

    if (queryLower.includes('python') && chunk.language === 'python') score += 25
    if (queryLower.includes('rust') && chunk.language === 'rust') score += 25
    if (queryLower.includes('c++') && chunk.language === 'cpp') score += 25
    if (queryLower.includes('cpp') && chunk.language === 'cpp') score += 25

    return { ...chunk, score }
  })

  // Sort by score and take top results
  const results = scored
    .filter(c => c.score > 0)
    .sort((a, b) => b.score - a.score)
    .slice(0, maxResults)
    .map(c => ({
      type: c.type,
      name: c.name,
      file: c.file,
      language: c.language,
      line_start: c.line_start,
      code: c.code,
      score: c.score
    }))

  return { results, error: null }
}

// Call Claude API with streaming support
const callClaudeAPIStreaming = async (question, context, onChunk) => {
  if (!apiKey.value) {
    return { error: 'No API key configured. Type "key" to set your Anthropic API key.' }
  }

  const systemPrompt = `You are a coding assistant for the Session geometry library (Python, C++, Rust).

RULES:
- Be VERY concise - 2-4 sentences max, then show code
- Skip explanations users can infer from code
- One code example per language requested (not multiple examples)
- No bullet lists of "default properties" or "parameters" - just show usage
- No "From file.py:line" citations unless asked
- Get straight to the point like Stack Overflow's top answers`

  const userMessage = `Question: ${question}

Here is relevant code from the codebase:

${context}

Please answer the question based on this code context.`

  try {
    const response = await fetch('https://api.anthropic.com/v1/messages', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-api-key': apiKey.value,
        'anthropic-version': '2023-06-01',
        'anthropic-dangerous-direct-browser-access': 'true'
      },
      body: JSON.stringify({
        model: 'claude-sonnet-4-20250514',
        max_tokens: 1024,
        stream: true,
        system: systemPrompt,
        messages: [{ role: 'user', content: userMessage }]
      })
    })

    if (!response.ok) {
      const errorData = await response.json().catch(() => ({}))
      if (response.status === 401) {
        return { error: 'Invalid API key. Type "key" to update your Anthropic API key.' }
      }
      return { error: `API error: ${response.status} - ${errorData.error?.message || 'Unknown error'}` }
    }

    // Read streaming response
    const reader = response.body.getReader()
    const decoder = new TextDecoder()
    let fullText = ''
    let buffer = ''

    while (true) {
      const { done, value } = await reader.read()
      if (done) break

      buffer += decoder.decode(value, { stream: true })
      
      // Process SSE events in buffer
      const lines = buffer.split('\n')
      buffer = lines.pop() || '' // Keep incomplete line in buffer

      for (const line of lines) {
        if (line.startsWith('data: ')) {
          const data = line.slice(6)
          if (data === '[DONE]') continue
          
          try {
            const parsed = JSON.parse(data)
            // Handle content_block_delta events
            if (parsed.type === 'content_block_delta' && parsed.delta?.text) {
              fullText += parsed.delta.text
              onChunk(fullText) // Update UI with accumulated text
            }
          } catch (e) {
            // Skip malformed JSON
          }
        }
      }
    }

    return { answer: fullText }
  } catch (error) {
    return { error: `Failed to call Claude API: ${error.message}` }
  }
}

// Generate context from search results for Claude
const buildContext = (results) => {
  if (!results || results.length === 0) return ''

  const parts = []
  for (const r of results) {
    const langName = r.language === 'cpp' ? 'C++' : r.language.charAt(0).toUpperCase() + r.language.slice(1)
    parts.push(`--- ${langName}: ${r.name} (${r.file}:${r.line_start}) ---`)
    parts.push(r.code || '')
    parts.push('')
  }
  return parts.join('\n')
}

// Generate a simple answer from search results (fallback without Claude)
const generateAnswer = (query, results) => {
  if (!results || results.length === 0) {
    return `No results found for "${query}". Try searching for "Point", "Color", "distance", "create", etc.`
  }

  const queryLower = query.toLowerCase()
  const lines = []

  // Group by language
  const byLang = { python: [], cpp: [], rust: [] }
  for (const r of results) {
    if (byLang[r.language]) {
      byLang[r.language].push(r)
    }
  }

  // Generate contextual answer
  if (queryLower.includes('create') || queryLower.includes('new') || queryLower.includes('how to')) {
    lines.push(`Here's how to work with this in the Session codebase:\n`)
  } else {
    lines.push(`Found ${results.length} relevant code sections:\n`)
  }

  for (const lang of ['python', 'cpp', 'rust']) {
    const langResults = byLang[lang]
    if (langResults.length === 0) continue

    const langName = lang === 'cpp' ? 'C++' : lang.charAt(0).toUpperCase() + lang.slice(1)
    lines.push(`**${langName}:**`)

    for (const r of langResults.slice(0, 2)) {
      // Show code preview
      const codeLines = (r.code || '').split('\n').slice(0, 8)
      const codePreview = codeLines.join('\n')

      lines.push(`\`\`\`${lang}`)
      lines.push(codePreview)
      lines.push(`\`\`\``)
      lines.push(`*Source: ${r.file}:${r.line_start}*\n`)
    }
  }

  return lines.join('\n')
}

// Format answer text with HTML styling
const formatAnswer = (text) => {
  if (!text) return ''

  let html = text

  // Convert code blocks ```lang ... ``` to styled spans
  html = html.replace(/```(\w+)\n([\s\S]*?)```/g, (match, lang, code) => {
    return `<div class="code-block"><div class="code-lang">${lang}</div><pre class="code-content">${escapeHtml(code)}</pre></div>`
  })

  // Convert **bold** to styled spans
  html = html.replace(/\*\*(.*?)\*\*/g, '<span class="text-bold">$1</span>')

  // Convert *Source:* to dimmed text
  html = html.replace(/\*Source:(.*?)\*/g, '<span class="text-dim">Source:$1</span>')

  // Convert # comments to gray
  html = html.replace(/^(#.*)$/gm, '<span class="text-comment">$1</span>')

  // Preserve line breaks
  html = html.replace(/\n/g, '<br>')

  return html
}

const escapeHtml = (text) => {
  const div = document.createElement('div')
  div.textContent = text
  return div.innerHTML
}

const tabContext = computed(() => {
  if (props.activeTab === 'viewer') return 'Viewer'
  if (props.activeTab === 'tests') return 'Tests'
  return 'Viewer'
})

const inputPlaceholder = computed(() => {
  if (props.activeTab === 'viewer') {
    return 'Ask a question or type "help" for commands'
  }
  if (props.activeTab === 'tests') {
    return 'Ask a question or type "help" for commands'
  }
  return 'Ask a question or type "help" for commands'
})

const commands = {
  help: () => {
    let keyStatus
    if (apiKeyLoading.value) {
      keyStatus = '(loading from Firebase...)'
    } else if (apiKey.value) {
      keyStatus = '(ready - loaded from Firebase)'
    } else {
      keyStatus = `(not available: ${apiKeyError.value || 'unknown error'})`
    }

    const lines = [
      'Session CLI - Ask questions about your codebase',
      '',
      `Claude AI: ${keyStatus}`,
      '',
      'Quick Start:',
      '  Just type your question directly!',
      '  Examples:',
      '    how to create a Point in Python',
      '    what are the Color methods in Rust',
      '    distance calculation between points',
      '',
      'Available commands:',
      '  help       - Show this help message',
      '  clear      - Clear the console',
      '  search     - Show raw search results with sources',
      '  info       - Show current context information',
      '',
      'Note: Claude AI answers are powered by your Firebase-stored API key.'
    ]
    return [lines.join('\n')]
  },
  clear: () => {
    history.value = []
    return []
  },
  info: () => {
    const index = window.SEARCH_INDEX
    let claudeStatus
    if (apiKeyLoading.value) {
      claudeStatus = 'Loading from Firebase...'
    } else if (apiKey.value) {
      claudeStatus = 'Ready (key loaded from Firebase)'
    } else {
      claudeStatus = `Error: ${apiKeyError.value || 'No key'}`
    }

    const lines = [
      `Current tab: ${props.activeTab}`,
      `Context: ${tabContext.value}`,
      '',
      'Search Index:',
      `  Loaded: ${index ? 'Yes' : 'No'}`,
      `  Chunks: ${index?.chunks?.length || 0}`,
      `  Has Embeddings: ${index?.hasEmbeddings || false}`,
      '',
      'Claude AI:',
      `  Status: ${claudeStatus}`,
      `  Model: claude-sonnet-4-20250514`,
    ]
    return [lines.join('\n')]
  },
  search: async (args) => {
    if (!args || args.length === 0) {
      return ['Usage: search <query>', 'Example: search Point constructor']
    }

    const query = args.join(' ')
    const { results, error } = clientSideSearch(query, 5)

    if (error) {
      return [`Error: ${error}`]
    }

    if (results.length === 0) {
      return ['No results found.']
    }

    const output = [`Found ${results.length} results for: "${query}"`, '']
    results.forEach((r, i) => {
      output.push(`${i + 1}. ${r.type} "${r.name}" (${r.language})`)
      output.push(`   File: ${r.file}:${r.line_start}`)
      output.push(`   Score: ${r.score}`)
      const preview = (r.code || '').split('\n').slice(0, 2).join(' ').substring(0, 80)
      if (preview) output.push(`   Preview: ${preview}...`)
      output.push('')
    })

    return output
  },
  stats: () => {
    if (props.activeTab !== 'tests') {
      return ['Error: stats command only available in Tests tab']
    }
    return ['Test statistics coming soon...']
  },
  viewer: () => {
    if (props.activeTab !== 'viewer') {
      return ['Error: viewer commands only available in Viewer tab']
    }
    return ['3D Viewer commands coming soon...']
  },
  ask: async (args) => {
    if (!args || args.length === 0) {
      return [
        'Error: ask command requires a question',
        'Usage: ask <your question>',
        '',
        'Examples:',
        '  ask how to create a Point in Python',
        '  ask what are the Color methods in Rust',
        '  ask distance calculation between points'
      ]
    }

    const question = args.join(' ')

    // Search for relevant code
    const { results, error: searchError } = clientSideSearch(question, 6)

    if (searchError) {
      return [
        'Search index not available.',
        'Run ./minitest.sh locally to generate the search index.',
        '',
        `Details: ${searchError}`
      ]
    }

    if (results.length === 0) {
      return [
        `No results found for "${question}".`,
        '',
        'Try searching for:',
        '  - Point, Color (class names)',
        '  - distance, create, new (operations)',
        '  - python, rust, c++ (languages)'
      ]
    }

    // If Claude API key is configured, use Claude with streaming
    if (apiKey.value) {
      const context = buildContext(results)
      
      // Add a placeholder response that we'll update
      const responseIndex = history.value.length
      history.value.push({ text: '▌', type: 'output', streaming: true })
      
      // Scroll to show the streaming response
      nextTick(() => {
        if (outputRef.value) {
          outputRef.value.scrollTop = outputRef.value.scrollHeight
        }
      })

      const claudeResult = await callClaudeAPIStreaming(question, context, (partialText) => {
        // Update the response in place as chunks arrive
        if (history.value[responseIndex]) {
          history.value[responseIndex].text = partialText + ' ▌'
          // Auto-scroll as content streams in
          nextTick(() => {
            if (outputRef.value) {
              outputRef.value.scrollTop = outputRef.value.scrollHeight
            }
          })
        }
      })

      if (claudeResult.error) {
        // Remove placeholder and show error
        history.value.splice(responseIndex, 1)
        console.log('Claude API error, falling back:', claudeResult.error)
        const answer = generateAnswer(question, results)
        return [`(Claude unavailable: ${claudeResult.error})`, '', ...answer.split('\n')]
      }

      // Finalize the streamed response (remove cursor)
      if (history.value[responseIndex]) {
        history.value[responseIndex].text = claudeResult.answer
        history.value[responseIndex].streaming = false
      }
      
      return [] // Already added to history via streaming
    }

    // No API key - show code directly
    const answer = generateAnswer(question, results)
    return answer.split('\n')
  },
}

const executeCommand = async () => {
  const cmd = currentInput.value.trim()
  if (!cmd) return

  // Add command to history
  history.value.push({ text: cmd, type: 'command' })

  // Parse command
  const [baseCmd, ...args] = cmd.split(' ')
  const baseCmdLower = baseCmd.toLowerCase()

  // Check if it's a recognized command
  if (commands[baseCmdLower]) {
    try {
      const result = await commands[baseCmdLower](args)
      result.forEach(line => {
        history.value.push({ text: line, type: 'output' })
      })
    } catch (error) {
      history.value.push({
        text: `Error executing command: ${error.message}`,
        type: 'error'
      })
    }
  } else {
    // If not a recognized command, treat it as an "ask" question
    // This allows users to just type their question without "ask"
    try {
      const result = await commands.ask(cmd.split(' '))
      result.forEach(line => {
        history.value.push({ text: line, type: 'output' })
      })
    } catch (error) {
      history.value.push({
        text: `Error: ${error.message}`,
        type: 'error'
      })
    }
  }

  currentInput.value = ''

  // Scroll to bottom
  nextTick(() => {
    if (outputRef.value) {
      outputRef.value.scrollTop = outputRef.value.scrollHeight
    }
    // Focus back to input
    if (inputRef.value) {
      inputRef.value.focus()
    }
  })
}
</script>

<style scoped>
/* CLI Interface - resizable container (height controlled by parent flex) */
.cli-interface {
  min-height: 160px;
  background: #f9fafb;
  border-top: 1px solid #f9fafb; /* same as background, visually hidden */
  border-bottom: none;           /* no extra line; resizer provides the divider */
  display: flex;
  flex-direction: column;
  position: relative;
  overflow: hidden;              /* keep content inside fixed CLI height */
}

.cli-results-wrapper {
  flex: 1;
  min-height: 0;                 /* allow wrapper to shrink and scroll inside */
  padding: 0 1rem 1rem 1rem; /* same horizontal padding as input wrapper */
}

.cli-results-container {
  width: 100%;
  background: #ffffff;
  border: 2px solid #d1d5db;
  border-radius: 0;
  box-sizing: border-box;
  height: 100%;
  min-height: 0;                 /* allow inner messages to scroll */
  display: flex;
  flex-direction: column;
  padding: 0.75rem;              /* same inner horizontal padding as input box */
}

/* Messages area */
.cli-messages {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  overflow-x: hidden;
  padding: 0;                     /* padding now comes from results container */
  display: flex;
  flex-direction: column;
  gap: 0.15rem; /* very small gap between messages */
}

/* Welcome section */
.cli-welcome {
  text-align: center;
  padding: 2rem 1rem;
  color: #6b7280;
}
.welcome-icon {
  font-size: 48px;
  margin-bottom: 1rem;
}
.cli-welcome h2 {
  margin: 0 0 0.5rem 0;
  font-size: 24px;
  color: #111827;
}
.cli-welcome p {
  margin: 0.25rem 0;
  font-size: 14px;
}
.welcome-hint {
  margin-top: 1rem;
  font-size: 13px;
  color: #9ca3af;
}

/* Message groups */
.message-group {
  display: flex;
  flex-direction: column;
  gap: 0.05rem; /* almost no gap between lines in same message block */
}

.message {
  width: 100%;
}

.command-message {
  text-align: right;
}

.response-message {
  text-align: left;
}

.message-content {
  display: block;
  max-width: 100%;
  padding: 0.05rem 0; /* minimal vertical padding */
  font-size: 14px;
  line-height: 1.3; /* tighter line height for more compact text */
  white-space: pre-wrap;
  word-wrap: break-word;
  overflow-wrap: break-word;
  word-break: break-word;
  color: #111827;
}

.command-message .message-content {
  font-weight: 600; /* user-typed commands bold */
  padding-right: 0.5rem; /* extra right offset to compensate for scrollbar */
}

.error-text {
  color: #dc2626;
}

/* Input wrapper */
.cli-input-wrapper {
  padding: 1rem;
  background: #ffffff;
  border-top: 1px solid #e5e7eb;
}

.cli-input-container {
  width: 100%;
  margin: 0;
  display: flex;
  gap: 0.5rem;
  background: white;
  border: 2px solid #d1d5db;
  border-radius: 0;
  padding: 0.5rem 0.75rem;       /* match results container inner horizontal padding */
  transition: border-color 0.2s;
}

.cli-input-container:focus-within {
  border-color: #2563eb;
  box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
}

.cli-input {
  flex: 1;
  background: transparent;
  border: none;
  font-size: 15px;
  color: #111827;
  outline: none;
  padding: 0.25rem 0;            /* vertical only; horizontal comes from container */
  font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
}

.cli-input::placeholder {
  color: #9ca3af;
}

.send-button {
  width: 40px;
  height: 40px;
  border-radius: 0.5rem;
  border: none;
  background: #2563eb;
  color: white;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
  flex-shrink: 0;
}

.send-button:hover:not(:disabled) {
  background: #1d4ed8;
  transform: scale(1.05);
}

.send-button:disabled {
  background: #e5e7eb;
  cursor: not-allowed;
}

.send-icon {
  font-size: 20px;
  font-weight: bold;
}

/* Custom scrollbar */
.cli-messages::-webkit-scrollbar {
  width: 8px;
}

.cli-messages::-webkit-scrollbar-track {
  background: #f3f4f6;
}

.cli-messages::-webkit-scrollbar-thumb {
  background: #d1d5db;
  border-radius: 4px;
}

.cli-messages::-webkit-scrollbar-thumb:hover {
  background: #9ca3af;
}

/* Formatted answer styling */
.code-block {
  background: #f3f4f6;
  border-left: 3px solid #3b82f6;
  margin: 0.5rem 0;
  border-radius: 4px;
  overflow: hidden;
}

.code-lang {
  background: #e5e7eb;
  padding: 0.25rem 0.75rem;
  font-size: 11px;
  font-weight: 600;
  color: #6b7280;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.code-content {
  padding: 0.75rem;
  margin: 0;
  font-family: 'Monaco', 'Menlo', 'Courier New', monospace;
  font-size: 13px;
  line-height: 1.5;
  color: #059669;
  background: #f9fafb;
  overflow-x: auto;
  white-space: pre;
}

.text-bold {
  font-weight: 600;
  color: #0ea5e9;
}

.text-dim {
  color: #9ca3af;
  font-size: 0.9em;
}

.text-comment {
  color: #9ca3af;
  font-style: italic;
}

.message-content {
  line-height: 1.6;
}
</style>

<!-- Non-scoped styles for v-html content (formatAnswer) -->
<style>
/* Code block styling for dynamically inserted HTML */
.cli-interface .message-content .code-block {
  background: #f3f4f6;
  border-left: 3px solid #3b82f6;
  margin: 0.5rem 0;
  border-radius: 4px;
  overflow: hidden;
}

.cli-interface .message-content .code-lang {
  background: #e5e7eb;
  padding: 0.25rem 0.75rem;
  font-size: 11px;
  font-weight: 600;
  color: #6b7280;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.cli-interface .message-content .code-content {
  padding: 0.75rem;
  margin: 0;
  font-family: 'Monaco', 'Menlo', 'Courier New', monospace;
  font-size: 13px;
  line-height: 1.5;
  color: #059669;
  background: #f9fafb;
  overflow-x: auto;
  white-space: pre;
}

.cli-interface .message-content .text-bold {
  font-weight: 600;
  color: #0ea5e9;
}

.cli-interface .message-content .text-dim {
  color: #9ca3af;
  font-size: 0.9em;
}

.cli-interface .message-content .text-comment {
  color: #9ca3af;
  font-style: italic;
}

/* Streaming cursor animation */
@keyframes blink {
  0%, 50% { opacity: 1; }
  51%, 100% { opacity: 0; }
}

.cli-interface .streaming-cursor {
  animation: blink 1s infinite;
  color: #2563eb;
}
</style>
