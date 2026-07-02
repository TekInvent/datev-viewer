use crate::models::{BookingRecord, DatevHeader};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a structural error during parsing of a DATEV CSV file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ParserError {
    pub message: String,
    pub offending_line: Option<String>,
    pub line_number: Option<usize>,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ParserError {}

/// Parses a decoded DATEV EXTF CSV string into a `DatevHeader` and a list of `BookingRecord`s.
///
/// This parser is designed to be robust and not panic on malformed inputs.
pub fn parse_datev_csv(
    csv_content: &str,
) -> Result<(DatevHeader, Vec<BookingRecord>), ParserError> {
    if csv_content.trim().is_empty() {
        return Err(ParserError {
            message: "The file is empty.".to_string(),
            offending_line: None,
            line_number: None,
        });
    }

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .flexible(true)
        .from_reader(csv_content.as_bytes());

    let mut records = Vec::new();
    for (i, result) in rdr.records().enumerate() {
        match result {
            Ok(record) => {
                records.push(record);
            }
            Err(e) => {
                let line_num = e.position().map(|pos| pos.line() as usize).unwrap_or(i + 1);
                let offending_line = get_line_by_number(csv_content, line_num);
                return Err(ParserError {
                    message: format!("CSV parsing error: {e}"),
                    offending_line,
                    line_number: Some(line_num),
                });
            }
        }
    }

    if records.is_empty() {
        return Err(ParserError {
            message: "The file is empty.".to_string(),
            offending_line: None,
            line_number: None,
        });
    }

    // Parse Row 1: Header Metadata
    let first_row = &records[0];
    let format_identifier = first_row.get(0).unwrap_or("").to_string();
    if format_identifier != "EXTF" {
        return Err(ParserError {
            message: format!(
                "Invalid DATEV header: expected 'EXTF' in the first field, found '{format_identifier}'",
            ),
            offending_line: get_line_by_number(csv_content, 1),
            line_number: Some(1),
        });
    }

    let fields: Vec<String> = first_row.iter().map(|s| s.to_string()).collect();
    let version = fields.get(1).cloned().unwrap_or_default();
    let export_date = fields.get(5).cloned().filter(|s| !s.is_empty());
    let consultant_number = fields.get(10).cloned().unwrap_or_default();
    let client_number = fields.get(11).cloned().unwrap_or_default();
    let fiscal_year_start = fields.get(12).cloned().filter(|s| !s.is_empty());
    let date_from = fields.get(14).cloned().unwrap_or_default();
    let date_to = fields.get(15).cloned().unwrap_or_default();

    let fiscal_year = fiscal_year_start.as_ref().map(|s| {
        if s.len() >= 4 {
            s[..4].to_string()
        } else {
            s.clone()
        }
    });

    let accounting_period = if !date_from.is_empty() && !date_to.is_empty() {
        format!("{date_from} - {date_to}")
    } else if !date_from.is_empty() {
        date_from.clone()
    } else {
        date_to.clone()
    };

    let mut unknown_fields = HashMap::new();
    for (i, field) in fields.iter().enumerate() {
        if i != 0 && i != 1 && i != 5 && i != 10 && i != 11 && i != 12 && i != 14 && i != 15 {
            unknown_fields.insert(format!("field_{}", i + 1), field.clone());
        }
    }

    let header = DatevHeader {
        format_identifier,
        version,
        consultant_number,
        client_number,
        accounting_period,
        fiscal_year,
        export_date,
        unknown_fields,
    };

    // Check Row 2: Column Headers
    if records.len() < 2 {
        return Err(ParserError {
            message: "Missing field definition header row (line 2).".to_string(),
            offending_line: None,
            line_number: None,
        });
    }

    let header_row = &records[1];
    let column_headers: Vec<String> = header_row.iter().map(|s| s.to_string()).collect();

    let mut amount_idx = None;
    let mut debit_credit_idx = None;
    let mut date_idx = None;
    let mut account_idx = None;
    let mut contra_account_idx = None;
    let mut tax_key_idx = None;
    let mut doc_ref_idx = None;
    let mut text_idx = None;

    for (i, header_name) in column_headers.iter().enumerate() {
        let normalized = header_name.to_lowercase();
        // Match amount column: prefer exact DATEV field name, then fall back to
        // generic keywords. Exclude the Soll/Haben field which also contains
        // "umsatz" in some variants (e.g. "Umsatz (Soll/Haben-Kennzeichen)").
        if normalized == "umsatz (ohne soll/haben-kz)"
            || normalized == "umsatz (ohne soll/haben-kennzeichen)"
            || (amount_idx.is_none()
                && (normalized.contains("betrag") || normalized.contains("amount"))
                && !normalized.contains("soll/haben"))
            || (amount_idx.is_none()
                && normalized.contains("umsatz")
                && !normalized.contains("soll/haben"))
        {
            amount_idx = Some(i);
        } else if normalized.contains("soll/haben")
            || normalized == "s/h"
            || normalized.contains("s/h-kennzeichen")
            || normalized.contains("debit/credit")
            || normalized == "d/c"
        {
            debit_credit_idx = Some(i);
        } else if normalized == "belegdatum"
            || normalized == "datum"
            || normalized == "beleg-datum"
            || normalized.contains("date")
        {
            date_idx = Some(i);
        } else if normalized == "gegenkonto"
            || normalized.contains("gegenkonto (ohne bu-schlüssel)")
            || normalized.contains("gegenkonto (ohne bu-schluessel)")
            || normalized.contains("contra account")
            || normalized.contains("contra-account")
        {
            contra_account_idx = Some(i);
        } else if normalized == "konto"
            || normalized == "kontonummer"
            || normalized == "konto-nr"
            || normalized.contains("account") && !normalized.contains("contra")
        {
            account_idx = Some(i);
        } else if normalized.contains("bu-schlüssel")
            || normalized.contains("bu-schluessel")
            || normalized.contains("steuerschlüssel")
            || normalized == "bu"
            || normalized.contains("tax key")
            || normalized.contains("tax-key")
        {
            tax_key_idx = Some(i);
        } else if normalized.contains("belegfeld 1")
            || normalized.contains("belegfeld1")
            || normalized.contains("beleg-feld 1")
            || normalized == "rechnungsnummer"
            || normalized.contains("doc ref")
            || normalized.contains("document reference")
        {
            doc_ref_idx = Some(i);
        } else if normalized.contains("buchungstext")
            || normalized == "text"
            || normalized.contains("booking text")
        {
            text_idx = Some(i);
        }
    }

    let mut booking_records = Vec::new();
    for (idx, r) in records.iter().enumerate().skip(2) {
        let raw_fields: Vec<String> = r.iter().map(|s| s.to_string()).collect();

        let amount = amount_idx
            .and_then(|i| raw_fields.get(i).cloned())
            .unwrap_or_default();
        let debit_credit = debit_credit_idx
            .and_then(|i| raw_fields.get(i).cloned())
            .unwrap_or_default();
        let date = date_idx
            .and_then(|i| raw_fields.get(i).cloned())
            .unwrap_or_default();
        let account = account_idx
            .and_then(|i| raw_fields.get(i).cloned())
            .unwrap_or_default();
        let contra_account = contra_account_idx
            .and_then(|i| raw_fields.get(i).cloned())
            .unwrap_or_default();
        let tax_key = tax_key_idx
            .and_then(|i| raw_fields.get(i).cloned())
            .unwrap_or_default();
        let document_reference = doc_ref_idx
            .and_then(|i| raw_fields.get(i).cloned())
            .unwrap_or_default();
        let booking_text = text_idx
            .and_then(|i| raw_fields.get(i).cloned())
            .unwrap_or_default();

        booking_records.push(BookingRecord {
            row_number: idx + 1,
            amount,
            debit_credit,
            date,
            account,
            contra_account,
            tax_key,
            document_reference,
            booking_text,
            raw_fields,
        });
    }

    Ok((header, booking_records))
}

fn get_line_by_number(content: &str, line_idx: usize) -> Option<String> {
    content
        .lines()
        .nth(line_idx.saturating_sub(1))
        .map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_datev_csv() {
        let csv_data = concat!(
            "\"EXTF\";700;21;\"Buchungsstapel\";13;20240130140440439;;\"RE\";;;29098;55003;20240101;4;20240101;20240831;\"Buchungsstapel\";\"WD\";1;\"EUR\";;;;;;;;;\r\n",
            "\"Umsatz\";\"Soll/Haben\";\"Belegdatum\";\"Konto\";\"Gegenkonto\";\"BU-Schlüssel\";\"Belegfeld 1\";\"Buchungstext\"\r\n",
            "\"123,45\";\"S\";\"3001\";\"1200\";\"1600\";\"\";\"RE-100\";\"Test Booking 1\"\r\n",
            "\"67,89\";\"H\";\"3101\";\"1400\";\"8400\";\"3\";\"RE-101\";\"Test Booking 2\"\r\n"
        );

        let result = parse_datev_csv(csv_data);
        assert!(result.is_ok());
        let (header, records) = result.unwrap();

        assert_eq!(header.format_identifier, "EXTF");
        assert_eq!(header.version, "700");
        assert_eq!(header.consultant_number, "29098");
        assert_eq!(header.client_number, "55003");
        assert_eq!(header.fiscal_year, Some("2024".to_string()));
        assert_eq!(header.export_date, Some("20240130140440439".to_string()));
        assert_eq!(header.accounting_period, "20240101 - 20240831");

        assert_eq!(records.len(), 2);

        // Record 1
        assert_eq!(records[0].row_number, 3);
        assert_eq!(records[0].amount, "123,45");
        assert_eq!(records[0].debit_credit, "S");
        assert_eq!(records[0].date, "3001");
        assert_eq!(records[0].account, "1200");
        assert_eq!(records[0].contra_account, "1600");
        assert_eq!(records[0].tax_key, "");
        assert_eq!(records[0].document_reference, "RE-100");
        assert_eq!(records[0].booking_text, "Test Booking 1");

        // Record 2
        assert_eq!(records[1].row_number, 4);
        assert_eq!(records[1].amount, "67,89");
        assert_eq!(records[1].debit_credit, "H");
        assert_eq!(records[1].date, "3101");
        assert_eq!(records[1].account, "1400");
        assert_eq!(records[1].contra_account, "8400");
        assert_eq!(records[1].tax_key, "3");
        assert_eq!(records[1].document_reference, "RE-101");
        assert_eq!(records[1].booking_text, "Test Booking 2");
    }

    #[test]
    fn test_parse_empty_content() {
        let result = parse_datev_csv("");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.message, "The file is empty.");
        assert_eq!(err.offending_line, None);
        assert_eq!(err.line_number, None);
    }

    #[test]
    fn test_parse_invalid_identifier() {
        let csv_data = "\"NOTEXTF\";700;21\n\"Umsatz\"\n";
        let result = parse_datev_csv(csv_data);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("Invalid DATEV header"));
        assert_eq!(err.line_number, Some(1));
        assert_eq!(err.offending_line, Some("\"NOTEXTF\";700;21".to_string()));
    }

    #[test]
    fn test_parse_missing_header_row() {
        let csv_data = "\"EXTF\";700;21\n";
        let result = parse_datev_csv(csv_data);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.message, "Missing field definition header row (line 2).");
        assert_eq!(err.line_number, None);
    }

    #[test]
    fn test_parse_english_headers_and_betrag_suffix() {
        let csv_data = concat!(
            "\"EXTF\";700;21;\"Buchungsstapel\";13;20240130140440439;;\"RE\";;;29098;55003;20240101;4;20240101;20240831;\"Buchungsstapel\";\"WD\";1;\"EUR\";;;;;;;;;\r\n",
            "\"Betrag mit Suffix\";\"Debit/Credit\";\"Date\";\"Account\";\"Contra Account\";\"Tax Key\";\"Doc Ref\";\"Booking Text\"\r\n",
            "\"123,45\";\"S\";\"3001\";\"1200\";\"1600\";\"\";\"RE-100\";\"Test Booking 1\"\r\n"
        );

        let result = parse_datev_csv(csv_data);
        assert!(result.is_ok());
        let (_, records) = result.unwrap();

        assert_eq!(records.len(), 1);
        assert_eq!(records[0].amount, "123,45");
        assert_eq!(records[0].debit_credit, "S");
        assert_eq!(records[0].date, "3001");
        assert_eq!(records[0].account, "1200");
        assert_eq!(records[0].contra_account, "1600");
        assert_eq!(records[0].tax_key, "");
        assert_eq!(records[0].document_reference, "RE-100");
        assert_eq!(records[0].booking_text, "Test Booking 1");
    }
}
