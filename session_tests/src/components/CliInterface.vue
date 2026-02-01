<template>
  <div class="cli-interface" ref="containerRef">
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
  </div>
</template>

<script setup>
import { ref, computed, nextTick, onMounted, onUnmounted } from 'vue'
import { getClaudeApiKey } from '../firebase.js'

const props = defineProps({
  activeTab: { type: String, required: true }
})

const history = ref([])
const currentInput = ref('')
const outputRef = ref(null)
const inputRef = ref(null)
const containerRef = ref(null)
let resizeObserver = null

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

  if (containerRef.value) {
    resizeObserver = new ResizeObserver(() => {
      nextTick(() => {
        if (outputRef.value) {
          outputRef.value.scrollTop = outputRef.value.scrollHeight
        }
      })
    })
    resizeObserver.observe(containerRef.value)
  }
})

onUnmounted(() => {
  if (resizeObserver) {
    resizeObserver.disconnect()
  }
})

// Synonym map for recipe search - maps user words to recipe tag words
const RECIPE_SYNONYMS = {
  perpendicular: ['frame', 'plane', 'normal'],
  normal: ['frame', 'plane', 'perpendicular'],
  frame: ['plane', 'perpendicular', 'frame_at'],
  frames: ['plane', 'perpendicular', 'frame_at'],
  planes: ['frame', 'perpendicular', 'plane'],
  subdivide: ['divide', 'divide_by_count', 'divide_by_length', 'split'],
  split: ['divide', 'subdivide', 'divide_by_count'],
  divide: ['subdivide', 'divide_by_count', 'divide_by_length'],
  points: ['point', 'divide_by_count', 'subdivide'],
  sample: ['divide', 'subdivide', 'divide_by_count'],
  evaluate: ['point_at', 'tangent_at', 'frame_at'],
  tangent: ['tangent_at', 'frame_at'],
  curvature: ['frame_at', 'kappa'],
  adaptive: ['to_polyline_adaptive', 'polyline'],
  polyline: ['to_polyline_adaptive', 'adaptive'],
}

// Recipe search - matches query words against recipe tags/titles with synonyms
const searchRecipes = (query, maxResults = 3) => {
  const index = window.API_INDEX
  if (!index || !index.recipes) return []

  const queryWords = query.toLowerCase().split(/\s+/).filter(w => w.length >= 2)
  if (queryWords.length === 0) return []

  // Expand query words with synonyms
  const expandedWords = new Set(queryWords)
  for (const word of queryWords) {
    const syns = RECIPE_SYNONYMS[word]
    if (syns) syns.forEach(s => expandedWords.add(s))
  }

  return index.recipes
    .map(recipe => {
      let score = 0
      const titleLower = recipe.title.toLowerCase()
      // Direct word matches score highest
      for (const word of queryWords) {
        if (recipe.tags.includes(word)) score += 20
        if (titleLower.includes(word)) score += 10
      }
      // Synonym matches score lower
      for (const word of expandedWords) {
        if (!queryWords.includes(word) && recipe.tags.includes(word)) score += 8
      }
      return { ...recipe, score }
    })
    .filter(r => r.score > 0)
    .sort((a, b) => b.score - a.score)
    .slice(0, maxResults)
}

// Concept-based search - returns unified API concepts with all languages
const searchConcepts = (query, maxResults = 3) => {
  const index = window.API_INDEX
  if (!index || !index.concepts) {
    return { results: [], error: 'API index not loaded. Run: python mcp/generate_browser_index.py' }
  }

  const queryLower = query.toLowerCase()
  const queryWords = queryLower.split(/\s+/).filter(w => w.length >= 2)

  // Detect language filter
  let langFilter = null
  if (queryLower.includes('python')) langFilter = 'python'
  else if (queryLower.includes('rust')) langFilter = 'rust'
  else if (queryLower.includes('c++') || queryLower.includes('cpp')) langFilter = 'cpp'

  // Score each concept
  const scored = index.concepts.map(concept => {
    let score = 0
    const nameLower = concept.name.toLowerCase()
    const methodName = concept.name.split('.')[1] || ''

    // Skip if language filter doesn't match
    if (langFilter && !concept.implementations[langFilter]) {
      return { ...concept, score: 0 }
    }

    // Exact method name match
    if (methodName && queryLower.includes(methodName.toLowerCase())) score += 100
    // Class name match
    if (nameLower.split('.')[0] && queryLower.includes(nameLower.split('.')[0])) score += 30
    // Partial name match
    for (const word of queryWords) {
      if (nameLower.includes(word)) score += 40
    }
    // Constructor patterns
    if ((queryLower.includes('create') || queryLower.includes('new') || queryLower.includes('how to')) &&
        (methodName === '__init__' || methodName === 'new' || methodName === 'constructor')) {
      score += 100
    }

    return { ...concept, score }
  })

  // Sort and filter
  let results = scored
    .filter(c => c.score > 0)
    .sort((a, b) => b.score - a.score)
    .slice(0, maxResults)

  // Keep only requested language in output
  if (langFilter) {
    results = results.map(c => ({
      ...c,
      implementations: { [langFilter]: c.implementations[langFilter] }
    }))
  }

  return { results, error: null, langFilter }
}

// Determine API endpoint - use Cloudflare Worker proxy in production, direct in development
const CLAUDE_ENDPOINT = import.meta.env.PROD 
  ? 'https://session-claude-proxy.petrasvestartas.workers.dev'
  : 'https://api.anthropic.com/v1/messages'

// Call Claude API with streaming support
const callClaudeAPIStreaming = async (question, context, onChunk) => {
  const systemPrompt = `You are a coding assistant for the Session geometry library.

CRITICAL RULES:
- ONLY use method names, class names, and signatures that appear in the provided context. NEVER invent or guess API names.
- There is NO namespace like "session_cpp::". C++ classes are used directly: Point, NurbsCurve, Primitives, etc.
- C++ includes use: #include "classname.h" (NOT "session_cpp/classname.h")
- If a Recipe is provided, copy its code exactly as your answer — recipes are verified correct code.
- If you cannot answer from the context, say "I don't have enough context for this" instead of guessing.
- ONE sentence description, then code blocks
- Show only languages present in context
- Use EXACT signatures from context, do not modify them`

  const userMessage = `Question: ${question}

Here is relevant code from the codebase:

${context}

Please answer the question based on this code context.`

  // Build headers - production uses minimal headers (proxy adds the rest)
  const headers = {
    'Content-Type': 'application/json'
  }
  
  if (!import.meta.env.PROD) {
    // Development: call Claude directly
    if (!apiKey.value) {
      return { error: 'No API key configured.' }
    }
    headers['x-api-key'] = apiKey.value
    headers['anthropic-version'] = '2023-06-01'
    headers['anthropic-dangerous-direct-browser-access'] = 'true'
  }

  try {
    const response = await fetch(CLAUDE_ENDPOINT, {
      method: 'POST',
      headers,
      body: JSON.stringify({
        model: 'claude-3-haiku-20240307',
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

// Generate context from concept results and recipes for Claude
const buildContext = (concepts, recipes = []) => {
  const parts = []

  if (recipes && recipes.length > 0) {
    parts.push('# Recipes (verified multi-step examples)\n')
    for (const recipe of recipes) {
      parts.push(`## ${recipe.title}`)
      for (const [lang, code] of Object.entries(recipe.code)) {
        const langName = lang === 'cpp' ? 'C++' : lang.charAt(0).toUpperCase() + lang.slice(1)
        parts.push(`\n### ${langName}`)
        parts.push('```' + lang)
        parts.push(code)
        parts.push('```')
      }
      parts.push('')
    }
  }

  if (concepts && concepts.length > 0) {
    parts.push('# API Methods\n')
    for (const concept of concepts) {
      parts.push(`## ${concept.name}`)
      for (const [lang, impl] of Object.entries(concept.implementations)) {
        if (!impl) continue
        const langName = lang === 'cpp' ? 'C++' : lang.charAt(0).toUpperCase() + lang.slice(1)
        parts.push(`\n### ${langName}: ${impl.sig}`)
        parts.push('```' + lang)
        parts.push(impl.code)
        parts.push('```')
      }
      parts.push('')
    }
  }

  return parts.join('\n')
}

// Generate answer from concepts and recipes - shows actual code from source files
const generateAnswer = (query, concepts, recipes = []) => {
  const lines = []

  if (recipes && recipes.length > 0) {
    const best = recipes[0]
    lines.push(`# ${best.title}\n`)
    for (const lang of ['python', 'cpp', 'rust']) {
      const code = best.code[lang]
      if (!code) continue
      const langName = lang === 'cpp' ? 'C++' : lang.charAt(0).toUpperCase() + lang.slice(1)
      lines.push(`**${langName}:**`)
      lines.push('```' + lang)
      lines.push(code)
      lines.push('```\n')
    }
  }

  if (concepts && concepts.length > 0) {
    const best = concepts[0]
    lines.push(`# ${best.name}\n`)
    for (const lang of ['python', 'cpp', 'rust']) {
      const impl = best.implementations[lang]
      if (!impl) continue
      const langName = lang === 'cpp' ? 'C++' : lang.charAt(0).toUpperCase() + lang.slice(1)
      lines.push(`**${langName}:** \`${impl.sig}\``)
      lines.push('```' + lang)
      const codeLines = impl.code.split('\n').slice(0, 15)
      lines.push(codeLines.join('\n'))
      if (impl.code.split('\n').length > 15) lines.push('// ...')
      lines.push('```\n')
    }
  }

  if (lines.length === 0) {
    return `No results found for "${query}". Try: "distance", "Point", "Color", "Line", "create"`
  }

  return lines.join('\n')
}

// Generate clean usage example for a method
const generateExample = (lang, cls, method, impl) => {
  if (!impl) return null
  
  // Common patterns
  if (method === 'distance' || method === 'squared_distance') {
    if (lang === 'python') return `p1 = Point(1.0, 2.0, 3.0)\np2 = Point(4.0, 5.0, 6.0)\ndist = p1.${method}(p2)`
    if (lang === 'cpp') return `Point p1(1.0, 2.0, 3.0);\nPoint p2(4.0, 5.0, 6.0);\ndouble dist = p1.${method}(p2);`
    if (lang === 'rust') return `let p1 = Point::new(1.0, 2.0, 3.0);\nlet p2 = Point::new(4.0, 5.0, 6.0);\nlet dist = p1.${method}(&p2, None);`
  }
  if (method === '__init__' || method === 'new') {
    if (lang === 'python') return `p = ${cls}(1.0, 2.0, 3.0)`
    if (lang === 'cpp') return `${cls} p(1.0, 2.0, 3.0);`
    if (lang === 'rust') return `let p = ${cls}::new(1.0, 2.0, 3.0);`
  }
  if (method === 'duplicate') {
    if (lang === 'python') return `copy = original.duplicate()`
    if (lang === 'cpp') return `auto copy = original.duplicate();`
    if (lang === 'rust') return `let copy = original.duplicate();`
  }
  if (method === 'to_protobuf') {
    if (lang === 'python') return `data = obj.to_protobuf()`
    if (lang === 'cpp') return `std::string data = obj.to_protobuf();`
    if (lang === 'rust') return `let data: Vec<u8> = obj.to_protobuf();`
  }
  if (method === 'from_protobuf') {
    if (lang === 'python') return `obj = ${cls}.from_protobuf(data)`
    if (lang === 'cpp') return `auto obj = ${cls}::from_protobuf(data);`
    if (lang === 'rust') return `let obj = ${cls}::from_protobuf(&data);`
  }
  if (method === 'transform') {
    if (lang === 'python') return `point.transform(matrix)`
    if (lang === 'cpp') return `point.transform(matrix);`
    if (lang === 'rust') return `point.transform(&matrix);`
  }
  // Color presets
  if (['white','black','red','green','blue','yellow','cyan','magenta'].includes(method)) {
    if (lang === 'python') return `c = Color.${method}()`
    if (lang === 'cpp') return `Color c = Color::${method}();`
    if (lang === 'rust') return `let c = Color::${method}();`
  }
  
  // Fallback: show signature
  return `// ${impl.sig}`
}

// Describe what a method does
const describeMethod = (method) => {
  const descriptions = {
    'distance': 'calculate distance between two points',
    'squared_distance': 'calculate squared distance (faster, no sqrt)',
    '__init__': 'create a new instance',
    'new': 'create a new instance',
    'duplicate': 'create a copy',
    'transform': 'apply a transformation matrix',
    'to_protobuf': 'serialize to protobuf format',
    'from_protobuf': 'deserialize from protobuf',
    'to_json': 'convert to JSON',
    'from_json': 'load from JSON',
    'white': 'create white color',
    'black': 'create black color',
    'red': 'create red color',
    'blue': 'create blue color',
  }
  return descriptions[method] || `use ${method}`
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

    // Search recipes first (compositional queries), then concepts
    const recipeResults = searchRecipes(question, 2)
    const { results, error: searchError } = searchConcepts(question, 3)

    if (searchError && recipeResults.length === 0) {
      return [
        'API index not available.',
        'Run: python mcp/generate_browser_index.py',
        '',
        `Details: ${searchError}`
      ]
    }

    if (results.length === 0 && recipeResults.length === 0) {
      return [
        `No results found for "${question}".`,
        '',
        'Try searching for:',
        '  - distance, duplicate, transform (methods)',
        '  - Point, Color (classes)',
        '  - "create a circle and subdivide into points" (recipes)'
      ]
    }

    // If a recipe matches strongly, show it directly — more reliable than LLM rewriting
    if (recipeResults.length > 0 && recipeResults[0].score >= 40) {
      const answer = generateAnswer(question, [], recipeResults)
      return answer.split('\n')
    }

    // If Claude API key is configured, use Claude with streaming
    if (apiKey.value) {
      const context = buildContext(results, recipeResults)

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
        const answer = generateAnswer(question, results, recipeResults)
        return [`(Claude unavailable: ${claudeResult.error})`, '', ...answer.split('\n')]
      }

      // Finalize the streamed response (remove cursor)
      if (history.value[responseIndex]) {
        history.value[responseIndex].text = claudeResult.answer
        history.value[responseIndex].streaming = false
      }

      return [] // Already added to history via streaming
    }

    // No API key - show recipe + concept code directly
    const answer = generateAnswer(question, results, recipeResults)
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
/* CLI Interface - Pure black theme */
.cli-interface {
  min-height: 160px;
  height: 100%;
  background: #000000;
  border: none;
  display: flex;
  flex-direction: column;
  position: relative;
  overflow: hidden;
}

.cli-results-wrapper {
  flex: 1;
  min-height: 0;
  overflow: hidden;
  padding: 0.25rem 1.5rem 1rem 1.5rem;
}

.cli-results-container {
  width: 100%;
  background: #000000;
  border: 1px solid #333333;
  border-radius: 4px;
  box-sizing: border-box;
  height: 100%;
  min-height: 0;
  display: flex;
  flex-direction: column;
  padding: 0.75rem;
  overflow: hidden;
}

/* Messages area */
.cli-messages {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  overflow-x: hidden;
  padding: 0;
  display: flex;
  flex-direction: column;
}

.cli-messages::before {
  content: '';
  flex: 1 1 auto;
}

/* Welcome section */
.cli-welcome {
  text-align: center;
  padding: 2rem 1rem;
  color: #888888;
}
.welcome-icon {
  font-size: 48px;
  margin-bottom: 1rem;
}
.cli-welcome h2 {
  margin: 0 0 0.5rem 0;
  font-size: 24px;
  color: #ffffff;
}
.cli-welcome p {
  margin: 0.25rem 0;
  font-size: 14px;
}
.welcome-hint {
  margin-top: 1rem;
  font-size: 13px;
  color: #666666;
}

/* Message groups */
.message-group {
  display: flex;
  flex-direction: column;
  gap: 0.05rem;
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
  padding: 0.05rem 0;
  font-size: 14px;
  line-height: 1.3;
  white-space: pre-wrap;
  word-wrap: break-word;
  overflow-wrap: break-word;
  word-break: break-word;
  color: #ffffff;
}

.command-message .message-content {
  font-weight: 600;
  padding-right: 0.5rem;
  color: #ffffff;
}

.error-text {
  color: #ff5555;
}

/* Input wrapper */
.cli-input-wrapper {
  padding: 1rem 1.5rem 1rem 1.5rem;
  background: #000000;
  border: none;
}

.cli-input-container {
  width: 100%;
  margin: 0;
  display: flex;
  gap: 0.5rem;
  background: #000000;
  border: 1px solid #333333;
  border-radius: 4px;
  padding: 0.5rem 0.75rem;
  transition: border-color 0.2s;
}

.cli-input-container:focus-within {
  border-color: #ffffff;
}

.cli-input {
  flex: 1;
  background: transparent;
  border: none;
  font-size: 14px;
  color: #ffffff;
  outline: none;
  padding: 0.25rem 0;
  font-family: inherit;
}

.cli-input::placeholder {
  color: #666666;
}

.send-button {
  width: 40px;
  height: 40px;
  border-radius: 0.5rem;
  border: none;
  background: #444444;
  color: #000000;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
  flex-shrink: 0;
}

.send-button:hover:not(:disabled) {
  background: #666666;
  transform: scale(1.05);
}

.send-button:disabled {
  background: #333333;
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
  background: #000000;
}

.cli-messages::-webkit-scrollbar-thumb {
  background: #333333;
  border-radius: 4px;
}

.cli-messages::-webkit-scrollbar-thumb:hover {
  background: #444444;
}

/* Formatted answer styling */
.code-block {
  background: #000000;
  border-left: 3px solid #444444;
  margin: 0.5rem 0;
  border-radius: 0;
  overflow: hidden;
}

.code-lang {
  background: #000000;
  padding: 0.25rem 0.75rem;
  font-size: 11px;
  font-weight: 600;
  color: #666666;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.code-content {
  padding: 0.75rem;
  margin: 0;
  font-family: 'Monaco', 'Menlo', 'Courier New', monospace;
  font-size: 13px;
  line-height: 1.5;
  color: #ffffff;
  background: #000000;
  overflow-x: auto;
  white-space: pre;
}

.text-bold {
  font-weight: 600;
  color: #ffffff;
}

.text-dim {
  color: #666666;
  font-size: 0.9em;
}

.text-comment {
  color: #888888;
  font-style: italic;
}

.message-content {
  line-height: 1.6;
}
</style>

<!-- Non-scoped styles for v-html content (formatAnswer) - Pure black theme -->
<style>
/* Code block styling for dynamically inserted HTML */
.cli-interface .message-content .code-block {
  background: #000000;
  border-left: 3px solid #444444;
  margin: 0.5rem 0;
  border-radius: 0;
  overflow: hidden;
}

.cli-interface .message-content .code-lang {
  background: #000000;
  padding: 0.25rem 0.75rem;
  font-size: 11px;
  font-weight: 600;
  color: #666666;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.cli-interface .message-content .code-content {
  padding: 0.75rem;
  margin: 0;
  font-family: 'Monaco', 'Menlo', 'Courier New', monospace;
  font-size: 13px;
  line-height: 1.5;
  color: #ffffff;
  background: #000000;
  overflow-x: auto;
  white-space: pre;
}

.cli-interface .message-content .text-bold {
  font-weight: 600;
  color: #ffffff;
}

.cli-interface .message-content .text-dim {
  color: #666666;
  font-size: 0.9em;
}

.cli-interface .message-content .text-comment {
  color: #888888;
  font-style: italic;
}

/* Streaming cursor animation */
@keyframes blink {
  0%, 50% { opacity: 1; }
  51%, 100% { opacity: 0; }
}

.cli-interface .streaming-cursor {
  animation: blink 1s infinite;
  color: #5588ff;
}
</style>
