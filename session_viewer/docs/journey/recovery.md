# Keep a working copy you can return to

A typing mistake should be a small investigation. Stop the development server before restoring files, so it does not rebuild a half-restored project. Run these commands from `session_viewer`.

## Save your own progress

```sh
npm --prefix ../session_tests run course -- save before-my-experiment
```

This copies your files into `workspace/journey-saves/before-my-experiment`. It leaves out build output and Git internals. Choose a new name for each save; existing saves are never overwritten. You can save unfinished work too, but call it something that tells you it was unfinished.

## Compare without changing anything

```sh
npm --prefix ../session_tests run course -- check 03-triangle
```

The command shows missing files or differences from the end of that lesson. It does not edit your project. A difference may be your intentional experiment; source comparison is a guide, not a measure of understanding. Build and run to check behavior.

## Return to one of your saves

```sh
npm --prefix ../session_tests run course -- restore before-my-experiment
```

Your current project is first moved intact to a dated `workspace/journey-before-…` folder. The saved files are copied into `workspace/journey`. Neither your current work nor your saved checkpoint is deleted. Restart Trunk from the restored project.

The compiler may need to rebuild cached output after a restore. That is machine work; you do not have to retype the program.

## Inspect the answer separately

If a difference is difficult to locate, assemble a reference in a new folder:

```sh
npm --prefix ../session_tests run course -- reference 03-triangle --output target/triangle-reference
```

The destination must not already exist and cannot be your handwritten project or a folder inside it. This is an optional reference for comparison, not the normal teaching workflow. Return to your own project and make the correction yourself.

## When the course itself is wrong

Keep your last working save and record the lesson ID, release name and first error. A correction must identify the affected block, explain the cause and pass the checkpoint checks. It should preserve your project and unaffected work. Follow the [small corrections for existing projects](corrections.md); each names the affected lessons and exact changes.

[Return to the course](../journey.md)
