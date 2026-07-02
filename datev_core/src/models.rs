use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the parsed DATEV header metadata.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DatevHeader {
    pub format_identifier: String,
    pub version: String,
    pub consultant_number: String,
    pub client_number: String,
    pub accounting_period: String,
    pub fiscal_year: Option<String>,
    pub export_date: Option<String>,
    pub unknown_fields: HashMap<String, String>,
}

/// Represents a single booking record row in the DATEV file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BookingRecord {
    pub row_number: usize,
    pub amount: String,
    pub debit_credit: String,
    pub date: String,
    pub account: String,
    pub contra_account: String,
    pub tax_key: String,
    pub document_reference: String,
    pub booking_text: String,
    pub raw_fields: Vec<String>,
}

/// Severity of a validation message or the overall file validation state.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValidationSeverity {
    Valid,
    Warning,
    Invalid,
}

/// A specific validation message detailing an error or warning in the file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValidationMessage {
    pub severity: ValidationSeverity,
    pub row_number: Option<usize>,
    pub column: Option<String>,
    pub description: String,
    pub offending_value: Option<String>,
}

/// Aggregated totals and statistics for the DATEV file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Totals {
    pub booking_count: usize,
    pub total_amount: f64,
    pub debit_total: Option<f64>,
    pub credit_total: Option<f64>,
    pub warning_count: usize,
    pub error_count: usize,
}

/// The root model aggregating all parsed contents, validation messages, and calculations.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DatevFile {
    pub header: DatevHeader,
    pub records: Vec<BookingRecord>,
    pub validation_messages: Vec<ValidationMessage>,
    pub validation_severity: ValidationSeverity,
    pub totals: Totals,
    pub encoding: String,
    pub line_ending: String,
    pub file_size: u64,
}
