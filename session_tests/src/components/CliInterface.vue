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

            <!-- Answer / output aligned left -->
            <div v-else class="message response-message">
              <div class="message-content" :class="{ 'error-text': item.type === 'error' }">{{ item.text }}</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, nextTick } from 'vue'

const props = defineProps({
  activeTab: { type: String, required: true }
})

const history = ref([])
const currentInput = ref('')
const outputRef = ref(null)
const inputRef = ref(null)

const tabContext = computed(() => {
  if (props.activeTab === 'viewer') return 'Viewer'
  if (props.activeTab === 'tests') return 'Tests'
  return 'Viewer'
})

const inputPlaceholder = computed(() => {
  if (props.activeTab === 'viewer') {
    return 'Type your command here or write "help" -> Viewer'
  }
  if (props.activeTab === 'tests') {
    return 'Type your command here or write "help" -> Tests'
  }
  return 'Type your command here or write "help"'
})

const commands = {
  help: () => {
    const lines = [
      'Available commands:',
      '  help       - Show this help message',
      '  clear      - Clear the console',
      '  search     - Search test database (tests tab only)',
      '  stats      - Show test statistics (tests tab only)',
      '  viewer     - 3D viewer commands (general tab only)',
      '  info       - Show current context information'
    ]
    return [lines.join('\n')]
  },
  clear: () => {
    history.value = []
    return []
  },
  info: () => {
    const lines = [
      `Current tab: ${props.activeTab}`,
      `Context: ${tabContext.value}`,
      `Commands available in this context`
    ]
    return [lines.join('\n')]
  },
  search: () => {
    if (props.activeTab !== 'tests') {
      return ['Error: search command only available in Tests tab']
    }
    return ['Search functionality coming soon...', 'Will support: test name, language, status filters']
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
  }
}

const executeCommand = () => {
  const cmd = currentInput.value.trim()
  if (!cmd) return

  // Add command to history
  history.value.push({ text: cmd, type: 'command' })

  // Parse and execute command
  const [baseCmd, ...args] = cmd.toLowerCase().split(' ')
  
  if (commands[baseCmd]) {
    const result = commands[baseCmd](args)
    result.forEach(line => {
      history.value.push({ text: line, type: 'output' })
    })
  } else {
    history.value.push({ 
      text: `Unknown command: ${baseCmd}. Type 'help' for available commands.`, 
      type: 'error' 
    })
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
</style>
