# Combine the extensions into the current viewer

## You are building

Continue from checkpoint 21 through the maintained viewer. This sequence resolves shared wiring explicitly; the individual feature lessons remain independent alternatives.

![Running viewer: Combine the extensions into the current viewer.](screenshots/extensions-command-interface.png)

Actual maintained viewer output. [Capture setup and five browser rounds](extensions/README.md).

## Starting point

Start from a fresh checkpoint **21**, not from another extension lesson. The lessons can be implemented separately. Every code block below is complete; there are no omitted method bodies. Execute every edit within one step before its check.

Use the tools installed in [00 · Environment](00-environment.md). From the maintained `session_viewer` repository, create your learning workspace once:

```bash
export COURSE_REPO="$PWD"
bash "$COURSE_REPO/docs/serve.sh" build --quiet
python3 "$COURSE_REPO/docs/extensions.py" --prepare "$HOME/viewer-integrated"
cd "$HOME/viewer-integrated/session_viewer"
export REGEN_PROTO=0
cargo check -j4 --lib
```

The build prepares the frozen checkpoint cache. The initializer copies its viewer and kernel into a new folder; it does **not** install the feature. Expected: `Finished` with no compiler errors. Keep this terminal in the new `session_viewer` directory. If the destination exists, use a new folder name.

For **CURRENT → REPLACE WITH**, find the complete CURRENT block in the named file and replace it once. For **ADD BELOW**, keep the shown anchor and insert the new block directly after it. For **NEW FILE**, create the named path and paste its complete block. Apply blocks in page order; compile only at the check marker. All required code and answers are visible here.

## Follow these checkpoints in order

1. [Refresh diagnostics and resource checks](current-1.md)
2. [Create, trim, extend and explode](current-2.md)
3. [Make control dragging respect object placement](current-3.md)
4. [Draw a solid, readable gumball](current-4.md)
5. [Build the nested session and graph panel](current-5.md)
6. [Build the egui panel and command interface](current-6.md)
7. [Match the maintained source exactly](current-7.md)

The final check compares every runtime source file, Cargo manifest, lockfile and browser entry point with the maintained viewer. Each checkpoint compiles for WebAssembly; the final one runs native library tests.
