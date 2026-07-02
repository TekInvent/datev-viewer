use crate::models::{BookingRecord, DatevHeader, ValidationMessage, ValidationSeverity};
use crate::reader::{DecodedFile, EncodingType};

/// Performs structural, header, and record validation on the DATEV file contents.
pub fn validate_file(
    decoded: &DecodedFile,
    header: &DatevHeader,
    records: &[BookingRecord],
) -> Vec<ValidationMessage> {
    let mut messages = Vec::new();

    // 1. File Validation
    validate_file_structure(decoded, records, &mut messages);

    // 2. Header Validation
    validate_header(header, &mut messages);

    // 3. Record Validation
    validate_records(records, header, &mut messages);

    messages
}

fn add_msg(
    messages: &mut Vec<ValidationMessage>,
    severity: ValidationSeverity,
    row_number: Option<usize>,
    column: Option<&str>,
    description: &str,
    offending_value: Option<&str>,
) {
    messages.push(ValidationMessage {
        severity,
        row_number,
        column: column.map(|s| s.to_string()),
        description: description.to_string(),
        offending_value: offending_value.map(|s| s.to_string()),
    });
}

fn validate_file_structure(
    decoded: &DecodedFile,
    records: &[BookingRecord],
    messages: &mut Vec<ValidationMessage>,
) {
    // Check empty
    if decoded.content.trim().is_empty() {
        add_msg(
            messages,
            ValidationSeverity::Invalid,
            None,
            None,
            "The file is empty.",
            None,
        );
        return;
    }

    // Check supported encoding (UTF-8 or Windows-1252)
    // Since our reader only decodes to these two, it's always technically supported,
    // but we can log an informational/valid check, or warning if it falls back.
    // Nothing special required here as the prompt just says "supported encoding".
    // We will just verify the encoding is Utf8 or Windows1252.
    match decoded.encoding {
        EncodingType::Utf8 | EncodingType::Windows1252 => {}
    }

    // Check consistent line endings
    let has_crlf = decoded.content.contains("\r\n");
    // Count \n not preceded by \r
    let mut has_lf_only = false;
    let bytes = decoded.content.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b'\n' && (i == 0 || bytes[i - 1] != b'\r') {
            has_lf_only = true;
            break;
        }
    }

    if has_crlf && has_lf_only {
        add_msg(
            messages,
            ValidationSeverity::Warning,
            None,
            None,
            "Inconsistent line endings: the file contains a mix of CRLF and LF.",
            None,
        );
    }

    // Check column count consistency from the parsed CSV header
    if let Some(expected_cols) = get_expected_column_count(&decoded.content) {
        for record in records {
            if record.raw_fields.len() != expected_cols {
                add_msg(
                    messages,
                    ValidationSeverity::Invalid,
                    Some(record.row_number),
                    None,
                    &format!(
                        "Inconsistent column count: expected {} columns, found {}.",
                        expected_cols,
                        record.raw_fields.len()
                    ),
                    Some(&format!("Fields: {:?}", record.raw_fields)),
                );
            }
        }
    }
}

fn get_expected_column_count(csv_content: &str) -> Option<usize> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .flexible(true)
        .from_reader(csv_content.as_bytes());

    let mut records = rdr.records();
    // Skip row 1 (metadata)
    let _ = records.next()?;
    // Row 2 is column headers
    let headers = records.next()?.ok()?;
    Some(headers.len())
}

fn validate_header(header: &DatevHeader, messages: &mut Vec<ValidationMessage>) {
    // 1. Mandatory header fields
    if header.format_identifier.is_empty() {
        add_msg(
            messages,
            ValidationSeverity::Invalid,
            Some(1),
            Some("Format Identifier"),
            "Mandatory header field 'Format Identifier' is missing.",
            None,
        );
    } else if header.format_identifier != "EXTF" {
        add_msg(
            messages,
            ValidationSeverity::Invalid,
            Some(1),
            Some("Format Identifier"),
            &format!(
                "Format identifier must be 'EXTF', found '{}'.",
                header.format_identifier
            ),
            Some(&header.format_identifier),
        );
    }

    if header.version.is_empty() {
        add_msg(
            messages,
            ValidationSeverity::Invalid,
            Some(1),
            Some("Version"),
            "Mandatory header field 'Version' is missing.",
            None,
        );
    } else {
        // Supported version (typically 700 or 510)
        if header.version != "700" && header.version != "510" {
            add_msg(
                messages,
                ValidationSeverity::Warning,
                Some(1),
                Some("Version"),
                &format!(
                    "Unsupported DATEV version '{}'. Only '700' and '510' are fully supported.",
                    header.version
                ),
                Some(&header.version),
            );
        }
    }

    if header.consultant_number.is_empty() {
        add_msg(
            messages,
            ValidationSeverity::Invalid,
            Some(1),
            Some("Consultant Number"),
            "Mandatory header field 'Consultant Number' is missing.",
            None,
        );
    } else {
        // Format check
        if !header.consultant_number.chars().all(|c| c.is_ascii_digit()) {
            add_msg(
                messages,
                ValidationSeverity::Invalid,
                Some(1),
                Some("Consultant Number"),
                "Consultant number must be numeric.",
                Some(&header.consultant_number),
            );
        } else {
            let len = header.consultant_number.len();
            if !(1..=7).contains(&len) {
                add_msg(
                    messages,
                    ValidationSeverity::Warning,
                    Some(1),
                    Some("Consultant Number"),
                    "Consultant number length should be between 1 and 7 digits.",
                    Some(&header.consultant_number),
                );
            }
            if let Ok(val) = header.consultant_number.parse::<u64>() {
                if val == 0 {
                    add_msg(
                        messages,
                        ValidationSeverity::Warning,
                        Some(1),
                        Some("Consultant Number"),
                        "Consultant number should not be 0.",
                        Some(&header.consultant_number),
                    );
                }
            }
        }
    }

    if header.client_number.is_empty() {
        add_msg(
            messages,
            ValidationSeverity::Invalid,
            Some(1),
            Some("Client Number"),
            "Mandatory header field 'Client Number' is missing.",
            None,
        );
    } else {
        // Format check
        if !header.client_number.chars().all(|c| c.is_ascii_digit()) {
            add_msg(
                messages,
                ValidationSeverity::Invalid,
                Some(1),
                Some("Client Number"),
                "Client number must be numeric.",
                Some(&header.client_number),
            );
        } else {
            let len = header.client_number.len();
            if !(1..=5).contains(&len) {
                add_msg(
                    messages,
                    ValidationSeverity::Warning,
                    Some(1),
                    Some("Client Number"),
                    "Client number length should be between 1 and 5 digits.",
                    Some(&header.client_number),
                );
            }
            if let Ok(val) = header.client_number.parse::<u64>() {
                if val == 0 {
                    add_msg(
                        messages,
                        ValidationSeverity::Warning,
                        Some(1),
                        Some("Client Number"),
                        "Client number should not be 0.",
                        Some(&header.client_number),
                    );
                }
            }
        }
    }

    if header.accounting_period.is_empty() {
        add_msg(
            messages,
            ValidationSeverity::Invalid,
            Some(1),
            Some("Accounting Period"),
            "Mandatory header field 'Accounting Period' is missing.",
            None,
        );
    } else {
        // accounting period format is "YYYYMMDD - YYYYMMDD" or "YYYYMMDD"
        // Let's validate the dates inside it.
        let dates: Vec<&str> = header
            .accounting_period
            .split('-')
            .map(|s| s.trim())
            .collect();
        let mut parsed_dates = Vec::new();
        for &date_str in &dates {
            if !date_str.is_empty() {
                match parse_yyyymmdd(date_str) {
                    Some((y, m, d)) => {
                        if !is_valid_date(y, m, d) {
                            add_msg(
                                messages,
                                ValidationSeverity::Warning,
                                Some(1),
                                Some("Accounting Period"),
                                &format!(
                                    "Invalid calendar date '{date_str}' in accounting period."
                                ),
                                Some(date_str),
                            );
                        }
                        parsed_dates.push((y, m, d));
                    }
                    None => {
                        add_msg(
                            messages,
                            ValidationSeverity::Warning,
                            Some(1),
                            Some("Accounting Period"),
                            &format!("Invalid date format '{date_str}' in accounting period. Expected YYYYMMDD."),
                            Some(date_str),
                        );
                    }
                }
            }
        }

        if parsed_dates.len() == 2 {
            let start = parsed_dates[0];
            let end = parsed_dates[1];
            if start > end {
                add_msg(
                    messages,
                    ValidationSeverity::Warning,
                    Some(1),
                    Some("Accounting Period"),
                    "Accounting period start date must be before or equal to end date.",
                    Some(&header.accounting_period),
                );
            }
        }
    }

    // Fiscal year validation
    if let Some(ref fy) = header.fiscal_year {
        if !fy.chars().all(|c| c.is_ascii_digit()) || fy.len() != 4 {
            add_msg(
                messages,
                ValidationSeverity::Warning,
                Some(1),
                Some("Fiscal Year"),
                "Fiscal year must be a 4-digit number.",
                Some(fy),
            );
        }
    }

    // Export date validation
    if let Some(ref ed) = header.export_date {
        // Standard format is YYYYMMDDHHMMSS or YYYYMMDDHHMMSSFFF
        if !ed.chars().all(|c| c.is_ascii_digit()) || (ed.len() != 14 && ed.len() != 17) {
            add_msg(
                messages,
                ValidationSeverity::Warning,
                Some(1),
                Some("Export Date"),
                "Export date format must be YYYYMMDDHHMMSS[FFF].",
                Some(ed),
            );
        } else {
            let y_str = &ed[0..4];
            let m_str = &ed[4..6];
            let d_str = &ed[6..8];
            let hh_str = &ed[8..10];
            let mm_str = &ed[10..12];
            let ss_str = &ed[12..14];

            let y = y_str.parse::<u32>().unwrap_or(0);
            let m = m_str.parse::<u32>().unwrap_or(0);
            let d = d_str.parse::<u32>().unwrap_or(0);
            let hh = hh_str.parse::<u32>().unwrap_or(99);
            let mm = mm_str.parse::<u32>().unwrap_or(99);
            let ss = ss_str.parse::<u32>().unwrap_or(99);

            if !is_valid_date(y, m, d) || hh > 23 || mm > 59 || ss > 59 {
                add_msg(
                    messages,
                    ValidationSeverity::Warning,
                    Some(1),
                    Some("Export Date"),
                    "Export date has invalid date or time values.",
                    Some(ed),
                );
            }
        }
    }
}

fn validate_records(
    records: &[BookingRecord],
    header: &DatevHeader,
    messages: &mut Vec<ValidationMessage>,
) {
    let fy_year = header
        .fiscal_year
        .as_ref()
        .and_then(|s| s.parse::<u32>().ok());

    for record in records {
        let row = record.row_number;

        // 1. Mandatory record fields
        if record.amount.is_empty() {
            add_msg(
                messages,
                ValidationSeverity::Invalid,
                Some(row),
                Some("Umsatz"),
                "Mandatory field 'Amount' (Umsatz) is missing.",
                None,
            );
        } else {
            // Amount format check
            let clean_amount = record.amount.replace(',', ".");
            if clean_amount.parse::<f64>().is_err() {
                add_msg(
                    messages,
                    ValidationSeverity::Invalid,
                    Some(row),
                    Some("Umsatz"),
                    &format!(
                        "Invalid amount format '{}'. Must be a valid decimal number.",
                        record.amount
                    ),
                    Some(&record.amount),
                );
            }
        }

        if record.debit_credit.is_empty() {
            add_msg(
                messages,
                ValidationSeverity::Invalid,
                Some(row),
                Some("Soll/Haben"),
                "Mandatory field 'Debit/Credit' (Soll/Haben) is missing.",
                None,
            );
        } else if record.debit_credit != "S" && record.debit_credit != "H" {
            add_msg(
                messages,
                ValidationSeverity::Invalid,
                Some(row),
                Some("Soll/Haben"),
                &format!(
                    "Invalid Debit/Credit indicator '{}'. Must be 'S' or 'H'.",
                    record.debit_credit
                ),
                Some(&record.debit_credit),
            );
        }

        if record.date.is_empty() {
            add_msg(
                messages,
                ValidationSeverity::Invalid,
                Some(row),
                Some("Belegdatum"),
                "Mandatory field 'Date' (Belegdatum) is missing.",
                None,
            );
        } else {
            // Date format check
            // Supports DDMM (4 digits), DMM (3 digits), YYYYMMDD (8 digits), DDMMYYYY (8 digits)
            validate_record_date(record.date.as_str(), row, fy_year, messages);
        }

        if record.account.is_empty() {
            add_msg(
                messages,
                ValidationSeverity::Invalid,
                Some(row),
                Some("Konto"),
                "Mandatory field 'Account' (Konto) is missing.",
                None,
            );
        } else {
            // Account fields
            if !record.account.chars().all(|c| c.is_ascii_digit()) {
                add_msg(
                    messages,
                    ValidationSeverity::Invalid,
                    Some(row),
                    Some("Konto"),
                    "Account number must be numeric.",
                    Some(&record.account),
                );
            } else {
                let len = record.account.len();
                if !(4..=8).contains(&len) {
                    add_msg(
                        messages,
                        ValidationSeverity::Warning,
                        Some(row),
                        Some("Konto"),
                        "Account number length should be between 4 and 8 digits.",
                        Some(&record.account),
                    );
                }
            }
        }

        // Contra Account fields
        if !record.contra_account.is_empty() {
            if !record.contra_account.chars().all(|c| c.is_ascii_digit()) {
                add_msg(
                    messages,
                    ValidationSeverity::Invalid,
                    Some(row),
                    Some("Gegenkonto"),
                    "Contra account number must be numeric.",
                    Some(&record.contra_account),
                );
            } else {
                let len = record.contra_account.len();
                if !(4..=8).contains(&len) {
                    add_msg(
                        messages,
                        ValidationSeverity::Warning,
                        Some(row),
                        Some("Gegenkonto"),
                        "Contra account number length should be between 4 and 8 digits.",
                        Some(&record.contra_account),
                    );
                }
            }
        }

        // Tax Key format
        if !record.tax_key.is_empty() {
            if !record.tax_key.chars().all(|c| c.is_ascii_digit()) {
                add_msg(
                    messages,
                    ValidationSeverity::Invalid,
                    Some(row),
                    Some("BU-Schlüssel"),
                    "Tax key (BU-Schlüssel) must be numeric.",
                    Some(&record.tax_key),
                );
            } else {
                let len = record.tax_key.len();
                if !(1..=2).contains(&len) {
                    add_msg(
                        messages,
                        ValidationSeverity::Warning,
                        Some(row),
                        Some("BU-Schlüssel"),
                        "Tax key (BU-Schlüssel) length should be 1 or 2 digits.",
                        Some(&record.tax_key),
                    );
                }
            }
        }
    }
}

fn validate_record_date(
    date_str: &str,
    row: usize,
    fy_year: Option<u32>,
    messages: &mut Vec<ValidationMessage>,
) {
    if !date_str.chars().all(|c| c.is_ascii_digit()) {
        add_msg(
            messages,
            ValidationSeverity::Invalid,
            Some(row),
            Some("Belegdatum"),
            "Date must contain only digits.",
            Some(date_str),
        );
        return;
    }

    let len = date_str.len();
    if len == 3 || len == 4 {
        // DDMM or DMM
        let padded = if len == 3 {
            format!("0{date_str}")
        } else {
            date_str.to_string()
        };

        let day = padded[0..2].parse::<u32>().unwrap_or(0);
        let month = padded[2..4].parse::<u32>().unwrap_or(0);
        let year = fy_year.unwrap_or(2024); // default to a leap year for checking February if not defined

        if !is_valid_date(year, month, day) {
            add_msg(
                messages,
                ValidationSeverity::Invalid,
                Some(row),
                Some("Belegdatum"),
                &format!("Invalid calendar date '{date_str}' (DDMM/DMM format)."),
                Some(date_str),
            );
        }
    } else if len == 8 {
        // YYYYMMDD or DDMMYYYY
        // Standard DATEV EXTF is YYYYMMDD
        let mut parsed = false;

        // Try YYYYMMDD
        let y1 = date_str[0..4].parse::<u32>().unwrap_or(0);
        let m1 = date_str[4..6].parse::<u32>().unwrap_or(0);
        let d1 = date_str[6..8].parse::<u32>().unwrap_or(0);
        if is_valid_date(y1, m1, d1) {
            parsed = true;
        }

        // Try DDMMYYYY if YYYYMMDD failed
        if !parsed {
            let d2 = date_str[0..2].parse::<u32>().unwrap_or(0);
            let m2 = date_str[2..4].parse::<u32>().unwrap_or(0);
            let y2 = date_str[4..8].parse::<u32>().unwrap_or(0);
            if is_valid_date(y2, m2, d2) {
                parsed = true;
            }
        }

        if !parsed {
            add_msg(
                messages,
                ValidationSeverity::Invalid,
                Some(row),
                Some("Belegdatum"),
                &format!("Invalid calendar date '{date_str}' (8-digit format)."),
                Some(date_str),
            );
        }
    } else {
        add_msg(
            messages,
            ValidationSeverity::Invalid,
            Some(row),
            Some("Belegdatum"),
            &format!("Invalid date length ({len} digits). Expected DDMM or YYYYMMDD."),
            Some(date_str),
        );
    }
}

fn parse_yyyymmdd(s: &str) -> Option<(u32, u32, u32)> {
    if s.len() == 8 && s.chars().all(|c| c.is_ascii_digit()) {
        let y = s[0..4].parse::<u32>().ok()?;
        let m = s[4..6].parse::<u32>().ok()?;
        let d = s[6..8].parse::<u32>().ok()?;
        Some((y, m, d))
    } else {
        None
    }
}

fn is_valid_date(year: u32, month: u32, day: u32) -> bool {
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return false;
    }
    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
            if is_leap {
                29
            } else {
                28
            }
        }
        _ => return false,
    };
    day <= days_in_month
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::LineEnding;
    use std::collections::HashMap;

    fn mock_decoded() -> DecodedFile {
        let content = "\"EXTF\";700;21;\"Buchungsstapel\";13;20240130140440;;\"RE\";;;29098;55003;20240101;4;20240101;20240831;\"Buchungsstapel\";\"WD\";1;\"EUR\";;;;;;;;;\r\n\"Umsatz\";\"Soll/Haben\";\"Belegdatum\";\"Konto\";\"Gegenkonto\";\"BU-Schlüssel\";\"Belegfeld 1\";\"Buchungstext\"\r\n\"123,45\";\"S\";\"3001\";\"1200\";\"1600\";\"\";\"RE-100\";\"Test Booking 1\"\r\n".to_string();
        let file_size = content.len() as u64;
        DecodedFile {
            content,
            encoding: EncodingType::Utf8,
            line_ending: LineEnding::CrLf,
            file_size,
        }
    }

    fn mock_header() -> DatevHeader {
        DatevHeader {
            format_identifier: "EXTF".to_string(),
            version: "700".to_string(),
            consultant_number: "29098".to_string(),
            client_number: "55003".to_string(),
            accounting_period: "20240101 - 20240831".to_string(),
            fiscal_year: Some("2024".to_string()),
            export_date: Some("20240130140440".to_string()),
            unknown_fields: HashMap::new(),
        }
    }

    fn mock_records() -> Vec<BookingRecord> {
        vec![BookingRecord {
            row_number: 3,
            amount: "123,45".to_string(),
            debit_credit: "S".to_string(),
            date: "3001".to_string(),
            account: "1200".to_string(),
            contra_account: "1600".to_string(),
            tax_key: "".to_string(),
            document_reference: "RE-100".to_string(),
            booking_text: "Test Booking 1".to_string(),
            raw_fields: vec![
                "123,45".to_string(),
                "S".to_string(),
                "3001".to_string(),
                "1200".to_string(),
                "1600".to_string(),
                "".to_string(),
                "RE-100".to_string(),
                "Test Booking 1".to_string(),
            ],
        }]
    }

    #[test]
    fn test_validate_valid_inputs() {
        let decoded = mock_decoded();
        let header = mock_header();
        let records = mock_records();
        let messages = validate_file(&decoded, &header, &records);
        assert!(
            messages.is_empty(),
            "Expected no messages, got: {:?}",
            messages
        );
    }

    #[test]
    fn test_validate_empty_file() {
        let decoded = DecodedFile {
            content: "".to_string(),
            encoding: EncodingType::Utf8,
            line_ending: LineEnding::Lf,
            file_size: 0,
        };
        let header = mock_header();
        let records = Vec::new();
        let messages = validate_file(&decoded, &header, &records);
        assert!(!messages.is_empty());
        assert_eq!(messages[0].severity, ValidationSeverity::Invalid);
        assert!(messages[0].description.contains("empty"));
    }

    #[test]
    fn test_validate_inconsistent_line_endings() {
        let mut decoded = mock_decoded();
        decoded.content = "\"EXTF\";700\r\n\"Umsatz\"\n".to_string();
        let header = mock_header();
        let records = Vec::new();
        let messages = validate_file(&decoded, &header, &records);
        let has_warning = messages.iter().any(|m| {
            m.severity == ValidationSeverity::Warning && m.description.contains("line endings")
        });
        assert!(has_warning, "Expected line endings warning");
    }

    #[test]
    fn test_validate_inconsistent_column_count() {
        let decoded = mock_decoded();
        let header = mock_header();
        let mut records = mock_records();
        // modify record to have mismatch column count
        records[0].raw_fields.pop();
        let messages = validate_file(&decoded, &header, &records);
        let has_column_err = messages.iter().any(|m| {
            m.severity == ValidationSeverity::Invalid && m.description.contains("column count")
        });
        assert!(has_column_err, "Expected column count error");
    }

    #[test]
    fn test_validate_invalid_header() {
        let decoded = mock_decoded();
        let mut header = mock_header();
        header.format_identifier = "NOTEXTF".to_string();
        header.version = "800".to_string(); // unsupported version
        header.consultant_number = "ABC".to_string(); // non-numeric
        header.client_number = "123456".to_string(); // too long

        let messages = validate_file(&decoded, &header, &Vec::new());

        let format_err = messages.iter().any(|m| {
            m.severity == ValidationSeverity::Invalid
                && m.description.contains("Format identifier must be 'EXTF'")
        });
        let version_warn = messages.iter().any(|m| {
            m.severity == ValidationSeverity::Warning
                && m.description.contains("Unsupported DATEV version")
        });
        let consultant_err = messages.iter().any(|m| {
            m.severity == ValidationSeverity::Invalid
                && m.description.contains("Consultant number must be numeric")
        });
        let client_warn = messages.iter().any(|m| {
            m.severity == ValidationSeverity::Warning
                && m.description
                    .contains("Client number length should be between")
        });

        assert!(format_err);
        assert!(version_warn);
        assert!(consultant_err);
        assert!(client_warn);
    }

    #[test]
    fn test_validate_header_date_errors() {
        let decoded = mock_decoded();
        let mut header = mock_header();
        header.accounting_period = "20240230 - 20240101".to_string(); // 30 Feb is invalid, and start > end
        header.export_date = Some("20241301000000".to_string()); // month 13 is invalid

        let messages = validate_file(&decoded, &header, &Vec::new());

        let ap_err_1 = messages
            .iter()
            .any(|m| m.description.contains("Invalid calendar date"));
        let ap_err_2 = messages
            .iter()
            .any(|m| m.description.contains("start date must be before or equal"));
        let ed_err = messages.iter().any(|m| {
            m.description
                .contains("Export date has invalid date or time")
        });

        assert!(ap_err_1);
        assert!(ap_err_2);
        assert!(ed_err);
    }

    #[test]
    fn test_validate_record_field_errors() {
        let decoded = mock_decoded();
        let header = mock_header();
        let records = vec![BookingRecord {
            row_number: 3,
            amount: "123a,45".to_string(),       // bad format
            debit_credit: "X".to_string(),       // bad S/H
            date: "3201".to_string(),            // 32nd Jan is invalid
            account: "123".to_string(),          // too short
            contra_account: "1600a".to_string(), // non-numeric
            tax_key: "999".to_string(),          // too long
            document_reference: "".to_string(),
            booking_text: "".to_string(),
            raw_fields: vec![
                "123a,45".to_string(),
                "X".to_string(),
                "3201".to_string(),
                "123".to_string(),
                "1600a".to_string(),
                "999".to_string(),
                "".to_string(),
                "".to_string(),
            ],
        }];

        let messages = validate_file(&decoded, &header, &records);

        let amount_err = messages
            .iter()
            .any(|m| m.description.contains("Invalid amount format"));
        let dc_err = messages
            .iter()
            .any(|m| m.description.contains("Invalid Debit/Credit indicator"));
        let date_err = messages
            .iter()
            .any(|m| m.description.contains("Invalid calendar date"));
        let account_warn = messages.iter().any(|m| {
            m.severity == ValidationSeverity::Warning
                && m.description
                    .contains("Account number length should be between")
        });
        let contra_err = messages.iter().any(|m| {
            m.severity == ValidationSeverity::Invalid
                && m.description
                    .contains("Contra account number must be numeric")
        });
        let tax_key_warn = messages.iter().any(|m| {
            m.severity == ValidationSeverity::Warning
                && m.description
                    .contains("Tax key (BU-Schlüssel) length should be")
        });

        assert!(amount_err, "Expected amount error");
        assert!(dc_err, "Expected D/C error");
        assert!(date_err, "Expected date error");
        assert!(account_warn, "Expected account length warning");
        assert!(contra_err, "Expected contra account non-numeric error");
        assert!(tax_key_warn, "Expected tax key length warning");
    }
}
