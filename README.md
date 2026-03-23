# Rust Backuper

A lightweight, automated backup tool written in Rust that synchronizes my server application data to Google Drive. Designed to run as a scheduled task, it handles Database, Redis cache snapshots and file backups with secure Google Drive integration.

---

## Usage

```bash
rs_backuper 

# To get the current version
rs_backuper --version
```

--- 

## Installation

### Download a Pre-built Binary

The easiest way to get started is to download a pre-built binary from the [Releases](../../releases) page.

**Linux (x86_64):**
```bash
curl -L https://github.com/tommylmt/Rust-Backuper/releases/latest/download/rs_backuper-linux-x86_64 -o rs_backuper
# If running an old Linux version
curl -L https://github.com/tommylmt/Rust-Backuper/releases/latest/download/rs_backuper-linux-x86_64-old -o rs_backuper

chmod +x rs_backuper
sudo mv rs_backuper /usr/bin/rs_backuper
```

**macOS (Apple Silicon / ARM64):**
```bash
curl -L https://github.com/tommylmt/Rust-Backuper/releases/latest/download/rs_backuper-macos-arm64 -o rs_backuper
chmod +x rs_backuper
sudo mv rs_backuper /usr/bin/rs_backuper
```

**macOS (Intel / x86_64):**
```bash
curl -L https://github.com/tommylmt/Rust-Backuper/releases/latest/download/rs_backuper-macos-x86_64 -o rs_backuper
chmod +x rs_backuper
sudo mv rs_backuper /usr/bin/rs_backuper
```

---

## Configuration

Rust Backuper is configured via a `backuper.conf` file. 
A first launch of `rs_backuper` creates a sample configuration in `/etc/rust-backuper/backuper.conf`.

It also create a cronjob with the following configuration: 

```
0 0 * * * root /usr/bin/rs_backuper
```

--- 

## Features

- **Google Drive Integration** — Uploads backups directly to Google Drive.
- **Redis Backup** — Captures and exports Redis cache snapshots as part of the backup pipeline.
- **Database Backup** - Runs and exports MySQL/MariaDB/PostgreSQL `.sql` dumps
- **Files Backup** - Create a full copy of your files
- **TOML Configuration** — Simple, human-readable configuration file to define backup sources, schedules, and credentials.
- **Multi-platform Binaries** — Pre-built releases for Linux x86_64, macOS ARM (Apple Silicon), and macOS Intel.

---

## Building from Source

### Prerequisites 

- Docker 
- Make

### Steps

**1. Clone the repository:**
```bash
git clone https://github.com/tommylmt/Rust-Backuper.git
cd Rust-Backuper
```

**2. Use the docker instance (using the Makefile):**
```bash
make start # pull the image and up the container
```

**3. Build in release mode:**
```bash
docker compose exec rust cargo build --release --bin rust_backuper
```

The compiled binary will be available at:
```
target/release/rust_backuper
```
