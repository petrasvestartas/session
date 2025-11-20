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

You can think of a "test section" as a *suite* like `point_test` or `color_test`.

### Step-by-step

1. **Produce JSON output for the new suite** in each language (optional per language):
   - Python: write to `session_tests/session_py/<suite_name>.json`
   - C++: write to `session_tests/session_cpp/<suite_name>.json`
   - Rust: write to `session_tests/session_rust/<suite_name>.json`

   The JSON structure should match the existing files in those folders
   (e.g. `point_test.json`, `color_test.json`). The easiest way is to copy
   one of those and adapt it.

2. **Run the new mini tests in `minitest.sh`**

   In `session_tests/minitest.sh`, before the `generate_test_data_js()` call,
   add the commands that build/run your new mini tests and write the JSON
   files mentioned above.

3. **Register the JSON files in `generate_test_data_js()`**

   Still in `minitest.sh`, in the `SOURCES` array inside
   `generate_test_data_js()` add entries for the new suite, e.g. for a
   `vector_test` suite:

   ```bash
   local SOURCES=(
     "session_py/point_test.json:python"
     "session_cpp/point_test.json:cpp"
     "session_rust/point_test.json:rust"
     "session_py/color_test.json:python"
     "session_cpp/color_test.json:cpp"
     "session_rust/color_test.json:rust"
     # New suite "vector_test"
     "session_py/vector_test.json:python"
     "session_cpp/vector_test.json:cpp"
     "session_rust/vector_test.json:rust"
   )
   ```

   When `generate_test_data_js()` runs, it will:
   - Read each JSON file.
   - Generate keys like `vector_test_python`, `vector_test_cpp`, etc.
   - Append their data into `window.TEST_DATA`.

4. **Run `minitest.sh`**

   From the repo root:

   ```bash
   bash session_tests/minitest.sh
   ```

   This regenerates `public/testData.js` and restarts the dev server.

5. **Verify in the UI**

   - Open `http://localhost:8769/`.
   - Go to the **Tests** tab.
   - Open the **Tests** dropdown in the top bar.
   - You should now see your new suite name (e.g. `vector_test`) as an option.
   - Selecting it will show the tests for that suite in the table via
     `TestsView.vue` and `TestViewer.vue`.

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

## Development Workflow (Step-by-step)

### A. Backend tests + data generation

1. Make code changes to Python / C++ / Rust implementations.
2. From the repo root, run:

   ```bash
   bash session_tests/minitest.sh
   ```

   This will:
   - Run Python mini tests (if the environment is available).
   - Build and run C++ and Rust mini tests.
   - Generate consolidated `public/testData.js` from the JSON outputs in
     `session_cpp/`, `session_py/`, `session_rust/`.
   - Run `npm install` (first time only) inside `session_tests/`.
   - Run `npm run build` (Vite production build).
   - Start the Vite dev server on **http://localhost:8769/**.
   - Open the browser to **http://localhost:8769/**.

3. Once running, the Vue app loads immediately using `window.TEST_DATA` from `testData.js`.

### B. Frontend-only development

If you just want to work on the Vue UI (no new backend test data):

1. In `session_tests/`:

   ```bash
   npm install        # first time only
   npm run dev        # starts Vite dev server on port 8769
   ```

2. Open `http://localhost:8769/` in your browser.
3. Use the **Viewer** and **Tests** tabs and the CLI to test interactions.

## Browser Requirements

- Modern browser with ES6 module support
- Must be served via HTTP (not file://) for proper CORS handling
- `minitest.sh` automatically starts the Vite dev server on port **8769**
