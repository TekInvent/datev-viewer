pub mod calculator;
pub mod models;
pub mod parser;
pub mod reader;
pub mod validator;

use crate::models::{DatevFile, ValidationSeverity};
use crate::reader::DecodedFile;

/// Processes a decoded DATEV file by parsing, validating, and calculating totals.
pub fn process_decoded_file(decoded: &DecodedFile) -> Result<DatevFile, parser::ParserError> {
    let (header, records) = parser::parse_datev_csv(&decoded.content)?;
    let validation_messages = validator::validate_file(decoded, &header, &records);

    let mut validation_severity = ValidationSeverity::Valid;
    for msg in &validation_messages {
        if msg.severity == ValidationSeverity::Invalid {
            validation_severity = ValidationSeverity::Invalid;
            break;
        } else if msg.severity == ValidationSeverity::Warning
            && validation_severity == ValidationSeverity::Valid
        {
            validation_severity = ValidationSeverity::Warning;
        }
    }

    let totals = calculator::calculate_totals(&records, &validation_messages);

    Ok(DatevFile {
        header,
        records,
        validation_messages,
        validation_severity,
        totals,
        encoding: decoded.encoding.to_string(),
        line_ending: decoded.line_ending.to_string(),
        file_size: decoded.file_size,
    })
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_process_decoded_file_valid() {
        let csv_data = concat!(
            "\"EXTF\";700;21;\"Buchungsstapel\";13;20240130140440;;\"RE\";;;29098;55003;20240101;4;20240101;20240831;\"Buchungsstapel\";\"WD\";1;\"EUR\";;;;;;;;;\r\n",
            "\"Umsatz\";\"Soll/Haben\";\"Belegdatum\";\"Konto\";\"Gegenkonto\";\"BU-Schlüssel\";\"Belegfeld 1\";\"Buchungstext\"\r\n",
            "\"123,45\";\"S\";\"3001\";\"1200\";\"1600\";\"\";\"RE-100\";\"Test Booking 1\"\r\n",
            "\"67,89\";\"H\";\"3101\";\"1400\";\"8400\";\"3\";\"RE-101\";\"Test Booking 2\"\r\n"
        );
        let decoded = DecodedFile {
            content: csv_data.to_string(),
            encoding: crate::reader::EncodingType::Utf8,
            line_ending: crate::reader::LineEnding::CrLf,
            file_size: csv_data.len() as u64,
        };
        let file_result = process_decoded_file(&decoded);
        assert!(file_result.is_ok());
        let datev_file = file_result.unwrap();

        assert_eq!(datev_file.header.format_identifier, "EXTF");
        assert_eq!(datev_file.records.len(), 2);
        assert_eq!(datev_file.validation_severity, ValidationSeverity::Valid);
        assert!(datev_file.validation_messages.is_empty());
        assert_eq!(datev_file.totals.booking_count, 2);
        assert_eq!(datev_file.totals.total_amount, 191.34);
    }
}
