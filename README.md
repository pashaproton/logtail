# logtail

## logtail v0.1.0 (Beta)

Initial beta release of **logtail** — a high-performance CLI tool for streaming and filtering large log files.

## Why logtail?

Because existing tools fall apart when logs get big or workflows get real.

- `tail -f` → no filtering
- `grep` → not built for real-time streaming
- heavy log tools → overkill for simple debugging

**logtail** focuses on doing one thing well:

> Stream large files, filter them in real time, and stay fast with minimal memory usage.

## Features

- Stream large files without loading them into memory
- Follow file updates (`--follow`, like `tail -f`)
- Filter using substring or regular expressions
- Case-insensitive matching
- Invert match
- Line numbers
- Count matches
- Works with files and stdin

---

## Installation

### Build from source

```bash
git clone https://github.com/YOUR_USERNAME/logtail.git
cd logtail
cargo build --release
```

## Usage

### Basic usage

```bash
logtail [FILE]
```

### Follow file (like tail -f)

```bash
logtail [FILE] --follow
```
or
```bash
logtail [FILE] -f
```

### Filter by substring

```bash
logtail [FILE] --contains "ERROR"
```
or
```bash
logtail [FILE] -c "ERROR"
```

### Filter by regex

```bash
logtail [FILE] --regex "ERROR|WARN"
```
or
```bash
logtail [FILE] -r "ERROR|WARN"
```

### Case-insensitive matching

```bash
logtail [FILE] -r "error" -i
```

### Invert match (exclude lines)

```bash
logtail [FILE] -c "DEBUG" -v
```

### Show line numbers

```bash
logtail [FILE] -n
```

### Count matches only

```bash
logtail [FILE] -c "ERROR" -C
```

### Pipe input (stdin)

```bash
cat [FILE] | logtail -c "ERROR"
```

## Examples

### Monitor errors in real time

```bash
logtail /var/log/app.log -f -r "ERROR|PANIC"
```

### Exclude noisy logs

```bash
logtail /var/log/app.log -v -c "DEBUG"
```

### Count failed requests

```bash
logtail /var/log/app.log -r "status=500" --count
```

### Debug logs with line numbers

```bash
logtail /var/log/app.log -c "DEBUG" -n
```

## Performance
**logtail** is designed with performance in mind:

- Uses streaming I/O (BufReader)
- Processes logs line-by-line
- Avoids loading full files into memory
- Reuses buffers to reduce allocations
- Compiles regex once

Memory usage is independent of file size.

## Limitations (Beta)
- Basic file rotation handling
- Case-insensitive substring matching allocates per line (to be optimized)
- No multi-file support yet
- No structured log parsing (JSON, etc.)

## Roadmap
- Improved file rotation detection (inode tracking)
- Faster case-insensitive matching
- Multi-file support
- JSON log filtering
- Highlighting matches
- Performance benchmarks

## License
MIT License