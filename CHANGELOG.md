# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [0.5.2] - 2026-10-27

### Added
- **Parts list export to Excel.** Pick a project's `Draw` folder and the app writes a one-sheet workbook listing every drawing in it.
    - The folder picker uses `rfd::FileDialog::pick_folder` and is wired to the `open-directory` callback on `AppWindow`.
    - The workbook and its sheet are named after the project folder (the parent of `Draw`). The file is saved inside `Draw`, e.g. `VDN271001\Draw\VDN271001.xlsx`.
    - Columns: `Nr.Crt.`, `Reper`, `Grosime`, `Material`, `Cantitate`, `Observatii`.
    - Each drawing gets one row. `Nr.Crt.` is numbered from 1 and `Reper` is the file name without its extension. The other columns are left empty to fill in by hand.
    - Rows are in natural order (`Part-2` before `Part-10`), ignoring case.
    - Only drawing files are listed (`.pdf`, `.dxf`, `.dwg`, `.idw`). A part saved in several formats gets a single row. Other files, such as `desktop.ini`, `Thumbs.db` or the workbook itself, are skipped.
    - Everything is in 16 pt font, with a bold header and 21 pt rows. Columns are sized to their longest entry. The empty columns are also set to 16 pt, so values typed in later match.
- `rust_xlsxwriter` dependency for writing `.xlsx` files.

### Notes
- An existing workbook with the same name is overwritten without asking.
- Saving fails if the workbook is open in Excel.
- Errors (no parent folder, no drawings found, write failure) only go to the console. They are not shown in the UI yet.
- If the project folder name isn't a valid Excel sheet name, the sheet keeps the default name `Sheet1`.

## [0.5.0]

### Added
- **Scheduled shutdown.** Turns the PC off after a delay. It's started from the shared `Page` header, so it's available on every page.
    - The countdown state and timer live in `AppWindow` rather than in each page, so the countdown keeps running when you switch tabs in the sidebar.
    - The deadline is saved as a Unix timestamp in `%LOCALAPPDATA%\cad-helper-3d\shutdown.txt`, so a pending shutdown survives restarting the app.
    - It runs `shutdown /s /t` with `CREATE_NO_WINDOW`, so no console window flashes up.
    - Cancelling treats exit code 1116 (no shutdown in progress) as success rather than an error.

### Notes
- Scheduled shutdown is Windows only. Support for other platforms is deferred.

## [0.4.0]

### Added
- **BOM Calculator** page for Fusion 360 BOM exports in `.csv`, `.xls`, `.xlsx` and `.xlsm`.
    - Rows for the same profile are merged by dropping the trailing `-<length>` from the part number.
    - The summary shows total length, total mass, and the number of 6 m and 12 m bars needed.
    - Bar counts are `ceil(total_length / bar_length)` for each profile, without cut planning or kerf allowance.
- `calamine` dependency for reading Excel files and `csv` for reading CSV files.

## [0.3.0]

### Added
- DXF export for the Voronoi generator. A save dialog (`rfd`) picks the destination, and the file is written as hand-built R12 ASCII DXF.

### Changed
- The Voronoi generator now uses the `voronoice` crate.

### Fixed
- Layout of the Isosceles tab in the Triangle calculator.
- Windows installer build in GitHub Actions:
    - The stale binary name `inventor-math-calculator.exe` is replaced with `cad-helper-3d`.
    - The installer script path is corrected to `build_assets/installer.iss` (it was `release/windows/`).