# Submodules

This repository uses Git submodules for the language implementations and shared data/proto definitions.

## Submodule list

- `session_cpp`  → https://github.com/petrasvestartas/session_cpp.git
- `session_py`   → https://github.com/petrasvestartas/session_py.git
- `session_rust` → https://github.com/petrasvestartas/session_rust.git
- `session_data` → https://github.com/petrasvestartas/session_data.git
- `session_proto` → https://github.com/petrasvestartas/session_proto.git

## Clone with all submodules

```bash
git clone --recurse-submodules https://github.com/petrasvestartas/session.git
cd session
```

If you already cloned without `--recurse-submodules`:

```bash
git submodule update --init --recursive
```

## Update repository and submodules

```bash
git pull
git submodule update --init --recursive
```

## Commit & push changes in submodules

From the repo root, commit and push all language submodules **and** this main repo (if there are changes):

```bash
./git_push_all.sh "your commit message"
```

## Add a new submodule

```bash
git submodule add <repo-url> <folder-name>
git submodule update --init --recursive
git commit -am "Add submodule <folder-name>"
git push
```
