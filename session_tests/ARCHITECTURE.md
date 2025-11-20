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
├── package.json            # npm dependencies and scripts
├── vite.config.js         # Vite configuration
├── index.html             # Entry HTML
├── testData.js            # Auto-generated test data (by minitest.sh)
├── src/
│   ├── main.js           # Vue app entry point
│   ├── App.vue           # Main app component
│   └── components/
│       ├── TestViewer.vue      # Test results display
│       └── CliInterface.vue    # ChatGPT-like CLI
├── dist/                  # Production build output
└── node_modules/          # npm dependencies
```

### 3. **Tab-Based Interface**
- **General Tab**: Description page with placeholder for 3D viewer
- **Tests Tab**: All test results with suite navigation
- More tabs can be easily added by editing the `tabs` array in `app.js`

### 4. **Horizontal Split Layout**
- Top panel: Main content (tests, 3D viewer, etc.)
- Bottom panel: CLI interface for user interaction
- CLI context changes based on active tab

## Component Details

### TestViewer.js
- Displays test results in a 3-column layout (Python, C++, Rust)
- Shows code snippets with syntax highlighting
- Displays timing comparisons with color-coded performance
- Shows failing checks for failed tests
- Sidebar navigation for different test suites

### CliInterface.js
- Terminal-style command interface
- Context-aware commands based on active tab
- Command history display
- Available commands:
  - `help` - Show available commands
  - `clear` - Clear console
  - `info` - Show current context
  - `search` - Search tests (Tests tab only)
  - `stats` - Show statistics (Tests tab only)
  - `viewer` - 3D viewer commands (General tab only)

## Adding New Test Suites

1. Add test execution to `minitest.sh` (before the testData.js generation)
2. Add the JSON output path to the `SOURCES` array in `generate_test_data_js()` function
3. The test suite will automatically appear in the sidebar

## Adding New Tabs

Edit `app.js` and add to the `tabs` array:
```javascript
const tabs = [
  { id: 'general', name: 'General' },
  { id: 'tests', name: 'Tests' },
  { id: 'my-new-tab', name: 'My New Tab' } // Add here
];
```

Then add the corresponding template in `index.html`:
```html
<div v-if="activeTab === 'my-new-tab'">
  <!-- Your tab content here -->
</div>
```

## Development Workflow

1. Make code changes to Python/C++/Rust implementations
2. Run `bash minitest.sh` to:
   - Execute all tests
   - Generate `testData.js`
   - Start HTTP server
   - Open browser
3. Results display instantly (no loading delay)
4. Use CLI to interact with the interface

## Browser Requirements

- Modern browser with ES6 module support
- Must be served via HTTP (not file://) for proper CORS handling
- `minitest.sh` automatically starts a local HTTP server on port 8765
