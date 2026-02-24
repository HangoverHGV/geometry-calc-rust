# Calculator Triunghi Dreptunghic

A terminal-based right triangle calculator written in Rust. It can compute sides (hypotenuse and legs) and all six trigonometric functions (sin, cos, tan, cot, sec, csc) from either an angle or side lengths.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (includes `cargo`)

## Install globally (run `calculator` from anywhere)

From inside the project folder, run:

```bash
cargo install --path .
```

This compiles the binary and places it in `~/.cargo/bin`, which Rust adds to PATH automatically during installation. After that, open any terminal and type:

```bash
calculator
```

To uninstall:

```bash
cargo uninstall calculator
```

---

## Build manually

```bash
cargo build --release
```

The compiled binary will be at `target/release/calculator.exe` (Windows) or `target/release/calculator` (Linux/macOS).

You can then copy that binary to any folder that is in your PATH (e.g. `C:\tools`) and call it by name from any terminal.

## Run without installing

From inside the project folder:

```bash
cargo run
```

## Usage

The calculator presents an interactive menu. Navigate by typing the number of your choice and pressing Enter.

### Main menu

```
=== Calculator Triunghi Dreptunghic ===
0. Exit
1. Triunghi dreptunghic - laturi
2. Unghiuri
```

---

### 1 — Right triangle sides

Compute a missing side given the other two.

```
--- Triunghi dreptunghic - laturi ---
0. Back
1. Ipotenuza   (hypotenuse from two legs)
2. Cateta      (leg from hypotenuse + other leg)
```

**Example — hypotenuse:**
```
Alege: 1
Cateta 1: 3
Cateta 2: 4
Ipotenuza = 5
```

**Example — leg:**
```
Alege: 2
Ipotenuza: 5
Cateta: 3
Cateta = 4
```

---

### 2 — Trigonometric functions

Choose a function (sin, cos, tan, cot, sec, csc), then choose how to compute it.

```
--- Functii trigonometrice ---
0. Back
1. sin
2. cos
3. tan
4. cot
5. sec
6. csc
```

After selecting a function you get two sub-options:

| Option | Input | Output |
|--------|-------|--------|
| 1 | Angle in degrees | Value of the function |
| 2 | Two side lengths | Value + angle in degrees + all six trig values |

**Example — sin from angle:**
```
Alege: 1   (sin)
Alege: 1   (from angle)
Unghi (grade): 30
sin(30) = 0.5
```

**Example — tan from sides:**
```
Alege: 3   (tan)
Alege: 2   (from sides)
Latura opusa: 3
Latura adiacenta: 4
tan  = Latura opusa / Latura adiacenta = 0.75
Unghiul = 36.8699 grade

-- Toate functiile pentru unghi = 36.8699 grade --
  sin = 0.6
  cos = 0.8
  tan = 0.75
  cot = 1.3333...
  sec = 1.25
  csc = 1.6666...
```

---

### Tips

- Enter `0` at any menu to go back or exit.
- All angles are in **degrees**.
- If you enter an invalid number the calculator will prompt you again.
