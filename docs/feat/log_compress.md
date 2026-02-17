# Log Compression Design (Not Implemented)

Date: 2026-02-18

## Summary

`lug` currently rotates log files (daily) but does not compress old logs. The `compress` field in `FileConfig` is a placeholder. This document proposes a low-risk, cross-platform compression strategy using a background worker that gzips inactive rotated log files.

## Goals

- Compress rotated logs without blocking log writes.
- Keep the solution platform-agnostic (macOS/Linux/Windows).
- Make behavior configurable via `config.toml`.

## Non-Goals

- No size-based rotation changes.
- No retention enforcement tied to `max_backups`.
- No async runtime requirement in `lug`.

## Current State

- Rotation is handled by `tracing_appender::rolling::RollingFileAppender`.
- No rotation callback/hook exists in `tracing_appender`.
- `FileConfig.compress` exists but is not used.
- Rotated filenames look like: `rusktop.log.2026-02-17`.

## Proposed Approach (Recommended)

Use a background thread to periodically scan the log directory and gzip rotated logs that are no longer being written.

### Key Ideas

1. **Periodic scan** (e.g., every 10 minutes).
2. **Inactive check**: Only compress files whose `modified()` age exceeds a threshold (e.g., 1 hour).
3. **Safe compression**: Write `*.gz` first; delete original only after success.

### Why This Approach

- No coupling with `tracing_appender` internals.
- No impact on write latency.
- Works with existing rotation naming.
- Easy to disable and test.

## Configuration Proposal

```toml
[log.file]
path = "logs/rusktop.log"
compress = true
compression_interval_secs = 600   # optional, default 600
compression_inactive_secs = 3600  # optional, default 3600
```

## Dependencies

Add one dependency in `crates/lug/Cargo.toml`:

```toml
flate2 = "1.0"
```

## Implementation Sketch

### Background Worker

```rust
fn spawn_compression_task(dir: PathBuf, base: String, cfg: CompressionConfig) {
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(Duration::from_secs(cfg.interval_secs));
            if let Err(err) = compress_old_logs(&dir, &base, &cfg) {
                eprintln!("log compression failed: {err}");
            }
        }
    });
}
```

### Compression Logic

```rust
fn compress_old_logs(dir: &Path, base: &str, cfg: &CompressionConfig) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if !is_rotated_log(&path, base) || is_gzipped(&path) {
            continue;
        }
        if is_file_inactive(&path, cfg.inactive_secs)? {
            compress_log_file(&path)?;
        }
    }
    Ok(())
}
```

### Gzip Function

```rust
fn compress_log_file(source: &Path) -> io::Result<()> {
    let target = source.with_extension(format!("{}.{ext}", "gz", ext = ""));
    let mut input = File::open(source)?;
    let output = File::create(&target)?;
    let mut encoder = GzEncoder::new(output, Compression::default());
    io::copy(&mut input, &mut encoder)?;
    encoder.finish()?;
    fs::remove_file(source)?;
    Ok(())
}
```

Note: Output naming should be `*.gz` appended to the rotated filename. Example: `rusktop.log.2026-02-17.gz`.

## Safety Rules

- Never compress the active log file.
- Never delete the original if compression fails.
- Skip files already ending in `.gz`.

## Testing Plan

1. Create a fake rotated file: `logs/rusktop.log.2026-02-17`.
2. Set its modified time to > 1 hour ago.
3. Run compression scan; expect `.gz` file created and original removed.
4. Ensure active file is untouched.

## Open Questions

- Should compression frequency be configurable in `rusktop-core` or only in `lug`?
- Should compression be disabled by default even when `compress = true` in dev mode?
