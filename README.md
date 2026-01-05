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