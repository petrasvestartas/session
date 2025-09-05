# Session Multi-Language Project

Multi-language project for vizualization.

## Project Structure

```
session/
├── session_py/     # Python implementation
├── session_rust/   # Rust implementation  
└── session_cpp/    # C++ implementation
```

## Alias

### Directory shortcuts

```bash
alias ls_session='cd /home/pv/brg/code_rust/session/'
```

### Build

```bash
alias r='(cd /home/pv/brg/code_rust/session/session_rust && cargo run)'
alias p='(cd /home/pv/brg/code_rust/session/session_py && conda activate session && python main.py)'
alias c='(cd /home/pv/brg/code_rust/session/session_cpp && ./build.sh)'
```

### Doc

```bash
alias rdoc='(cd /home/pv/brg/code_rust/session/session_rust && cargo doc)'
alias pdoc='(cd /home/pv/brg/code_rust/session/session_py && ./doc.sh)'
alias cdoc='(cd /home/pv/brg/code_rust/session/session_cpp && ./doc.sh)'
```

