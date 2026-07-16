# 3D CAD Helper

A sleek, lightweight desktop utility designed to accelerate engineering and geometry calculations for CAD and fabrication workflows. Built entirely in **Rust** and powered by the **Slint UI** toolkit for native, hardware-accelerated performance.

---

## 📸 Features & Usage Examples

### Number of Elements
Calculate precise placement spacing when relying on design constraints like edge distance or the center of a circle.

![Number of Elements Screenshot](README_IMGs/NofEl.png)

### Right-Angle Triangle Solver
Quickly compute hypotenuse, opposite/adjacent sides (catheti), and angles without breaking your CAD workflow.

![Right Angle Triangle Screenshot](README_IMGs/RightAngleTriangle.png)

### Force & Pressure Calculator
Instantly resolve mechanical force calculations, conversions, and target area calculations within an intuitive interface.

![Force Calculator Screenshot](README_IMGs/force_calculator.png)
---

## 🛠️ Build & Release Automation

This project uses `cargo-make` for local automation and integrates seamlessly with **GitHub Actions** for cross-platform distribution.

### Prerequisites

Ensure you have the following installed on your system:
* [Rust & Cargo](https://rustup.rs/)
* [cargo-make](https://github.com/sagiegurari/cargo-make) (Install via `cargo install cargo-make`)

### Local Compilation

To list all available build tasks defined in [Makefile.toml](Makefile.toml):
```shell
cargo make --list-all-steps
```
To automatically compile, package, and prepare a production-ready release for your current operating system:
```shell
cargo make release-all
```
