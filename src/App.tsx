import { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { open } from "@tauri-apps/plugin-dialog";
import { DatevFile, OpenFileError, DatevHeader, Totals as TotalsType } from "./types";
import { ValidationResults } from "./components/ValidationResults";
import { BookingTable } from "./components/BookingTable";
import { LanguageSwitcher, useLocale } from "./i18n";

type AppState = "STARTUP" | "REVIEW" | "ERROR";

function formatFileSize(bytes: number): string {
  if (bytes === 0) return "0 bytes";
  if (bytes < 1024) return `${bytes} bytes`;
  const kb = bytes / 1024;
  return `${kb.toFixed(2)} KB`;
}

interface ToolbarProps {
  onOpen: () => void;
  onReload: () => void;
  onClose: () => void;
}

function Toolbar({ onOpen, onReload, onClose }: ToolbarProps) {
  const { t } = useLocale();

  return (
    <div className="toolbar">
      <button className="toolbar-btn" onClick={onOpen}>
        <span>📂</span> {t("toolbar_open")}
      </button>
      <button className="toolbar-btn" onClick={onReload}>
        <span>🔄</span> {t("toolbar_reload")}
      </button>
      <button className="toolbar-btn" onClick={onClose}>
        <span>✕</span> {t("toolbar_close_file")}
      </button>
    </div>
  );
}

interface FileSummaryProps {
  fileName: string;
  filePath: string;
  fileSize: number;
  encoding: string;
  lineEnding: string;
  recordCount: number;
}

function FileSummary({
  fileName,
  filePath,
  fileSize,
  encoding,
  lineEnding,
  recordCount,
}: FileSummaryProps) {
  const { t } = useLocale();

  return (
    <div className="card file-summary-card">
      <h3>{t("file_summary_title")}</h3>
      <div className="summary-grid">
        <div className="summary-item">
          <span className="label">{t("file_summary_filename")}</span>
          <span className="value filename" title={fileName}>{fileName}</span>
        </div>
        <div className="summary-item full-width">
          <span className="label">{t("file_summary_path")}</span>
          <span className="value path" title={filePath}>{filePath}</span>
        </div>
        <div className="summary-item">
          <span className="label">{t("file_summary_size")}</span>
          <span className="value">{formatFileSize(fileSize)}</span>
        </div>
        <div className="summary-item">
          <span className="label">{t("file_summary_encoding")}</span>
          <span className="value">{encoding}</span>
        </div>
        <div className="summary-item">
          <span className="label">{t("file_summary_line_endings")}</span>
          <span className="value">{lineEnding}</span>
        </div>
        <div className="summary-item">
          <span className="label">{t("file_summary_record_count")}</span>
          <span className="value">{recordCount}</span>
        </div>
        <div className="summary-item">
          <span className="label">{t("file_summary_parser_status")}</span>
          <span className="value status-badge ok">{t("file_summary_parsed")}</span>
        </div>
      </div>
    </div>
  );
}

interface HeaderMetadataProps {
  header: DatevHeader;
}

function HeaderMetadata({ header }: HeaderMetadataProps) {
  const { t } = useLocale();

  const fields = [
    { label: t("header_format_identifier"), value: header.format_identifier },
    { label: t("header_format_version"), value: header.version },
    { label: t("header_consultant_number"), value: header.consultant_number },
    { label: t("header_client_number"), value: header.client_number },
    { label: t("header_accounting_period"), value: header.accounting_period },
    { label: t("header_fiscal_year"), value: header.fiscal_year || "-" },
    { label: t("header_export_date"), value: header.export_date || "-" },
  ];

  const unknownFields = Object.entries(header.unknown_fields);

  return (
    <div className="card header-metadata-card">
      <h3>{t("header_metadata_title")}</h3>
      <div className="metadata-grid">
        {fields.map((f, i) => (
          <div key={i} className="metadata-item">
            <span className="label">{f.label}</span>
            <span className="value">{f.value}</span>
          </div>
        ))}
      </div>

      {unknownFields.length > 0 && (
        <div className="unknown-fields-section">
          <h4>{t("header_unknown_fields")}</h4>
          <div className="metadata-grid">
            {unknownFields.map(([key, value]) => (
              <div key={key} className="metadata-item">
                <span className="label">{t("header_field_number", { field: key })}</span>
                <span className="value">{value}</span>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}

interface TotalsProps {
  totals: TotalsType;
}

function Totals({ totals }: TotalsProps) {
  const { t, formatNumber } = useLocale();

  return (
    <div className="card totals-card">
      <h3>{t("totals_title")}</h3>
      <div className="totals-grid">
        <div className="totals-item">
          <span className="label">{t("totals_booking_count")}</span>
          <span className="value count">{totals.booking_count}</span>
        </div>
        <div className="totals-item">
          <span className="label">{t("totals_total_amount")}</span>
          <span className="value amount">{formatNumber(totals.total_amount)}</span>
        </div>
        <div className="totals-item">
          <span className="label">{t("totals_debit_total")}</span>
          <span className="value amount debit">{formatNumber(totals.debit_total)}</span>
        </div>
        <div className="totals-item">
          <span className="label">{t("totals_credit_total")}</span>
          <span className="value amount credit">{formatNumber(totals.credit_total)}</span>
        </div>
        <div className="totals-item">
          <span className="label">{t("totals_warnings")}</span>
          <span className="value count warning">{totals.warning_count}</span>
        </div>
        <div className="totals-item">
          <span className="label">{t("totals_errors")}</span>
          <span className="value count error">{totals.error_count}</span>
        </div>
      </div>
    </div>
  );
}

function App() {
  const { t } = useLocale();
  const [appState, setAppState] = useState<AppState>("STARTUP");
  const [filePath, setFilePath] = useState<string | null>(null);
  const [fileName, setFileName] = useState<string | null>(null);
  const [fileData, setFileData] = useState<DatevFile | null>(null);
  const [error, setError] = useState<OpenFileError | null>(null);
  const [isDragging, setIsDragging] = useState(false);

  const filePathRef = useRef<string | null>(null);
  useEffect(() => {
    filePathRef.current = filePath;
  }, [filePath]);

  const getFileName = (path: string) => {
    return path.split(/[/\\]/).pop() || path;
  };

  const handleFileOpen = async (path: string) => {
    try {
      const file = await invoke<DatevFile>("open_datev_file", { path });
      setFilePath(path);
      setFileName(getFileName(path));
      setFileData(file);
      setError(null);
      setAppState("REVIEW");
    } catch (err: any) {
      setFilePath(path);
      setFileName(getFileName(path));
      setFileData(null);
      setError(err as OpenFileError);
      setAppState("ERROR");
    }
  };

  const triggerOpenFileSelector = async () => {
    try {
      const selected = await open({
        multiple: false,
        directory: false,
        filters: [
          {
            name: "DATEV Files",
            extensions: ["csv", "txt"],
          },
        ],
      });
      if (selected && typeof selected === "string") {
        await handleFileOpen(selected);
      }
    } catch (err) {
      console.error("Failed to open dialog:", err);
    }
  };

  const handleCloseFile = () => {
    setFilePath(null);
    setFileName(null);
    setFileData(null);
    setError(null);
    setAppState("STARTUP");
  };

  const handleReloadFile = () => {
    if (filePathRef.current) {
      handleFileOpen(filePathRef.current);
    }
  };

  useEffect(() => {
    let active = true;
    let unlistenMenuOpen: (() => void) | null = null;
    let unlistenMenuClose: (() => void) | null = null;
    let unlistenMenuReload: (() => void) | null = null;
    let unlistenDragDrop: (() => void) | null = null;

    const setupListeners = async () => {
      // Listen to File -> Open DATEV File...
      const menuOpen = await listen("menu-open-file", () => {
        triggerOpenFileSelector();
      });
      if (!active) {
        menuOpen();
        return;
      }
      unlistenMenuOpen = menuOpen;

      // Listen to File -> Close File
      const menuClose = await listen("menu-close-file", () => {
        handleCloseFile();
      });
      if (!active) {
        menuClose();
        return;
      }
      unlistenMenuClose = menuClose;

      // Listen to File -> Reload File
      const menuReload = await listen("menu-reload-file", () => {
        handleReloadFile();
      });
      if (!active) {
        menuReload();
        return;
      }
      unlistenMenuReload = menuReload;

      // Listen to drag & drop events on the window
      const dragDrop = await getCurrentWindow().onDragDropEvent((event) => {
        switch (event.payload.type) {
          case "enter":
            setIsDragging(true);
            break;
          case "over":
            break;
          case "drop":
            setIsDragging(false);
            const paths = event.payload.paths;
            if (paths && paths.length > 0) {
              handleFileOpen(paths[0]);
            }
            break;
          case "leave":
            setIsDragging(false);
            break;
        }
      });
      if (!active) {
        dragDrop();
        return;
      }
      unlistenDragDrop = dragDrop;
    };

    setupListeners();

    return () => {
      active = false;
      if (unlistenMenuOpen) unlistenMenuOpen();
      if (unlistenMenuClose) unlistenMenuClose();
      if (unlistenMenuReload) unlistenMenuReload();
      if (unlistenDragDrop) unlistenDragDrop();
    };
  }, []);

  return (
    <div className={`app-container ${isDragging ? "dragging" : ""}`}>
      <div className="language-switcher-global">
        <LanguageSwitcher />
      </div>

      {isDragging && (
        <div className="drag-overlay">
          <div className="drag-overlay-text">{t("drag_overlay")}</div>
        </div>
      )}

      {appState === "STARTUP" && (
        <div className="startup-screen">
          <div className="startup-content">
            <h1 className="startup-title">{t("startup_title")}</h1>
            <p className="startup-divider">{t("startup_or")}</p>
            <button
              className="startup-button"
              onClick={triggerOpenFileSelector}
            >
              {t("startup_button")}
            </button>
          </div>
        </div>
      )}

      {appState === "REVIEW" && fileData && (
        <div className="review-screen">
          <Toolbar
            onOpen={triggerOpenFileSelector}
            onReload={handleReloadFile}
            onClose={handleCloseFile}
          />
          <div className="review-layout">
            <div className="review-sidebar">
              <FileSummary
                fileName={fileName || ""}
                filePath={filePath || ""}
                fileSize={fileData.file_size}
                encoding={fileData.encoding}
                lineEnding={fileData.line_ending}
                recordCount={fileData.records.length}
              />
              <HeaderMetadata header={fileData.header} />
              <Totals totals={fileData.totals} />
            </div>
            <div className="review-main">
              <ValidationResults
                severity={fileData.validation_severity}
                messages={fileData.validation_messages}
              />
              <BookingTable records={fileData.records} />
            </div>
          </div>
        </div>
      )}

      {appState === "ERROR" && (
        <div className="error-screen">
          <div className="error-container">
            <div className="error-header">
              <span className="error-badge">{t("error_badge")}</span>
              <h1 className="error-title">{t("error_title")}</h1>
            </div>

            <div className="error-details">
              <div className="error-field">
                <span className="field-label">{t("error_field_file")}</span>
                <span className="field-value filename">{fileName}</span>
              </div>
              {filePath && (
                <div className="error-field path-field">
                  <span className="field-label">{t("error_field_path")}</span>
                  <span className="field-value path">{filePath}</span>
                </div>
              )}
              <div className="error-field">
                <span className="field-label">{t("error_field_reason")}</span>
                <span className="field-value error-message">{error?.message}</span>
              </div>

              {(error?.encoding || error?.line_ending) && (
                <div className="error-meta-grid">
                  {error.encoding && (
                    <div className="meta-item">
                      <span className="meta-label">{t("error_detected_encoding")}</span>
                      <span className="meta-value">{error.encoding}</span>
                    </div>
                  )}
                  {error.line_ending && (
                    <div className="meta-item">
                      <span className="meta-label">{t("error_line_endings")}</span>
                      <span className="meta-value">{error.line_ending}</span>
                    </div>
                  )}
                </div>
              )}

              {error?.offending_line !== undefined && error?.offending_line !== null && (
                <div className="error-code-block">
                  <div className="code-block-header">
                    <span>
                      {error.line_number !== null
                        ? t("error_problematic_line_number", { line: error.line_number })
                        : t("error_problematic_line")}
                    </span>
                  </div>
                  <pre className="code-content">
                    <code>
                      {error.line_number !== null && (
                        <span className="code-line-number">{error.line_number}: </span>
                      )}
                      {error.offending_line}
                    </code>
                  </pre>
                </div>
              )}
            </div>

            <div className="error-actions">
              <button className="primary-btn" onClick={triggerOpenFileSelector}>
                {t("error_open_another")}
              </button>
              <button className="secondary-btn" onClick={handleCloseFile}>
                {t("error_back_home")}
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

export default App;
