import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
  type ReactNode,
} from "react";

export type Locale = "en" | "de";

export interface Translations {
  toolbar_open: string;
  toolbar_reload: string;
  toolbar_close_file: string;
  file_summary_title: string;
  file_summary_filename: string;
  file_summary_path: string;
  file_summary_size: string;
  file_summary_encoding: string;
  file_summary_line_endings: string;
  file_summary_record_count: string;
  file_summary_parser_status: string;
  file_summary_parsed: string;
  header_metadata_title: string;
  header_format_identifier: string;
  header_format_version: string;
  header_consultant_number: string;
  header_client_number: string;
  header_accounting_period: string;
  header_fiscal_year: string;
  header_export_date: string;
  header_unknown_fields: string;
  header_field_number: string;
  totals_title: string;
  totals_booking_count: string;
  totals_total_amount: string;
  totals_debit_total: string;
  totals_credit_total: string;
  totals_warnings: string;
  totals_errors: string;
  startup_title: string;
  startup_or: string;
  startup_button: string;
  drag_overlay: string;
  error_badge: string;
  error_title: string;
  error_field_file: string;
  error_field_path: string;
  error_field_reason: string;
  error_detected_encoding: string;
  error_line_endings: string;
  error_problematic_line: string;
  error_problematic_line_number: string;
  error_open_another: string;
  error_back_home: string;
  validation_status_prefix: string;
  validation_valid_label: string;
  validation_valid_text: string;
  validation_warning_label: string;
  validation_warning_text: string;
  validation_invalid_label: string;
  validation_invalid_text: string;
  validation_errors_one: string;
  validation_errors_other: string;
  validation_warnings_one: string;
  validation_warnings_other: string;
  validation_log_title: string;
  validation_row: string;
  validation_col: string;
  validation_offending_label: string;
  table_col_row: string;
  table_col_amount: string;
  table_col_dc: string;
  table_col_date: string;
  table_col_account: string;
  table_col_contra_account: string;
  table_col_tax_key: string;
  table_col_doc_ref: string;
  table_col_booking_text: string;
  table_search_placeholder: string;
  table_rows_selected_one: string;
  table_rows_selected_other: string;
  table_cell_selected: string;
  table_copy: string;
  table_clear_selection: string;
  table_copied_toast: string;
  table_empty_state: string;
}

export const translations: Record<Locale, Translations> = {
  en: {
    toolbar_open: "Open",
    toolbar_reload: "Reload",
    toolbar_close_file: "Close File",
    file_summary_title: "File Summary",
    file_summary_filename: "Filename",
    file_summary_path: "Absolute Path",
    file_summary_size: "Size",
    file_summary_encoding: "Encoding",
    file_summary_line_endings: "Line Endings",
    file_summary_record_count: "Record Count",
    file_summary_parser_status: "Parser Status",
    file_summary_parsed: "Parsed",
    header_metadata_title: "Header Metadata",
    header_format_identifier: "Format Identifier",
    header_format_version: "Format Version",
    header_consultant_number: "Consultant Number",
    header_client_number: "Client Number",
    header_accounting_period: "Accounting Period",
    header_fiscal_year: "Fiscal Year",
    header_export_date: "Export Date",
    header_unknown_fields: "Unknown Fields",
    header_field_number: "Field {field}",
    totals_title: "Totals & Statistics",
    totals_booking_count: "Booking Count",
    totals_total_amount: "Total Amount",
    totals_debit_total: "Debit Total",
    totals_credit_total: "Credit Total",
    totals_warnings: "Warnings",
    totals_errors: "Errors",
    startup_title: "Drag and drop a DATEV file here",
    startup_or: "or",
    startup_button: "File → Open DATEV File…",
    drag_overlay: "Drop the DATEV file here",
    error_badge: "Error",
    error_title: "Failed to Open DATEV File",
    error_field_file: "File:",
    error_field_path: "Path:",
    error_field_reason: "Reason:",
    error_detected_encoding: "Detected Encoding",
    error_line_endings: "Line Endings",
    error_problematic_line: "Problematic Line",
    error_problematic_line_number: "Problematic Line ({line})",
    error_open_another: "Open Another File…",
    error_back_home: "Back to Home",
    validation_status_prefix: "Status:",
    validation_valid_label: "Valid",
    validation_valid_text:
      "No issues detected. The file is ready for import.",
    validation_warning_label: "Warning",
    validation_warning_text:
      "The file is structurally valid but contains warnings or suspicious values.",
    validation_invalid_label: "Invalid",
    validation_invalid_text:
      "The file contains structural errors and cannot be imported safely.",
    validation_errors_one: "{count} Error",
    validation_errors_other: "{count} Errors",
    validation_warnings_one: "{count} Warning",
    validation_warnings_other: "{count} Warnings",
    validation_log_title: "Detailed Validation Log",
    validation_row: "Row",
    validation_col: "Col",
    validation_offending_label: "Offending value:",
    table_col_row: "Row",
    table_col_amount: "Amount",
    table_col_dc: "D/C",
    table_col_date: "Date",
    table_col_account: "Account",
    table_col_contra_account: "Contra Account",
    table_col_tax_key: "Tax Key",
    table_col_doc_ref: "Doc Ref",
    table_col_booking_text: "Booking Text",
    table_search_placeholder: "Search bookings…",
    table_rows_selected_one: "{count} row selected",
    table_rows_selected_other: "{count} rows selected",
    table_cell_selected: "Cell selected",
    table_copy: "Copy",
    table_clear_selection: "Clear Selection",
    table_copied_toast: "Copied to clipboard!",
    table_empty_state: "No matching bookings found.",
  },
  de: {
    toolbar_open: "Öffnen",
    toolbar_reload: "Neu laden",
    toolbar_close_file: "Datei schließen",
    file_summary_title: "Dateiübersicht",
    file_summary_filename: "Dateiname",
    file_summary_path: "Absoluter Pfad",
    file_summary_size: "Größe",
    file_summary_encoding: "Kodierung",
    file_summary_line_endings: "Zeilenenden",
    file_summary_record_count: "Datensätze",
    file_summary_parser_status: "Parser-Status",
    file_summary_parsed: "Geparst",
    header_metadata_title: "Header-Metadaten",
    header_format_identifier: "Format-Kennung",
    header_format_version: "Format-Version",
    header_consultant_number: "Beraternummer",
    header_client_number: "Mandantennummer",
    header_accounting_period: "Buchungszeitraum",
    header_fiscal_year: "Wirtschaftsjahr",
    header_export_date: "Exportdatum",
    header_unknown_fields: "Unbekannte Felder",
    header_field_number: "Feld {field}",
    totals_title: "Summen & Statistik",
    totals_booking_count: "Buchungsanzahl",
    totals_total_amount: "Gesamtbetrag",
    totals_debit_total: "Soll-Summe",
    totals_credit_total: "Haben-Summe",
    totals_warnings: "Warnungen",
    totals_errors: "Fehler",
    startup_title: "DATEV-Datei hier ablegen",
    startup_or: "oder",
    startup_button: "Datei → DATEV-Datei öffnen…",
    drag_overlay: "DATEV-Datei hier ablegen",
    error_badge: "Fehler",
    error_title: "DATEV-Datei konnte nicht geöffnet werden",
    error_field_file: "Datei:",
    error_field_path: "Pfad:",
    error_field_reason: "Ursache:",
    error_detected_encoding: "Erkannte Kodierung",
    error_line_endings: "Zeilenenden",
    error_problematic_line: "Problematische Zeile",
    error_problematic_line_number: "Problematische Zeile ({line})",
    error_open_another: "Andere Datei öffnen…",
    error_back_home: "Zurück zur Startseite",
    validation_status_prefix: "Status:",
    validation_valid_label: "Gültig",
    validation_valid_text:
      "Keine Probleme gefunden. Die Datei ist importbereit.",
    validation_warning_label: "Warnung",
    validation_warning_text:
      "Die Datei ist strukturell gültig, enthält aber Warnungen oder verdächtige Werte.",
    validation_invalid_label: "Ungültig",
    validation_invalid_text:
      "Die Datei enthält strukturelle Fehler und kann nicht sicher importiert werden.",
    validation_errors_one: "{count} Fehler",
    validation_errors_other: "{count} Fehler",
    validation_warnings_one: "{count} Warnung",
    validation_warnings_other: "{count} Warnungen",
    validation_log_title: "Detailliertes Validierungsprotokoll",
    validation_row: "Zeile",
    validation_col: "Sp.",
    validation_offending_label: "Fehlerhafter Wert:",
    table_col_row: "Zeile",
    table_col_amount: "Betrag",
    table_col_dc: "S/H",
    table_col_date: "Datum",
    table_col_account: "Konto",
    table_col_contra_account: "Gegenkonto",
    table_col_tax_key: "Steuerschlüssel",
    table_col_doc_ref: "Belegnummer",
    table_col_booking_text: "Buchungstext",
    table_search_placeholder: "Buchungen suchen…",
    table_rows_selected_one: "{count} Zeile ausgewählt",
    table_rows_selected_other: "{count} Zeilen ausgewählt",
    table_cell_selected: "Zelle ausgewählt",
    table_copy: "Kopieren",
    table_clear_selection: "Auswahl aufheben",
    table_copied_toast: "In Zwischenablage kopiert!",
    table_empty_state: "Keine übereinstimmenden Buchungen gefunden.",
  },
};

const LOCALE_STORAGE_KEY = "locale";

function isLocale(value: string | null): value is Locale {
  return value === "en" || value === "de";
}

function readStoredLocale(): Locale {
  try {
    const stored = localStorage.getItem(LOCALE_STORAGE_KEY);
    return isLocale(stored) ? stored : "en";
  } catch {
    return "en";
  }
}

function interpolate(
  template: string,
  params?: Record<string, string | number>,
): string {
  if (!params) {
    return template;
  }

  return template.replace(/\{(\w+)\}/g, (match, name: string) => {
    const value = params[name];
    return value === undefined ? match : String(value);
  });
}

type TranslationKey = keyof Translations;
type CountBaseKey =
  | "validation_errors"
  | "validation_warnings"
  | "table_rows_selected";

export interface LocaleContextValue {
  locale: Locale;
  setLocale: (locale: Locale) => void;
  t: (key: TranslationKey, params?: Record<string, string | number>) => string;
  tCount: (
    baseKey: CountBaseKey,
    count: number,
    params?: Record<string, string | number>,
  ) => string;
  formatNumber: (value: number | null | undefined) => string;
}

export const LocaleContext = createContext<LocaleContextValue | null>(null);

export function LocaleProvider({ children }: { children: ReactNode }) {
  const [locale, setLocaleState] = useState<Locale>(readStoredLocale);

  const setLocale = useCallback((next: Locale) => {
    setLocaleState(next);
    try {
      localStorage.setItem(LOCALE_STORAGE_KEY, next);
    } catch {
      // Ignore storage failures; locale still updates in memory.
    }
  }, []);

  useEffect(() => {
    document.documentElement.lang = locale;
  }, [locale]);

  const activeTranslations = translations[locale];

  const t = useCallback(
    (key: TranslationKey, params?: Record<string, string | number>) =>
      interpolate(activeTranslations[key], params),
    [activeTranslations],
  );

  const tCount = useCallback(
    (
      baseKey: CountBaseKey,
      count: number,
      params?: Record<string, string | number>,
    ) => {
      const suffix = count === 1 ? "_one" : "_other";
      const key = `${baseKey}${suffix}` as TranslationKey;
      return interpolate(activeTranslations[key], { ...params, count });
    },
    [activeTranslations],
  );

  const formatNumber = useCallback(
    (value: number | null | undefined) => {
      if (value === null || value === undefined) {
        return "-";
      }

      const bcp47 = locale === "de" ? "de-DE" : "en-US";
      return new Intl.NumberFormat(bcp47, {
        minimumFractionDigits: 2,
        maximumFractionDigits: 2,
      }).format(value);
    },
    [locale],
  );

  const value = useMemo(
    () => ({ locale, setLocale, t, tCount, formatNumber }),
    [locale, setLocale, t, tCount, formatNumber],
  );

  return (
    <LocaleContext.Provider value={value}>{children}</LocaleContext.Provider>
  );
}

export function useLocale(): LocaleContextValue {
  const context = useContext(LocaleContext);
  if (!context) {
    throw new Error("useLocale must be used within a LocaleProvider");
  }
  return context;
}

export function LanguageSwitcher() {
  const { locale, setLocale } = useLocale();

  return (
    <select
      className="language-switcher"
      value={locale}
      onChange={(event) => setLocale(event.target.value as Locale)}
      aria-label="Language"
    >
      <option value="en">English</option>
      <option value="de">Deutsch</option>
    </select>
  );
}
