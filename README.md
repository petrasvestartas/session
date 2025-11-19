# Session repo with language submodules

This repo aggregates language-specific implementations as git submodules:

- `session_cpp` → https://github.com/petrasvestartas/session_cpp.git
- `session_py` → https://github.com/petrasvestartas/session_py.git
- `session_rust` → https://github.com/petrasvestartas/session_rust.git
- `session_data` → https://github.com/petrasvestartas/session_data.git
- `session_proto` → https://github.com/petrasvestartas/session_proto.git

## Clone with submodules

Clone this repo and fetch all submodules in one step:

```bash
git clone --recurse-submodules <this-repo-url>
```

If you already cloned it without `--recurse-submodules`, run:

```bash
git submodule update --init --recursive
```

## Pull / update

After updating the main repo (e.g. `git pull`), make sure submodules are up to date:

```bash
git submodule update --init --recursive
```

## Commit & push changes

Work inside each submodule directory (e.g. `session_cpp/`) like a normal Git repo.

From the main repo root you can use the helper script to commit & push all language repos at once:

```bash
./git_push_all.sh "your commit message"
```

This script commits & pushes in `session_cpp`, `session_py`, `session_rust`, `session_data`, and `session_proto` (for those directories that exist).

> Note: after changing submodules, commit once more in the root repo to record the updated submodule references.

## Add a new submodule

To add another repo as a submodule under a folder name (for example `session_new`):

```bash
git submodule add <repo-url> session_new
git submodule update --init --recursive
git commit -am "Add submodule session_new"
```

Then push the main repo as usual:

```bash
git push
```
