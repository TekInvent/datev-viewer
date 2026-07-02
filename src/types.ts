export interface DatevHeader {
  format_identifier: string;
  version: string;
  consultant_number: string;
  client_number: string;
  accounting_period: string;
  fiscal_year: string | null;
  export_date: string | null;
  unknown_fields: Record<string, string>;
}

export interface BookingRecord {
  row_number: number;
  amount: string;
  debit_credit: string;
  date: string;
  account: string;
  contra_account: string;
  tax_key: string;
  document_reference: string;
  booking_text: string;
  raw_fields: string[];
}

export type ValidationSeverity = 'Valid' | 'Warning' | 'Invalid';

export interface ValidationMessage {
  severity: ValidationSeverity;
  row_number: number | null;
  column: string | null;
  description: string;
  offending_value: string | null;
}

export interface Totals {
  booking_count: number;
  total_amount: number;
  debit_total: number | null;
  credit_total: number | null;
  warning_count: number;
  error_count: number;
}

export interface DatevFile {
  header: DatevHeader;
  records: BookingRecord[];
  validation_messages: ValidationMessage[];
  validation_severity: ValidationSeverity;
  totals: Totals;
  encoding: string;
  line_ending: string;
  file_size: number;
}

export interface OpenFileError {
  message: string;
  encoding: string | null;
  line_ending: string | null;
  offending_line: string | null;
  line_number: number | null;
}
