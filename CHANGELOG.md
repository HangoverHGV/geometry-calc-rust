# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [1.0.1] - 2026-11-16

### Changed
- **Batch printing no longer freezes the app.** Each print batch now runs in a separate worker process, a second copy of the app started with `--print-worker`.
    - The app sends the job (printer, orientation, files) to the worker as JSON on stdin and reads a one-line JSON report from its stdout. A background thread waits for the worker, so the window stays responsive while pdfium renders and spools the pages.
    - The worker starts with `CREATE_NO_WINDOW` and never opens a window. If the app is closed mid-batch, the worker still finishes the batch.
    - While a batch is printing, the print menu item shows "Printing..." and is disabled. Starting a second batch shows "Still printing the previous batch." instead, so two batches don't interleave in the queue.
    - When files fail, the message now names up to three of them instead of only giving a count. Every failure and its error still goes to the console.
    - If the worker crashes, the app reports "The print worker stopped unexpectedly" with its exit code instead of hanging.
- `print_files` is replaced by `print_job` (inside the worker). Process handling and the JSON protocol live in `src/batch_printing/worker.rs`, with unit tests.

### Fixed
- A message that arrived while another was still showing was hidden early. It used whatever was left of the previous message's 2 seconds. Each message now gets its full 2 seconds (`Timer.restart()`).

## [1.0.0] - 2026-11-12

### Added
- **Price Offer page** for the `oferta-typst` template. Every variable in `oferta.json` has a field, with its JSON key shown under the label.
    - Sections: Offer, Client, Items (`items[]`, add and remove rows), Commercial terms, and a totals preview.
    - The form state lives in the `OfertaState` global, so it survives switching tabs in the sidebar.
    - Quantities and prices can be typed Romanian or English style (`7,5`, `7.5`, `1.234,56`). Each row's value and the totals use the template's rounding and `12.000,00` formatting. A value that isn't a number shows `?`.
    - **Open JSON** / **Save JSON** read and write the template's data format. Optional fields left empty are omitted, so the template uses its own defaults (`30 de zile`, `Contracost`, 21% VAT). Whole numbers are written as integers and `termen_zile` stays text for ranges like `10-15`.
    - Saving is refused when a quantity, price or VAT rate isn't a number. Empty required fields only trigger a warning after the save.
    - **Generate PDF** renders the offer with the `oferta-typst` template inside the app, so Typst doesn't need to be installed. The template, logo, stamp and Liberation Sans fonts are compiled into the executable; change anything in `oferta-typst/` and rebuild to pick it up.
        - The form goes in as the JSON string `sys.inputs.data`, the same input as the CLI's `--input data=...`. The output is pixel-identical to `typst compile`.
        - The PDF is suggested next to the open JSON file with the same name, otherwise as `oferta <client> <date>.pdf`.
        - Rendering runs on a background thread. The button shows "Generating…" until it's done.
    - Logic and serde mapping in `src/oferta/`, with unit tests, including a round trip of `oferta-typst/oferta.json` and three PDF renders.
- `typst`, `typst-as-lib`, `typst-layout` and `typst-pdf` dependencies for rendering the PDF.

### Notes
- **New** clears the form without asking.
- Generate PDF doesn't save the JSON. Use Save JSON as well if you want to edit the offer later.
- Writing the PDF fails while the file is open in a viewer that locks it, such as Acrobat.
- Typst 0.15 needs Rust 1.92 or newer.

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