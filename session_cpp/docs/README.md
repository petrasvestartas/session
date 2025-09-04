# Documentation

## Ubuntu

**Install:**
```bash
sudo apt update
sudo apt install -y doxygen graphviz
```

**Generate:**
```bash
# From project root
./docs.sh
```

**Output:** `docs_output/html/index.html`

**Troubleshooting:**
- "dot: not found": `sudo apt install graphviz`
- No output: `rm -rf docs_output && ./docs.sh`

---

## macOS

**Install:**
```bash
brew install doxygen graphviz
```

**Generate:**
```bash
# From project root
./docs.sh
```

**Output:** `docs_output/html/index.html`

**Troubleshooting:**
- "dot: not found": `brew install graphviz`
- No output: `rm -rf docs_output && ./docs.sh`

---

## Windows

**Install:**
1. Download and install [Doxygen](https://www.doxygen.nl/download.html)
2. Download and install [Graphviz](https://graphviz.org/download/)
3. Add both to system PATH

**Generate:**
```cmd
REM From project root
docs.sh
```

**Output:** `docs_output/html/index.html`

**Troubleshooting:**
- "dot: not found": Install Graphviz and add to PATH
- No output: `rmdir /s docs_output && docs.sh`
