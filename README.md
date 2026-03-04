# LiquidCAN Protocol

## Overview
The LiquidCAN Protocol is the CAN communication protocol for Liquid Projects at the TU Wien Space Team. 

Please read the `LiquidCAN.pdf` file for a detailed description of the protocol. 

- `Cpp-Implementation/` - Reference implementation in Cpp
- `Rust-Implementation/` - Reference implementation in Rust

## Versioning and Updates

This repository serves as the central location for all versioning updates for the protocol. Every update must be accompanied by:

1. A commit on main
2. A new version number
3. An updated PDF file with the new version number
4. Updates to the corresponding Rust and Cpp implementations
5. A git tag for releases

### Generating Change Diffs

When creating issues or pull requests with protocol changes, generate a PDF diff to visualize the changes:

```bash
latexdiff-vc --git --flatten --pdf -r <old-commit> LiquidCAN.tex 
```

## Development

### Running CI Checks

The repository includes a CI script (`ci-rust.sh`) that runs all quality checks on the Rust implementation. This script is used both locally and in GitHub Actions 

**Run all checks:**
```bash
./ci-rust.sh
# or explicitly
./ci-rust.sh all
```

**Run individual checks:**
```bash
./ci-rust.sh build         # Build the project
./ci-rust.sh test          # Run tests
./ci-rust.sh test-macros   # Run macro tests
./ci-rust.sh fmt           # Check code formatting
./ci-rust.sh clippy        # Run clippy linter
./ci-rust.sh clippy-macros # Run clippy on macros
```
You can fix formatting or linter issues by adding the -fix suffix to the command. e.g: `./ci-rust.sh clippy-fix`