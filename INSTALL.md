# rsort - Installation Guide

A high-performance Rust CLI tool for removing duplicate entries from large text files using streaming I/O and optimized hash-based deduplication.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation Methods](#installation-methods)
  - [Method 1: Build from Source (Recommended)](#method-1-build-from-source-recommended)
  - [Method 2: Install via Cargo](#method-2-install-via-cargo)
  - [Method 3: Manual Installation](#method-3-manual-installation)
- [Post-Installation](#post-installation)
- [Verification](#verification)
- [Troubleshooting](#troubleshooting)

## Prerequisites

### Required

- **Rust Toolchain** (version 1.70 or later)
  - Install from [rustup.rs](https://rustup.rs/)
  - Or use your system package manager

### Optional

- **Git** (if cloning from repository)
- **Make** or **Cargo** (included with Rust)

## Installation Methods

### Method 1: Build from Source (Recommended)

This method gives you the latest version and optimized binary.

#### Step 1: Install Rust

**Linux/macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

**Windows:**
```powershell
# Download and run rustup-init.exe from https://rustup.rs/
# Or use winget:
winget install Rustlang.Rustup
```

**Verify installation:**
```bash
rustc --version
cargo --version
```

#### Step 2: Clone or Download the Project

**Option A: If you have the source code:**
```bash
cd /path/to/rsort
```

**Option B: If cloning from Git:**
```bash
git clone <repository-url>
cd rsort
```

#### Step 3: Build the Project

```bash
# Build in release mode (optimized)
cargo build --release
```

**Build time:** Typically 2-5 minutes depending on your system.

#### Step 4: Locate the Binary

The compiled binary will be at:
- **Linux/macOS:** `target/release/rsort`
- **Windows:** `target/release/rsort.exe`

### Method 2: Install via Cargo

If the package is published to crates.io:

```bash
cargo install rsort
```

This installs `rsort` to `~/.cargo/bin/` (or `%USERPROFILE%\.cargo\bin` on Windows).

### Method 3: Manual Installation

#### Step 1: Build the Binary

Follow Method 1, Steps 1-3.

#### Step 2: Install to System Path

**Linux/macOS:**
```bash
# Copy to a system directory
sudo cp target/release/rsort /usr/local/bin/

# Or to user directory (no sudo needed)
mkdir -p ~/.local/bin
cp target/release/rsort ~/.local/bin/
export PATH="$HOME/.local/bin:$PATH"  # Add to ~/.bashrc or ~/.zshrc
```

**Windows:**
```powershell
# Copy to a directory in your PATH
# For example, to C:\Program Files\rsort\
# Then add that directory to your system PATH
```

## Post-Installation

### Add to PATH (if not already)

**Linux/macOS:**

Add to `~/.bashrc`, `~/.zshrc`, or `~/.profile`:
```bash
export PATH="$HOME/.cargo/bin:$PATH"
# Or if using ~/.local/bin:
export PATH="$HOME/.local/bin:$PATH"
```

Then reload:
```bash
source ~/.bashrc  # or source ~/.zshrc
```

**Windows:**

1. Open System Properties → Environment Variables
2. Edit `Path` variable
3. Add `C:\Users\<YourUsername>\.cargo\bin`
4. Restart terminal

### Verify Installation

```bash
rsort --help
```

You should see:
```
High-performance deduplication tool for large files

Usage: rsort <INPUT> <OUTPUT> [OPTIONS]

Arguments:
  <INPUT>   Input file path
  <OUTPUT>  Output file path

Options:
  -h, --help               Print help
  --chunk-size <MB>        Chunk size in MB [default: 50]
  --threads <N>            Number of parallel threads
```

## Verification

Test the installation with a small file:

```bash
# Create a test file
cat > test_input.txt << EOF
line1
line2
line1
line3
line2
line4
EOF

# Run rsort
rsort test_input.txt test_output.txt

# Check result
cat test_output.txt
# Should show: line1, line2, line3, line4 (duplicates removed)
```

## Troubleshooting

### Issue: `cargo: command not found`

**Solution:**
- Ensure Rust is installed: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Reload your shell: `source ~/.cargo/env`
- Verify: `cargo --version`

### Issue: `rsort: command not found`

**Solution:**
- Check if binary exists: `ls target/release/rsort` (or `target\release\rsort.exe` on Windows)
- Add to PATH (see Post-Installation section)
- Use full path: `./target/release/rsort` (or `target\release\rsort.exe` on Windows)

### Issue: Permission Denied

**Solution:**
```bash
# Make binary executable (Linux/macOS)
chmod +x target/release/rsort

# Or if copying to system directory:
sudo chmod +x /usr/local/bin/rsort
```

### Issue: Build Fails

**Common causes:**
- Outdated Rust version: `rustup update`
- Missing dependencies: Install build tools (gcc, make, etc.)
- Network issues: Check internet connection for dependency downloads

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get update
sudo apt-get install build-essential
```

**macOS:**
```bash
xcode-select --install
```

**Windows:**
Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022)

### Issue: Out of Memory During Build

**Solution:**
- Close other applications
- Build with fewer parallel jobs: `cargo build --release -j 1`
- Increase swap space (Linux)

## Quick Start

After installation, you can immediately use rsort:

```bash
# Basic usage
rsort input.txt output.txt

# With options
rsort large_file.txt deduplicated.txt --chunk-size 100
```

## System Requirements

- **RAM:** Minimum 2GB, recommended 4GB+ for large files
- **Disk Space:** ~100MB for installation, plus space for output file
- **OS:** Linux, macOS, or Windows (WSL recommended on Windows)

## Uninstallation

**If installed via Cargo:**
```bash
cargo uninstall rsort
```

**If installed manually:**
```bash
# Remove binary
rm /usr/local/bin/rsort  # or wherever you installed it
# Remove from PATH if added manually
```

## Support

For issues, questions, or contributions, please refer to the main README.md file or the project repository.

