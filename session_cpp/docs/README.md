# Session C++ Documentation

Modern C++ API documentation using **[doxygen-awesome-css](https://github.com/jothepro/doxygen-awesome-css)** theme.

## ✨ Features
- 🌘 **Dark mode toggle**
- 📋 **Copy buttons** for code snippets  
- 📑 **Interactive table of contents**
- 🌲 **Tree-view navigation**
- 📱 **Mobile responsive design**

## 🚀 Quick Start

### Ubuntu
**Install:** (Graphviz no longer needed!)
```bash
sudo apt update
sudo apt install -y doxygen
```

### macOS
**Install:**
```bash
brew install doxygen
```

### Windows
**Install:**
1. Download and install [Doxygen](https://www.doxygen.nl/download.html)
2. Add to system PATH

## 📖 Generate Documentation

**Option 1: Modern build script (Recommended)**
```bash
# From session_cpp directory
./build_docs.sh           # Build docs
./build_docs.sh --open    # Build and open in browser
```

**Option 2: Traditional script**
```bash
# From session_cpp directory  
./doc.sh
```

**Output:** `docs_output/html/index.html`

## 🛠️ Troubleshooting
- **No output:** `rm -rf docs_output && ./build_docs.sh`
- **Permission denied:** `chmod +x build_docs.sh doc.sh`
- **Doxygen not found:** Ensure Doxygen is installed and in PATH

## 📁 Structure
```
docs/
├── Doxyfile              # Main configuration
├── header.html           # Custom header with JS extensions
├── doxygen-awesome/      # Theme files
│   ├── doxygen-awesome.css
│   └── *.js             # Interactive extensions
└── README.md            # This file
```

**Note:** Graphviz is no longer required since we use doxygen-awesome-css theme instead of generated diagrams.
