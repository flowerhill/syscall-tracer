# rust-strace-sample[WIP]

A toy system call tracer for Linux written in Rust.

## Description

`rust-strace-sample` is a simple command-line tool that traces system calls made by a target process using Linux's ptrace API. Currently, it monitors `write` system calls and displays their arguments.

## Requirements

- Linux operating system
- Rust toolchain (Edition 2024)

## Dependencies

- `anyhow` - Error handling
- `clap` - Command-line argument parsing
- `nix` - Unix system calls wrapper

## Installation

```bash
cargo build --release
```

## Usage

```bash
cargo run -- <command> [args...]
```

### Examples

Trace a simple command:
```bash
cargo run -- ls -la
```

Trace echo command:
```bash
cargo run -- echo "Hello, World!"
```

## How it works

The tracer uses Linux's `ptrace` system call to:
1. Spawn a child process that will be traced
2. Monitor the child process's system calls
3. Intercept and log specific system calls (currently `write`)
4. Display system call arguments (file descriptor, buffer pointer, count)

## Limitations

- Linux only (uses ptrace which is not available on macOS or Windows)
- Currently only traces `write` system calls
- Displays raw register values rather than interpreted data

## Testing

Run the test suite:
```bash
cargo test
```

## License

This is a toy project for educational purposes.
