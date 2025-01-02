# Rust Project Setup and Basic Commands

This guide covers essential commands to set up and manage a Rust project locally.

## Prerequisites
- Install Rust (via [rustup](https://rustup.rs/))
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
- Verify installation:
```bash
rustc --version
cargo --version
```

---

## Creating a New Rust Project
### 1. Create a New Project
```bash
cargo new my_project
```
- `my_project` is the name of your project.
- This creates a new directory with basic Rust files.

### 2. Navigate to Project Directory
```bash
cd my_project
```

### 3. Build the Project
```bash
cargo build
```
- Builds the project and outputs an executable in `target/debug/`.

### 4. Run the Project
```bash
cargo run
```
- Compiles and runs the project.

### 5. Check for Compilation Errors
```bash
cargo check
```
- Quickly checks the code for errors without producing an executable.

---

## Adding Dependencies
### 1. Open `Cargo.toml` and Add Dependency
```toml
[dependencies]
serde = "1.0"
```
- Example adds `serde` for serialization.

### 2. Build with New Dependencies
```bash
cargo build
```

---

## Testing
### 1. Write Tests
- Add tests in `src/lib.rs` or `src/main.rs` under `#[cfg(test)]` module.

### 2. Run Tests
```bash
cargo test
```

---

## Formatting
```bash
cargo fmt
```
- Formats code according to Rust style guidelines.

---

## Linting
```bash
cargo clippy
```
- Provides additional lint checks.

---

## Building for Release
```bash
cargo build --release
```
- Builds an optimized release version of the project.

---

## Running in REPL (Interactive Mode)
```bash
cargo install evcxr_repl
evcxr
```
- Interactive Rust shell for quick prototyping.

---

## Clean Build Artifacts
```bash
cargo clean
```
- Removes the `target` directory and build artifacts.

---

## Updating Rust
```bash
rustup update
```
- Updates Rust to the latest stable version.

---

## Common Cargo Commands Summary
```bash
cargo new <project_name>       # Create new project
cargo build                    # Build project
cargo run                      # Run project
cargo test                     # Run tests
cargo fmt                      # Format code
cargo clippy                   # Lint project
cargo clean                    # Clean build artifacts
```

