# Sift

Fast log analysis for large files.

Built in Rust.

---

## Why this exists

Most log tools break down when logs get large:

- grep is fast but limited
- scripts are flexible but slow
- observability platforms are powerful but heavy

Sift is a local CLI tool that focuses on one thing:

> **Make large log files instantly readable and searchable.**

---

## What it does

- Streams large log files without loading them into memory
- Processes logs in parallel
- Supports JSON, plain text, and nginx formats
- Filters logs with simple query expressions
- Produces fast summaries (errors, top messages, counts)

---

## Install

### From source

```bash
git clone https://github.com/yourname/sift
cd sift
cargo build --release