# Session Library - Combined Documentation

This directory contains the unified documentation system for the Session Library, combining documentation from all language implementations (Rust, C++, and Python) into a single, beautiful interface.

## 🚀 Quick Start

### Build All Documentation

**Unix/Linux/macOS:**
```bash
./build_docs.sh           # Build all documentation
./build_docs.sh --open    # Build and open in browser
```

**Windows:**
```cmd
build_docs.bat            # Build all documentation
build_docs.bat --open     # Build and open in browser
```

### View Documentation

After building, open `combined_docs/index.html` in your browser to access the unified documentation portal.

## 📁 Structure

```
session_docs/
├── index.html           # Main documentation portal (beautiful landing page)
├── build_docs.sh        # Unix/Linux/macOS build script
├── build_docs.bat       # Windows build script  
├── README.md           # This file
└── combined_docs/      # Generated combined documentation
    ├── index.html      # Copied from above
    ├── rust/           # Rust documentation (cargo doc)
    │   └── html/
    ├── cpp/            # C++ documentation (doxygen with awesome theme)
    │   └── html/
    └── python/         # Python documentation (sphinx)
        └── html/
```

## 🛠️ Requirements

The build scripts automatically detect available tools and skip missing components:

### Rust Documentation
- **Required:** Rust toolchain (`cargo`)
- **Output:** Generated from `cargo doc --no-deps --document-private-items`

### C++ Documentation  
- **Required:** Doxygen
- **Features:** Modern doxygen-awesome theme with dark mode
- **Output:** Generated from existing `doc.sh` or direct Doxygen call

### Python Documentation
- **Required:** Python 3, pip
- **Dependencies:** sphinx, sphinx-rtd-theme, myst-parser (auto-installed)
- **Output:** Generated from existing `doc.sh` or direct Sphinx call

## 🎨 Features

### Beautiful Landing Page
- **Responsive design** - Works on desktop, tablet, and mobile
- **Modern styling** - Gradient backgrounds, hover effects, smooth animations
- **Language cards** - Interactive cards for each documentation set
- **Feature showcase** - Highlights library capabilities
- **Professional appearance** - Suitable for public documentation

### Cross-Platform Support
- **Unix/Linux/macOS** - Full bash script with error handling
- **Windows** - Native batch file with equivalent functionality
- **Git Bash support** - Windows batch can use bash scripts when available
- **Graceful degradation** - Skips unavailable components

### GitHub Actions Integration
The build process is designed to work seamlessly with CI/CD:
- Uses the same logic as GitHub Actions workflow
- Consistent output structure
- Error handling for missing dependencies
- Suitable for automated deployment

## 📋 Build Process

1. **Setup**: Creates output directories and copies main index.html
2. **Rust**: Builds with `cargo doc` if Rust is available
3. **C++**: Builds with Doxygen using existing scripts
4. **Python**: Installs dependencies and builds with Sphinx
5. **Summary**: Reports what was successfully built
6. **Optional**: Opens documentation in browser with `--open` flag

## 🔧 Customization

### Main Landing Page
Edit `index.html` to customize:
- Colors and styling (CSS variables)
- Content and descriptions
- Layout and sections
- Branding and links

### Build Scripts
Modify `build_docs.sh` or `build_docs.bat` to:
- Change build commands
- Add pre/post-build steps
- Customize output locations
- Add additional checks

## 🌐 Deployment

### Local Development
```bash
# Build and serve locally
./build_docs.sh --open
```

### GitHub Pages
The generated `combined_docs` folder is ready for GitHub Pages deployment.

### Web Server
Copy `combined_docs/` to any web server. All paths are relative and will work without modification.

## 📖 Usage Examples

### Full Build
```bash
# Build everything available
./build_docs.sh
```

### Check What's Built
```bash
# Run and check the summary output
./build_docs.sh
# Look for ✅ (success) or ❌ (failed/skipped) indicators
```

### Quick Development Workflow
```bash
# Build and immediately open for review
./build_docs.sh --open
```

## 🎯 Integration with GitHub Actions

This documentation system is designed to integrate seamlessly with the GitHub Actions workflow in `.github/workflows/build-docs.yml`. The workflow uses the same structure and copies the `combined_docs/` output for deployment.

The main index.html is maintained here rather than inline in the workflow YAML, making it easier to:
- Edit and preview the landing page
- Maintain proper HTML formatting
- Version control the documentation design
- Test locally before deployment
