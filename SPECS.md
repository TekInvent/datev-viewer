# Teki DATEV Viewer v1.0
## Product Requirements Specification (AI Implementation Contract)

**Document Purpose**

This specification is the authoritative implementation contract for an autonomous Software Engineering AI Agent.

The intended readers of this document are AI coding agents (e.g. Codex, GPT-5.5, Claude Code, OpenCode, Hermes) and human reviewers.

The intended users of the application are **human users**, primarily:

- entrepreneurs generating DATEV exports from their own software;
- accounting professionals reviewing DATEV exports before importing them into accounting software.

The AI agent shall implement exactly the functionality described in this document. It shall not invent additional features, workflows or architecture unless explicitly requested in a future revision.

---

# 1. Objective

Implement a lightweight desktop application named **Teki DATEV Viewer**.

The application is a **read-only DATEV inspection tool**.

Its only purpose is to allow a human to inspect and validate a generated DATEV file before importing it into accounting software.

The application is **not** an accounting application.

The application is **not** a DATEV generator.

The application is **not** a DATEV editor.

---

# 2. Scope

Version 1 shall support:

- opening a DATEV EXTF file;
- parsing the file;
- displaying its contents in a human-readable format;
- performing structural validation;
- displaying validation results;
- calculating summary totals.

No other business functionality is included.

---

# 3. Mandatory Technology Stack

The implementation shall use:

- Tauri 2
- React
- TypeScript
- Rust

Responsibilities:

### Rust

- file loading;
- encoding detection;
- line ending detection;
- DATEV parsing;
- validation;
- totals calculation;
- domain model.

### React

- user interface;
- application state presentation;
- drag & drop;
- tables;
- search/filter UI.

Business logic shall not exist in the React frontend.

---

# 4. Supported Platforms

Version 1 supports:

- macOS (Apple Silicon)
- Windows x64

Linux is out of scope.

---

# 5. Supported Input

Supported input:

- DATEV EXTF CSV

Accepted extensions:

- .csv
- .txt

Compatibility shall be determined from file contents rather than filename.

---

# 6. Read-Only Requirement

The application shall never:

- modify an opened file;
- overwrite a file;
- automatically repair a file;
- save changes.

No editing functionality exists.

---

# 7. Startup Screen

On startup the application displays a single main window.

Appearance:

- white background;
- clean minimalist layout;
- centered instruction.

Displayed text:

> Drag and drop a DATEV file here
>
> or
>
> File → Open DATEV File…

The entire window shall accept drag & drop.

No additional controls are visible before a file is opened.

---

# 8. Application Menu

The application shall provide standard desktop menus.

## File

- Open DATEV File…
- Close File
- Reload File
- Exit (Windows)
- Quit (macOS)

## Help

- About Teki DATEV Viewer

No other menu items exist in Version 1.

---

# 9. Opening a File

A file may be opened by:

- drag & drop;
- File → Open DATEV File…

After opening:

1. read the file;
2. detect encoding;
3. detect line endings;
4. parse DATEV structure;
5. validate contents;
6. display review screen.

If parsing fails, display the error screen.

---

# 10. Review Screen

The review screen consists of:

1. Toolbar
2. File Summary
3. Header Metadata
4. Validation Results
5. Booking Table
6. Totals

---

# 11. Toolbar

Buttons:

- Open
- Reload

No editing actions exist.

---

# 12. File Summary

Display:

- filename;
- absolute path;
- file size;
- detected encoding;
- detected line endings;
- booking record count;
- parser status.

---

# 13. Header Metadata

Display parsed DATEV header fields as key/value pairs.

Examples include:

- format identifier;
- format version;
- consultant number;
- client number;
- accounting period;
- fiscal year;
- export date.

Unknown fields shall also be displayed.

---

# 14. Validation Results

Exactly three overall states exist.

## Valid

Green indicator.

No validation errors.

---

## Warning

Yellow indicator.

The file is structurally valid but contains suspicious values.

---

## Invalid

Red indicator.

The file contains structural errors.

---

Each validation message displays:

- severity;
- row number (when applicable);
- column;
- description;
- offending value.

---

# 15. Booking Table

The booking table is read-only.

Supported functionality:

- scrolling;
- horizontal scrolling;
- sortable columns;
- resizable columns;
- search;
- instant filtering;
- row selection;
- copy selected cells;
- copy selected rows.

Displayed values shall exactly match the parsed file.

---

# 16. Totals

Display:

- booking count;
- total amount;
- debit total (when available);
- credit total (when available);
- warning count;
- error count.

---

# 17. Validation Rules

## File Validation

Verify:

- readable file;
- non-empty file;
- supported encoding;
- consistent line endings;
- valid CSV structure;
- valid DATEV header;
- consistent column count.

---

## Header Validation

Verify:

- mandatory header fields;
- supported DATEV version;
- consultant number format;
- client number format;
- valid date values.

---

## Record Validation

Verify:

- mandatory fields;
- amount format;
- date format;
- account fields;
- contra account fields;
- tax key format;
- column count consistency.

---

# 18. Error Screen

If parsing fails display:

- error title;
- filename;
- parser error;
- detected encoding (when available);
- first problematic line (when available).

Malformed input shall never terminate the application.

---

# 19. Architecture Requirements

The implementation shall separate presentation from business logic.

```
React UI
    │
View Models
    │
Rust API
    │
datev_core
```

The Rust backend shall contain:

- parser;
- validator;
- domain model;
- totals calculator.

The frontend shall contain only presentation logic.

The Rust backend shall be reusable independently from the desktop application.

---

# 20. Non-Functional Requirements

The application shall:

- operate completely offline;
- perform no network communication;
- collect no telemetry;
- require no user account;
- start quickly;
- consume minimal memory;
- behave deterministically for identical input.

---

# 21. Out of Scope

The AI agent shall **not** implement:

- DATEV generation;
- editing DATEV files;
- importing into accounting software;
- exporting reports;
- printing;
- cloud synchronization;
- online services;
- authentication;
- account plan validation;
- tax calculations;
- document management;
- batch processing;
- plugins.

These features are explicitly excluded from Version 1.

---

# 22. Acceptance Criteria

The implementation is complete only if all of the following are true:

- The application runs on macOS (Apple Silicon) and Windows x64.
- The startup screen displays the drag-and-drop interface.
- File → Open DATEV File… opens supported files.
- DATEV EXTF files are parsed successfully.
- Header metadata is displayed.
- Booking records are displayed in a searchable and sortable read-only table.
- Validation results are displayed using the defined status model.
- Totals are calculated correctly.
- Invalid files produce meaningful error diagnostics.
- The source file is never modified.
- All business logic resides in the Rust backend.
- The React frontend contains presentation logic only.

---

# 23. AI Agent Constraints

The AI implementation agent shall adhere to the following rules:

- Implement only the functionality explicitly described in this specification.
- Do not introduce additional features, settings, configuration options, or workflows.
- Do not substitute the mandated technology stack.
- Prefer simple, maintainable implementations over framework-heavy abstractions.
- Keep the codebase modular, readable, and suitable for future extension.
- Preserve a strict separation between UI and business logic.
- When implementation details are not specified, choose the simplest solution consistent with modern engineering best practices.
