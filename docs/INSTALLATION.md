# Installation Guide

**Version**: 1.0.0
**Last Updated**: January 31, 2026
**Project**: Holographic Multi-Scale Cosmic Creation Simulator

---

## Table of Contents

1. [System Requirements](#system-requirements)
2. [Installing Rust](#installing-rust)
3. [Installing the Simulator](#installing-the-simulator)
4. [Verifying Installation](#verifying-installation)
5. [Uninstalling](#uninstalling)
6. [Troubleshooting](#troubleshooting)

---

## System Requirements

### Minimum Requirements

- **Operating System**: Linux, macOS, or Windows
- **Rust**: 1.70.0 or later
- **Memory**: 4GB RAM
- **Disk Space**: 500MB

### Recommended Requirements

- **Operating System**: Linux (Ubuntu 20.04+)
- **Rust**: 1.75.0 or later
- **Memory**: 8GB RAM
- **Disk Space**: 1GB

---

## Installing Rust

### Linux/macOS

1. **Install Rust using rustup**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Follow the prompts** and select option 1 (default installation)

3. **Reload your shell**:
   ```bash
   source $HOME/.cargo/env
   ```

4. **Verify installation**:
   ```bash
   rustc --version
   cargo --version
   ```

### Windows

1. **Download rustup-init.exe** from https://rustup.rs/

2. **Run the installer** and follow the prompts

3. **Restart your terminal** to apply changes

4. **Verify installation**:
   ```cmd
   rustc --version
   cargo --version
   ```

---

## Installing the Simulator

### Method 1: From Source

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd 03_Game
   ```

2. **Build the project**:
   ```bash
   cargo build --release
   ```

3. **Run tests**:
   ```bash
   cargo test --lib
   ```

### Method 2: Using Cargo

1. **Add to your Cargo.toml**:
   ```toml
   [dependencies]
   holonic_realms = "1.0.0"
   ```

2. **Build your project**:
   ```bash
   cargo build
   ```

### Method 3: From Crates.io (when available)

```bash
cargo install holonic_realms
```

---

## Verifying Installation

### Check Build

```bash
cargo build --release
```

Expected output:
```
Compiling holonic_realms v1.0.0
Finished release [optimized] target(s) in X.XXs
```

### Run Tests

```bash
cargo test --lib
```

Expected output:
```
running X tests
test test_name ... ok
...

test result: ok. X passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Generate Documentation

```bash
cargo doc --no-deps
```

Expected output:
```
Documenting holonic_realms v1.0.0
Finished dev [unoptimized + debuginfo] target(s) in X.XXs
```

### Run Example Program

Create `test_installation.rs`:

```rust
use holonic_realms::holographic::{ComplexVector, HolographicField, InvolutionLayer};

fn main() {
    let cv = ComplexVector::new(1.0, 0.0);
    println!("Complex Vector: amplitude={}, phase={}", cv.amplitude, cv.phase);

    let field = HolographicField::new(1.0, 1000.0, InvolutionLayer::Violet);
    println!("Holographic Field: layer={:?}", field.layer);

    println!("Installation successful!");
}
```

Run it:
```bash
cargo run --example test_installation
```

---

## Uninstalling

### From Source

```bash
# Remove the repository
rm -rf 03_Game

# Remove cached artifacts
cargo clean
```

### Using Cargo

```bash
cargo uninstall holonic_realms
```

### Remove Rust (Optional)

```bash
rustup self uninstall
```

---

## Troubleshooting

### Build Fails

**Problem**: Build fails with errors

**Solution**:
```bash
# Clean build
cargo clean
cargo build --release

# Update Rust
rustup update
```

### Tests Fail

**Problem**: Tests fail with errors

**Solution**:
```bash
# Run tests with output
cargo test --lib -- --nocapture

# Run specific test
cargo test --lib test_name
```

### Cannot Find Rust

**Problem**: `rustc` or `cargo` command not found

**Solution**:
```bash
# Add Rust to PATH (Linux/macOS)
source $HOME/.cargo/env

# Add Rust to PATH (Windows)
# Add %USERPROFILE%\.cargo\bin to your PATH environment variable
```

### Permission Denied

**Problem**: Permission denied when installing

**Solution**:
```bash
# Use sudo (Linux/macOS)
sudo cargo install holonic_realms

# Run as administrator (Windows)
```

### Out of Memory

**Problem**: Build fails due to out of memory

**Solution**:
```bash
# Limit parallel jobs
cargo build --release -j 4
```

---

## Next Steps

After successful installation:

1. Read the [Getting Started Guide](GETTING_STARTED.md)
2. Follow the [Tutorial](TUTORIAL.md)
3. Explore [Usage Examples](EXAMPLES.md)
4. Review the [API Documentation](API.md)

---

## Additional Resources

- [Rust Installation Guide](https://www.rust-lang.org/tools/install)
- [Cargo Guide](https://doc.rust-lang.org/cargo/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Troubleshooting Guide](TROUBLESHOOTING.md)

---

**Installation Guide Version**: 1.0.0
**Last Updated**: January 31, 2026
**Project**: Holographic Multi-Scale Cosmic Creation Simulator