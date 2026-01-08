# Task Plan: Fix Vue Test Viewer Auto-Reload + GitHub Actions Review

## Goal
Make the test viewer auto-reload when test JSON changes, preserving the current URL/suite selection.

## Phases
- [x] Phase 1: Add Vite plugin to watch public/testData.js
- [x] Phase 2: Add debouncing to prevent multiple reloads
- [ ] Phase 3: Fix URL preservation on reload
- [ ] Phase 4: Reduce C++ build verbosity (secondary goal)

## Key Questions
1. Why is the URL not preserved after reload?
2. Is the full-reload clearing the URL hash/query params?

## Decisions Made
- Used fs.watch with debounce instead of chokidar (simpler)
- 500ms debounce to batch file change events

## Errors Encountered
- MSBuild `/v:m` flag failed due to MSYS2 path conversion
- Resolution: Use grep filtering instead of MSBuild flags

## Status
**Phase 3 complete** - Auto-reload working with URL preservation

## Phase 4: GitHub Actions fixes
- [x] Fixed bash script permissions: `chmod +x bash/*.sh bash/lib/*.sh`
- [x] Updated CLAUDE.md with CI permission note
- [x] Made numba optional to fix macos-15-intel LLVM build failure
- [ ] Verify CI passes after push

## Notes
- `full-reload` with `path: '*'` tells Vite to reload all connected clients while preserving their current URL
- The TestsView reads `route.query.suite` on mount to restore the active suite
