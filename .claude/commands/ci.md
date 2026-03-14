Triage recent CI failures for this project.

1. Run `gh run list --limit 5` — show recent workflow runs with status
2. For each failed run, run `gh run view <id> --log-failed` to get failure logs
3. Parse each failure and identify:
   - Which language (Python / Rust / C++)
   - Which test class (e.g. mesh, nurbscurve)
   - Which test name
   - The error message (one line, no stack trace noise)
   - Which CI platform (ubuntu / macos / windows)
4. Check if the failure matches a known bug pattern from MEMORY.md
5. Report all failures in a compact table: Platform | Language | Class | Test | Error

If no failures found, confirm CI is green.
