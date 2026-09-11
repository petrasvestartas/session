# Test Viewer Syntax Highlighting

## Architecture: Hybrid Tree-Sitter + Context-Aware Tokenizer

File: `session_tests/src/components/TestViewer.vue`

The highlighting uses a **two-layer** approach for cross-language consistency:

### Layer 1: Tree-Sitter (Reliable AST Nodes Only)

Tree-sitter queries are **minimal** — only node types that are unambiguous and never crash:

| Token Type | C++ Node | Python Node | Rust Node |
|-----------|----------|-------------|-----------|
| Comments | `comment` | `comment` | `line_comment`, `block_comment` |
| Strings | `string_literal`, `raw_string_literal`, `char_literal`, `system_lib_string` | `string` | `string_literal`, `raw_string_literal`, `char_literal` |
| Numbers | `number_literal` | `integer`, `float` | `integer_literal`, `float_literal` |
| Types | `type_identifier`, `primitive_type`, `sized_type_specifier` | `(type (identifier))` | `type_identifier`, `primitive_type` |
| Other | `namespace_identifier` | — | `boolean_literal`, `attribute_item`, `macro_invocation` |

**What was removed and why:**
- `(field_identifier) @property` — caused inconsistent method coloring between C++/Rust (captured as property/red) vs Python (gap tokenizer colored as method/blue)
- `(identifier) @variable` catch-all — stole captures from more specific patterns due to dedup logic, making functions/methods appear as plain variables
- ALL bare string literals (`"return" @keyword`, `"(" @bracket`, etc.) — crash with "Bad node name" in web-tree-sitter WASM grammars
- Complex contextual patterns (`(call_expression function: ...)`, `(function_definition name: ...)`) — unreliable across grammar versions, sometimes match and sometimes don't

### Layer 2: `highlightGap()` — Context-Aware Tokenizer

All text NOT captured by tree-sitter goes through `highlightGap()`. This handles keywords, functions, methods, operators, and brackets **identically** across all languages.

**Tokenization regex:** `/(#?\w+)|([(){}\[\]])|([+\-*/%=!<>&|^~?:.,;@#]+)|(\s+)/g`

4 token groups: words, brackets, operators, whitespace.

**Classification logic (in priority order):**

1. **Keywords** — word is in `KEYWORDS[lang]` set → purple (`ts-kw`)
2. **Module name** — word follows a module keyword (`from`, `import`, `use`, `mod`, `namespace`, `using`, `package`) and is NOT followed by `(` → gold module (`ts-mod`). Examples: `from session_py`, `use crate`, `namespace std`
3. **PascalCase** — matches `/^[A-Z][a-zA-Z0-9]+$/` → gold type (`ts-ty`). Checked BEFORE function/method rules so constructors like `Point(...)`, `NurbsCurve::create(...)` stay gold, not blue
4. **Method call** — word is after `.`/`::`/`->` AND followed by `(` → blue (`ts-mt`)
5. **Function call** — word is followed by `(` → blue (`ts-fn`)
6. **ALL_CAPS** — matches `/^[A-Z][A-Z0-9_]+$/` → orange constant (`ts-cb`)
7. **Brackets** — `(){}[]` → red (`ts-pb`)
8. **Operators** — `+`, `-`, `::`, `.`, `->`, `=`, etc. → cyan (`ts-op`)
9. **Plain identifier** — uncolored, inherits default `#abb2bf`

**Context detection:** The tokenizer builds a token array first, then for each word token:
- Scans forward (skipping whitespace) to find if next symbol is `(`
- Scans backward (skipping whitespace) to find if previous symbol is `.`/`::`/`->`
- Scans backward for previous word to check if it's a module keyword (`from`, `import`, `use`, etc.)

### Why This Architecture

| Approach | Problem |
|----------|---------|
| Tree-sitter queries only | Bare string literals crash; complex patterns unreliable across grammar versions; no catch-all without stealing captures |
| Regex only | Can't reliably detect comments (especially multi-line), strings with escapes, or nested template types |
| **Hybrid (current)** | Tree-sitter handles what it's best at (comments, strings, numbers, types); regex handles what it's best at (keywords, function call detection, operators) |

### Color Theme

| Class | Color | Used For |
|-------|-------|----------|
| `ts-kw` | `#c678dd` purple | keywords (`let`, `def`, `return`, `#include`) |
| `ts-ty` | `#00e5ff` neon marine blue | types, PascalCase (`Point`, `NurbsCurve`) |
| `ts-tyd` | `#00e5ff` neon marine blue bold | type definitions |
| `ts-tyb` | `#56b6c2` cyan | builtin types (`int`, `f64`, `bool`) |
| `ts-fn` | `#61afef` blue | function calls (`create(`, `len(`) |
| `ts-mt` | `#61afef` blue | method calls (`.set_domain(`, `::new(`) |
| `ts-mc` | `#61afef` blue bold | macros (`vec!`, `println!`) |
| `ts-s` | `#98c379` green | strings |
| `ts-n` | `#e5e54b` synthetic yellow | numbers |
| `ts-pm` | `#e5e54b` synthetic yellow italic | parameters |
| `ts-pl` | `#e5e54b` synthetic yellow | parameter labels |
| `ts-cb` | `#e5e54b` synthetic yellow | constants (`ALL_CAPS`, `true`, `false`) |
| `ts-c` | `#5c6370` grey italic | comments |
| `ts-op` | `#56b6c2` cyan | operators (`+`, `-`, `=`, `::`, `->`) |
| `ts-pb` | `#ff79c6` pink | brackets (`()`, `{}`, `[]`) |
| `ts-pd` | `#ff79c6` pink | delimiters (`,`, `;`) |
| `ts-mod` | `#00e5ff` neon marine blue | modules/namespaces |
| `ts-dec` | `#00e5ff` neon marine blue | decorators/attributes |

### Gap Tokenizer Punctuation Groups

The `highlightGap` regex separates punctuation into 3 distinct groups:
- **Brackets** `([(){}\[\]])` → `ts-pb` pink
- **Delimiters** `([,;])` → `ts-pd` pink
- **Operators** `([+\-*/%=!<>&|^~?:.@#]+)` → `ts-op` cyan

### Protobuf Schema Highlighting

Protobuf schemas (`.proto` content displayed in the "Serialization Protobuf" section) are highlighted using `highlightGap` with a dedicated `proto` keyword set — no tree-sitter needed.

**`formatProto()`** splits content into lines, handles `//` comments separately, then runs each line through `highlightGap('proto')`.

**Proto keywords:** `syntax`, `message`, `enum`, `service`, `rpc`, `returns`, `option`, `import`, `package`, `repeated`, `optional`, `required`, `oneof`, `map`, `reserved`, `double`, `float`, `int32`, `int64`, `uint32`, `uint64`, `sint32`, `sint64`, `fixed32`, `fixed64`, `bool`, `string`, `bytes`, `true`, `false`

**Result:**
- `message Point { ... }` → `message` purple, `Point` neon marine blue (PascalCase), `{` `}` pink
- `repeated double x = 1;` → `repeated` `double` purple, `x` plain, `=` cyan, `1` yellow, `;` pink
- `// comment` → grey italic

### WASM Dependencies

Installed: `web-tree-sitter@0.24.0` + `tree-sitter-wasms@0.1.13`

WASM files in `session_tests/public/`:
- `tree-sitter.wasm` (parser core)
- `tree-sitter-cpp.wasm`, `tree-sitter-python.wasm`, `tree-sitter-rust.wasm`, `tree-sitter-json.wasm`

### Key Rules When Modifying

1. **Never add bare string literals** to queries (e.g., `"return" @keyword`) — they crash
2. **Never add `(identifier) @variable` catch-all** — it steals captures from specific patterns
3. **Never add `(field_identifier) @property`** — breaks method coloring consistency across languages
4. **Keep queries minimal** — only add patterns for node types that are guaranteed named nodes in the grammar version
5. **Add new keywords** to `KEYWORDS[lang]` sets, not to tree-sitter queries
6. **Function/method detection** is done by `highlightGap` via lookahead/lookback — same logic for all languages
7. **Dedup logic** uses `>=` (prefer later captures) so more specific patterns listed later in a query override general ones
