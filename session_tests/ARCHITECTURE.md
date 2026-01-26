# Session Test Viewer Architecture

## Overview
The Session Test Viewer is a **proper Vue 3 + Vite application** with instant loading, modern build tooling, and a ChatGPT-like CLI interface.

## Technology Stack
- **Vue 3** - Progressive JavaScript framework with Composition API
- **Vite** - Next-generation frontend build tool
- **highlight.js** - Syntax highlighting for code snippets
- **npm** - Package management

## Key Improvements

### 1. **Proper Vue 3 Project with npm**
- Full npm-based setup with `package.json`
- Vite for fast development and optimized production builds
- Single File Components (.vue) with scoped styles
- Hot Module Replacement (HMR) during development

### 2. **Instant Loading with Pre-processed Data**
- `minitest.sh` generates `testData.js` with all test results
- No async JSON fetches on page load - data available immediately
- Significantly faster page load times

### 3. **Modular Component Architecture**
```
session_tests/
├── package.json              # npm dependencies and scripts
├── vite.config.js           # Vite configuration (port 8769, build)
├── index.html               # Vite entry HTML (mounts Vue, loads /testData.js)
├── public/
│   └── testData.js          # Auto-generated test data (by minitest.sh)
├── src/
│   ├── main.js              # Vue app entry point
│   ├── App.vue              # Root component, renders MainLayout
│   ├── router.js            # Vue Router configuration (Viewer / Tests)
│   ├── components/
│   │   ├── layout/
│   │   │   └── MainLayout.vue   # Tabs, CLI, resizer, and page content
│   │   ├── TestViewer.vue       # Test results display (table)
│   │   └── CliInterface.vue     # ChatGPT-like CLI (commands + history)
│   └── views/
│       ├── GeneralView.vue      # Viewer tab content (3D placeholder)
│       └── TestsView.vue        # Tests tab content (wraps TestViewer)
├── session_cpp/               # JSON mini-test outputs (C++)
├── session_py/                # JSON mini-test outputs (Python)
├── session_rust/              # JSON mini-test outputs (Rust)
├── dist/                      # Production build output (Vite build)
└── node_modules/              # npm dependencies
```

### 3. **Tab-Based Interface**
- **Viewer tab** (`/viewer`):
  - Main 3D viewer area (currently a placeholder box)
  - CLI in *Viewer* mode (e.g. future viewer commands)
- **Tests tab** (`/tests`):
  - Test results table (Python / C++ / Rust columns)
  - Suite selection via dropdown submenu on the **Tests** tab in the top bar
  - CLI in *Tests* mode (search/stats to be implemented)
- Routing and tabs are defined in `src/router.js` and `MainLayout.vue`.

### 4. **Horizontal Split Layout**
- Top panel: Main content (tests, 3D viewer, etc.)
- Bottom panel: CLI interface for user interaction
- CLI context changes based on active tab

## Component Details

### TestViewer.vue
- Displays test results in a 3-column layout (Python, C++, Rust)
- Shows code snippets with syntax highlighting (highlight.js)
- Displays timing comparisons with color-coded performance
- Shows failing checks for failed tests
- Test suite is selected via the **Tests tab dropdown**, not a sidebar

### CliInterface.vue
- ChatGPT-like command interface
- Input box on top, answers in a resizable results box below
- Commands:
  - `help`   – Show available commands (concise multi-line block)
  - `clear`  – Clear console
  - `info`   – Show current tab + context
  - `search` – Search tests (Tests tab only, placeholder)
  - `stats`  – Show statistics (Tests tab only, placeholder)
  - `viewer` – 3D viewer commands (Viewer tab only, placeholder)
- Command history is kept in memory while the page is open.

## Adding New Test Suites

See `CLAUDE.md` in the repo root for the full workflow.

## Adding New Tabs

1. **Add a route** in `src/router.js`:

```js
import MyNewView from './views/MyNewView.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', redirect: '/viewer' },
    { path: '/viewer', component: GeneralView },
    { path: '/tests', component: TestsView },
    { path: '/my-new-tab', component: MyNewView } // New tab
  ]
})
```

2. **Add a link** in `MainLayout.vue` if you want a top-level tab:

```html
<router-link
  :to="'/my-new-tab'"
  class="tab-button"
  active-class="active">
  My New Tab
</router-link>
```

3. **Create the view component** in `src/views/MyNewView.vue`:

```vue
<template>
  <div class="my-new-view">
    <!-- Your tab content here -->
  </div>
</template>

<script setup>
// Optional: component logic
</script>

<style scoped>
/* Optional: styles */
</style>
```

## Development Workflow

### Run all tests + Vue viewer
```bash
./bash/minitest.sh
```

### Options
```bash
./bash/minitest.sh --py         # Python only
./bash/minitest.sh --cpp        # C++ only
./bash/minitest.sh --rust       # Rust only
./bash/minitest.sh --fast       # Skip dependency installs
./bash/minitest.sh --no-web     # Skip Vue server
./bash/minitest.sh --kill       # Stop dev server
```

### Frontend-only development
```bash
cd session_tests && npm install && npm run dev
```

Opens at http://localhost:8769/
