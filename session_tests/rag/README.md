# RAG Code Assistant

Ask questions about your Session codebase using Claude AI.

## 📁 Location

All RAG files are in `session_tests/rag/`:
- `rag_pipeline.py` - Code ingestion
- `rag_api.py` - Flask REST API
- `rag_answer_generator.py` - Claude AI
- `rag_requirements.txt` - Dependencies
- `start.sh` - Startup script

Database: `session_tests/rag_db/`

## 🚀 Quick Start

### 1. Install Dependencies (First Time)

```bash
cd session_tests/rag
pip install -r rag_requirements.txt
```

### 2. Ingest Code (First Time)

```bash
python3 rag_pipeline.py ingest
```

Creates `rag_db/` with Point & Color classes.

### 3. Start RAG API

```bash
./start.sh
```

Or just run from root:
```bash
./minitest.sh
```

### 4. Use Vue Interface

Open http://localhost:8769/session/

Ask questions in the command line at the bottom:
- `how to create a point`
- `how to measure distance between points`
- `what color methods are available`

## 📝 Adding New Classes to RAG

When you add a new class (e.g., `Vector`, `Mesh`, `BoundingBox`):

### 1. Add Files to Ingest List

Edit `rag_pipeline.py` around **line 306**:

```python
files_to_ingest = [
    # Python
    (self.repo_root / "session_py/src/session_py/point.py", "python"),
    (self.repo_root / "session_py/src/session_py/color.py", "python"),
    (self.repo_root / "session_py/src/session_py/vector.py", "python"),  # NEW

    # C++
    (self.repo_root / "session_cpp/src/point.h", "cpp"),
    (self.repo_root / "session_cpp/src/point.cpp", "cpp"),
    (self.repo_root / "session_cpp/src/color.h", "cpp"),
    (self.repo_root / "session_cpp/src/color.cpp", "cpp"),
    (self.repo_root / "session_cpp/src/vector.h", "cpp"),  # NEW

    # Rust
    (self.repo_root / "session_rust/src/point.rs", "rust"),
    (self.repo_root / "session_rust/src/color.rs", "rust"),
    (self.repo_root / "session_rust/src/vector.rs", "rust"),  # NEW
]
```

### 2. Reingest Code

```bash
# Clear old database and reingest with new files
python3 rag_pipeline.py clear
python3 rag_pipeline.py ingest
```

### 3. Restart RAG API

```bash
pkill -f rag_api.py
python3 rag_api.py &
```

That's it! Now you can ask about Vector class too.

## ⚡ Speed Optimization

### Why Is It Slow?

The system has 3 steps for each query:

1. **Embedding** (~200ms) - Convert question to vector using sentence-transformer model
2. **Search** (~100ms) - Find similar code in ChromaDB
3. **Claude API** (~2-3 seconds) - Generate answer

**Total: ~3 seconds per query**

### How to Make It Faster

#### Option 1: Use Faster Model (Recommended)
The current model is `claude-3-haiku-20240307` (fast and cheap).

To use an even faster model, edit `rag_answer_generator.py` line 101:
```python
model="claude-3-haiku-20240307",  # Current: ~2-3s, $0.001/query
```

#### Option 2: Cache Common Questions
Edit `rag_answer_generator.py` to add simple caching:

```python
_answer_cache = {}

def generate_answer(query: str, results: list) -> str:
    # Check cache first
    if query in _answer_cache:
        return _answer_cache[query]

    # ... existing code ...

    # Cache result before returning
    _answer_cache[query] = answer
    return answer
```

#### Option 3: Reduce Embedding Model Load Time
The sentence transformer model loads on first query (~500ms). This is normal and happens once when server starts.

#### Option 4: Use Local LLM (Free but Slower)
If Claude API is too slow/expensive, you can use a local LLM like Ollama:

```bash
# Install Ollama
curl -fsSL https://ollama.com/install.sh | sh

# Download a fast model
ollama pull llama3.2:3b

# Modify rag_answer_generator.py to use Ollama instead of Claude
```

## 🔧 Troubleshooting

### "No module named 'anthropic'"
```bash
pip install anthropic
```

### "Port 8770 already in use"
```bash
pkill -f rag_api.py
```

### "ChromaDB collection not found"
```bash
python3 rag_pipeline.py ingest
```

### Answers are wrong/incomplete
- Make sure the relevant code files are added to `rag_pipeline.py`
- Reingest: `python3 rag_pipeline.py clear && python3 rag_pipeline.py ingest`
- Check logs: `tail -f /tmp/rag_test.log`

## 📊 Current Performance

- **Database**: 83 code documents (Point + Color in 3 languages)
- **Query time**: ~3 seconds (mostly Claude API)
- **Cost**: ~$0.001 per query (Claude Haiku)
- **Accuracy**: High (Claude AI with actual code context)

## 🎓 How It Works

```
User Question
    ↓
[Sentence Transformer] → Convert to vector (200ms)
    ↓
[ChromaDB Search] → Find similar code (100ms)
    ↓
[Claude API] → Generate answer with code context (2-3s)
    ↓
Formatted Answer (Python/C++/Rust examples)
```

## 🔗 Integration Points

- **Vue Interface**: `session_tests/src/components/CliInterface.vue`
- **Flask API**: Runs on `http://localhost:8770/api/ask`
- **Vector DB**: Stored in `./rag_db/` (persistent)
- **MCP Server**: For Claude Desktop integration (optional)

---

**Need help?** Check logs at `/tmp/rag_test.log`
