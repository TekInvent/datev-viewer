import { useState, useMemo, useRef, useEffect } from "react";
import { useVirtualizer } from "@tanstack/react-virtual";
import { useLocale } from "../i18n";
import { BookingRecord } from "../types";

/**
 * Formats a raw DATEV Belegdatum value for display.
 * DATEV stores dates as 3-digit (DMM) or 4-digit (DDMM) strings.
 * This converts them to the readable "DD.MM." notation.
 * Other lengths (e.g. 8-digit full dates) are passed through unchanged.
 */
function formatBelegdatum(raw: string): string {
  if ((raw.length === 3 || raw.length === 4) && /^\d+$/.test(raw)) {
    const padded = raw.padStart(4, "0");
    return `${padded.slice(0, 2)}.${padded.slice(2)}.`;
  }
  return raw;
}

interface BookingTableProps {
  records: BookingRecord[];
}

interface SortState {
  column: keyof BookingRecord | null;
  direction: "asc" | "desc" | null;
}

export function BookingTable({ records }: BookingTableProps) {
  const { t, tCount } = useLocale();

  // 1. Search State
  const [searchQuery, setSearchQuery] = useState("");

  // 2. Sorting State
  const [sortState, setSortState] = useState<SortState>({
    column: null,
    direction: null,
  });

  // 3. Columns configuration with resizing
  const columns = useMemo(
    () => [
      { key: "row_number", label: t("table_col_row") },
      { key: "amount", label: t("table_col_amount") },
      { key: "debit_credit", label: t("table_col_dc") },
      { key: "date", label: t("table_col_date") },
      { key: "account", label: t("table_col_account") },
      { key: "contra_account", label: t("table_col_contra_account") },
      { key: "tax_key", label: t("table_col_tax_key") },
      { key: "document_reference", label: t("table_col_doc_ref") },
      { key: "booking_text", label: t("table_col_booking_text") },
    ],
    [t],
  );

  const [widths, setWidths] = useState<Record<string, number>>({
    row_number: 65,
    amount: 110,
    debit_credit: 60,
    date: 90,
    account: 100,
    contra_account: 120,
    tax_key: 80,
    document_reference: 130,
    booking_text: 220,
  });

  // Calculate total width of table
  const totalWidth = useMemo(() => {
    return columns.reduce((sum, col) => sum + (widths[col.key] || 100), 0);
  }, [columns, widths]);

  // 4. Selection State
  const [selectedRows, setSelectedRows] = useState<Set<number>>(new Set());
  const [selectedCell, setSelectedCell] = useState<{
    rowIndex: number;
    columnKey: string;
  } | null>(null);
  const [lastSelectedRowIndex, setLastSelectedRowIndex] = useState<number | null>(null);

  // Copy status feedback
  const [copiedNotification, setCopiedNotification] = useState(false);

  // Refs for scroll synchronization
  const headerRef = useRef<HTMLDivElement>(null);
  const parentRef = useRef<HTMLDivElement>(null);

  // Synchronize horizontal scrolling of header with body
  const handleScroll = (e: React.UIEvent<HTMLDivElement>) => {
    if (headerRef.current) {
      headerRef.current.scrollLeft = e.currentTarget.scrollLeft;
    }
  };

  // Reset selections when query or sort changes
  useEffect(() => {
    setSelectedRows(new Set());
    setSelectedCell(null);
    setLastSelectedRowIndex(null);
  }, [searchQuery, sortState]);

  // 5. Instant Filtering
  const filteredRecords = useMemo(() => {
    if (!searchQuery) return records;
    const q = searchQuery.toLowerCase();
    return records.filter((r) => {
      return (
        String(r.row_number).includes(q) ||
        r.amount.toLowerCase().includes(q) ||
        r.debit_credit.toLowerCase().includes(q) ||
        r.date.toLowerCase().includes(q) ||
        r.account.toLowerCase().includes(q) ||
        r.contra_account.toLowerCase().includes(q) ||
        r.tax_key.toLowerCase().includes(q) ||
        r.document_reference.toLowerCase().includes(q) ||
        r.booking_text.toLowerCase().includes(q)
      );
    });
  }, [records, searchQuery]);

  // 6. Sorting
  const sortedRecords = useMemo(() => {
    if (!sortState.column || !sortState.direction) return filteredRecords;
    const col = sortState.column;
    const dir = sortState.direction === "asc" ? 1 : -1;

    return [...filteredRecords].sort((a, b) => {
      const valA = a[col];
      const valB = b[col];

      if (col === "amount") {
        // Parse float for amount numeric sorting
        const numA = parseFloat(String(valA).replace(",", ".")) || 0;
        const numB = parseFloat(String(valB).replace(",", ".")) || 0;
        return (numA - numB) * dir;
      }

      if (col === "row_number") {
        return (Number(valA) - Number(valB)) * dir;
      }

      // Default string sorting
      return String(valA).localeCompare(String(valB)) * dir;
    });
  }, [filteredRecords, sortState]);

  // 7. Virtualizer Configuration
  const rowVirtualizer = useVirtualizer({
    count: sortedRecords.length,
    getScrollElement: () => parentRef.current,
    estimateSize: () => 36, // height of row
    overscan: 10,
  });

  // 8. Sorting Handler
  const handleSort = (columnKey: string) => {
    const col = columnKey as keyof BookingRecord;
    setSortState((prev) => {
      if (prev.column === col) {
        if (prev.direction === "asc") {
          return { column: col, direction: "desc" };
        } else if (prev.direction === "desc") {
          return { column: null, direction: null };
        }
      }
      return { column: col, direction: "asc" };
    });
  };

  // 9. Resize Handler
  const handleResizeStart = (key: string, e: React.MouseEvent) => {
    e.preventDefault();
    const startX = e.clientX;
    const startWidth = widths[key] || 100;

    const handleMouseMove = (moveEvent: MouseEvent) => {
      const deltaX = moveEvent.clientX - startX;
      const newWidth = Math.max(50, startWidth + deltaX);
      setWidths((prev) => ({
        ...prev,
        [key]: newWidth,
      }));
    };

    const handleMouseUp = () => {
      document.removeEventListener("mousemove", handleMouseMove);
      document.removeEventListener("mouseup", handleMouseUp);
    };

    document.addEventListener("mousemove", handleMouseMove);
    document.addEventListener("mouseup", handleMouseUp);
  };

  // 10. Selection Handlers
  const handleRowHeaderClick = (index: number, event: React.MouseEvent) => {
    setSelectedCell(null);
    const newSelected = new Set(selectedRows);

    if (event.shiftKey && lastSelectedRowIndex !== null) {
      const start = Math.min(lastSelectedRowIndex, index);
      const end = Math.max(lastSelectedRowIndex, index);
      newSelected.clear();
      for (let i = start; i <= end; i++) {
        newSelected.add(i);
      }
    } else if (event.ctrlKey || event.metaKey) {
      if (newSelected.has(index)) {
        newSelected.delete(index);
      } else {
        newSelected.add(index);
      }
      setLastSelectedRowIndex(index);
    } else {
      newSelected.clear();
      newSelected.add(index);
      setLastSelectedRowIndex(index);
    }

    setSelectedRows(newSelected);
  };

  const handleCellClick = (rowIndex: number, columnKey: string) => {
    setSelectedRows(new Set());
    setLastSelectedRowIndex(null);
    setSelectedCell({ rowIndex, columnKey });
  };

  const clearSelection = () => {
    setSelectedRows(new Set());
    setSelectedCell(null);
    setLastSelectedRowIndex(null);
  };

  // 11. Clipboard Functionality
  const getSelectedText = () => {
    if (selectedRows.size > 0) {
      const selectedList = sortedRecords.filter((_, idx) => selectedRows.has(idx));
      const headerLine = columns.map((c) => c.label).join("\t");
      const rowsLine = selectedList
        .map((r) =>
          columns.map((c) => String(r[c.key as keyof BookingRecord] ?? "")).join("\t")
        )
        .join("\n");
      return `${headerLine}\n${rowsLine}`;
    } else if (selectedCell) {
      const record = sortedRecords[selectedCell.rowIndex];
      if (record) {
        return String(record[selectedCell.columnKey as keyof BookingRecord] ?? "");
      }
    }
    return "";
  };

  const executeCopy = () => {
    const text = getSelectedText();
    if (text) {
      navigator.clipboard.writeText(text).then(() => {
        setCopiedNotification(true);
        setTimeout(() => setCopiedNotification(false), 2000);
      });
    }
  };

  // Handle global keyboard shortcut for Copy (Ctrl+C / Cmd+C)
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (
        document.activeElement?.tagName === "INPUT" ||
        document.activeElement?.tagName === "TEXTAREA"
      ) {
        return;
      }

      if ((e.ctrlKey || e.metaKey) && e.key === "c") {
        const text = getSelectedText();
        if (text) {
          e.preventDefault();
          navigator.clipboard.writeText(text).then(() => {
            setCopiedNotification(true);
            setTimeout(() => setCopiedNotification(false), 2000);
          });
        }
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => {
      window.removeEventListener("keydown", handleKeyDown);
    };
  }, [selectedRows, selectedCell, sortedRecords, columns]);

  return (
    <div className="booking-table-card">
      <div className="table-controls">
        <div className="search-box-container">
          <span className="search-icon">🔍</span>
          <input
            type="text"
            className="table-search-input"
            placeholder={t("table_search_placeholder")}
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
          />
          {searchQuery && (
            <button className="clear-search-btn" onClick={() => setSearchQuery("")}>
              &times;
            </button>
          )}
        </div>

        <div className="selection-actions">
          {selectedRows.size > 0 && (
            <span className="selection-info">
              {tCount("table_rows_selected", selectedRows.size)}
            </span>
          )}
          {selectedCell && (
            <span className="selection-info">{t("table_cell_selected")}</span>
          )}
          {(selectedRows.size > 0 || selectedCell) && (
            <>
              <button className="table-action-btn copy" onClick={executeCopy}>
                📋 {t("table_copy")}
              </button>
              <button className="table-action-btn clear" onClick={clearSelection}>
                {t("table_clear_selection")}
              </button>
            </>
          )}
          {copiedNotification && (
            <span className="copied-toast">{t("table_copied_toast")}</span>
          )}
        </div>
      </div>

      <div className="booking-table-grid-container">
        {/* Table Header Scroll Sync wrapper */}
        <div
          ref={headerRef}
          className="table-header-scroll-wrapper"
          style={{ overflow: "hidden", width: "100%" }}
        >
          <div
            className="table-header-row"
            style={{ display: "flex", width: `${totalWidth}px` }}
          >
            {columns.map((col) => {
              const isSorted = sortState.column === col.key;
              const sortDirection = sortState.direction;
              return (
                <div
                  key={col.key}
                  className={`table-header-cell ${col.key} ${isSorted ? "sorted" : ""}`}
                  style={{ width: `${widths[col.key]}px`, position: "relative" }}
                >
                  <div
                    className="header-label-container"
                    onClick={() => handleSort(col.key)}
                  >
                    <span className="header-label">{col.label}</span>
                    {isSorted && (
                      <span className="sort-arrow">
                        {sortDirection === "asc" ? " ▴" : " ▾"}
                      </span>
                    )}
                  </div>
                  {/* Resizer */}
                  <div
                    className="column-resizer"
                    onMouseDown={(e) => handleResizeStart(col.key, e)}
                  />
                </div>
              );
            })}
          </div>
        </div>

        {/* Table Body Scroll container */}
        <div
          ref={parentRef}
          className="table-body-scroll-wrapper"
          style={{ overflow: "auto", flex: 1 }}
          onScroll={handleScroll}
        >
          {sortedRecords.length === 0 ? (
            <div className="table-empty-state">{t("table_empty_state")}</div>
          ) : (
            <div
              className="table-body-virtual"
              style={{
                height: `${rowVirtualizer.getTotalSize()}px`,
                width: `${totalWidth}px`,
                position: "relative",
              }}
            >
              {rowVirtualizer.getVirtualItems().map((virtualRow) => {
                const record = sortedRecords[virtualRow.index];
                const isRowSelected = selectedRows.has(virtualRow.index);

                return (
                  <div
                    key={virtualRow.key}
                    className={`table-row ${isRowSelected ? "row-selected" : ""}`}
                    style={{
                      position: "absolute",
                      top: 0,
                      left: 0,
                      width: "100%",
                      height: `${virtualRow.size}px`,
                      transform: `translateY(${virtualRow.start}px)`,
                      display: "flex",
                    }}
                  >
                    {columns.map((col) => {
                      const isCellSelected =
                        selectedCell?.rowIndex === virtualRow.index &&
                        selectedCell?.columnKey === col.key;
                      const val = record[col.key as keyof BookingRecord];

                      if (col.key === "row_number") {
                        return (
                          <div
                            key={col.key}
                            className={`table-cell ${col.key} row-selector-cell`}
                            style={{ width: `${widths[col.key]}px` }}
                            onClick={(e) => handleRowHeaderClick(virtualRow.index, e)}
                          >
                            {val}
                          </div>
                        );
                      }

                      return (
                        <div
                          key={col.key}
                          className={`table-cell ${col.key} ${
                            isCellSelected ? "cell-selected" : ""
                          }`}
                          style={{ width: `${widths[col.key]}px` }}
                          onClick={() => handleCellClick(virtualRow.index, col.key)}
                        >
                          <span className="cell-value" title={String(val ?? "")}>
                            {col.key === "date" ? formatBelegdatum(String(val ?? "")) : val}
                          </span>
                        </div>
                      );
                    })}
                  </div>
                );
              })}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
