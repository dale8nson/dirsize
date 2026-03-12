# dirsize

A simple command-line utility to calculate the total size of a directory and all its subdirectories, with optional `.gitignore` support.

## Installation

```bash
cargo install --path .
```

## Usage

```bash
dirsize <PATH> [OPTIONS]
```

**Options:**

| Flag | Description |
|------|-------------|
| `-g`, `--gitignore` | Ignore paths listed in `.gitignore` (if one exists in the target directory) |

## Examples

Get the total size of the current directory:
```bash
dirsize .
```

Get the size of a project, excluding build artifacts and other gitignored files:
```bash
dirsize ~/dev/my-project --gitignore
```

Output is automatically scaled to the most appropriate unit (B, KB, MB, GB, or TB):
```
1.23 GB
```

## How it works

`dirsize` recursively walks a directory tree, summing the size of every file it encounters. When `--gitignore` is passed, it reads the `.gitignore` at the root of the target directory, resolves each pattern via glob expansion, and skips any matching paths during traversal.

## Built with

- [`clap`](https://github.com/clap-rs/clap) — argument parsing
- [`glob`](https://github.com/rust-lang/glob) — `.gitignore` pattern matching
