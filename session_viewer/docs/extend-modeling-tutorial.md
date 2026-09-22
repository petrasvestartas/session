# Create, trim, extend and explode

Type coordinates on the command line to create, trim, extend and explode geometry, each as one undo step.

![Running viewer: Create, trim, extend and explode.](screenshots/extensions-modeling.png)

## Starting point

Copy checkpoint 21 and check it builds; the finished steps are in `lessons/modeling-1/` … `lessons/modeling-2/`.

```bash
cp -r docs/lessons/21 docs/lessons/my-modeling
cd docs/lessons/my-modeling
cargo check -j4 --lib
```

## Step 1 · Build the document operations

Add `modeling.rs`: one enum of operations and `Scene::model`, which checks the request, then changes the document.

### `src/app/mod.rs`

`lessons/modeling-1/src/app/mod.rs` · type this, added at the start of `mod manifest`

```rust
--8<-- "lessons/modeling-1/src/app/mod.rs:step-1"
```

### `src/app/modeling.rs`

`lessons/modeling-1/src/app/modeling.rs` · type this, new file

```rust
--8<-- "lessons/modeling-1/src/app/modeling.rs"
```

### `src/app/scene.rs`

`lessons/modeling-1/src/app/scene.rs` · type this, added after the `pub last_edited: Option<usize>,` line

```rust
--8<-- "lessons/modeling-1/src/app/scene.rs:step-1"
```

`lessons/modeling-1/src/app/scene.rs` · type this, replaces the `last_edited: None,` line block

```rust
--8<-- "lessons/modeling-1/src/app/scene.rs:step-1b"
```

### Check step 1

Run `cargo check -j4 --lib`.

## Step 2 · Connect the command line to the operations

Parse the new verbs on the command line and hand them to `Scene::model`.

### `src/app/command.rs`

`lessons/modeling-2/src/app/command.rs` · type this, replaces the `enum Command` block

```rust
--8<-- "lessons/modeling-2/src/app/command.rs:step-2"
```

`lessons/modeling-2/src/app/command.rs` · type this, added at the start of `fn parse`

```rust
--8<-- "lessons/modeling-2/src/app/command.rs:step-2b"
```

`lessons/modeling-2/src/app/command.rs` · type this, replaces the `let rest: Vec<&str> = words.collect();` line block

```rust
--8<-- "lessons/modeling-2/src/app/command.rs:step-2c"
```

`lessons/modeling-2/src/app/command.rs` · type this, added after the `assert!(parse("move sideways").is_err());` line

```rust
--8<-- "lessons/modeling-2/src/app/command.rs:step-2d"
```

`lessons/modeling-2/src/app/command.rs` · type this, added at the end of the `mod tests` block

```rust
--8<-- "lessons/modeling-2/src/app/command.rs:step-2e"
```

### `src/state/edit.rs`

`lessons/modeling-2/src/state/edit.rs` · type this, replaces the `fn run_command` block

```rust
--8<-- "lessons/modeling-2/src/state/edit.rs:step-2"
```

`lessons/modeling-2/src/state/edit.rs` · type this, added after the `match command {` line

```rust
--8<-- "lessons/modeling-2/src/state/edit.rs:step-2b"
```

`lessons/modeling-2/src/state/edit.rs` · type this, replaces the `Command::Delete => {` line block

```rust
--8<-- "lessons/modeling-2/src/state/edit.rs:step-2c"
```

### Check step 2

Run `cargo check -j4 --lib`.

## Check

Run `cargo xtest -j4 --lib app::modeling`, then `trunk serve --port 8780` and open <http://localhost:8780/?data=off&inspect=1>.

For the screenshot scene, copy the [nested fixture](extensions/nested.pb) and its [manifest](extensions/nested.yaml) into `assets/` as `extension-nested.pb` / `.yaml` and open <http://localhost:8780/?scene=extension-nested.yaml&data=off&inspect=1>.

## Try

Type `line 0,0,0 100,0,0`, select it, type `trim 0.2 0.8`, then undo.

## Finished code

step 1 in `lessons/modeling-1/`, step 2 in `lessons/modeling-2/`.

## Expected viewer result

A new line, trimmed to its middle 60 % and selected.

[![Full viewer result for extend modeling tutorial](screenshots/extensions-modeling.png)](screenshots/extensions-modeling.png)
