# Session Multi-Language Project

Multi-language project for vizualization.

## Plan

- [x] Convert python to C++ and Rust: point, vector, color
- [x] Separate python tests to to separate files same as in c++
- [ ] Convert python to C++ and Rust: tree, objects, graph, session
- [ ] Python: Finish the python point, vector structures
- [ ] Convert python to C++ and Rust: point, vector
- [ ] Python: Line


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
alias ls_session_py='cd /home/pv/brg/code_rust/session/session_py/'
alias ls_session_py_src='cd /home/pv/brg/code_rust/session/session_py/src/session_py/'
alias ls_session_cpp='cd /home/pv/brg/code_rust/session/session_cpp/'
alias ls_session_cpp_src='cd /home/pv/brg/code_rust/session/session_cpp/src/'
alias ls_session_rust='cd /home/pv/brg/code_rust/session/session_rust'
alias ls_session_rust_src='cd /home/pv/brg/code_rust/session/session_rust/src/'
```

### Unlock files

```bash
chmod +x /Users/petras/brg/code_rust/session/session_cpp/test.sh
```

### Build

```bash
# Linux
alias r='(cd /home/pv/brg/code_rust/session/session_rust && cargo run)'
alias p='(cd /home/pv/brg/code_rust/session/session_py && conda activate session && python main.py)'
alias c='(cd /home/pv/brg/code_rust/session/session_cpp && ./build.sh)'

alias ct='(cd /home/pv/brg/code_rust/session/session_cpp && ./test.sh)'
alias rt='(cd /home/pv/brg/code_rust/session/session_rust && ./test.sh)'
alias pt='(cd /home/pv/brg/code_rust/session/session_py && ./test.sh)'

# macOS
alias c='(cd /Users/petras/brg/code_rust/session/session_cpp && ./build.sh)'
alias r='(cd /Users/petras/brg/code_rust/session/session_rust && cargo run)'
alias p='(cd /Users/petras/brg/code_rust/session/session_py && conda activate session && python main.py)'

alias ct='(cd /Users/petras/brg/code_rust/session/session_cpp && ./test.sh)'
alias rt='(cd /Users/petras/brg/code_rust/session/session_rust && ./test.sh)'
alias pt='(cd /Users/petras/brg/code_rust/session/session_py && ./test.sh)'

# Windows
doskey c=cd /d "c:\brg\code_rust\session\session_cpp" ^& build.bat
doskey r=cd /d "c:\brg\code_rust\session\session_rust" ^& cargo run
doskey p=cd /d "c:\brg\code_rust\session\session_py" ^& conda activate session ^& python main.py

doskey ct=cd /d "c:\brg\code_rust\session\session_cpp" ^& test.bat
doskey rt=cd /d "c:\brg\code_rust\session\session_rust" ^& test.bat
doskey pt=cd /d "c:\brg\code_rust\session\session_py" ^& test.bat
```

### Doc

```bash
alias rdoc='(cd /home/pv/brg/code_rust/session/session_rust && cargo doc)'
alias pdoc='(cd /home/pv/brg/code_rust/session/session_py && ./doc.sh)'
alias cdoc='(cd /home/pv/brg/code_rust/session/session_cpp && ./doc.sh)'
```

